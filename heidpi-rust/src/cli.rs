/**
 * Handles user input from the command line, parses it, and routes it to the appropriate functions. 
 * It supports operations like starting the application with specific configurations or running in different modes
 * (e.g. server mode, pcap file processing mode)
 */

use anyhow::{bail, ensure}; // Error handling, in this case: return early with an error and if a condition is not met
use clap::Parser; // Parse command-line arguments into Self
use std::path::PathBuf; // path manipulation

use tokio::sync::OnceCell; // Thread safe cell that can be written to only once

use crate::stream;
use crate::geoip::GeoIP;

use crate::config::Config; 
use crate::config::LoggingConfig;
use crate::config::Event; 
use crate::logging::Logging;

static ONCON: OnceCell<Config> = OnceCell::const_new();


#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
pub enum Cli {
    #[allow(rustdoc::broken_intra_doc_links)]
    #[allow(rustdoc::invalid_html_tags)]
    Start {
        #[clap(short, long)]
        config_file: Option<PathBuf>,

        /// nDPIsrvd host IP
        #[clap(long)]
        host: String,
        /// nDPIsrvd TCP port
        #[clap(long)]
        port: String,    

        /// where to write log files
        #[clap(short, long)]
        write: Option<PathBuf>,

        /// Enable daemon events
        #[clap(long, default_value_t=false)]
        daemon_events: bool,
        /// Enable packet events
        #[clap(long, default_value_t=false)]
        packet_events: bool,
        /// Enable error events
        #[clap(long, default_value_t=false)]
        error_events: bool,
        /// Enable flow events
        #[clap(long, default_value_t=true)]
        flow_events: bool,
    },

    Man,
}

impl Cli {
    pub async fn run() -> anyhow::Result<()> {
        let cli = Self::parse();
        use Cli::*;
        match cli {
            Man => {
                let man_cmd = std::process::Command::new("man")
                    .args(["1", "heidpi"])
                    .status();

                // if !(man_cmd.is_ok() && man_cmd.unwrap().success()) {
                //     println!(include_str!(env!("HEIDPI_MAN")));
                // }
            }
            Start { 
                config_file,

                host,
                port,
        
                write,
        
                daemon_events,
                packet_events,
                error_events,
                flow_events,  
             } => {
                // TODO Handle parameters
                // TODO Handle config, should be global accessible
                //let mut v = stream::connect("127.0.0.1:7000").await; // Because it should be global accessible, we're not using this hard coded version

                // if a configuration file is provided, it is read and parsed. Otherwise default values are used.
                let config = if let Some(config_path) = config_file{
                    Config::from_file(&config_path)?
                } else{
                    Config::new(
                        LoggingConfig {
                            level: "info".to_string(),
                            encoding: "utf-8".to_string(),
                            format: "plain".to_string(),
                            datefmt: "%Y-%m-%d %H:%M:%S".to_string(),
                        },
                        Event {
                            ignore_fields: vec![],
                            ignore_risks: vec![],
                            flow_event_name: vec![],
                            geoip: None,
                            filename: "flow.log".to_string(),
                        },
                        Event {
                            ignore_fields: vec![],
                            ignore_risks: vec![],
                            flow_event_name: vec![],
                            geoip: None,
                            filename: "daemon.log".to_string(),
                        },
                        Event {
                            ignore_fields: vec![],
                            ignore_risks: vec![],
                            flow_event_name: vec![],
                            geoip: None,
                            filename: "packet.log".to_string(),
                        },
                        Event {
                            ignore_fields: vec![],
                            ignore_risks: vec![],
                            flow_event_name: vec![],
                            geoip: None,
                            filename: "error.log".to_string(),
                        },
                    )
                };

                ONCON.set(config).unwrap();

                //Access global configuration
                let config = ONCON.get().unwrap();
                println!("Using configuration: {:?}", config);

                // Create loggers for each event type
                let flow_logger = Logging::new("flow".to_string(), PathBuf::from(&config.flow_event.filename));
                let daemon_logger = Logging::new("daemon".to_string(), PathBuf::from(&config.daemon_event.filename));
                let packet_logger = Logging::new("packet".to_string(), PathBuf::from(&config.packet_event.filename));
                let error_logger = Logging::new("error".to_string(), PathBuf::from(&config.error_event.filename));

                // Initialize GeoIP if necessary
                let geoip = if let Some(geoip_config) = &config.flow_event.geoip {
                    if geoip_config.enabled {
                        Some(GeoIP::new(&geoip_config.filepath)?)
                    } else {
                        None
                    }
                } else {
                    None
                };

                // Connect to the stream and process data
                stream::connect(
                    &format!("{}:{}", host, port),
                    geoip,
                    flow_logger,
                    daemon_logger,
                    packet_logger,
                    error_logger,
                ).await?;
            }
        }                

        Ok(())
    }
}
