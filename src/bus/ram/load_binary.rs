use crate::models::bus::BUS;

use std::io::Read;

impl BUS {
    pub fn load_binary(&mut self, path: &str) -> std::io::Result<()> {
        let mut file = std::fs::File::open(path)?;

        let bytes_read = file.read(&mut self.ram)?;

        println!("INFO: Loaded {} bytes from {} into RAM", bytes_read, path);
        Ok(())
    }
}
