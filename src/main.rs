/// Little Computer 3 VM written in Rust
/// Read technical reference here: https://en.wikipedia.org/wiki/Little_Computer_3Instruction set architecture reference: https://www.jmeiners.com/lc3-vm/supplies/lc3-isa.pdf
mod constant;
mod cpu;
mod instruction;
mod register;
mod trap;

use crate::constant::{IMMEDIATE_MODE, NEGATIVE_BIT, POSITIVE_BIT, REGISTER_MODE};
use crate::cpu::LC3Cpu;
use crate::instruction::LC3Instruction;
use crate::register::{LC3CPURegister::*, LC3ConditionalFlags};
use crate::trap::TrapRoutine;
use std::fs::File;
use std::io::BufReader;
use std::process::abort;

use byteorder::{BigEndian, ReadBytesExt};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,

    #[structopt(long)]
    print_asm: bool, // Future feature
}

/// Read image from a provided input path
fn load_image(cpu: &LC3Cpu) {
    let cli = Cli::from_args();

    let f = File::open(cli.path).expect("couldn't open file");
    let mut f = BufReader::new(f);

    // Note how we're using `read_u16` _and_ BigEndian to read the binary file.
    let base_address = f.read_u16::<BigEndian>().expect("error");

    // Here we're loading the program in memory
    let mut address = base_address;
    loop {
        match f.read_u16::<BigEndian>() {
            Ok(instruction) => {
                cpu.mem_write(address, instruction);
                address += 1;
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::UnexpectedEof {
                    println!("OK")
                } else {
                    println!("failed: {}", e);
                }
                break;
            }
        }
    }
}

/// Function to sign extend a 16 bit integer
/// - x: unsigned 16-bit integer
/// - bit_count: number of significant bits in `x`. How many bits of `x` should be considered when performing the sign extension
/// e.g. x = 1100100 with the bit_count = 6 => 1 is the right most index in the bit set.
pub fn sign_extend(mut x: u16, bit_count: i32) -> u16 {
    // Get the rightmost bit index in the bit set and check if the value of the bit is 1 (negative) or 0 (positive)
    // - 0xFFFF in hexadecimal = 1111 1111 1111 1111 in binary
    // - If the sign is negative (0) => do OR operation to bit mask with `bit_count` most significant bits set to 1
    if ((x >> (bit_count - 1)) & 1) == NEGATIVE_BIT {
        x |= 0xFFFF << bit_count;
    }
    x
}

