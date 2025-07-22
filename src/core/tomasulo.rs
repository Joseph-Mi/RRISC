use crate::isa::Instruction;
use crate::core::CpuState;
use crate::memory::Memory;

#[derive(Debug, Clone)]
pub struct ReservationStation {
    pub busy: bool,
    pub op: Option<Instruction>,
    pub vj: Option<u16>,    //val of source operand J (ready when Some)
    pub vk: Option<u16>,    //val of source operand K (ready when Some)
    pub qj: Option<usize>,  //tag of producer for operand J 
    pub qk: Option<usize>,  //tag of producer for operand K 
    pub tag: usize,         //tag of this station 
    pub cycles_remaining: u32, //execution countdown (0 means ready)
}

impl ReservationStation {
    pub fn new() -> Self {
        Self {
            busy: false,
            op: None,
            vj: None,
            vk: None,
            qj: None,
            qk: None,
            tag: 0,
            cycles_remaining: 0,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.busy && self.vj.is_some() && self.vk.is_some()
    }

    pub fn clear(&mut self) {
        self.busy = false;
        self.op = None;
        self.vj = None;
        self.vk = None;
        self.qj = None;
        self.qk = None;
        self.tag = 0;
        self.cycles_remaining = 0;
    }
}

// ReservationStation pool for different functional units
#[derive(Debug)]
pub struct ReservationStationPool {
    pub alu_stations: Vec<ReservationStation>,
    pub load_stations: Vec<ReservationStation>,
    pub store_stations: Vec<ReservationStation>,
}

impl ReservationStationPool {
    pub fn new() -> Self {
        Self {
            alu_stations: vec![ReservationStation::new(); 4],   // 4 ALU reservation stations
            load_stations: vec![ReservationStation::new(); 2],  // 2 Load stations
            store_stations: vec![ReservationStation::new(); 2], // 2 Store stations
        }
    }

    pub fn find_free_alu_station(&mut self) -> Option<&mut ReservationStation> {
        self.alu_stations.iter_mut().find(|rs| !rs.busy)
    }

    pub fn find_free_load_station(&mut self) -> Option<&mut ReservationStation> {
        self.load_stations.iter_mut().find(|rs| !rs.busy)
    }

    pub fn find_free_store_station(&mut self) -> Option<&mut ReservationStation> {
        self.store_stations.iter_mut().find(|rs| !rs.busy)
    }

    pub fn get_ready_instructions(&mut self) -> Vec<(usize, Instruction, u16, u16)> {
        let mut ready = Vec::new();

        // Check ALU stations
        for (i, rs) in self.alu_stations.iter_mut().enumerate() {
            if rs.is_ready() && rs.cycles_remaining == 0 {
                if let (Some(inst), Some(vj), Some(vk)) = (rs.op, rs.vj, rs.vk) {
                    ready.push((rs.tag, inst, vj, vk));
                }
            }
        }

        // Check Load stations
        for (i, rs) in self.load_stations.iter_mut().enumerate() {
            if rs.is_ready() && rs.cycles_remaining == 0 {
                if let (Some(inst), Some(vj), Some(vk)) = (rs.op, rs.vj, rs.vk) {
                    ready.push((rs.tag, inst, vj, vk));
                }
            }
        }

        // Check Store stations  
        for (i, rs) in self.store_stations.iter_mut().enumerate() {
            if rs.is_ready() && rs.cycles_remaining == 0 {
                if let (Some(inst), Some(vj), Some(vk)) = (rs.op, rs.vj, rs.vk) {
                    ready.push((rs.tag, inst, vj, vk));
                }
            }
        }

        ready
    }

