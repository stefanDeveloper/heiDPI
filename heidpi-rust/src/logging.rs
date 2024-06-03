/**
 * Provides a logging mechanism that writes logs to files. it defines a Logging struct and implements methods to write logs safely,
 * ensuring that log files are correctly managed and closed after writing.
 */

use anyhow::bail;
use std::io::prelude::*;
use std::{fs::OpenOptions, path::PathBuf};

// TODO Generate logging struct (Be aware it should generic to be used for daemon, packet, flow and error)
// Generate logging struct (being generic to be used for daemon, packet, flow, and error)
#[derive(Clone)]
pub struct Logging {
    log_type: String,
    filepath: PathBuf,
}


// TODO Write file (Be aware when file is opened, don't forget to close it)
impl Logging {

    //Create new Logging instance
    pub fn new(log_type: String, filepath: PathBuf) -> Self {
        Logging {log_type, filepath}
    }
    
    // Write data to the log file (Be aware when the file is opened, don't forget to close it)
    pub fn write(&self, data: &str, filepath: PathBuf) -> anyhow::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(filepath)
            .unwrap();

        if let Err(e) = writeln!(file, "{}", data) {
            bail!("Couldn't write to file: {}", e);
        }

        file.flush()?;
        // File is automatically closed here when file goes out of scope

        Ok(())
    }
}
