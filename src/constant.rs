/**
Microprocessor: 16-bit microprocessors
Addressable memory: 16-bit memory
Spec: LC-3 has 2^16 = 65,536 memory locations
=> Memory max is around 128KB

How to calculate the maximum memory?

1. 65536 * 16 bit-memory = 1048576 total bits required
2. 1048576 /  8 = 131072 / 1024 = 128 KB

 **/
pub(crate) const MEMORY_MAX: usize = 1 << 16;
pub(crate) const CPU_REGISTER_COUNT: usize = 10;
/**
- There are just 16 opcodes in LC-3.
- Each instruction is 16 bits long, with the left 4 bits storing the opcode.
- The rest of the bits are used to store the parameters.
 **/
pub(crate) const CPU_OPCODE_COUNT: usize = 16;
pub(crate) const CPU_INSTRUCTION_BIT_WIDTH: usize = 16;
pub(crate) const CPU_OPCODE_BIT_SIZE: usize = 4;
/**
 The lower addresses are left empty to leave space for the trap routine code.
**/
pub(crate) const PROGRAM_COUNTER_START: i32 = 0x3000;

pub(crate) const POSITIVE_BIT: u16 = 0;
pub(crate) const NEGATIVE_BIT: u16 = 1;
pub(crate) const IMMEDIATE_MODE: u16 = 1;
pub(crate) const REGISTER_MODE: u16 = 0;