    pub fn update_from_cdb(&mut self, tag: usize, value: u16) {
        // Update ALU stations
        for rs in &mut self.alu_stations {
            if rs.qj == Some(tag) {
                rs.vj = Some(value);
                rs.qj = None;
            }
            if rs.qk == Some(tag) {
                rs.vk = Some(value);
                rs.qk = None;
            }
        }

        // Update Load stations
        for rs in &mut self.load_stations {
            if rs.qj == Some(tag) {
                rs.vj = Some(value);
                rs.qj = None;
            }
            if rs.qk == Some(tag) {
                rs.vk = Some(value);
                rs.qk = None;
            }
        }

        // Update Store stations
        for rs in &mut self.store_stations {
            if rs.qj == Some(tag) {
                rs.vj = Some(value);
                rs.qj = None;
            }
            if rs.qk == Some(tag) {
                rs.vk = Some(value);
                rs.qk = None;
            }
        }
    }
}

// Reorder Buffer Entry
#[derive(Debug, Clone)]
pub struct ReorderBufferEntry {
    pub valid: bool,                    // Entry is active
    pub ready: bool,                    // Result is available (complete bit)
    pub instruction: Option<Instruction>,
    pub dest_reg: Option<u8>,           // Destination register
    pub result: Option<u16>,            // Computed result
    pub exception: bool,                // Exception occurred
    pub pc: u16,                       // Program counter for this instruction
}

impl ReorderBufferEntry {
    pub fn new() -> Self {
        Self {
            valid: false,
            ready: false,
            instruction: None,
            dest_reg: None,
            result: None,
            exception: false,
            pc: 0,
        }
    }

    pub fn clear(&mut self) {
        self.valid = false;
        self.ready = false;
        self.instruction = None;
        self.dest_reg = None;
        self.result = None;
        self.exception = false;
        self.pc = 0;
    }
}

// Reorder Buffer - Circular buffer for in-order commit
#[derive(Debug)]
pub struct ReorderBuffer {
    pub entries: Vec<ReorderBufferEntry>,
    pub head: usize,        // Points to oldest instruction (commit point)
    pub tail: usize,        // Points to next free entry
    pub size: usize,        // Maximum entries
    pub count: usize,       // Current number of entries
}

impl ReorderBuffer {
    pub fn new(size: usize) -> Self {
        Self {
            entries: vec![ReorderBufferEntry::new(); size],
            head: 0,
            tail: 0,
            size,
            count: 0,
        }
    }

    pub fn is_full(&self) -> bool {
        self.count == self.size
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn allocate(&mut self, instruction: Instruction, dest_reg: Option<u8>, pc: u16) -> Option<usize> {
        if self.is_full() {
            return None;
        }

        let entry = &mut self.entries[self.tail];
        entry.valid = true;
        entry.ready = false;
        entry.instruction = Some(instruction);
        entry.dest_reg = dest_reg;
        entry.result = None;
        entry.exception = false;
        entry.pc = pc;

        let tag = self.tail;
        self.tail = (self.tail + 1) % self.size;
        self.count += 1;

        Some(tag)
    }

    pub fn complete(&mut self, tag: usize, result: Option<u16>) {
        if tag < self.entries.len() && self.entries[tag].valid {
            self.entries[tag].ready = true;
            self.entries[tag].result = result;
        }
    }

    pub fn can_commit(&self) -> bool {
        !self.is_empty() && self.entries[self.head].valid && self.entries[self.head].ready
    }

    pub fn commit(&mut self) -> Option<ReorderBufferEntry> {
        if !self.can_commit() {
            return None;
        }

        let entry = self.entries[self.head].clone();
        self.entries[self.head].clear();
        self.head = (self.head + 1) % self.size;
        self.count -= 1;

        Some(entry)
    }

    pub fn update_from_cdb(&mut self, tag: usize, value: u16) {
        self.complete(tag, Some(value));
    }
}

// Register Rename Table Entry
#[derive(Debug, Clone)]
pub struct RenameEntry {
    pub producer_tag: Option<usize>,    // ROB entry producing this register
    pub ready: bool,                    // Is the value available?
}

impl RenameEntry {
    pub fn new() -> Self {
        Self {
            producer_tag: None,
            ready: true,
        }
    }
}

// Register Rename Table
#[derive(Debug)]
pub struct RegisterRenameTable {
    pub entries: Vec<RenameEntry>,
}

impl RegisterRenameTable {
    pub fn new(num_registers: usize) -> Self {
        Self {
            entries: vec![RenameEntry::new(); num_registers],
        }
    }

    pub fn rename_register(&mut self, reg: u8, producer_tag: usize) {
        if (reg as usize) < self.entries.len() {
            self.entries[reg as usize].producer_tag = Some(producer_tag);
            self.entries[reg as usize].ready = false;
        }
    }

