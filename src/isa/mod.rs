#[derive(Debug)]
pub enum Instruction {
    // List your instructions, e.g.,
    Load { dst: u8, addr: u16 },
    LoadImm { dst: u8, value: i16 },
    Store { src: u8, addr: u16 },

    Move { dst: u8, src: u8 },
    MoveIfZero { dst: u8, src: u8 },
    MoveIfNotZero { dst: u8, src: u8 },

    Add { dst: u8, src1: u8, src2: u8 },
    Sub { dst: u8, src1: u8, src2: u8 },
    AddImm { dst: u8, src: u8, imm: i16 },
    SubImm { dst: u8, src: u8, imm: i16 },

    Jump { addr: u16 },
    JumpReg { reg: u8 },
    BranchEqual { src1: u8, src2: u8, addr: u16 },
    BranchNotEqual { src1: u8, src2: u8, addr: u16 },
    BranchLessThan { src1: u8, src2: u8, addr: u16 },
    BranchGreaterThan { src1: u8, src2: u8, addr: u16 },

    Cmp { src1: u8, src2: u8 },
    CmpImm { src: u8, imm: i16 },

    And { dst: u8, src1: u8, src2: u8 },
    Or { dst: u8, src1: u8, src2: u8 },
    Xor { dst: u8, src1: u8, src2: u8 },
    Not { dst: u8, src: u8 },

    ShiftLeft { dst: u8, src: u8, amount: u8 },
    ShiftRight { dst: u8, src: u8, amount: u8 },

    Push { src: u8 }, // Push register onto stack
    Pop { dst: u8 },  // Pop from stack into register

    Nop,
    Halt,
}

pub fn decode(raw: u32) -> Option<Instruction> {
    // Decode the raw instruction bits
    let opcode = (raw >> 24) as u4;
    opcode = opcode & 0xF; // Mask to 6 bits

    match opcode {
        // basic
        0x00 => Some(Instruction::Nop),
        0x01 => Some(Instruction::Halt),

        // mem operations   
        0x02 => Some(Instruction::Load { dst: (raw >> 16) as u8, addr: (raw & 0xFFFF) as u16 }),
        0x03 => Some(Instruction::LoadImm { dst: (raw >> 16) as u8, value: (raw & 0xFFFF) as i16 }),
        0x04 => Some(Instruction::Store { src: (raw >> 16) as u8, addr: (raw & 0xFFFF) as u16 }),
        0x05 => 
        _ => None, // Unknown instruction
    }
}

// register operations
fn decode_r_type(raw: u32) -> Option<Instruction> {
    let func = (raw & 0x3F) as u8;
    let dst = ((raw >> 16) & 0x1F) as u8;
    let src1 = ((raw >> 11) & 0x1F) as u8;
    let src2 = ((raw >> 6) & 0x1F) as u8;

    match func {
        0x20 => Some(Instruction::Add { dst, src1, src2 }),
        0x22 => Some(Instruction::Sub { dst, src1, src2 }),
        0x24 => Some(Instruction::And { dst, src1, src2 }),
        0x25 => Some(Instruction::Or { dst, src1, src2 }),
        0x26 => Some(Instruction::Xor { dst, src1, src2 }),
        0x27 => Some(Instruction::Not { dst, src: src1 }),
        _ => None,
    }
}

// immediate operations
fn decode_i_type(raw: u32) -> Option<Instruction> {
    let opcode = (raw >> 24) as u8;
    let dst = ((raw >> 16) & 0x1F) as u8;
    let src = ((raw >> 11) & 0x1F) as u8;
    let imm = (raw & 0xFFFF) as i16;

    match opcode {
        0x08 => Some(Instruction::AddImm { dst, src, imm }),
        0x09 => Some(Instruction::SubImm { dst, src, imm }),
        _ => None,
    }
}
