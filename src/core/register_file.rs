pub struct RegisterFile {
    pub regs: [u16; 256], // Support up to 256 registers for 8-bit addressing
}

impl RegisterFile {
    pub fn new() -> Self {
        Self { regs: [0; 256] }
    }

    pub fn read(&self, idx: u8) -> u16 {
        self.regs[idx as usize]
    }

    pub fn read_10bit(&self, idx: u16) -> u16 {
        if idx < 1024 {
            // For 10-bit addressing, extend register file or map to memory
            if idx < 256 {
                self.regs[idx as usize]
            } else {
                // self.memory.read(idx (- 256))
                0
            }
        } else {
            0
        }
    }

    pub fn write(&mut self, idx: u8, value: u16) {
        self.regs[idx as usize] = value;
    }

    pub fn write_10bit(&mut self, idx: u16, value: u16) {
        if idx < 256 {
            self.regs[idx as usize] = value;
        }
        // For idx >= 256, could map to memory or extended registers
    }
}
