pub mod alu;
pub mod register_file;
pub mod control_unit;
pub mod execute;

use crate::memory::Memory;

#[derive(Debug)]
pub struct CpuState {
    pub regs: register_file::RegisterFile,
    pub pc: u16,
    pub halted: bool,
    pub flags: StatusFlags,
}

#[derive(Debug)]
pub struct StatusFlags {
    pub zero: bool,
    pub carry: bool,
    pub negative: bool,
    pub overflow: bool,
}

impl CpuState {
    pub fn new() -> Self {
        Self {
            regs: register_file::RegisterFile::new(),
            pc: 0,
            halted: false,
            flags: StatusFlags {
                zero: false,
                carry: false,
                negative: false,
                overflow: false,
            },
        }
    }

    pub fn set_flags_from_result(&mut self, result: u16) {
        self.flags.zero = result == 0;
        self.flags.negative = (result as i16) < 0;
    }
}
