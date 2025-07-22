use crate::core::CpuState;
use crate::isa;
use crate::memory::Memory;

/// Main execution function that can switch between in-order and out-of-order execution
pub fn step(cpu: &mut CpuState, mem: &mut Memory) -> bool {
    if cpu.halted {
        return false;
    }

    if cpu.out_of_order_enabled {
        // Use Tomasulo's algorithm (out-of-order execution)
        cpu.pipeline.step(cpu, mem)
    } else {
        // Use original in-order execution
        step_in_order(cpu, mem)
    }
}

/// Original in-order execution (preserved for compatibility and comparison)
pub fn step_in_order(cpu: &mut CpuState, mem: &mut Memory) -> bool {
    if cpu.halted {
        return false;
    }

    let raw = mem.fetch(cpu.pc);
    if let Some(inst) = isa::decode(raw) {
        // Save PC before execution in case of jumps
        let old_pc = cpu.pc;
        crate::core::execute::execute(inst, cpu, mem);
        // Only increment PC if it wasn't changed by a jump/branch
        if cpu.pc == old_pc && !cpu.halted {
            cpu.pc += 4; // 4-byte instructions
        }
        true
    } else {
        cpu.halted = true;
        false
    }
}

/// Run the CPU until it halts
pub fn run(cpu: &mut CpuState, mem: &mut Memory) {
    while !cpu.halted {
        if !step(cpu, mem) {
            break;
        }
    }
}

/// Run the CPU for a specific number of cycles
pub fn run_cycles(cpu: &mut CpuState, mem: &mut Memory, cycles: u32) -> u32 {
    let mut executed = 0;
    for _ in 0..cycles {
        if step(cpu, mem) {
            executed += 1;
        } else {
            break;
        }
    }
    executed
}

/// Enable out-of-order execution mode
pub fn enable_out_of_order(cpu: &mut CpuState) {
    cpu.enable_out_of_order();
    println!("Out-of-order execution enabled (Tomasulo's algorithm)");
}

/// Disable out-of-order execution mode (fallback to in-order)
pub fn disable_out_of_order(cpu: &mut CpuState) {
    cpu.disable_out_of_order();
    println!("Out-of-order execution disabled (in-order mode)");
}

/// Print CPU state for debugging
pub fn print_cpu_state(cpu: &CpuState) {
    println!("=== CPU State ===");
    println!("PC: 0x{:04X}", cpu.pc);
    println!("Halted: {}", cpu.halted);
    println!("Out-of-order enabled: {}", cpu.out_of_order_enabled);
    println!("Cycles: {}", cpu.pipeline.cycles);

    if cpu.out_of_order_enabled {
        println!("ROB entries: {}", cpu.reorder_buffer.count);
        println!("Instruction queue: {}", cpu.pipeline.instruction_queue.len());
    }

    println!("Flags: Zero={}, Carry={}, Negative={}, Overflow={}", 
             cpu.flags.zero, cpu.flags.carry, cpu.flags.negative, cpu.flags.overflow);
}