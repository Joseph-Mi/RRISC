pub mod alu;
pub mod register_file;
pub mod control_unit;

// Example struct - top-level CPU harness
pub struct Cpu<'a> {
    // References to submodules
    // e.g., register file, ALU, memory (as trait)
}

impl<'a> Cpu<'a> {
    pub fn new(memory: &'a mut dyn crate::memory::MemoryBus) -> Self {
        // Initialize submodules here
        todo!()
    }

    pub fn run(&mut self) {
        // Main CPU simulation loop
        todo!()
    }
}
