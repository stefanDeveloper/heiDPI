pub mod cli;
pub mod logging;
pub mod stream;
pub mod process;
pub mod geoip;
pub mod config;

use log::{error, info};
use cli::Cli;
use std::process::exit;
use geoip::GeoIP;

/// Catches errors, prints them through the logger, then exits
#[tokio::main]
pub async fn main() {
    //default to displaying warning and error log messages only
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("trace")).init();

    match Cli::run().await {
        Ok(_) => {
            info!("Application exited successfully.");
        }
        Err(e) => {
            error!("{e}");
            exit(1);
        }
    }
}