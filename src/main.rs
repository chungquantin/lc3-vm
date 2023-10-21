use crate::cond::LC3ConditionalFlags;
use crate::constant::{IMMEDIATE_MODE, NEGATIVE_BIT, REGISTER_MODE};
use crate::cpu::LC3Cpu;
use crate::instruction::LC3Instruction;
use crate::register::LC3CPURegister;

mod cond;
mod constant;
mod cpu;
/** Little Computer 3 VM written in Rust
Read technical reference here: https://en.wikipedia.org/wiki/Little_Computer_3
Instruction set architecture reference: https://www.jmeiners.com/lc3-vm/supplies/lc3-isa.pdf
**/
mod instruction;
mod register;

fn mem_read(data: u16) -> u16 {
    0
}

fn mem_write(sp: u16, data: u16) {}

fn load_image(path: String) {}

/** Read image from a provided input path **/
fn read_image_path() -> String {
    /** Load image from user input path **/
    let mut line = String::new();
    println!("Enter your name :");
    let no_of_bytes = std::io::stdin().read_line(&mut line).unwrap();
    /** If there is no input provided, throw error exit the program **/
    if no_of_bytes == 0 {
        panic!("Program image path can't be empty");
    }
    return line;
}

/** Function to sign extend a 16 bit integer
 - x: unsigned 16-bit integer
 - bit_count: number of significant bits in `x`. How many bits of `x` should be considered when performing the sign extension
  e.g. x = 1100100 with the bit_count = 6 => 1 is the right most index in the bit set.
**/
pub fn sign_extend(mut x: u16, bit_count: i32) -> u16 {
    /// Get the rightmost bit index in the bit set and check if the value of the bit is 1 (negative) or 0 (positive)
    /// - 0xFFFF in hexadecimal = 1111 1111 1111 1111 in binary
    /// - If the sign is negative (0) => do OR operation to bit mask with `bit_count` most significant bits set to 1
    if (x >> (bit_count - 1)) & 1 == NEGATIVE_BIT {
        x |= (0xFFFF << bit_count);
    }
    x
}

