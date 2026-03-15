pub(crate) struct BUS {
    pub(crate) ram: Vec<u8>,
    pub(crate) vram: Vec<u32>,
    pub(crate) mtime: u64,
    pub(crate) mtimecmp: u64,
    pub(crate) key_buff: u32,
}
