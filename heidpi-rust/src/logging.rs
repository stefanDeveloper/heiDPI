use anyhow::bail;
use std::io::prelude::*;
use std::{fs::OpenOptions, path::PathBuf};

struct Logging {
    
}



impl Logging {
    pub fn write(&self, data: &str, filepath: PathBuf) -> anyhow::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(filepath)
            .unwrap();

        if let Err(e) = writeln!(file, "{}", data) {
            bail!("Couldn't write to file: {}", e);
        }

        file.flush();

        Ok(())
    }
}
