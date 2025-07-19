use crate::core::{CpuState, execute};
use crate::isa;
use crate::memory::Memory;

pub fn step(cpu: &mut CpuState, mem: &mut Memory) -> bool {
    if cpu.halted {
        return false;
    }

    let raw = mem.fetch(cpu.pc);
    if let Some(inst) = isa::decode(raw) {
        // Save PC before execution in case of jumps
        let old_pc = cpu.pc;
        execute::execute(inst, cpu, mem);
        
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

pub fn run(cpu: &mut CpuState, mem: &mut Memory) {
    while !cpu.halted {
        if !step(cpu, mem) {
            break;
        }
    }
}

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
