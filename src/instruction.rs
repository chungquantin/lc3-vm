use crate::constant;

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
            0b1101 => LC3Instruction::RES,
            0b0001 => LC3Instruction::ADD,
            0b0101 => LC3Instruction::AND,
            0b0000 => LC3Instruction::BR,
            0b1100 => LC3Instruction::JMP,
            0b0100 => LC3Instruction::JSR,
            0b0010 => LC3Instruction::LD,
            0b1010 => LC3Instruction::LDI,
            0b0110 => LC3Instruction::LDR,
            0b1110 => LC3Instruction::LEA,
            0b1001 => LC3Instruction::NOT,
            0b1000 => LC3Instruction::RTI,
            0b0011 => LC3Instruction::ST,
            0b1011 => LC3Instruction::STI,
            0b0111 => LC3Instruction::STR,
            0b1111 => LC3Instruction::TRAP,
            _ => panic!("No opcode found {}", opcode),
        })
    }
}
