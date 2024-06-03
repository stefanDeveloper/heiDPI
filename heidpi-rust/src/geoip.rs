/**
 * Implements functionality to read from a GeoIP database and fetch geographical information based on IP addresses.
 */

use std::net::IpAddr;
use anyhow::{Result, Ok};
use maxminddb::{geoip2, Reader}; // Reader for the MaxMind DB format. Reading the contents of the database files
use serde::{Deserialize, Serialize};

use std::sync::{Arc, Mutex}; // For Mutex

/// .
// TODO create geoip2 object, be aware multiple threads will read this object, check Tokio how to do it! (Mutex...)
pub struct GeoIP{
    reader: Arc<Mutex<Reader<Vec<u8>>>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CityInfo {
    pub city_name: Option<String>,
    pub country_name: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}


impl GeoIP {

    // Create a new GeoIP object
    pub fn new(db_path: &str)-> anyhow::Result<Self> {
        let db_reader = Reader::open_readfile(db_path)?;
        Ok(Self{
            reader: Arc::new(Mutex::new(db_reader)),
        }

        )
    }

    // Get geolocation information for a given IP address
    pub fn get_geoip(&self, ip: &IpAddr) -> anyhow::Result<CityInfo> {
        //let reader = maxminddb::Reader::open_readfile("test-data/test-data/GeoIP2-City-Test.mmdb").unwrap();
        let reader = self.reader.lock().unwrap();
        let city: geoip2::City = reader.lookup(*ip)?;

        // Extract and copy the relevant data to CityInfo
        let city_name = city.city.as_ref()
            .and_then(|city| city.names.as_ref().and_then(|names| names.get("en").map(|s| s.to_string())));
        let country_name = city.country.as_ref()
            .and_then(|country| country.names.as_ref().and_then(|names| names.get("en").map(|s| s.to_string())));
        let location = city.location;
        let latitude = location.as_ref().and_then(|loc| loc.latitude);
        let longitude = location.as_ref().and_then(|loc| loc.longitude);

        Ok(CityInfo {
            city_name,
            country_name,
            latitude,
            longitude,
        })   
    }
}