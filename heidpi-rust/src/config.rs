use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    logging: Logging,
    flow_event: Event,
    daemon_event: Event,
    packet_event: Event,
    error_event: Event,
}

#[derive(Deserialize)]
struct Logging {
    level: String,
    encoding: String,
    format: String,
    datefmt: String,
}

#[derive(Deserialize)]
struct Event {
    ignore_fields: Array<String>,
    ignore_risks: Array<String>,
    flow_event_name: Array<String>,
    geoip: Option<GeoIP>,
    filename: String,
}

#[derive(Deserialize)]
struct GeoIP {
    enabled: Boolean,
    filepath: String,
    keys: Array<String>,
}
