use crate::bus::BUS;

impl BUS {
    pub(crate) fn write_mtime(&mut self, val: u64){
        self.mtime = val;
    }
}
