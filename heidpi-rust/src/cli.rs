use clap::{Parser};
use std::path::PathBuf;
use anyhow::{bail, ensure};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
pub enum Cli {
    #[allow(rustdoc::broken_intra_doc_links)]
    #[allow(rustdoc::invalid_html_tags)]
    Start {

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
            },
            Start => {
                print!("perfect")
            }
        }

        Ok(())
    }

}