    pub fn get_register_info(&self, reg: u8) -> (bool, Option<usize>) {
        if (reg as usize) < self.entries.len() {
            let entry = &self.entries[reg as usize];
            (entry.ready, entry.producer_tag)
        } else {
            (true, None)
        }
    }

    pub fn update_from_cdb(&mut self, tag: usize) {
        for entry in &mut self.entries {
            if entry.producer_tag == Some(tag) {
                entry.ready = true;
                entry.producer_tag = None;
            }
        }
    }
}

// Common Data Bus for broadcasting results
#[derive(Debug)]
pub struct CommonDataBus {
    pub valid: bool,        // Is there data on the bus this cycle?
    pub tag: usize,         // Which ROB entry is producing this data?
    pub value: u16,         // The actual data value
}

impl CommonDataBus {
    pub fn new() -> Self {
        Self {
            valid: false,
            tag: 0,
            value: 0,
        }
    }

    pub fn broadcast(&mut self, tag: usize, value: u16) {
        self.valid = true;
        self.tag = tag;
        self.value = value;
    }

    pub fn clear(&mut self) {
        self.valid = false;
        self.tag = 0;
        self.value = 0;
    }
}

// Main Pipeline Controller for Tomasulo's Algorithm
#[derive(Debug)]
pub struct PipelineController {
    pub cycles: u64,
    pub instruction_queue: Vec<(Instruction, u16)>, // (instruction, pc)
}

impl PipelineController {
    pub fn new() -> Self {
        Self {
            cycles: 0,
            instruction_queue: Vec::new(),
        }
    }

    pub fn step(&mut self, cpu: &mut CpuState, mem: &mut Memory) -> bool {
        // Execute stages in reverse order to avoid conflicts
        self.commit_stage(cpu, mem);
        self.writeback_stage(cpu);
        self.execute_stage(cpu);
        self.issue_stage(cpu, mem);

        self.cycles += 1;
        !cpu.halted
    }

    // Issue Stage: Decode instructions, rename registers, allocate reservation stations
    fn issue_stage(&mut self, cpu: &mut CpuState, mem: &mut Memory) {
        // Fetch instruction if queue has space
        if self.instruction_queue.len() < 4 && !cpu.halted { // Keep 4 instructions buffered
            let raw = mem.fetch(cpu.pc);
            if let Some(inst) = crate::isa::decode(raw) {
                self.instruction_queue.push((inst, cpu.pc));
                cpu.pc += 4; // 4-byte instructions
            } else {
                cpu.halted = true;
                return;
            }
        }

        // Try to issue the oldest instruction
        if let Some((instruction, pc)) = self.instruction_queue.first().cloned() {
            if self.try_issue_instruction(instruction, pc, cpu) {
                self.instruction_queue.remove(0);
            }
        }
    }

    fn try_issue_instruction(&mut self, instruction: Instruction, pc: u16, cpu: &mut CpuState) -> bool {
        use crate::isa::Instruction;

        match instruction {
            // ALU instructions
            Instruction::Add { dst, src1, src2 } |
            Instruction::Sub { dst, src1, src2 } |
            Instruction::And { dst, src1, src2 } |
            Instruction::Or { dst, src1, src2 } |
            Instruction::Xor { dst, src1, src2 } => {
                self.issue_alu_instruction(instruction, dst, Some(src1), Some(src2 as u8), pc, cpu)
            },

            Instruction::AddImm { dst, src, imm } |
            Instruction::SubImm { dst, src, imm } => {
                // For immediate instructions, we treat the immediate as always ready
                self.issue_alu_instruction(instruction, dst, Some(src), None, pc, cpu)
            },

            Instruction::Not { dst, src } => {
                self.issue_alu_instruction(instruction, dst, Some(src), None, pc, cpu)
            },

            Instruction::Load { dst, addr } => {
                self.issue_load_instruction(instruction, dst, addr, pc, cpu)
            },

            Instruction::Store { src, addr } => {
                self.issue_store_instruction(instruction, src, addr, pc, cpu)
            },

            _ => {
                // For unsupported instructions, fall back to in-order execution
                crate::core::execute::execute(instruction, cpu, &mut mem);
                true
            }
        }
    }

