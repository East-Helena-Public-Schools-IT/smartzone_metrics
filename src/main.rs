use std::{collections::HashMap, sync::Arc};
use opentelemetry::{
    global,
    metrics::{Meter, UpDownCounter},
    KeyValue,
};
use prometheus::Registry;
use rocket::{
    get, routes, State,
};
use smartzone::Auth;
use tokio::sync::RwLock;

mod smartzone;

struct Meters {
    counters: HashMap<String, UpDownCounter<i64>>,
    meter: Meter,
}

#[rocket::main]
async fn main() {
    let (meter_provider, registry) = init_meter_provider();
    // Create a meter from the above MeterProvider.
    let meter = global::meter("smartzone");
    // Create a Counter Instrument.
    let mut session = smartzone::Auth::new(
        std::env::var("RUST_USERNAME").expect("Set RUST_USERNAME"),
        std::env::var("RUST_PASSWORD").expect("Set RUST_PASSWORD"),
    );
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
            .query_aps(
                aps.iter()
                    .map(|f| smartzone::Filter::from(f))
                    .collect::<smartzone::FilterContainer>(),
            )
            .await;

        // Set metrics for each ap
        for ap in &full_aps.list {
            let key = "AP".to_string()
                + &ap
                    .ap_mac
                    .as_bytes()
                    .iter()
                    .map(|b| *b as char)
                    .filter(|c| *c != ':')
                    .collect::<String>();

            let mut lock = meters.write().await;

            // Look for the gauge, if it already exists just add more data to it
            let counter = if let Some(x) = lock.counters.get(&key) {
                x
            } else {
                // Otherwise create a new gauge and add it to the pool.
                let meter = &lock.meter;
                let x = meter.i64_up_down_counter(key.clone()).init();
                lock.counters.insert(key.to_string(), x);
                lock.counters
                    .get(&key)
                    .expect("How on earth is it not in the hashmap?")
            };

            // AFAIK it mutates an internal state
            counter.add(
                ap.alerts,
                &[
                    KeyValue::new("DeviceName", ap.device_name.clone()),
                    KeyValue::new("IP", ap.ip.clone()),
                    KeyValue::new("TotalClients", ap.num_clients.to_string()),
                    KeyValue::new("TX", ap.tx.to_string()),
                    KeyValue::new("RX", ap.rx.to_string()),
                    KeyValue::new("Status", ap.status.to_string()),
                ],
            );
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
