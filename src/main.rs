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
    let mut hardware_bus = BUS::new();

    // Load the binary into RAM
    if let Err(e) = hardware_bus.load_binary(&bin_path) {
        eprintln!("Failed to load binary '{}': {}", bin_path, e);
        return;
    }

    // Initialize CPU
    let mut iron_clad = CPU::new(hardware_bus);

    println!("CPU Initialized! PC start at {:#X}", iron_clad.pc);

    let mut exit = false;
    while !exit {
        if CPU::step(&mut iron_clad) {
            print!("INFO: End of Program reached!");
            exit = true;
        }
    }
}
