#![no_std]
#![no_main]

use panic_halt as _; // Panic handler
use cortex_m_rt::entry; // Entry point macro

mod core;
mod isa;
mod memory;
mod peripherals;

use core::{CpuState, control_unit};
use memory::Memory;

#[entry]
fn main() -> ! {
    // Initialize the CPU and memory
    let mut cpu = CpuState::new();
    let mut mem = Memory::new();

    // Example program: Load immediate 42 into register 0, then halt
    let program = [
        0x01, 0x00, 0x00, 0x2A, // LoadImm { dst: 0, value: 42 }
        0xE0, 0x00, 0x00, 0x00, // Halt
    ];
    
    mem.load_program(&program, 0);

    // Run the emulator

    control_unit::run(&mut cpu, &mut mem);

    // In a real application, you might want to do something here
    // like output results via UART or blink an LED

    loop {
        // Keep the processor alive
        cortex_m::asm::wfi(); // Wait for interrupt
    }
}
