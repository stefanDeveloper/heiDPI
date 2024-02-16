use log::{info, trace, warn};
use log::{Level, Metadata, Record};
use log::{LevelFilter, SetLoggerError};
use serde_json::{error, Value};
use std::io::{self, BufRead, BufReader};
use std::net::TcpStream;
use std::str;
use std::time::Duration;
use std::{thread, time};

static LOGGER: SimpleLogger = SimpleLogger;
const NETWORK_BUFFER_LENGTH_DIGITS: usize = 5;

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

fn main() {
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
            Ok(stream) => {
                info!("Connected");

                match stream.set_read_timeout(Some(Duration::new(60, 0))) {
                    Ok(..) => info!("Set Read Timeout"),
                    Err(..) => panic!("Setting Read Timeout Failed"),
                };

                match stream.set_write_timeout(Some(Duration::new(15, 0))) {
                    Ok(..) => info!("Set Write Timeout"),
                    Err(..) => panic!("Setting Write Timeout Failed"),
                };

                loop {
                    trace!("Loop");

                    let mut packet = BufReader::new(&stream);
                    let mut xml = vec![];

                    match packet.read_until(b'\n', &mut xml) {
                        Ok(bytes) => {
                            if bytes == 0 {
                                // This will break us out back out to the first loop to recreate the connection.
                                break;
                            } else {
                                trace!("Read {:?} Bytes", bytes);
                                let s = match str::from_utf8(&xml[NETWORK_BUFFER_LENGTH_DIGITS..]) {
                                    Ok(v) => v,
                                    Err(e) => {
                                        warn!("Invalid UTF-8 sequence: '{}', retrun empty string.", e);
                                        ""
                                    },
                                };
                                let v: Value = match serde_json::from_str(s) {
                                    Ok(json) => {
                                        trace!("Converted result: {}", json);
                                        json
                                    }
                                    Err(_e) => {
                                        warn!("Invalid JSON object: '{}' for string: {}", _e, s);
                                        serde_json::Value::Null
                                    }
                                };
                            }
                        }
                        Err(ref _e) if _e.kind() == io::ErrorKind::WouldBlock => {
                            warn!("No data, send heartbeat");
                            continue;
                        }
                        Err(e) => {
                            warn!("Encountered IO error: {}", e.to_string());
                        }
                    };
                }
            }
        }
    }
}
