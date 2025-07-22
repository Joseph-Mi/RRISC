pub fn sign_extend_10(val: u16) -> i16 {
    let sign_bit = 1 << 9;
    if (val & sign_bit) == 1 {
        (val | !0x3FF) as i16
    } else {
        val as i16
    }
}