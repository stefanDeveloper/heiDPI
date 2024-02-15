use std::str;
use std::net::TcpStream;
use std::io::{self, BufRead, BufReader};
use std::time::Duration;
use std::{thread, time};
use serde_json::Value;

const NETWORK_BUFFER_LENGTH_DIGITS: usize = 5;

fn main()
{
    loop
    {
        match TcpStream::connect("127.0.0.1:7000")
        {
            Err(_e) =>
            {
                println!("Could not connect to Server");

                // We don't want to hammer the server with reconnection attempts, so we wait for 5 seconds.
                let five_seconds = time::Duration::from_millis(5000);
                thread::sleep(five_seconds);

                continue;
            },
            Ok(stream) =>
            {
                println!("Connected");

                // match stream.set_read_timeout(Some(Duration::new(60, 0)))
                // {
                //     Ok(..) => println!("Set Read Timeout"),
                //     Err(..) => panic!("Setting Read Timeout Failed")
                // };
                
                // match stream.set_write_timeout(Some(Duration::new(15, 0)))
                // {
                //     Ok(..) => println!("Set Write Timeout"),
                //     Err(..) => panic!("Setting Write Timeout Failed")
                // };

                loop
                {
                    println!("Loop");

                    let mut packet = BufReader::new(&stream);
                    let mut xml = vec![];

                    match packet.read_until(b'\n', &mut xml)
                    {
                        Ok(bytes) =>
                        {
                            if bytes == 0
                            {
                                // This will break us out back out to the first loop to recreate the connection.
                                break;
                            }
                            else
                            {
                                println!("Read {:?} Bytes", bytes);
                                // let s = match str::from_utf8(&xml[NETWORK_BUFFER_LENGTH_DIGITS..]) {
                                let s = match str::from_utf8(&xml) {
                                    Ok(v) => v,
                                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                                };
                                println!("result: {}", s);
                                // let v: Value = serde_json::from_str(s).unwrap();
                            }
                        },
                        Err(ref _e) if _e.kind() == io::ErrorKind::WouldBlock =>
                        {
                            println!("No data, send heartbeat");
                            // send_heartbeat(&mut stream);
                            continue;
                        },
                        Err(e) => {
                            println!("Encountered IO error: {}", e.to_string());
                        }
                    };
                };
            }
        }
    }
}