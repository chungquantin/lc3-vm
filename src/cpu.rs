use crate::constant;
use crate::constant::NEGATIVE_BIT;
use crate::register::{LC3CPURegister::*, LC3ConditionalFlags::*, MemoryMappedRegister};
use std::io::Read;

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

    fn handle_keyboard(&mut self) {
        let mut buffer = [0; 1];
        std::io::stdin().read_exact(&mut buffer).unwrap();
        if buffer[0] != 0 {
            self.mem_write(MemoryMappedRegister::KBSR as u16, 1 << 15);
            self.mem_write(MemoryMappedRegister::KBDR as u16, buffer[0] as u16);
        } else {
            self.mem_write(MemoryMappedRegister::KBSR as u16, 0)
        }
    }

    pub fn mem_read(self: &mut Self, address: u16) -> u16 {
        if address == MemoryMappedRegister::KBSR as u16 {
            self.handle_keyboard();
        }
        self.memory[address as usize]
    }

    pub fn mem_write(self: &mut Self, address: u16, data: u16) {
        self.memory[address as usize] = data;
    }
}
