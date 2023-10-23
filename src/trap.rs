use crate::cpu::LC3Cpu;
use crate::register::LC3CPURegister;
use crate::register::LC3CPURegister::*;
use std::io::{Read, Write};
use std::{io, process};

#[derive(Debug)]
/// You may be wondering why the trap codes are not included in the instructions. This is because they do not actually introduce any new functionality to the LC-3, they just provide a convenient way to perform a task (similar to OS system calls)
pub(crate) enum TrapRoutine {
    GETC,  /* get character from keyboard, not echoed onto the terminal */
    OUT,   /* output a character */
    PUTS,  /* output a word string */
    IN,    /* get character from keyboard, echoed onto the terminal */
    PUTSP, /* output a byte string */
    HALT,  /* halt the program */
}

impl TrapRoutine {
    pub fn from_bytes(trap_code_bytes: u16) -> Option<Self> {
        Some(match trap_code_bytes {
            0x20 => TrapRoutine::GETC, /* get character from keyboard, not echoed onto the terminal */
            0x21 => TrapRoutine::OUT,  /* output a character */
            0x22 => TrapRoutine::PUTS, /* output a word string */
            0x23 => TrapRoutine::IN,   /* get character from keyboard, echoed onto the terminal */
            0x24 => TrapRoutine::PUTSP, /* output a byte string */
            0x25 => TrapRoutine::HALT, /* halt the program */
            _ => panic!("No trap code found {}", trap_code_bytes),
        })
    }

    ///Trap routine is a special interrupt that sends the signal to switch to kernel mode and switch back to user land when the execution finishes
    pub fn execute(cpu: &mut LC3Cpu, trap_code_bytes: u16) {
        // When a trap code is called, the PC is moved to that code’s address. The CPU executes the procedure’s instructions, and when it is complete, the PC is reset to the location following the initial call.
        let trap_code_option = TrapRoutine::from_bytes(trap_code_bytes);
        match trap_code_option {
            Some(trap_code) => {
                println!("TRAP: {:?}", trap_code);
                match trap_code {
                    TrapRoutine::GETC => {
                        let mut buffer = [0; 1];
                        io::stdin().read_exact(&mut buffer).unwrap();
                        cpu.registers[R0 as usize] = buffer[0] as u16;
                    }
                    TrapRoutine::IN => {
                        print!("Enter a  character : ");
                        io::stdout().flush().expect("failed to flush");
                        let char = io::stdin()
                            .bytes()
                            .next()
                            .and_then(|result| result.ok())
                            .map(|byte| byte as u16)
                            .unwrap();
                        cpu.registers[LC3CPURegister::R0 as usize] = char;
                    }
                    TrapRoutine::OUT => {
                        let c = cpu.registers[R0 as usize] as u8;
                        print!("{}", c as char);
                    }
                    TrapRoutine::PUTS => {
                        let mut index = cpu.registers[R0 as usize];
                        let mut c = cpu.mem_read(index);
                        while c != 0x0000 {
                            print!("{}", (c as u8) as char);
                            index += 1;
                            c = cpu.mem_read(index);
                        }
                        io::stdout().flush().expect("Failed to flush");
                    }
                    TrapRoutine::PUTSP => {
                        let mut index = cpu.registers[R0 as usize];
                        let mut c = cpu.mem_read(index);
                        while c != 0x0000 {
                            let c1 = ((c & 0xFF) as u8) as char;
                            print!("{}", c1);
                            let c2 = ((c >> 8) as u8) as char;
                            if c2 != '\0' {
                                print!("{}", c2);
                            }
                            index += 1;
                            c = cpu.mem_read(index);
                        }
                        io::stdout().flush().expect("failed to flush");
                    }
                    TrapRoutine::HALT => {
                        println!("HALT detected");
                        io::stdout().flush().expect("failed to flush");
                        process::exit(1);
                    }
                }
            }
            None => panic!("No trap code found"),
        }
    }
}
