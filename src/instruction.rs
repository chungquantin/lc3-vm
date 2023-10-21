use crate::constant;
use crate::constant::{IMMEDIATE_MODE, NEGATIVE_BIT, REGISTER_MODE};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum LC3Instruction {
    ADD,  /* add ( DR, SR1, mode, SR2 | imm5) */
    AND,  /* bitwise and */
    BR,   /* branch */
    LD,   /* load */
    ST,   /* store */
    JSR,  /* jump register */
    LDR,  /* load register */
    STR,  /* store register */
    RTI,  /* unused */
    NOT,  /* bitwise not */
    LDI,  /* load indirect */
    STI,  /* store indirect */
    JMP,  /* jump */
    RES,  /* reserved (unused) */
    LEA,  /* load effective address */
    TRAP, /* execute trap */
}

impl LC3Instruction {
    /// Decodes machine code bytes from the iterator to an Instruction.
    pub fn from_bytes(instruction_bytes: u16) -> Option<Self> {
        let right_shift_val = constant::CPU_INSTRUCTION_BIT_WIDTH - constant::CPU_OPCODE_BIT_SIZE;
        let opcode: u16 = instruction_bytes >> right_shift_val;
        /// Opcode is a first 4 bits
        Some(match opcode {
            1101 => LC3Instruction::RES,
            /// Structure: 0001 | DR (3 bits) | SR1 (3 bits) | mode (1 bit) | rest (SR 2 (2 bits left most) and imm5 (4 bits))
            0001 => LC3Instruction::ADD,
            0101 => LC3Instruction::AND,
            0000 => LC3Instruction::BR,
            1100 => LC3Instruction::JMP,
            0100 => LC3Instruction::JSR,
            0010 => LC3Instruction::LD,
            1010 => LC3Instruction::LDI,
            0110 => LC3Instruction::LDR,
            1110 => LC3Instruction::LEA,
            1001 => LC3Instruction::NOT,
            1000 => LC3Instruction::RTI,
            0011 => LC3Instruction::ST,
            1011 => LC3Instruction::STI,
            0111 => LC3Instruction::STR,
            1111 => LC3Instruction::TRAP,
            _ => panic!("No opcode found {}", opcode),
        })
    }
}
