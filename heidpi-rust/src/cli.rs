use anyhow::{bail, ensure};
use clap::Parser;
use std::path::PathBuf;

use crate::stream::{self, NDpidTcpstream};

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
    pub fn run() -> anyhow::Result<()> {
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
                let mut v = NDpidTcpstream::connect("127.0.0.1:7000");
            }
        }

        Ok(())
    }
}
