use crate::isa::Instruction;
use crate::core::{CpuState, alu};
use crate::memory::Memory;

pub fn execute(instruction: Instruction, cpu: &mut CpuState, mem: &mut Memory) {
    match instruction {
        Instruction::Load { dst, addr } => {
            let value = mem.load_u16(addr);
            cpu.regs.write(dst, value);
        }

        Instruction::LoadImm { dst, value } => {
            cpu.regs.write(dst, value as u16);
        }

        Instruction::Store { src, addr } => {
            let value = cpu.regs.read(src);
            mem.store_u16(addr, value);
        }

        Instruction::Move { dst, src } => {
            let value = cpu.regs.read_10bit(src);
            cpu.regs.write(dst, value);
        }

        Instruction::MoveIfZero { dst, src } => {
            if cpu.flags.zero {
                let value = cpu.regs.read_10bit(src);
                cpu.regs.write(dst, value);
            }
        }

        Instruction::MoveIfNotZero { dst, src } => {
            if !cpu.flags.zero {
                let value = cpu.regs.read_10bit(src);
                cpu.regs.write(dst, value);
            }
        }

        Instruction::MoveWide { dst, src } => {
            let value = cpu.regs.read(src);
            cpu.regs.write_10bit(dst, value);
        }

        Instruction::MoveWideIfZero { dst, src } => {
            if cpu.flags.zero {
                let value = cpu.regs.read(src);
                cpu.regs.write_10bit(dst, value);
            }
        }

        Instruction::MoveWideIfNotZero { dst, src } => {
            if !cpu.flags.zero {
                let value = cpu.regs.read(src);
                cpu.regs.write_10bit(dst, value);
            }
        }

        Instruction::Add { dst, src1, src2 } => {
            let val1 = cpu.regs.read(src1);
            let val2 = cpu.regs.read_10bit(src2);
            let (result, carry) = alu::add(val1, val2);
            cpu.regs.write(dst, result);
            cpu.flags.carry = carry;
            cpu.set_flags_from_result(result);
        }

        Instruction::Sub { dst, src1, src2 } => {
            let val1 = cpu.regs.read(src1);
            let val2 = cpu.regs.read_10bit(src2);
            let (result, carry) = alu::sub(val1, val2);
            cpu.regs.write(dst, result);
            cpu.flags.carry = carry;
            cpu.set_flags_from_result(result);
        }

        Instruction::AddImm { dst, src, imm } => {
            let val = cpu.regs.read(src);
            let (result, carry) = alu::add(val, imm as u16);
            cpu.regs.write(dst, result);
            cpu.flags.carry = carry;
            cpu.set_flags_from_result(result);
        }

        Instruction::SubImm { dst, src, imm } => {
            let val = cpu.regs.read(src);
            let (result, carry) = alu::sub(val, imm as u16);
            cpu.regs.write(dst, result);
            cpu.flags.carry = carry;
            cpu.set_flags_from_result(result);
        }

        Instruction::And { dst, src1, src2 } => {
            let val1 = cpu.regs.read(src1);
            let val2 = cpu.regs.read_10bit(src2);
            let result = alu::and(val1, val2);
            cpu.regs.write(dst, result);
            cpu.set_flags_from_result(result);
        }

        Instruction::Or { dst, src1, src2 } => {
            let val1 = cpu.regs.read(src1);
            let val2 = cpu.regs.read_10bit(src2);
            let result = alu::or(val1, val2);
            cpu.regs.write(dst, result);
            cpu.set_flags_from_result(result);
        }

        Instruction::Xor { dst, src1, src2 } => {
            let val1 = cpu.regs.read(src1);
            let val2 = cpu.regs.read_10bit(src2);
            let result = alu::xor(val1, val2);
            cpu.regs.write(dst, result);
            cpu.set_flags_from_result(result);
        }

        Instruction::Not { dst, src } => {
            let val = cpu.regs.read(src);
            let result = alu::not(val);
            cpu.regs.write(dst, result);
            cpu.set_flags_from_result(result);
        }

        Instruction::ShiftLeft { dst, src, amount } => {
            let val = cpu.regs.read(src);
            let result = alu::shift_left(val, amount);
            cpu.regs.write(dst, result);
            cpu.set_flags_from_result(result);
        }

        Instruction::ShiftRight { dst, src, amount } => {
            let val = cpu.regs.read(src);
            let result = alu::shift_right(val, amount);
            cpu.regs.write(dst, result);
            cpu.set_flags_from_result(result);
        }

        Instruction::Jump { addr } => {
            cpu.pc = addr;
        }

        Instruction::JumpReg { reg } => {
            cpu.pc = cpu.regs.read_10bit(reg);
        }

        Instruction::BranchEqual { src1, src2, addr } => {
            let val1 = cpu.regs.read(src1);
            let val2 = cpu.regs.read_10bit(src2);
            if val1 == val2 {
                cpu.pc = addr;
            }
        }

        Instruction::BranchNotEqual { src1, src2, addr } => {
            let val1 = cpu.regs.read(src1);
            let val2 = cpu.regs.read_10bit(src2);
            if val1 != val2 {
                cpu.pc = addr;
            }
        }

        Instruction::BranchLessThan { src1, src2, addr } => {
            let val1 = cpu.regs.read(src1) as i16;
            let val2 = cpu.regs.read_10bit(src2) as i16;
            if val1 < val2 {
                cpu.pc = addr;
            }
        }

        Instruction::BranchGreaterThan { src1, src2, addr } => {
            let val1 = cpu.regs.read(src1) as i16;
            let val2 = cpu.regs.read_10bit(src2) as i16;
            if val1 > val2 {
                cpu.pc = addr;
            }
        }

        Instruction::Cmp { src1, src2 } => {
            let val1 = cpu.regs.read(src1);
            let val2 = cpu.regs.read_10bit(src2);
            let (result, carry) = alu::sub(val1, val2);
            cpu.flags.carry = carry;
            cpu.set_flags_from_result(result);
        }

        Instruction::CmpImm { src, imm } => {
            let val = cpu.regs.read(src);
            let (result, carry) = alu::sub(val, imm as u16);
            cpu.flags.carry = carry;
            cpu.set_flags_from_result(result);
        }

        Instruction::Push { src } => {
            let value = cpu.regs.read_10bit(src);
            // Implement stack push (requires stack pointer)
            // For now, just decrement PC and store
        }

        Instruction::Pop { dst } => {
            // Implement stack pop (requires stack pointer)
            // For now, just load from memory
        }

        Instruction::Nop => {
            // Do nothing
        }

        Instruction::Halt => {
            cpu.halted = true;
        }
    }
}
