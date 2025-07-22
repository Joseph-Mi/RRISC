

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    // Memory
    Load    { dst: u8, addr: u16 },
    LoadImm { dst: u8, value: i16 },
    Store   { src: u8, addr: u16 },

    // Moves
    Move          { dst: u8, src: u8 },
    MoveIfZero    { dst: u8, src: u8 },
    MoveIfNotZero { dst: u8, src: u8 },

    // Arithmetic
    Add    { dst: u8, src1: u8, src2: u8 },
    Sub    { dst: u8, src1: u8, src2: u8 },
    AddImm { dst: u8, src: u8, imm: i16 },
    SubImm { dst: u8, src: u8, imm: i16 },

    Mult { dst: u8, src1: u8, src2: u8 },
    MulImm { dst: u8, src: u8, imm: i16 },
    Div  { dst: u8, src1: u8, src2: u8 },
    Mod  { dst: u8, src1: u8, src2: u8 },

    // Jumps/branches
    Jump    { addr: u16 },
    JumpReg { reg: u16 },
    Call    { addr: u16 }, // Call function at addr
    Ret,

    BranchEqual       { src1: u8, src2: u8, addr: u16 },
    BranchNotEqual    { src1: u8, src2: u8, addr: u16 },
    BranchLessThan    { src1: u8, src2: u8, addr: u16 },
    BranchGreaterThan { src1: u8, src2: u8, addr: u16 },

    // Comparison
    Cmp    { src1: u8, src2: u8 },
    CmpImm { src: u8, imm: i16 },

    // Logic
    And { dst: u8, src1: u8, src2: u8 },
    Or  { dst: u8, src1: u8, src2: u8 },
    Xor { dst: u8, src1: u8, src2: u8 },
    Not { dst: u8, src: u8 },

    // Shifts
    ShiftLeft  { dst: u8, src: u8, amount: u16 },
    ShiftRight { dst: u8, src: u8, amount: u16 },

    // Stack
    Push { src: u16 }, // 10-bit reg index
    Pop  { dst: u16 }, // 10-bit reg index



    Nop,
    Halt,
}

fn sign_extend_10(val: u16) -> i16 {
    let sign_bit = 1 << 9;
    if (val & sign_bit) != 0 {
        (val | !0x3FF) as i16
    } else {
        val as i16
    }
}

pub fn decode(raw: u32) -> Option<Instruction> {
    // 31  28 27 26 25  18 17  10  9    0
    // [pri] [sec] [reg1] [reg2] [imm]
    // [pri] [sec] [reg1] [reg2] [2 bits] [reg3]
    //   4    2     8      8      10 bits
    let primary_opcode = ((raw >> 28) & 0xF) as u8;
    let secondary_opcode = ((raw >> 26) & 0x3) as u8;
    let dst = ((raw >> 18) & 0xFF) as u8;
    let src1 = ((raw >> 10) & 0xFF) as u8;
    let src2 = (raw & 0xFF) as u8;
    let imm10 = (raw & 0x3FF) as u16;

    match primary_opcode {
        0xF => Some(Instruction::Nop),
        0xE => Some(Instruction::Halt),

        0x0 => decode_memory(secondary_opcode, dst, src2),
        0x1 => decode_arithmetic(secondary_opcode, dst, src1, src2),
        0x2 => decode_logical(secondary_opcode, dst, src1, src2),
        0x3 => decode_branch(secondary_opcode, src1, src2, dst), // dst repurposed as addr
        0x4 => decode_jump(secondary_opcode, src2, dst),
        0x5 => decode_compare(secondary_opcode, src1, src2),
        0x6 => decode_shift(secondary_opcode, dst, src1, src2),
        0x7 => decode_stack(secondary_opcode, src2),
        0x8 => decode_move(secondary_opcode, dst, src2),
        0x9 => decode_complex_arithmetic(secondary_opcode, dst, src1, src2),
        _   => None,
    }
}

