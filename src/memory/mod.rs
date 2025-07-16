pub trait MemoryBus {
    fn load(&self, addr: u16) -> u8;
    fn store(&mut self, addr: u16, value: u8);
}

pub struct Memory {
    // Internal RAM/ROM arrays
}

impl MemoryBus for Memory {
    fn load(&self, addr: u16) -> u8 {
        todo!()
    }
    fn store(&mut self, addr: u16, value: u8) {
        todo!()
    }
}
