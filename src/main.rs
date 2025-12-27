mod models;
mod bus;

use crate::models::bus::BUS;
use crate::models::cpu::CPU;

fn main() {
    println!("Hello, world!");
    let mut bus = BUS::new();
    let mut cpu = CPU::new(bus);
}
