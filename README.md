# Iron Clad

**Iron Clad** is a RISC-V 32-bit CPU emulator written in Rust. It is designed to simulate the behavior of a RISC-V processor, including instruction fetching, decoding, and execution, interacting with a simulated memory bus.

## Features

- **Architecture:** Emulates a 32-bit RISC-V CPU.
- **Components:**
  - **CPU:** Includes 32 general-purpose registers (32-bit) and a Program Counter (PC).
  - **BUS:** Simulates the system bus and memory access.
  - **RAM:** Supports loading binary executables into a simulated memory space.
- **Execution:** Basic fetch-decode-execute cycle.
- **Documentation:** detailed technical specs available in `docs/specs/`.

## Prerequisites

- **Rust:** You need the Rust toolchain installed (Cargo, rustc). You can install it via [rustup.rs](https://rustup.rs/).

## Installation

Clone the repository:

```bash
git clone https://github.com/Pegasis1002/Iron-clad.git
cd iron-clad
```

## Usage

To run the emulator, provide the path to a binary file you wish to execute.

```bash
cargo run -- <path_to_binary>
```

### Example

```bash
cargo run -- test_bins/all_formats.bin
```

This will:
1. Initialize the system BUS.
2. Load the specified binary into the simulated RAM.
3. Initialize the CPU with the Program Counter starting at `0x8000_0000`.
4. Step through the instructions (currently limited to 15 steps or until a halt condition).

## Project Structure

- `src/main.rs`: Entry point of the application. Handles argument parsing and main loop.
- `src/models/`: Contains the data structures for the `CPU` and `BUS`.
- `src/bus/`: Implementation of memory management and binary loading.
- `src/cpu/`: Implementation of CPU logic, including the `step` function and instruction decoding.
- `docs/specs/`: Detailed technical specifications and notes (in Neorg format).

## Development

To build the project:

```bash
cargo build
```
