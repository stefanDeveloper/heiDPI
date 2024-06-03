/**
 * Manages the network aspects, including establishing connections and reading data from sockets.
 */
use log::{info, trace, warn};
use serde_json::Value;
use std::io::{self};
use std::str;
use std::time::Duration;
use std::{thread, time};
use tokio::net::TcpStream;

use tokio::sync::mpsc;
use tokio::task;

use crate::process::process;
use crate::geoip::GeoIP;
use crate::logging::Logging;

const NETWORK_BUFFER_LENGTH_DIGITS: usize = 5;
const NETWORK_BUFFER_MAX_SIZE: usize = 33792;
const EOL: &str = "\n";

#[derive(Debug)]
pub struct HeiDPITcpstream {
    event_type: HeiDPIEventType,
    data: Value,
}

#[derive(Debug)]
pub enum HeiDPIEventType {
    PACKET,
    FLOW,
    DAEMON,
    ERROR,
}

pub async fn connect(
    connection: &str,
    geoip: Option<GeoIP>,
    flow_logger: Logging,
    daemon_logger: Logging,
    packet_logger: Logging,
    error_logger: Logging,
) -> anyhow::Result<()> {
    loop {
        match std::net::TcpStream::connect(connection) {
            Err(_e) => {
                warn!("Could not connect to Server");

                // We don't want to hammer the server with reconnection attempts, so we wait for 5 seconds.
                let five_seconds = time::Duration::from_millis(5000);
                thread::sleep(five_seconds);

                continue;
            }
            Ok(std_stream) => {
                info!("Connected");

                match std_stream.set_nonblocking(true) {
                    Ok(..) => info!("Non-blocking State"),
                    Err(..) => panic!("Non-blocking State Failed"),
                };

                match std_stream.set_read_timeout(Some(Duration::new(60, 0))) {
                    Ok(..) => info!("Set Read Timeout"),
                    Err(..) => panic!("Setting Read Timeout Failed"),
                };

                let stream = TcpStream::from_std(std_stream)?;
                let (tx, mut rx) = mpsc::channel::<(Value, Logging)>(100);
                let mut buf = vec![0u8; NETWORK_BUFFER_MAX_SIZE];

                loop {
                    match stream.try_read(&mut buf) {
                        Ok(data) => {
                            trace!("read {} bytes", data);
                            match std::str::from_utf8(&buf[..data]) {
                                Ok(json_str) => {
                                    for s_plit_n in json_str.split(EOL).into_iter() {
                                        if s_plit_n.len() > NETWORK_BUFFER_LENGTH_DIGITS {
                                            let v: Value = match serde_json::from_str(
                                                &s_plit_n[NETWORK_BUFFER_LENGTH_DIGITS..],
                                            ) {
                                                Ok(json) => {
                                                    trace!("Converted result: {}", json);
                                                    json
                                                }
                                                Err(_e) => {
                                                    warn!("Invalid JSON object: '{}'.", _e);
                                                    serde_json::Value::Null
                                                }
                                            };
                                            // TODO Multithreading?
                                            // TODO Call processing (geoip2, remove risk, remove attributes, ignore event types, ...) and save to file
                                            if let Some(event_type) = v.get("event_type").and_then(|e| e.as_str()) {
                                                let logger = match event_type {
                                                    "flow" => flow_logger.clone(),
                                                    "daemon" => daemon_logger.clone(),
                                                    "packet" => packet_logger.clone(),
                                                    "error" => error_logger.clone(),
                                                    _ => continue,
                                                };

                                                if let Err(e) = tx.send((v, logger)).await {
                                                    warn!("Failed to send data for processing: {}", e);
                                                }
                                            //process(v);
                                            }
                                            
                                        }
                                    }
                                }
                                Err(_) => {
                                    warn!("BUG: Invalid UTF-8 in buffer");
                                }
                            }
                        }
                        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                            continue;
                        }
                        Err(..) => {
                            let five_seconds = time::Duration::from_millis(10);
                            thread::sleep(five_seconds);

                            continue;
                        }
                    }
                }
            }
        }
    }
}
