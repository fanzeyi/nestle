use bitflags::bitflags;

bitflags! {
    struct Status: u8 {
        const CARRY = 0b0000_0001;
        const ZERO = 0b0000_0010;
        const INTERRUPT_DISABLED = 0b0000_0100;
        const DECIMAL = 0b0000_1000;
        const BREAK = 0b0001_0000;
        const OVERFLOW = 0b0010_0000;
        const NEGATIVE = 0b0100_0000;
    }
}

#[derive(Debug)]
pub struct Register {
    pc: u16,
    stack: u8,
    acc: u8,
    idx_x: u8,
    idx_y: u8,
    status: Status,
}
