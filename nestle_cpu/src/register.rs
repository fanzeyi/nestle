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
pub struct Registers {
    pc: u16,   // 0x34
    stack: u8, // 0xFD
    acc: u8,   // 0
    idx_x: u8, // 0
    idx_y: u8, // 0
    status: Status,
}

impl Registers {
    pub fn with_pc(pc: u16) -> Self {
        Self {
            pc,
            ..Default::default()
        }
    }
}

impl Default for Registers {
    fn default() -> Self {
        Self {
            pc: 0x34,
            stack: 0xFD,
            acc: 0,
            idx_x: 0,
            idx_y: 0,
            status: Status::empty(),
        }
    }
}
