/**
 * Contains the core logic got processing data, such as filtering JSON objects or adding geolocation information.
 * It provides functions to transform and filter data according to the configuration settings.
 */
use std::net::IpAddr;

use serde_json::Value;

use crate::geoip::GeoIP;
use crate::config::Event;

pub fn process(json: Value, geoip: Option<&GeoIP>, event_config: &Event) -> Value {
    let mut json = json;

    // TODO remove risk in ignore
    json = remove_risks(json, &event_config.ignore_risks);
    // TODO remove attributes
    json = remove_attributes(json, &event_config.ignore_fields);
    // TODO remove event types
    json = remove_event_types(json, &event_config.flow_event_name);
    // TODO get geoip of string (if boolean is set)
    if let Some(geoip) = geoip {
        json = add_geoip_info(json, geoip);
    }

    // return processed string
    json   
}

fn remove_risks(mut json: Value, ignore_risks: &[String]) -> Value {
    if let Some(obj) = json.as_object_mut() {
        for risk in ignore_risks {
            obj.remove(risk);
        }
    }
    json
}

fn remove_attributes(mut json: Value, ignore_fields: &[String]) -> Value {
    if let Some(obj) = json.as_object_mut() {
        for field in ignore_fields {
            obj.remove(field);
        }
    }
    json
}

fn remove_event_types(mut json: Value, ignore_event_types: &[String]) -> Value {
    if let Some(obj) = json.as_object_mut() {
        for event_type in ignore_event_types {
            obj.remove(event_type);
        }
    }
    json
}

fn add_geoip_info(mut json: Value, geoip: &GeoIP) -> Value {
    if let Some(obj) = json.as_object_mut() {
        if let Some(ip_str) = obj.get("ip").and_then(|ip| ip.as_str()) {
            if let Ok(ip) = ip_str.parse::<IpAddr>() {
                if let Ok(city) = geoip.get_geoip(&ip) {
                    obj.insert("geoip".to_string(), serde_json::json!(city));
                }
            }
        }
    }
    json
}

