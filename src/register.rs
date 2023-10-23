/**
LC-3 has 10 total registers, each of which is 16 bits.
Most of them are general purpose, but a few have designated roles.
 **/
#[allow(non_camel_case_types)]
pub(crate) enum LC3CPURegister {
    /** General purpose register (R0 - R7) **/
    R0 = 0x0,
    R1 = 0x1,
    R2 = 0x2,
    R3 = 0x3,
    R4 = 0x4,
    R5 = 0x5,
    R6 = 0x6,
    R7 = 0x7,
    /** Program counter register **/
    PC = 0x8,
    /** Conditional register **/
    COND = 0x9,
}

/// Memory Mapped Register: Some special registers are not accessible from the normal register table.
/// Instead, a special address is reserved for them in memory.
pub(crate) enum MemoryMappedRegister {
    KBSR = 0xFE00, /* keyboard status */
    KBDR = 0xFE02, /* keyboard data */
}

/** The LC-3 uses only 3 condition flags which indicate the sign of the previous calculation.
Why are we storing 1-2-4 instead of 1-2-3 ? Because the conditional flags are represented in a bit set format `nzp` not the index like register. Hence, 1 - 2 -4 => 111 => Three states: nz1 - n1p - 1zp
**/
#[allow(non_camel_case_types)]
pub(crate) enum LC3ConditionalFlags {
    POS = 1 << 0, /* P */
    ZRO = 1 << 1, /* Z */
    NEG = 1 << 2, /* N */
}
