use crate::bus::BUS;
use crate::bus::ram::clint::CLINT_MTIME;

impl BUS {
    pub(crate) fn write_mtime(&mut self, val: u64){
        BUS::write_x64( self, val);
    }
}
