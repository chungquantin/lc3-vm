use crate::instruction::LC3Instruction;
mod constant;
/** Little Computer 3 VM written in Rust
Read technical reference here: https://en.wikipedia.org/wiki/Little_Computer_3
**/
mod instruction;

/**
LC-3 has 10 total registers, each of which is 16 bits.
Most of them are general purpose, but a few have designated roles.
 **/
#[allow(non_camel_case_types)]
enum LC3CPURegister {
    /** General purpose register (R0 - R7) **/
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    /** Program counter register **/
    PC,
    /** Conditional register **/
    COND,
}

/** The LC-3 uses only 3 condition flags which indicate the sign of the previous calculation. **/
#[allow(non_camel_case_types)]
enum LC3ConditionalFlags {
    POS = 1 << 0, /* P */
    ZRO = 1 << 1, /* Z */
    NEG = 1 << 2, /* N */
}

#[allow(non_camel_case_types)]
struct LC3Cpu {
    /** Registers have a size of 17 bit **/
    registers: [u16; constant::CPU_REGISTER_COUNT],
    opcodes: [u16; constant::CPU_OPCODE_COUNT],
}

impl Default for LC3Cpu {
    fn default() -> Self {
        return LC3Cpu {
            opcodes: [0; constant::CPU_OPCODE_COUNT],
            registers: [0; constant::CPU_REGISTER_COUNT],
        };
    }
}

fn fetch(data: u16) -> u16 {
    0
}

fn main() {
    let cpu = LC3Cpu::default();
    /** Conditional flag always requires a value, set a zero flag by default **/
    cpu.registers[LC3CPURegister::COND] = LC3ConditionalFlags::ZRO;

    /** Set the PC to starting position => 0x3000 is the default **/
    cpu.registers[LC3CPURegister::PC] = constant::PROGRAM_COUNTER_START;

    loop {
        cpu.registers[LC3CPURegister::PC] += 1;
        let instruction: u16 = fetch(cpu.registers[LC3CPURegister::PC]);
        let Some(instruction) = LC3Instruction::from_bytes(instruction);
        match instruction {
            LC3Instruction::BR => {}   /* branch */
            LC3Instruction::ADD => {}  /* add  */
            LC3Instruction::LD => {}   /* load */
            LC3Instruction::ST => {}   /* store */
            LC3Instruction::JSR => {}  /* jump register */
            LC3Instruction::AND => {}  /* bitwise and */
            LC3Instruction::LDR => {}  /* load register */
            LC3Instruction::STR => {}  /* store register */
            LC3Instruction::RTI => {}  /* unused */
            LC3Instruction::NOT => {}  /* bitwise not */
            LC3Instruction::LDI => {}  /* load indirect */
            LC3Instruction::STI => {}  /* store indirect */
            LC3Instruction::JMP => {}  /* jump */
            LC3Instruction::RES => {}  /* reserved (unused) */
            LC3Instruction::LEA => {}  /* load effective address */
            LC3Instruction::TRAP => {} /* execute trap */
        }
    }
}
