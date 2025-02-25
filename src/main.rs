use std::sync::Arc;
use opentelemetry::{
    global,
    metrics::Meter,
    KeyValue,
};
use prometheus::Registry;
use rocket::{
    get, routes, State,
};
use smartzone::{Auth, FilterContainer, Query};
use tokio::sync::RwLock;

mod smartzone;
mod ap;
mod client;

struct Meters {
    meter: Meter,
}

#[rocket::main]
async fn main() {
    let (meter_provider, registry) = init_meter_provider();
    // Create a meter from the above MeterProvider.
    let meter = global::meter("smartzone");
    // Create a Counter Instrument.
    let mut session = smartzone::Auth::new(
        dotenvy::var("RUST_USERNAME").expect("Set RUST_USERNAME"),
        dotenvy::var("RUST_PASSWORD").expect("Set RUST_PASSWORD"),
    ).await;
    session.login().await;
    let auth = Arc::new(session);

    let dd = Meters {
        meter,
    };

    let _ = rocket::build()
        .mount("/", routes![metrics, clients])
        .manage(registry)
        .manage(RwLock::new(dd))
        .manage(auth.clone())
        .launch()
        .await;

    auth.logout().await;
    meter_provider.shutdown().unwrap();
}

#[get("/clients")]
async fn clients(
    state: &State<Registry>,
    auth: &State<Arc<Auth>>,
    meters: &State<RwLock<Meters>>,
) -> String {
    // Go thru all the zones
    for zone in &auth.get_zones().await {
        // Get all the APs in the zone
        let mut filter: FilterContainer = zone.into();
        let mut left = true;
        let mut all = Vec::new();
        while left {
            let mut clients = auth.query::<client::Client>(filter.clone(), Query::Clients).await;
            left = clients.has_more;
            filter.page += 1;
            all.append(&mut clients.list);
        }
        for client in all {
            let data = vec![
                KeyValue::new("ApMac", client.ap_mac.clone()),
                KeyValue::new("ApName", client.ap_name.clone()),
                KeyValue::new("Hostname", client.hostname.clone()),
                KeyValue::new("Mac", client.client_mac.clone()),
                KeyValue::new("Zone", zone.name.clone()),
            ];

            let mut data_verbose = vec![
                KeyValue::new("OsType", client.os_type.clone()),
                KeyValue::new("OsVendorType", client.os_vendor_type.clone()),
                KeyValue::new("IP", client.ip_address.clone()),
                KeyValue::new("Vlan", client.vlan.to_string()),
                KeyValue::new("ModelName", client.model_name.clone()),
                KeyValue::new("SSID", client.ssid.clone()),
                KeyValue::new("SessionStartTime", client.session_start_time.to_string()),
            ];
            data_verbose.append(&mut data.clone());

            let lock = meters.write().await;
            let meter = &lock.meter;

            // rx
            let g = meter.u64_gauge("client_rx").with_description("rx bytes").init();
            g.record(client.rx_bytes, &data_verbose);

            // tx
            let g = meter.u64_gauge("client_tx").with_description("tx bytes").init();
            g.record(client.tx_bytes, &data_verbose);

        }
    }
    let mut buffer = String::new();
    let encoder = prometheus::TextEncoder::new();
    let metric_families = state.gather();
    encoder.encode_utf8(&metric_families, &mut buffer).unwrap();
    buffer
}

// "/metrics" Is where prometheus expects to gather metrics at.
// If you change this make sure your prometheus config reflects the change.
#[get("/metrics")]
async fn metrics(
    state: &State<Registry>,
    auth: &State<Arc<Auth>>,
    meters: &State<RwLock<Meters>>,
) -> String {
    // Go thru all the zones
    for zone in &auth.get_zones().await {
        // Get all the APs in the zone
        let mut filter: FilterContainer = zone.into();
        let mut left = true;
        let mut all_aps = Vec::new();
        while left {
            // let mut aps = auth.query_aps(filter.clone()).await;
            let mut aps = auth.query::<ap::AP>(filter.clone(), Query::Aps).await;
            left = aps.has_more;
            filter.page += 1;
            all_aps.append(&mut aps.list);
        }

        // Set metrics for each ap
        for ap in &all_aps{
            
            let data = vec![
                    KeyValue::new("DeviceName", ap.device_name.clone()),
                    KeyValue::new("MAC", ap.ap_mac.clone()),
                    KeyValue::new("IP", ap.ip.clone()),
                    KeyValue::new("Zone", zone.name.clone()),
                ];

            let mut data_verbose = vec![
                    KeyValue::new("Status", ap.status.clone()),
                    KeyValue::new("LastSeen", ap.last_seen.to_string()),
                    KeyValue::new("Model", ap.model.to_string()),
            ];
            data_verbose.append(&mut data.clone());

            let lock = meters.write().await;
            let meter = &lock.meter;

            // Uptime
            let tx = meter.u64_gauge("ap_uptime").with_description("AP's transmitted traffic").init();
            tx.record(ap.uptime, &data_verbose);

            // TX
            let tx = meter.u64_gauge("ap_tx").with_description("AP's transmitted traffic").init();
            tx.record(ap.tx, &data);

            // RX
            let rx = meter.u64_gauge("ap_rx").with_description("AP's received traffic").init();
            rx.record(ap.rx, &data);

            // TotalClients
            let clients = meter.u64_gauge("ap_clients").with_description("Total number of clients connected to this AP").init();
            clients.record(ap.num_clients, &data);

            // alerts
            let alerts = meter.u64_gauge("ap_alerts").with_description("Total number of alerts").init();
            alerts.record(ap.alerts, &data);

            // airtime utilization flagged
            let flags = meter.u64_gauge("ap_airtime_24g").init();
            flags.record(ap.airtime_24g, &data);
            let flags = meter.u64_gauge("ap_airtime_5g").init();
            flags.record(ap.airtime_5g, &data);

            // connection failures
            let fail = meter.f64_gauge("ap_failures").init();
            fail.record(ap.connection_failures as f64, &data);

            // latency flagged
            let flags = meter.u64_gauge("ap_latency_5g").init();
            flags.record(ap.latency50_g, &data);
            let flags = meter.u64_gauge("ap_latency_24g").init();
            flags.record(ap.latency24g, &data);

        }
    }

    let mut buffer = String::new();
    let encoder = prometheus::TextEncoder::new();
    let metric_families = state.gather();
    encoder.encode_utf8(&metric_families, &mut buffer).unwrap();
    buffer
}

fn init_meter_provider() -> (opentelemetry_sdk::metrics::SdkMeterProvider, Registry) {
    use opentelemetry_sdk::metrics::SdkMeterProvider;

    let registry = Registry::new();
    let exporter = opentelemetry_prometheus::exporter()
        .with_registry(registry.clone())
        .build()
        .unwrap();
    let provider = SdkMeterProvider::builder()
        .with_reader(exporter)
        //        .with_resource(Resource::new([KeyValue::new(
        //            "service.name",
        //            "metrics-basic-example",
        //        )]))
        .build();
    global::set_meter_provider(provider.clone());
    (provider, registry)
}
