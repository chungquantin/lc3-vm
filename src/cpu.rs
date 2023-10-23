use crate::constant;
use crate::constant::NEGATIVE_BIT;
use crate::register::{LC3CPURegister::*, LC3ConditionalFlags::*};

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub(crate) struct LC3Cpu {
    /** Registers have a size of 17 bit **/
    pub registers: [u16; constant::CPU_REGISTER_COUNT],
    pub memory: [u16; constant::MEMORY_MAX],
}

impl Default for LC3Cpu {
    fn default() -> Self {
        return LC3Cpu {
            registers: [0; constant::CPU_REGISTER_COUNT],
            memory: [0; constant::MEMORY_MAX],
        };
    }
}

impl LC3Cpu {
    pub fn update_flags(self: &mut Self, register: u16) {
        if self.registers[register as usize] == 0 {
            self.registers[COND as usize] = ZRO as u16;
        } else if self.registers[register as usize] >> 15 == NEGATIVE_BIT {
            // the left-most bit indicates negative (1)
            self.registers[COND as usize] = NEG as u16;
        } else {
            self.registers[COND as usize] = POS as u16;
        }
    }

    pub fn mem_read(self: &Self, sp: u16) -> u16 {
        return 0;
    }

    pub fn mem_write(self: &Self, sp: u16, data: u16) {}
}
