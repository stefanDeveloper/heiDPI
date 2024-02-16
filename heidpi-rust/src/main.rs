use log::{info, trace, warn};
use log::{Level, Metadata, Record};
use log::{LevelFilter, SetLoggerError};
use serde_json::{error, Value};
use std::io::{self, BufRead, BufReader, Read};
use std::net::TcpStream;
use std::str;
use std::time::Duration;
use std::{thread, time};

static LOGGER: SimpleLogger = SimpleLogger;
const NETWORK_BUFFER_LENGTH_DIGITS: usize = 5;
const NETWORK_BUFFER_MAX_SIZE: usize = 33792;

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

#[tokio::main]
async fn main() {
    let _ = log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Trace));
    loop {
        match TcpStream::connect("127.0.0.1:7000") {
            Err(_e) => {
                warn!("Could not connect to Server");

                // We don't want to hammer the server with reconnection attempts, so we wait for 5 seconds.
                let five_seconds = time::Duration::from_millis(5000);
                thread::sleep(five_seconds);

                continue;
            }
            Ok(mut stream) => {
                info!("Connected");
                match stream.set_nonblocking(true) {
                    Ok(..) => info!("Non-blocking State"),
                    Err(..) => panic!("Non-blocking State Failed"),
                };
                match stream.set_read_timeout(Some(Duration::new(60, 0))) {
                    Ok(..) => info!("Set Read Timeout"),
                    Err(..) => panic!("Setting Read Timeout Failed"),
                };

                match stream.set_write_timeout(Some(Duration::new(15, 0))) {
                    Ok(..) => info!("Set Write Timeout"),
                    Err(..) => panic!("Setting Write Timeout Failed"),
                };

                let mut buf = [0u8; NETWORK_BUFFER_MAX_SIZE];

                loop {
                    trace!("Loop");

                    let mut buf_used = 0;

                    match stream.read(&mut buf[buf_used..]) {
                        Ok(data) => {
                            info!("{:?}", data);
                            info!("Buffer used: {:?}", buf_used);
                            let json_str_start = match std::str::from_utf8(&buf[buf_used..data]) {
                                Ok(json_str) => {
                                    // info!("{:?}", json_str);
                                    buf_used += data;
                                    // TODO error handling
                                    let s = match str::from_utf8(json_str.as_bytes()) {
                                        Ok(v) => v,
                                        Err(e) => {
                                            warn!("Invalid UTF-8 sequence: '{}', retrun empty string.", e);
                                            ""
                                        }
                                    };
                                    let mut s_split = s.split("\n").into_iter();

                                    for s_plit_n in s_split {
                                        if s_plit_n.len() > NETWORK_BUFFER_LENGTH_DIGITS {
                                            let v: Value = match serde_json::from_str(
                                                &s_plit_n[NETWORK_BUFFER_LENGTH_DIGITS..],
                                            ) {
                                                Ok(json) => {
                                                    info!("Converted result: {}", json);
                                                    json
                                                }
                                                Err(_e) => {
                                                    panic!(
                                                        "Invalid JSON object: '{}' for string: {}",
                                                        _e, s
                                                    );
                                                    serde_json::Value::Null
                                                }
                                            };
                                        };
                                    }
                                }
                                Err(_) => {
                                    warn!("BUG: Invalid UTF-8 in buffer");
                                }
                            };
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
