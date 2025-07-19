pub fn add(a: u16, b: u16) -> (u16, bool) {
    let result = a.wrapping_add(b);
    let carry = (a as u32 + b as u32) > 0xFFFF;
    (result, carry)
}

pub fn sub(a: u16, b: u16) -> (u16, bool) {
    let result = a.wrapping_sub(b);
    let carry = a < b;
    (result, carry)
}

pub fn and(a: u16, b: u16) -> u16 {
    a & b
}

pub fn or(a: u16, b: u16) -> u16 {
    a | b
}

pub fn xor(a: u16, b: u16) -> u16 {
    a ^ b
}

pub fn not(a: u16) -> u16 {
    !a
}

pub fn shift_left(a: u16, amount: u16) -> u16 {
    if amount >= 16 {
        0
    } else {
        a << amount
    }
}

pub fn shift_right(a: u16, amount: u16) -> u16 {
    if amount >= 16 {
        0
    } else {
        a >> amount
    }
}
