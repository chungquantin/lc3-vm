/**
Spec: LC-3 has 2^16 = 65,536 memory locations
=> Memory max is around 128KB
 **/
pub(crate) const MEMORY_MAX: u16 = 1 << 16;
pub(crate) const CPU_REGISTER_COUNT: usize = 10;
/**
- There are just 16 opcodes in LC-3.
- Each instruction is 16 bits long, with the left 4 bits storing the opcode.
- The rest of the bits are used to store the parameters.
 **/
pub(crate) const CPU_OPCODE_COUNT: usize = 16;
pub(crate) const CPU_INSTRUCTION_BIT_WIDTH: usize = 16;
pub(crate) const CPU_OPCODE_BIT_SIZE: usize = 4;

pub(crate) const PROGRAM_COUNTER_START: i32 = 0x3000;
