fn main() {
    // Example: create memory, CPU, and run a simple loop
    let mut memory = memory::Memory::default();
    let mut cpu = core::cpu::Cpu::new(&mut memory);
    cpu.run();
}

