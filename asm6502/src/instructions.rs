use asm6502_derive::Asm6502;

#[derive(Debug, PartialEq)]
pub enum AddressMode {
    Implicit,
    Accumulator,
    Immediate(u8),
    // zero page
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

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(
    immediate = 0x29,
    zero = 0x25,
    zero_x = 0x35,
    absolute = 0x2D,
    absolute_x = 0x3D,
    absolute_y = 0x39,
    indirect_x = 0x21,
    indirect_y = 0x31
)]
struct AND(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(
    accumulator = 0x0A,
    zero = 0x06,
    zero_x = 0x16,
    absolute = 0x0E,
    absolute_x = 0x1E
)]
struct ASL(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(relative = 0x90)]
struct BCC(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(relative = 0xB0)]
struct BCS(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(relative = 0xF0)]
struct BEQ(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(zero = 0x24, absolute = 0x2C)]
struct BIT(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(relative = 0x30)]
struct BMI(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(relative = 0xD0)]
struct BNE(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(relative = 0x10)]
struct BPL(AddressMode);
#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(implicit = 0x00)]
struct BRK(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(relative = 0x50)]
struct BVC(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(relative = 0x70)]
struct BVS(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(implicit = 0x18)]
struct CLC(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(implicit = 0xD8)]
struct CLD(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(implicit = 0x58)]
struct CLI(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(implicit = 0xB8)]
struct CLV(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(
    immediate = 0xC9,
    zero = 0xC5,
    zero_x = 0xD5,
    absolute = 0xCD,
    absolute_x = 0xDD,
    absolute_y = 0xD9,
    indirect_x = 0xC1,
    indirect_y = 0xD1
)]
struct CMP(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(immediate = 0xE0, zero = 0xE4, absolute = 0xEC)]
struct CPX(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(immediate = 0xC0, zero = 0xC4, absolute = 0xCC)]
struct CPY(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(zero = 0xC6, zero_x = 0xD6, absolute = 0xCE, absolute_x = 0xDE)]
struct DEC(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(implicit = 0xCA)]
struct DEX(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(implicit = 0x88)]
struct DEY(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(
    immediate = 0x49,
    zero = 0x45,
    zero_x = 0x55,
    absolute = 0x4D,
    absolute_x = 0x5D,
    absolute_y = 0x59,
    indirect_x = 0x41,
    indirect_y = 0x51
)]
struct EOR(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(zero = 0xE6, zero_x = 0xF6, absolute = 0xEE, absolute_x = 0xFE)]
struct INC(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(implicit = 0xE8)]
struct INX(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(implicit = 0xC8)]
struct INY(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(absolute = 0x4C, indirect = 0x6C)]
struct JMP(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(absolute = 0x20)]
struct JSR(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(
    immediate = 0xA9,
    zero = 0xA5,
    zero_x = 0xB5,
    absolute = 0xAD,
    absolute_x = 0xBD,
    absolute_y = 0xB9,
    indirect_x = 0xA1,
    indirect_y = 0xB1
)]
struct LDA(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(
    immediate = 0xA2,
    zero = 0xA6,
    zero_y = 0xB6,
    absolute = 0xAE,
    absolute_y = 0xBE
)]
struct LDX(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(
    immediate = 0xA0,
    zero = 0xA4,
    zero_x = 0xB4,
    absolute = 0xAC,
    absolute_x = 0xBC
)]
struct LDY(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(
    accumulator = 0x4A,
    zero = 0x46,
    zero_x = 0x56,
    absolute = 0x4E,
    absolute_x = 0x5E
)]
struct LSR(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(implicit = 0xEA)]
struct NOP(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(
    immediate = 0x09,
    zero = 0x05,
    zero_x = 0x15,
    absolute = 0x0D,
    absolute_x = 0x1D,
    absolute_y = 0x19,
    indirect_x = 0x01,
    indirect_y = 0x11
)]
struct ORA(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(implicit = 0x48)]
struct PHA(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(implicit = 0x08)]
struct PHP(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(implicit = 0x68)]
struct PLA(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(implicit = 0x28)]
struct PLP(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(
    accumulator = 0x2A,
    zero = 0x26,
    zero_x = 0x36,
    absolute = 0x2E,
    absolute_x = 0x3E
)]
struct ROL(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(
    accumulator = 0x6A,
    zero = 0x66,
    zero_x = 0x76,
    absolute = 0x6E,
    absolute_x = 0x7E
)]
struct ROR(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(implicit = 0x40)]
struct RTI(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(implicit = 0x60)]
struct RTS(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(
    immediate = 0xE9,
    zero = 0xE5,
    zero_x = 0xF5,
    absolute = 0xED,
    absolute_x = 0xFD,
    absolute_y = 0xF9,
    indirect_x = 0xE1,
    indirect_y = 0xF1
)]
struct SBC(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(implicit = 0x38)]
struct SEC(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(implicit = 0xF8)]
struct SED(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(implicit = 0x78)]
struct SEI(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(
    zero = 0x85,
    zero_x = 0x95,
    absolute = 0x8D,
    absolute_x = 0x9D,
    absolute_y = 0x99,
    indirect_x = 0x81,
    indirect_y = 0x91
)]
struct STA(AddressMode);
#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(zero = 0x86, zero_y = 0x96, absolute = 0x8E)]
struct STX(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(zero = 0x84, zero_x = 0x94, absolute = 0x8C)]
struct STY(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(implicit = 0xAA)]
struct TAX(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(implicit = 0xA8)]
struct TAY(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(implicit = 0xBA)]
struct TSX(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(implicit = 0x8A)]
struct TXA(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(implicit = 0x9A)]
struct TXS(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(implicit = 0x98)]
struct TYA(AddressMode);

#[test]
fn test_adc() {
    use AddressMode::*;

    println!("{}", ADC::from_bytes(b"\x69\xc4").unwrap());
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
