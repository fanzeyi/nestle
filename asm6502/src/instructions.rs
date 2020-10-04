use asm6502_derive::Asm6502;

#[derive(Debug, PartialEq)]
pub enum AddressMode {
    Implicit,
    Accumulator,
    Immediate(u8),
    Zero(u8),
    ZeroX(u8),
    ZeroY(u8),
    Relative(u8),
    Absolute(u16),
    AbsoluteX(u16),
    AbsoluteY(u16),
    Indirect(u16),
    IndirectX(u8),
    IndirectY(u8),
}

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(
    immediate = 0x69,
    zero = 0x65,
    zero_x = 0x75,
    absolute = 0x6d,
    absolute_x = 0x7d,
    absolute_y = 0x79,
    indirect_x = 0x61,
    indirect_y = 0x71
)]
struct ADC(AddressMode);

#[test]
fn test_from() {
    use AddressMode::*;

    assert_eq!(ADC::from_bytes(b"\x69\x01").unwrap(), ADC(Immediate(0x01)));
    assert_eq!(ADC::from_bytes(b"\x65\x01").unwrap(), ADC(Zero(0x01)));
    assert_eq!(ADC::from_bytes(b"\x75\x01").unwrap(), ADC(ZeroX(0x01)));
    assert_eq!(
        ADC::from_bytes(b"\x6D\x01\x02").unwrap(),
        ADC(Absolute(0x0201))
    );
    assert_eq!(
        ADC::from_bytes(b"\x7D\x01\x02").unwrap(),
        ADC(AbsoluteX(0x0201))
    );
    assert_eq!(
        ADC::from_bytes(b"\x79\x01\x02").unwrap(),
        ADC(AbsoluteY(0x0201))
    );
}
