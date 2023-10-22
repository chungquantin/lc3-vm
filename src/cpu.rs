use crate::constant;
use crate::constant::NEGATIVE_BIT;
use crate::register::{LC3CPURegister, LC3ConditionalFlags};

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub(crate) struct LC3Cpu {
    /** Registers have a size of 17 bit **/
    pub registers: [u16; constant::CPU_REGISTER_COUNT],
    pub opcodes: [u16; constant::CPU_OPCODE_COUNT],
    pub stack: [u16; constant::MEMORY_MAX],
}

impl Default for LC3Cpu {
    fn default() -> Self {
        return LC3Cpu {
            opcodes: [0; constant::CPU_OPCODE_COUNT],
            registers: [0; constant::CPU_REGISTER_COUNT],
            stack: [0; constant::MEMORY_MAX],
        };
    }
}

impl LC3Cpu {
    pub fn update_flags(self: &Self, register: u16) {
        if self.registers[register] == 0 {
            self.registers[LC3CPURegister::COND] = LC3ConditionalFlags::ZRO;
        } else if self.registers[register] >> 15 == NEGATIVE_BIT {
            /** the left-most bit indicates negative (1) **/
            self.registers[LC3CPURegister::COND] = LC3ConditionalFlags::NEG;
        } else {
            self.registers[LC3CPURegister::COND] = LC3ConditionalFlags::POS;
        }
    }

    pub fn mem_read(self: &Self, sp: u16) -> u16 {
    }

    pub fn mem_write(self: &Self, sp: u16, data: u16) {}
}
