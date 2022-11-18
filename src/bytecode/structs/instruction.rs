use crate::bytecode::enums::{
    instruction_type::{InstructionType, INSTRUCTION_TYPE_MAP},
    opcode_type::{OpcodeType, OPCODE_TYPE_MAP},
};

pub struct Instruction {
    pub data: u32,
    pub opcode: OpcodeType,
    pub instruction_type: InstructionType,
    pub data_a: u8,
}

impl Instruction {
    pub fn new(data: u32) -> Self {
        Self {
            data,
            opcode: OPCODE_TYPE_MAP[(data & 0x3f) as usize],
            instruction_type: INSTRUCTION_TYPE_MAP[(data & 0x3f) as usize],
            data_a: ((data >> 6) & 0xff) as u8,
        }
    }
}