    fn issue_alu_instruction(&mut self, instruction: Instruction, dst: u8, src1: Option<u8>, src2: Option<u8>, pc: u16, cpu: &mut CpuState) -> bool {
        // Check if we can allocate a reservation station
        if let Some(rs) = cpu.reservation_stations.find_free_alu_station() {
            // Check if we can allocate a ROB entry
            if let Some(rob_tag) = cpu.reorder_buffer.allocate(instruction, Some(dst), pc) {
                rs.busy = true;
                rs.op = Some(instruction);
                rs.tag = rob_tag;
                rs.cycles_remaining = self.get_execution_cycles(&instruction);

                // Handle first source operand
                if let Some(src1) = src1 {
                    let (ready, producer_tag) = cpu.rename_table.get_register_info(src1);
                    if ready {
                        rs.vj = Some(cpu.regs.read(src1));
                        rs.qj = None;
                    } else {
                        rs.vj = None;
                        rs.qj = producer_tag;
                    }
                } else {
                    rs.vj = Some(0); // No source needed
                    rs.qj = None;
                }

                // Handle second source operand
                if let Some(src2) = src2 {
                    let (ready, producer_tag) = cpu.rename_table.get_register_info(src2);
                    if ready {
                        rs.vk = Some(cpu.regs.read(src2));
                        rs.qk = None;
                    } else {
                        rs.vk = None;
                        rs.qk = producer_tag;
                    }
                } else {
                    // Handle immediate values or single-operand instructions
                    rs.vk = Some(self.get_immediate_value(&instruction));
                    rs.qk = None;
                }

                // Rename the destination register
                cpu.rename_table.rename_register(dst, rob_tag);

                return true;
            }
        }
        false
    }

    fn issue_load_instruction(&mut self, instruction: Instruction, dst: u8, addr: u16, pc: u16, cpu: &mut CpuState) -> bool {
        if let Some(rs) = cpu.reservation_stations.find_free_load_station() {
            if let Some(rob_tag) = cpu.reorder_buffer.allocate(instruction, Some(dst), pc) {
                rs.busy = true;
                rs.op = Some(instruction);
                rs.tag = rob_tag;
                rs.cycles_remaining = 2; // Load takes 2 cycles
                rs.vj = Some(addr);
                rs.vk = Some(0); // Not used for loads
                rs.qj = None;
                rs.qk = None;

                cpu.rename_table.rename_register(dst, rob_tag);
                return true;
            }
        }
        false
    }

    fn issue_store_instruction(&mut self, instruction: Instruction, src: u8, addr: u16, pc: u16, cpu: &mut CpuState) -> bool {
        if let Some(rs) = cpu.reservation_stations.find_free_store_station() {
            if let Some(rob_tag) = cpu.reorder_buffer.allocate(instruction, None, pc) {
                rs.busy = true;
                rs.op = Some(instruction);
                rs.tag = rob_tag;
                rs.cycles_remaining = 1; // Store takes 1 cycle to compute address

                let (ready, producer_tag) = cpu.rename_table.get_register_info(src);
                if ready {
                    rs.vj = Some(cpu.regs.read(src));
                    rs.qj = None;
                } else {
                    rs.vj = None;
                    rs.qj = producer_tag;
                }

                rs.vk = Some(addr);
                rs.qk = None;

                return true;
            }
        }
        false
    }

    // Execute Stage: Execute ready instructions in parallel functional units
    fn execute_stage(&mut self, cpu: &mut CpuState) {
        // Decrement cycles for all busy reservation stations
        for rs in &mut cpu.reservation_stations.alu_stations {
            if rs.busy && rs.cycles_remaining > 0 {
                rs.cycles_remaining -= 1;
            }
        }

        for rs in &mut cpu.reservation_stations.load_stations {
            if rs.busy && rs.cycles_remaining > 0 {
                rs.cycles_remaining -= 1;
            }
        }

        for rs in &mut cpu.reservation_stations.store_stations {
            if rs.busy && rs.cycles_remaining > 0 {
                rs.cycles_remaining -= 1;
            }
        }
    }