fn main() {
    let cpu = LC3Cpu::default();
    /** Conditional flag always requires a value, set a zero flag by default **/
    cpu.registers[LC3CPURegister::COND] = LC3ConditionalFlags::ZRO;

    /** Set the PC to starting position => 0x3000 is the default **/
    cpu.registers[LC3CPURegister::PC] = constant::PROGRAM_COUNTER_START;

    /** User console **/
    let path = read_image_path();
    load_image(path);

    loop {
        cpu.registers[LC3CPURegister::PC] += 1;
        let bit_set: u16 = mem_read(cpu.registers[LC3CPURegister::PC]);
        let Some(instruction) = LC3Instruction::from_bytes(bit_set);
        match instruction {
            LC3Instruction::ADD => {
                let dr = (bit_set >> 9) & 0x7;
                let sr1 = (bit_set >> 6) & 0x7;
                let mode = (bit_set >> 5) & 0x1;
                match mode {
                    IMMEDIATE_MODE => {
                        let imm5 = sign_extend(bit_set & 0x1F, 5);
                        cpu.registers[dr] = cpu.registers[sr1] + imm5;
                    }
                    REGISTER_MODE => {
                        let sr2: u16 = bit_set & 0x7;
                        cpu.registers[dr] = cpu.registers[sr1] + cpu.registers[sr2];
                    }
                    _ => panic!("Invalid mode"),
                }
                cpu.update_flags(dr);
            } /* add  */
            LC3Instruction::AND => {
                let dr = (bit_set >> 9) & 0x7;
                let sr1 = (bit_set >> 6) & 0x7;
                let mode = (bit_set >> 5) & 0x1;
                match mode {
                    IMMEDIATE_MODE => {
                        let imm5 = sign_extend(bit_set & 0x1F, 5);
                        cpu.registers[dr] = cpu.registers[sr1] && imm5;
                    }
                    REGISTER_MODE => {
                        let sr2: u16 = bit_set & 0x7;
                        cpu.registers[dr] = cpu.registers[sr1] && cpu.registers[sr2];
                    }
                    _ => panic!("Invalid mode"),
                }
                cpu.update_flags(dr);
            } /* bitwise and */
            LC3Instruction::BR => {
                // If any of the condition codes tested is set, the program branches to the location
                // specified by adding the sign-extended pc_offset_9 field to the incremented PC.
                let cond_flag = (bit_set >> 9) & 0x7;
                let pc_offset_9 = sign_extend(bit_set & 0x1FF, 9);
                if cond_flag && cpu.registers[LC3CPURegister::COND] {
                    cpu.registers[LC3CPURegister::PC] += pc_offset_9;
                }
            } /* branch */
            LC3Instruction::JMP => {
                // The program unconditionally jumps to the location specified by the contents of the base register
                let base_register = (bit_set >> 6) & 0x7;
                cpu.registers[LC3CPURegister::PC] = base_register;
            } /* jump */
            LC3Instruction::LD => {
                let dr = (bit_set >> 9) & 0x7;
                let pc_offset_9 = sign_extend(bit_set & 0x1FF, 9);
                cpu.registers[dr] = mem_read(cpu.registers[LC3CPURegister::PC] + pc_offset_9);
                cpu.update_flags(dr);
            } /* load */
            LC3Instruction::ST => {
                let sr = (instruction >> 9) & 0x7;
                let pc_offset = sign_extend(instruction & 0x1FF, 9);
                mem_write(
                    cpu.registers[LC3CPURegister::PC] + pc_offset,
                    cpu.registers[sr],
                );
            } /* store */
            LC3Instruction::JSR => {
                let mode = (bit_set >> 11) & 0x1;
                cpu.registers[LC3CPURegister::R7] = cpu.registers[LC3CPURegister::PC];
                match mode {
                    IMMEDIATE_MODE => {
                        let pc_offset_11 = sign_extend(bit_set & 0x7FF, 11);
                        cpu.registers[LC3CPURegister::PC] += pc_offset_11;
                    }
                    REGISTER_MODE => {
                        let base_r = (bit_set >> 6) & 0x7;
                        cpu.registers[LC3CPURegister::PC] += base_r;
                    }
                    _ => panic!("Invalid mode"),
                }
            } /* jump register */
            LC3Instruction::LDR => {
                let dr = (instruction >> 9) & 0x7;
                let base_r = (instruction >> 6) & 0x7;
                let offset_6 = sign_extend(instruction >> 0x3F, 6);
                cpu.registers[dr] = mem_read(cpu.registers[base_r] + offset_6);
                cpu.update_flags(dr);
            } /* load register */
            LC3Instruction::STR => {
                let sr = (instruction >> 9) & 0x7;
                let base_r = (instruction >> 6) & 0x7;
                let offset_6 = sign_extend(instruction >> 0x3F, 6);
                mem_write(cpu.registers[base_r] + offset_6, cpu.registers[sr]);
            } /* store register */
            LC3Instruction::NOT => {
                let dr = (instruction >> 9) & 0x7;
                let sr = (instruction >> 6) & 0x7;
                cpu.registers[dr] = !cpu.registers[sr];
                cpu.update_flags(dr);
            } /* bitwise not */
            LC3Instruction::LDI => {
                let dr = (instruction >> 9) & 0x7;
                let pc_offset_9 = sign_extend(instruction & 0x1FF, 9);
                cpu.registers[dr] = mem_read(cpu.registers[LC3CPURegister::PC] + pc_offset_9);
                cpu.update_flags(dr);
            } /* load indirect */
            LC3Instruction::STI => {
                let sr = (instruction >> 9) & 0x7;
                let pc_offset_9 = sign_extend(instruction & 0x1FF, 9);
                mem_write(
                    mem_read(cpu.registers[LC3CPURegister::PC] + pc_offset_9),
                    cpu.registers[sr],
                );
            } /* store indirect */
            LC3Instruction::RES => {}  /* reserved (unused) */
            LC3Instruction::LEA => {}  /* load effective address */
            LC3Instruction::TRAP => {
                let trapvect8 = instruction & 0xFF;
            } /* execute trap */
            _ => unimplemented!()
        }
    }
}
