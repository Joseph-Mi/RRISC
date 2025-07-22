pub mod alu;
pub mod register_file;
pub mod control_unit;
pub mod execute;
pub mod tomasulo;  // Add the new tomasulo module

use crate::isa::Instruction;

// Re-export the Tomasulo components for easier access
pub use tomasulo::{
    ReservationStationPool, 
    ReorderBuffer, 
    RegisterRenameTable, 
    CommonDataBus, 
    PipelineController
};

#[derive(Debug)]
pub struct CpuState {
    // Existing components
    pub regs: register_file::RegisterFile,
    pub pc: u16,
    pub halted: bool,
    pub flags: StatusFlags,

    // Tomasulo components
    pub reservation_stations: ReservationStationPool,
    pub reorder_buffer: ReorderBuffer,
    pub rename_table: RegisterRenameTable,
    pub common_data_bus: CommonDataBus,

    // Pipeline controller
    pub pipeline: PipelineController,

    // Execution mode flag
    pub out_of_order_enabled: bool,
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
            reservation_stations: ReservationStationPool::new(),
            reorder_buffer: ReorderBuffer::new(16), // 16-entry ROB
            rename_table: RegisterRenameTable::new(256), // 256 registers
            common_data_bus: CommonDataBus::new(),
            pipeline: PipelineController::new(),
            out_of_order_enabled: false, // Start with in-order for compatibility
        }
    }

    pub fn set_flags_from_result(&mut self, result: u16) {
        self.flags.zero = result == 0;
        self.flags.negative = (result as i16) < 0;
    }

    // Method to enable out-of-order execution
    pub fn enable_out_of_order(&mut self) {
        self.out_of_order_enabled = true;
    }

    // Method to disable out-of-order execution (fallback to in-order)
    pub fn disable_out_of_order(&mut self) {
        self.out_of_order_enabled = false;
    }
} 