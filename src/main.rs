mod models;
mod bus;
mod cpu;

use crate::models::bus::BUS;
use crate::models::cpu::CPU;

fn main() {
    // initialize bus and cpu
    let hardware_bus = BUS::new();
    let mut iron_clad = CPU::new(hardware_bus);

    println!("CPU Initialized! PC start at {:#X}", iron_clad.pc);
}
