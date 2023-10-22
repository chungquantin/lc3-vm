use crate::cpu::LC3Cpu;
use crate::register::LC3CPURegister;
use libc::{c_char, c_int, fflush, getchar, putchar, puts, STDOUT_FILENO};
use std::ffi::CString;
use std::process;

/**
You may be wondering why the trap codes are not included in the instructions. This is because they do not actually introduce any new functionality to the LC-3, they just provide a convenient way to perform a task (similar to OS system calls)
 **/

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

    /** Trap routine is a special interrupt that sends the signal to switch to kernel mode and switch back to user land when the execution finishes **/
    pub fn execute(cpu: &LC3Cpu, trap_code_bytes: u16) {
        /**
        When a trap code is called, the PC is moved to that code’s address. The CPU executes the procedure’s instructions, and when it is complete, the PC is reset to the location following the initial call.
        **/
        let Some(trap_code) = TrapRoutine::from_bytes(trap_code_bytes);
        match trap_code {
            TrapRoutine::GETC => {
                cpu.registers[LC3CPURegister::R0] = unsafe {
                    getchar();
                };
                cpu.update_flags(LC3CPURegister::R0 as u16);
            }
            TrapRoutine::IN => {
                println!("Enter a character:");
                let c = unsafe { getchar() };
                unsafe {
                    putchar(c);
                    fflush(STDOUT_FILENO as *mut libc::FILE);
                }
                cpu.registers[LC3CPURegister::R0] = c;
                cpu.update_flags(LC3CPURegister::R0 as u16);
            }
            TrapRoutine::OUT => unsafe {
                putchar(cpu.registers[LC3CPURegister::R0]);
                fflush(STDOUT_FILENO as *mut libc::FILE)
            },
            TrapRoutine::PUTS => {
                let mut index = cpu.registers[LC3CPURegister::R0];
                let mut c = cpu.mem_read(index);
                while c != 0x0000 {
                    unsafe { putchar(c as c_int) };
                    index += 1;
                    c = cpu.mem_read(index);
                }
                unsafe {
                    fflush(STDOUT_FILENO as *mut libc::FILE);
                }
            }
            TrapRoutine::PUTSP => {
                let mut index = cpu.registers[LC3CPURegister::R0];
                let mut c = cpu.mem_read(index);
                while index != 0x0000 {
                    let c1 = c & 0xFF;
                    unsafe { putchar(c1 as c_int) };
                    let c2 = (c >> 8) as u8 as char;
                    if c2 != '\0' {
                        unsafe { putchar(c2 as c_int) };
                    }
                    index += 1;
                    c = cpu.mem_read(index);
                }
            }
            TrapRoutine::HALT => {
                /** Convert Rust &str to CString. Reference: https://users.rust-lang.org/t/converting-str-to-const-c-char/23115/3 **/
                let c_str = CString::new("HALT").unwrap();
                let c_world: *const c_char = c_str.as_ptr() as *const c_char;
                unsafe {
                    puts(c_world);
                    fflush(STDOUT_FILENO as *mut libc::FILE);
                }
                process::exit(1);
            }
        }
    }
}
