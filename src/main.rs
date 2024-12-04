use std::{collections::HashMap, sync::Arc};
use opentelemetry::{
    global,
    metrics::{Gauge, Meter},
    KeyValue,
};
use prometheus::Registry;
use rocket::{
    get, routes, State,
};
use smartzone::Auth;
use tokio::sync::RwLock;

mod smartzone;
mod ap;

struct Meters {
    counters: HashMap<String, Gauge<i64>>,
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
        counters: HashMap::new(),
        meter,
    };

    let _ = rocket::build()
        .mount("/", routes![metrics])
        .manage(registry)
        .manage(RwLock::new(dd))
        .manage(auth.clone())
        .launch()
        .await;

    auth.logout().await;
    meter_provider.shutdown().unwrap();
}

// "/metrics" Is where prometheus expects to gather metrics at.
// If you change this make sure your prometheus config reflects the change.
#[get("/metrics")]
async fn metrics(
    state: &State<Registry>,
    auth: &State<Arc<Auth>>,
    meters: &State<RwLock<Meters>>,
) -> String {
    let mut buffer = String::new();
    let encoder = prometheus::TextEncoder::new();

    // Go thru all the zones
    for zone in &auth.get_zones().await {
        // Get all the APs in the zone
        let aps = auth.get_aps_in_zone(zone).await;
        let full_aps = auth
        // TODO query aps will NOT return all the aps in a zone, it has the `hasMore` flag for a reason
            .query_aps(
                aps.iter()
                    .map(smartzone::Filter::from)
                    .collect::<smartzone::FilterContainer>(),
            )
            .await;

        // Set metrics for each ap
        for ap in &full_aps.list {
            
            let data = vec![
                    KeyValue::new("DeviceName", ap.device_name.clone()),
                    KeyValue::new("MAC", ap.ap_mac.clone()),
                    KeyValue::new("IP", ap.ip.clone()),
                    KeyValue::new("Status", ap.status.clone()),
                    KeyValue::new("Zone", zone.name.clone()),
                    KeyValue::new("LastSeen", ap.last_seen.to_string()),
                    KeyValue::new("Model", ap.model.to_string()),
                ];

            let lock = meters.write().await;
            let meter = &lock.meter;

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