// Memory operations 0x0
fn decode_memory(secondary: u8, dst: u8, addr10: u16) -> Option<Instruction> {
    match secondary {
        0b00 => Some(Instruction::Load    { dst, addr: addr10 }),
        0b01 => Some(Instruction::LoadImm { dst, value: sign_extend_10(addr10) }),
        0b10 => Some(Instruction::Store   { src: dst, addr: addr10 }),
        _    => None,
    }
}

// Arithmetic operations 0x1
fn decode_arithmetic(secondary: u8, dst: u8, src1: u8, src2_imm10: u16) -> Option<Instruction> {
    match secondary {
        0b00 => Some(Instruction::Add    { dst, src1, src2: src2_imm10 }),
        0b01 => Some(Instruction::Sub    { dst, src1, src2: src2_imm10 }),
        0b10 => Some(Instruction::AddImm { dst, src: src1, imm: sign_extend_10(src2_imm10) }),
        0b11 => Some(Instruction::SubImm { dst, src: src1, imm: sign_extend_10(src2_imm10) }),
        _    => None,
    }
}

// Logical operations 0x2
fn decode_logical(secondary: u8, dst: u8, src1: u8, src2_imm10: u16) -> Option<Instruction> {
    match secondary {
        0b00 => Some(Instruction::And { dst, src1, src2: src2_imm10 }),
        0b01 => Some(Instruction::Or  { dst, src1, src2: src2_imm10 }),
        0b10 => Some(Instruction::Xor { dst, src1, src2: src2_imm10 }),
        0b11 => Some(Instruction::Not { dst, src: src1 }),
        _    => None,
    }
}

// Branch operations 0x3
fn decode_branch(secondary: u8, src1: u8, src2_10: u16, addr8: u8) -> Option<Instruction> {
    let addr = addr8 as u16;
    match secondary {
        0b00 => Some(Instruction::BranchEqual      { src1, src2: src2_10, addr }),
        0b01 => Some(Instruction::BranchNotEqual   { src1, src2: src2_10, addr }),
        0b10 => Some(Instruction::BranchLessThan   { src1, src2: src2_10, addr }),
        0b11 => Some(Instruction::BranchGreaterThan{ src1, src2: src2_10, addr }),
        _    => None,
    }
}

// Jump operations 0x4
fn decode_jump(secondary: u8, addr10: u16, reg8: u8) -> Option<Instruction> {
    match secondary {
        0b00 => Some(Instruction::Jump    { addr: addr10 }),
        0b01 => Some(Instruction::JumpReg { reg: reg8 as u16 }),
        0b10 => Some(Instruction::Call    { addr: addr10 }),
        0b11 => Some(Instruction::Ret),
        _    => None,
    }
}

// Compare operations 0x5
fn decode_compare(secondary: u8, src1: u8, src2_imm10: u16) -> Option<Instruction> {
    match secondary {
        0b00 => Some(Instruction::Cmp { src1, src2: src2_imm10 }),
        0b01 => Some(Instruction::CmpImm { src: src1, imm: sign_extend_10(src2_imm10) }),
        _    => None,
    }
}

// Shift operations 0x6
fn decode_shift(secondary: u8, dst: u8, src: u8, amt10: u16) -> Option<Instruction> {
    match secondary {
        0b00 => Some(Instruction::ShiftLeft  { dst, src, amount: amt10 }),
        0b01 => Some(Instruction::ShiftRight { dst, src, amount: amt10 }),
        _    => None,
    }
}

// Stack operations 0x7
fn decode_stack(secondary: u8, reg10: u16) -> Option<Instruction> {
    match secondary {
        0b00 => Some(Instruction::Push { src: reg10 }),
        0b01 => Some(Instruction::Pop  { dst: reg10 }),
        _    => None,
    }
}

// Move operations 0x8 
fn decode_move(secondary: u8, dst: u8, src10: u16) -> Option<Instruction> {
    match secondary {
        0b00 => Some(Instruction::Move          { dst, src: src10 }),
        0b01 => Some(Instruction::MoveIfZero    { dst, src: src10 }),
        0b10 => Some(Instruction::MoveIfNotZero { dst, src: src10 }),
        _    => None,
    }
}

