use crate::geoip::get_geoip;
use std::net::IpAddr;

pub fn process(json: &IpAddr) {
    get_geoip("");
}