use crate::bus::{BUS, ram::clint::CLINT_MTIME};


impl BUS {
    pub(crate) fn read_mtime(self) -> u64 {
        let mtime: u64 = BUS::read_x64(&self, CLINT_MTIME, 8);

        mtime
    }
}
