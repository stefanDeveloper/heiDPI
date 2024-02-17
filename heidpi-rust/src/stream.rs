use log::{info, trace, warn};
use serde_json::Value;
use std::io::{self, Read};
// use std::net::TcpStream;
use std::str;
use std::time::Duration;
use std::{thread, time};
use tokio::io::Interest;
use tokio::net::TcpStream;

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

pub async fn connect(connection: &str) -> anyhow::Result<()> {
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
                let mut buf = vec![0u8; NETWORK_BUFFER_MAX_SIZE];
                let ready = stream
                    .ready(Interest::READABLE | Interest::WRITABLE)
                    .await?;
                if ready.is_readable() {
                    loop {
                        match stream.try_read(&mut buf) {
                            Ok(data) => {
                                println!("read {} bytes", data);
                                // match std::str::from_utf8(&buf[..data]) {
                                //     Ok(json_str) => {
                                //         let s = match str::from_utf8(json_str.as_bytes()) {
                                //             Ok(v) => v,
                                //             Err(e) => {
                                //                 warn!("Invalid UTF-8 sequence: '{}'.", e);
                                //                 break;
                                //             }
                                //         };
                                //         for s_plit_n in s.split(EOL).into_iter() {
                                //             if s_plit_n.len() > NETWORK_BUFFER_LENGTH_DIGITS {
                                //                 // TODO Call processing and save to file
                                //                 tokio::spawn(async move {
                                //                     // process(socket).await;

                                //                     let v: Value = match serde_json::from_str(
                                //                         &s_plit_n[NETWORK_BUFFER_LENGTH_DIGITS..],
                                //                     ) {
                                //                         Ok(json) => {
                                //                             trace!("Converted result: {}", json);
                                //                             json
                                //                         }
                                //                         Err(_e) => {
                                //                             warn!(
                                //                         "Invalid JSON object: '{}' for string: {}",
                                //                         _e, s
                                //                     );
                                //                             serde_json::Value::Null
                                //                         }
                                //                     };
                                //                 });
                                //             };
                                //         }
                                //     }
                                //     Err(_) => {
                                //         warn!("BUG: Invalid UTF-8 in buffer");
                                //     }
                                // };
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
}
