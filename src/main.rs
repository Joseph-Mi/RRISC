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

    // Test program demonstrating Tomasulo's algorithm benefits
    // This program has data dependencies that benefit from out-of-order execution
    let program = [
        // Load immediate values
        0x01, 0x00, 0x00, 0x0A, // LoadImm { dst: 0, value: 10 }  - R0 = 10
        0x01, 0x01, 0x00, 0x14, // LoadImm { dst: 1, value: 20 }  - R1 = 20
        0x01, 0x02, 0x00, 0x05, // LoadImm { dst: 2, value: 5 }   - R2 = 5
        0x01, 0x03, 0x00, 0x03, // LoadImm { dst: 3, value: 3 }   - R3 = 3

        // Arithmetic operations with dependencies
        0x10, 0x04, 0x00, 0x01, // Add { dst: 4, src1: 0, src2: 1 } - R4 = R0 + R1 (10 + 20 = 30)
        0x11, 0x05, 0x02, 0x03, // Sub { dst: 5, src1: 2, src2: 3 } - R5 = R2 - R3 (5 - 3 = 2)
        0x10, 0x06, 0x04, 0x05, // Add { dst: 6, src1: 4, src2: 5 } - R6 = R4 + R5 (30 + 2 = 32)

        // Independent operations that can execute in parallel
        0x20, 0x07, 0x00, 0x01, // And { dst: 7, src1: 0, src2: 1 } - R7 = R0 & R1
        0x21, 0x08, 0x02, 0x03, // Or  { dst: 8, src1: 2, src2: 3 } - R8 = R2 | R3

        0xE0, 0x00, 0x00, 0x00, // Halt
    ];

    mem.load_program(&program, 0);

    // Test 1: Run with in-order execution
    println!("=== Testing In-Order Execution ===");
    let mut cpu_in_order = cpu.clone();
    let mut mem_in_order = mem.clone();

    control_unit::disable_out_of_order(&mut cpu_in_order);
    let start_cycles = cpu_in_order.pipeline.cycles;
    control_unit::run(&mut cpu_in_order, &mut mem_in_order);
    let in_order_cycles = cpu_in_order.pipeline.cycles - start_cycles;

    println!("In-order execution completed in {} cycles", in_order_cycles);
    control_unit::print_cpu_state(&cpu_in_order);

    // Test 2: Run with out-of-order execution (Tomasulo's algorithm)
    println!("\n=== Testing Out-of-Order Execution (Tomasulo) ===");
    let mut cpu_ooo = CpuState::new();
    let mut mem_ooo = Memory::new();
    mem_ooo.load_program(&program, 0);

    control_unit::enable_out_of_order(&mut cpu_ooo);
    let start_cycles_ooo = cpu_ooo.pipeline.cycles;
    control_unit::run(&mut cpu_ooo, &mut mem_ooo);
    let ooo_cycles = cpu_ooo.pipeline.cycles - start_cycles_ooo;

    println!("Out-of-order execution completed in {} cycles", ooo_cycles);
    control_unit::print_cpu_state(&cpu_ooo);

    // Compare results
    println!("\n=== Performance Comparison ===");
    println!("In-order cycles: {}", in_order_cycles);
    println!("Out-of-order cycles: {}", ooo_cycles);

    if ooo_cycles < in_order_cycles {
        let improvement = ((in_order_cycles - ooo_cycles) as f32 / in_order_cycles as f32) * 100.0;
        println!("Performance improvement: {:.1}%", improvement);
    } else if ooo_cycles > in_order_cycles {
        println!("Out-of-order had more overhead in this simple case");
    } else {
        println!("Same performance for this workload");
    }

    // Verify correctness by comparing register values
    println!("\n=== Correctness Verification ===");
    let mut registers_match = true;
    for i in 0..10 {
        let in_order_val = cpu_in_order.regs.read(i);
        let ooo_val = cpu_ooo.regs.read(i);
        if in_order_val != ooo_val {
            println!("MISMATCH: R{} = {} (in-order) vs {} (out-of-order)", 
                     i, in_order_val, ooo_val);
            registers_match = false;
        } else {
            println!("R{} = {} (both modes)", i, in_order_val);
        }
    }

    if registers_match {
        println!("✅ All register values match - Tomasulo implementation is correct!");
    } else {
        println!("❌ Register values differ - check Tomasulo implementation");
    }

    loop {
        // Keep the processor alive
        cortex_m::asm::wfi(); // Wait for interrupt
    }
}