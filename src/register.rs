/**
LC-3 has 10 total registers, each of which is 16 bits.
Most of them are general purpose, but a few have designated roles.
 **/
#[allow(non_camel_case_types)]
pub(crate) enum LC3CPURegister {
    /** General purpose register (R0 - R7) **/
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    /** Program counter register **/
    PC,
    /** Conditional register **/
    COND,
}
