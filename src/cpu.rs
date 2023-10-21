use crate::cond::LC3ConditionalFlags;
use crate::constant;
use crate::constant::{IMMEDIATE_MODE, NEGATIVE_BIT, REGISTER_MODE};
use crate::register::LC3CPURegister;

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub(crate) struct LC3Cpu {
    /** Registers have a size of 17 bit **/
    pub(crate) registers: [u16; constant::CPU_REGISTER_COUNT],
    pub(crate) opcodes: [u16; constant::CPU_OPCODE_COUNT],
}

impl Default for LC3Cpu {
    fn default() -> Self {
        return LC3Cpu {
            opcodes: [0; constant::CPU_OPCODE_COUNT],
            registers: [0; constant::CPU_REGISTER_COUNT],
        };
    }
}

impl LC3Cpu {
    pub fn update_flags(self: &mut Self, register: u16) {
        if self.registers[register] == 0 {
            self.registers[LC3CPURegister::COND] = LC3ConditionalFlags::ZRO;
        } else if self.registers[register] >> 15 == NEGATIVE_BIT {
            /** the left-most bit indicates negative (1) **/
            self.registers[LC3CPURegister::COND] = LC3ConditionalFlags::NEG;
        } else {
            self.registers[LC3CPURegister::COND] = LC3ConditionalFlags::POS;
        }
    }
}
