use crate::bus::BUS;
use elf::abi::PT_LOAD;
use elf::endian::AnyEndian;
use elf::ElfBytes;

impl BUS {
    pub fn load_elf(&mut self, path: &str) -> u32 {
        let file_data = std::fs::read(path).expect("Could not read ELF file");
        let elf = ElfBytes::<AnyEndian>::minimal_parse(&file_data).expect("Failed to parse ELF");

        if let Some(segments) = elf.segments() {
            for phdr in segments {
                if phdr.p_type == PT_LOAD {
                    let addr = phdr.p_paddr as u32;
                    let offset = phdr.p_offset as usize;
                    let size = phdr.p_filesz as usize;
                    let segment_data = &file_data[offset..offset + size];

                    for (i, &byte) in segment_data.iter().enumerate() {
                        let target_addr = addr + i as u32;

                        // ONLY load if the address is within our 128MB RAM range
                        if target_addr >= 0x8000_0000 && target_addr < 0x8000_0000 + (128 * 1024 * 1024) {
                            let ram_index = (target_addr - 0x8000_0000) as usize;
                            self.ram[ram_index] = byte;
                        } else {
                            // Optional: Print a warning for segments being ignored
                            // println!("Skipping ELF segment at {:#X} (outside of RAM)", target_addr);
                        }
                    }
                }
            }
        }
        elf.ehdr.e_entry as u32 // The entry point for our PC
    }
}
