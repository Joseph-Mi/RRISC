pub struct Memory {
    pub data: [u8; 65536], // 64KB of emulated memory size, 
                           // can shrink to 32KB (32768) if desired
}

impl Memory {
    pub fn new() -> Self {
        Self { data: [0; 65536] }
    }

    pub fn read(&self, addr: u16) -> u8 {
        if (addr as usize) < self.data.len() {
            self.data[addr as usize]
        } else {
            0 // Return 0 for out-of-bounds reads
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        if (addr as usize) < self.data.len() {
            self.data[addr as usize] = value;
        }
        // Silently ignore out-of-bounds writes
    }

    // NEW: Missing functions that execute.rs needs
    pub fn load_u16(&self, addr: u16) -> u16 {
        let low = self.read(addr);
        let high = self.read(addr.wrapping_add(1));
        (high as u16) << 8 | (low as u16)
    }

    pub fn store_u16(&mut self, addr: u16, value: u16) {
        self.write(addr, value as u8);
        self.write(addr.wrapping_add(1), (value >> 8) as u8);
    }

    pub fn fetch(&self, addr: u16) -> u32 {
        // Fetch 4 bytes to form a u32 instruction (little endian)
        let idx = addr as usize;
        if idx + 3 < self.data.len() {
            ((self.data[idx] as u32) << 0) |
            ((self.data[idx + 1] as u32) << 8) |
            ((self.data[idx + 2] as u32) << 16) |
            ((self.data[idx + 3] as u32) << 24)
        } else {
            0 // Return NOP for out-of-bounds
        }
    }

    pub fn load_program(&mut self, program: &[u8], start_addr: u16) {
        let start = start_addr as usize;
        let end = (start + program.len()).min(self.data.len());
        if start < self.data.len() {
            self.data[start..end].copy_from_slice(&program[..end - start]);
        }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}