fn main() {
    let mut cpu = LC3Cpu::default();
    // Conditional flag always requires a value, set a zero flag by default
    cpu.registers[COND as usize] = LC3ConditionalFlags::ZRO as u16;

    // Set the PC to starting position => 0x3000 is the default
    cpu.registers[PC as usize] = constant::PROGRAM_COUNTER_START;

    // User console
    load_image(&cpu);

    loop {
        cpu.registers[PC as usize] += 1;
        let instruction: u16 = cpu.mem_read(cpu.registers[PC as usize]);
        match LC3Instruction::from_bytes(instruction) {
            Some(opcode) => {
                match opcode {
                    LC3Instruction::ADD => {
                        let dr = (instruction >> 9) & 0x7;
                        let sr1 = (instruction >> 6) & 0x7;
                        let mode = (instruction >> 5) & 0x1;
                        match mode {
                            IMMEDIATE_MODE => {
                                let imm5 = sign_extend(instruction & 0x1F, 5);
                                cpu.registers[dr as usize] = cpu.registers[sr1 as usize] + imm5;
                            }
                            REGISTER_MODE => {
                                let sr2: u16 = instruction & 0x7;
                                cpu.registers[dr as usize] =
                                    cpu.registers[sr1 as usize] + cpu.registers[sr2 as usize];
                            }
                            _ => panic!("Invalid mode"),
                        }
                        cpu.update_flags(dr);
                    } /* add  */
                    LC3Instruction::AND => {
                        let dr = (instruction >> 9) & 0x7;
                        let sr1 = (instruction >> 6) & 0x7;
                        let mode = (instruction >> 5) & 0x1;
                        match mode {
                            IMMEDIATE_MODE => {
                                let imm5 = sign_extend(instruction & 0x1F, 5);
                                cpu.registers[dr as usize] = cpu.registers[sr1 as usize] & imm5;
                            }
                            REGISTER_MODE => {
                                let sr2: u16 = instruction & 0x7;
                                cpu.registers[dr as usize] =
                                    cpu.registers[sr1 as usize] & cpu.registers[sr2 as usize];
                            }
                            _ => panic!("Invalid mode"),
                        }
                        cpu.update_flags(dr);
                    } /* bitwise and */
                    LC3Instruction::BR => {
                        // If any of the condition codes tested is set, the program branches to the location
                        // specified by adding the sign-extended pc_offset_9 field to the incremented PC.
                        let cond_flag = (instruction >> 9) & 0x7;
                        let pc_offset_9 = sign_extend(instruction & 0x1FF, 9);
                        if cond_flag & cpu.registers[COND as usize] != POSITIVE_BIT {
                            cpu.registers[PC as usize] += pc_offset_9;
                        }
                    } /* branch */
                    LC3Instruction::JMP => {
                        // The program unconditionally jumps to the location specified by the contents of the base register
                        let base_register = (instruction >> 6) & 0x7;
                        cpu.registers[PC as usize] = cpu.registers[base_register as usize];
                    } /* jump */
                    LC3Instruction::LD => {
                        let dr = (instruction >> 9) & 0x7;
                        let pc_offset_9 = sign_extend(instruction & 0x1FF, 9);
                        cpu.registers[dr as usize] =
                            cpu.mem_read(cpu.registers[PC as usize] + pc_offset_9);
                        cpu.update_flags(dr);
                    } /* load */
                    LC3Instruction::ST => {
                        let sr = (instruction >> 9) & 0x7;
                        let pc_offset = sign_extend(instruction & 0x1FF, 9);
                        cpu.mem_write(
                            cpu.registers[PC as usize] + pc_offset,
                            cpu.registers[sr as usize],
                        );
                    } /* store */
                    LC3Instruction::JSR => {
                        let mode = (instruction >> 11) & 0x1;
                        cpu.registers[R7 as usize] = cpu.registers[PC as usize];
                        match mode {
                            IMMEDIATE_MODE => {
                                /* JSR */
                                let pc_offset_11 = sign_extend(instruction & 0x7FF, 11);
                                cpu.registers[PC as usize] += pc_offset_11;
                            }
                            REGISTER_MODE => {
                                /* JSRR */
                                let base_r = (instruction >> 6) & 0x7;
                                cpu.registers[PC as usize] += cpu.registers[base_r as usize];
                            }
                            _ => panic!("Invalid mode"),
                        }
                    } /* jump register */
                    LC3Instruction::LDR => {
                        let dr = (instruction >> 9) & 0x7;
                        let base_r = (instruction >> 6) & 0x7;
                        let offset_6 = sign_extend(instruction & 0x3F, 6);
                        cpu.registers[dr as usize] =
                            cpu.mem_read(cpu.registers[base_r as usize] + offset_6);
                        cpu.update_flags(dr);
                    } /* load register */
                    LC3Instruction::STR => {
                        let sr = (instruction >> 9) & 0x7;
                        let base_r = (instruction >> 6) & 0x7;
                        let offset_6 = sign_extend(instruction & 0x3F, 6);
                        cpu.mem_write(
                            cpu.registers[base_r as usize] + offset_6,
                            cpu.registers[sr as usize],
                        );
                    } /* store register */
                    LC3Instruction::NOT => {
                        let dr = (instruction >> 9) & 0x7;
                        let sr = (instruction >> 6) & 0x7;
                        cpu.registers[dr as usize] = !cpu.registers[sr as usize];
                        cpu.update_flags(dr);
                    } /* bitwise not */
                    LC3Instruction::LDI => {
                        let dr = (instruction >> 9) & 0x7;
                        let pc_offset_9 = sign_extend(instruction & 0x1FF, 9);
                        cpu.registers[dr as usize] =
                            cpu.mem_read(cpu.mem_read(cpu.registers[PC as usize] + pc_offset_9));
                        cpu.update_flags(dr);
                    } /* load indirect */
                    LC3Instruction::STI => {
                        let sr = (instruction >> 9) & 0x7;
                        let pc_offset_9 = sign_extend(instruction & 0x1FF, 9);
                        cpu.mem_write(
                            cpu.mem_read(cpu.registers[PC as usize] + pc_offset_9),
                            cpu.registers[sr as usize],
                        );
                    } /* store indirect */
                    LC3Instruction::LEA => {
                        let dr = (instruction >> 9) & 0x7;
                        let pc_offset_9 = sign_extend(instruction & 0x1FF, 9);
                        cpu.registers[dr as usize] = cpu.registers[PC as usize] + pc_offset_9;
                        cpu.update_flags(dr);
                    } /* load effective address */
                    LC3Instruction::RTI | LC3Instruction::RES => {
                        abort();
                    }
                    LC3Instruction::TRAP => {
                        TrapRoutine::execute(&mut cpu, instruction & 0xFF);
                    } /* execute trap */
                }
            }
            None => panic!("Invalid instruction opcode"),
        }
    }
}
