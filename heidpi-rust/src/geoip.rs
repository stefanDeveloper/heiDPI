use std::net::IpAddr;
use maxminddb::Reader;

/// .
struct GeoIP {
    
}

impl GeoIP {
    pub fn get_geoip(ip: &IpAddr) -> anyhow::Result<()> {
        let reader = maxminddb::Reader::open_readfile("test-data/test-data/GeoIP2-City-Test.mmdb").unwrap();
        Ok(())
    }
}