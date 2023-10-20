use crate::constant;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum LC3Instruction {
    BR,   /* branch */
    ADD,  /* add  */
    LD,   /* load */
    ST,   /* store */
    JSR,  /* jump register */
    AND,  /* bitwise and */
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
            0x0 => LC3Instruction::BR,
            0x1 => LC3Instruction::ADD,
            0x2 => LC3Instruction::LD,
            0x3 => LC3Instruction::ST,
            0x4 => LC3Instruction::JSR,
            0x5 => LC3Instruction::AND,
            0x6 => LC3Instruction::LDR,
            0x7 => LC3Instruction::STR,
            0x8 => LC3Instruction::RTI,
            0x9 => LC3Instruction::NOT,
            0x10 => LC3Instruction::LDI,
            0x11 => LC3Instruction::STI,
            0x12 => LC3Instruction::JMP,
            0x13 => LC3Instruction::RES,
            0x14 => LC3Instruction::LEA,
            0x15 => LC3Instruction::TRAP,
            _ => panic!("No opcode found {}", opcode),
        })
    }
}
