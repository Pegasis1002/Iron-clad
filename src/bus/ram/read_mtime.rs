use crate::bus::{BUS};


impl BUS {
    pub(crate) fn read_mtime(&self) -> u64 {
        self.mtime
    }
}
