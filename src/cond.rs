/** The LC-3 uses only 3 condition flags which indicate the sign of the previous calculation. **/
#[allow(non_camel_case_types)]
pub(crate) enum LC3ConditionalFlags {
    POS = 1 << 0, /* P */
    ZRO = 1 << 1, /* Z */
    NEG = 1 << 2, /* N */
}