    // Write Result Stage: Broadcast completed results via Common Data Bus
    fn writeback_stage(&mut self, cpu: &mut CpuState) {
        cpu.common_data_bus.clear();

        // Find a completed instruction to write back
        let ready_instructions = cpu.reservation_stations.get_ready_instructions();

        if let Some((tag, instruction, vj, vk)) = ready_instructions.first() {
            let result = self.compute_result(*instruction, *vj, *vk, cpu);

            // Broadcast on CDB
            cpu.common_data_bus.broadcast(*tag, result);

            // Update ROB
            cpu.reorder_buffer.complete(*tag, Some(result));

            // Clear the reservation station
            self.clear_reservation_station_by_tag(cpu, *tag);
        }

        // Update all components from CDB
        if cpu.common_data_bus.valid {
            cpu.reservation_stations.update_from_cdb(cpu.common_data_bus.tag, cpu.common_data_bus.value);
            cpu.rename_table.update_from_cdb(cpu.common_data_bus.tag);
        }
    }

    // Commit Stage: Update architectural state in program order
    fn commit_stage(&mut self, cpu: &mut CpuState, mem: &mut Memory) {
        if let Some(entry) = cpu.reorder_buffer.commit() {
            if let Some(instruction) = entry.instruction {
                match instruction {
                    crate::isa::Instruction::Store { src: _, addr } => {
                        if let Some(value) = entry.result {
                            mem.store_u16(addr, value);
                        }
                    },
                    _ => {
                        // Update register file for non-store instructions
                        if let (Some(reg), Some(value)) = (entry.dest_reg, entry.result) {
                            cpu.regs.write(reg, value);
                        }
                    }
                }
            }
        }
    }

    fn clear_reservation_station_by_tag(&self, cpu: &mut CpuState, tag: usize) {
        for rs in &mut cpu.reservation_stations.alu_stations {
            if rs.tag == tag && rs.busy {
                rs.clear();
                return;
            }
        }

        for rs in &mut cpu.reservation_stations.load_stations {
            if rs.tag == tag && rs.busy {
                rs.clear();
                return;
            }
        }

        for rs in &mut cpu.reservation_stations.store_stations {
            if rs.tag == tag && rs.busy {
                rs.clear();
                return;
            }
        }
    }

    fn get_execution_cycles(&self, instruction: &Instruction) -> u32 {
        use crate::isa::Instruction;
        match instruction {
            Instruction::Add { .. } | Instruction::Sub { .. } | 
            Instruction::And { .. } | Instruction::Or { .. } | 
            Instruction::Xor { .. } | Instruction::Not { .. } |
            Instruction::AddImm { .. } | Instruction::SubImm { .. } => 1,

            Instruction::Load { .. } => 2,
            Instruction::Store { .. } => 1,
            _ => 1,
        }
    }

    fn get_immediate_value(&self, instruction: &Instruction) -> u16 {
        use crate::isa::Instruction;
        match instruction {
            Instruction::AddImm { imm, .. } | Instruction::SubImm { imm, .. } => *imm as u16,
            _ => 0,
        }
    }

    fn compute_result(&self, instruction: Instruction, vj: u16, vk: u16, cpu: &CpuState) -> u16 {
        use crate::isa::Instruction;
        match instruction {
            Instruction::Add { .. } => crate::core::alu::add(vj, vk).0,
            Instruction::Sub { .. } => crate::core::alu::sub(vj, vk).0,
            Instruction::And { .. } => crate::core::alu::and(vj, vk),
            Instruction::Or { .. } => crate::core::alu::or(vj, vk),
            Instruction::Xor { .. } => crate::core::alu::xor(vj, vk),
            Instruction::Not { .. } => crate::core::alu::not(vj),
            Instruction::AddImm { imm, .. } => crate::core::alu::add(vj, imm as u16).0,
            Instruction::SubImm { imm, .. } => crate::core::alu::sub(vj, imm as u16).0,
            Instruction::Load { addr, .. } => {
                // For loads, vj contains the address
                cpu.regs.read(0) // Placeholder - should read from memory
            },
            _ => 0,
        }
    }
}