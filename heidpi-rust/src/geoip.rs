use std::net::IpAddr;
use anyhow::Ok;
use maxminddb::Reader;

/// .
// TODO create geoip2 object, be aware multiple threads will read this object, check Tokio how to do it! (Mutex...)
struct GeoIP {
    
}


impl GeoIP {
    pub fn new()-> anyhow::Result<()> {
        Ok(())
    }
    pub fn get_geoip(ip: &IpAddr) -> anyhow::Result<()> {
        let reader = maxminddb::Reader::open_readfile("test-data/test-data/GeoIP2-City-Test.mmdb").unwrap();
        Ok(())
    }
}