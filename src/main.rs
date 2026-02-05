mod models;
mod bus;
mod cpu;

use crate::models::bus::BUS;
use crate::models::cpu::CPU;

fn main() {
    // Arguments
    // Collect the arguments
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: iron_clad <path_to_binary>");
        return;
    }
    // Get the path from args
    let bin_path = &args[1];

    // initialize bus
    let hardware_bus = BUS::new();
    /*
    // Load the binary into RAM
    if let Err(e) = hardware_bus.load_binary(&bin_path) {
        eprintln!("Failed to load binary '{}': {}", bin_path, e);
        return;
    }*/

    // Initialize CPU
    let mut iron_clad = CPU::new(hardware_bus, models::cpu::Mode::Machine);

    let entry_point = iron_clad.bus.load_elf(bin_path);
    iron_clad.pc = entry_point;

    iron_clad.reg[2] = 0x8000_0000 + (128 * 1024 * 1024) as u32;

    println!("CPU Initialized! PC start at {:#X}", iron_clad.pc);

    let mut exit = false;
    while !exit {
        if CPU::step(&mut iron_clad) {
            print!("INFO: End of Program reached!");
            exit = true;
        }
    }
}
