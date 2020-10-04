use asm6502_derive::Asm6502;
use enum_dispatch::enum_dispatch;

#[derive(Debug, PartialEq)]
pub enum AddressMode {
    Implicit,
    Accumulator,
    Immediate(u8),
    Zero(u8),
    ZeroX(u8),
    ZeroY(u8),
    Relative(i8),
    Absolute(u16),
    AbsoluteX(u16),
    AbsoluteY(u16),
    Indirect(u16),
    IndirectX(u8),
    IndirectY(u8),
}

impl AddressMode {
    pub fn size(&self) -> u8 {
        use AddressMode::*;
        match self {
            Implicit => 1,
            Accumulator => 1,
            Immediate(u8) => 2,
            Zero(u8) => 2,
            ZeroX(u8) => 2,
            ZeroY(u8) => 2,
            Relative(i8) => 2,
            Absolute(u16) => 3,
            AbsoluteX(u16) => 3,
            AbsoluteY(u16) => 3,
            Indirect(u16) => 3,
            IndirectX(u8) => 2,
            IndirectY(u8) => 2,
        }
    }
}

pub trait InstructionConstruct {
    fn from_peekable<'a, I: Iterator<Item = &'a u8> + 'a>(
        bytes: &mut std::iter::Peekable<I>,
    ) -> Option<Self>
    where
        Self: Sized;

    fn from_bytes(bytes: &[u8]) -> Option<Self>
    where
        Self: Sized,
    {
        Self::from_peekable(&mut bytes.iter().peekable())
    }

    fn name(&self) -> &'static str;
}

#[enum_dispatch]
pub trait Instruction: std::fmt::Display {
    fn format(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }

    fn size(&self) -> u8;
}

#[enum_dispatch(Instruction)]
pub enum Opcode {
    ADC(ADC),
    AND(AND),
    ASL(ASL),
    BCC(BCC),
    BCS(BCS),
    BEQ(BEQ),
    BIT(BIT),
    BMI(BMI),
    BNE(BNE),
    BPL(BPL),
    BRK(BRK),
    BVC(BVC),
    BVS(BVS),
    CLC(CLC),
    CLD(CLD),
    CLI(CLI),
    CLV(CLV),
    CMP(CMP),
    CPX(CPX),
    CPY(CPY),
    DEC(DEC),
    DEX(DEX),
    DEY(DEY),
    EOR(EOR),
    INC(INC),
    INX(INX),
    INY(INY),
    JMP(JMP),
    JSR(JSR),
    LDA(LDA),
    LDX(LDX),
    LDY(LDY),
    LSR(LSR),
    NOP(NOP),
    ORA(ORA),
    PHA(PHA),
    PHP(PHP),
    PLA(PLA),
    PLP(PLP),
    ROL(ROL),
    ROR(ROR),
    RTI(RTI),
    RTS(RTS),
    SBC(SBC),
    SEC(SEC),
    SED(SED),
    SEI(SEI),
    STA(STA),
    STX(STX),
    STY(STY),
    TAX(TAX),
    TAY(TAY),
    TSX(TSX),
    TXA(TXA),
    TXS(TXS),
    TYA(TYA),
}

impl std::fmt::Display for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.format(f)
    }
}

macro_rules! try_from_peekable {
    ($input: ident, $id: tt) => {
        if let Some(op) = $id::from_peekable($input) {
            return Some(Self::$id(op));
        }
    };
}

impl Opcode {
    pub fn from_peekable<'a, I: Iterator<Item = &'a u8> + 'a>(
        bytes: &mut std::iter::Peekable<I>,
    ) -> Option<Self> {
        try_from_peekable!(bytes, ADC);
        try_from_peekable!(bytes, AND);
        try_from_peekable!(bytes, ASL);
        try_from_peekable!(bytes, BCC);
        try_from_peekable!(bytes, BCS);
        try_from_peekable!(bytes, BEQ);
        try_from_peekable!(bytes, BIT);
        try_from_peekable!(bytes, BMI);
        try_from_peekable!(bytes, BNE);
        try_from_peekable!(bytes, BPL);
        try_from_peekable!(bytes, BRK);
        try_from_peekable!(bytes, BVC);
        try_from_peekable!(bytes, BVS);
        try_from_peekable!(bytes, CLC);
        try_from_peekable!(bytes, CLD);
        try_from_peekable!(bytes, CLI);
        try_from_peekable!(bytes, CLV);
        try_from_peekable!(bytes, CMP);
        try_from_peekable!(bytes, CPX);
        try_from_peekable!(bytes, CPY);
        try_from_peekable!(bytes, DEC);
        try_from_peekable!(bytes, DEX);
        try_from_peekable!(bytes, DEY);
        try_from_peekable!(bytes, EOR);
        try_from_peekable!(bytes, INC);
        try_from_peekable!(bytes, INX);
        try_from_peekable!(bytes, INY);
        try_from_peekable!(bytes, JMP);
        try_from_peekable!(bytes, JSR);
        try_from_peekable!(bytes, LDA);
        try_from_peekable!(bytes, LDX);
        try_from_peekable!(bytes, LDY);
        try_from_peekable!(bytes, LSR);
        try_from_peekable!(bytes, NOP);
        try_from_peekable!(bytes, ORA);
        try_from_peekable!(bytes, PHA);
        try_from_peekable!(bytes, PHP);
        try_from_peekable!(bytes, PLA);
        try_from_peekable!(bytes, PLP);
        try_from_peekable!(bytes, ROL);
        try_from_peekable!(bytes, ROR);
        try_from_peekable!(bytes, RTI);
        try_from_peekable!(bytes, RTS);
        try_from_peekable!(bytes, SBC);
        try_from_peekable!(bytes, SEC);
        try_from_peekable!(bytes, SED);
        try_from_peekable!(bytes, SEI);
        try_from_peekable!(bytes, STA);
        try_from_peekable!(bytes, STX);
        try_from_peekable!(bytes, STY);
        try_from_peekable!(bytes, TAX);
        try_from_peekable!(bytes, TAY);
        try_from_peekable!(bytes, TSX);
        try_from_peekable!(bytes, TXA);
        try_from_peekable!(bytes, TXS);
        try_from_peekable!(bytes, TYA);
        None
    }
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
pub struct ADC(AddressMode);

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
pub struct AND(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(
    accumulator = 0x0A,
    zero = 0x06,
    zero_x = 0x16,
    absolute = 0x0E,
    absolute_x = 0x1E
)]
pub struct ASL(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(relative = 0x90)]
pub struct BCC(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(relative = 0xB0)]
pub struct BCS(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(relative = 0xF0)]
pub struct BEQ(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(zero = 0x24, absolute = 0x2C)]
pub struct BIT(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(relative = 0x30)]
pub struct BMI(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(relative = 0xD0)]
pub struct BNE(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(relative = 0x10)]
pub struct BPL(AddressMode);
#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(implicit = 0x00)]
pub struct BRK(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(relative = 0x50)]
pub struct BVC(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(relative = 0x70)]
pub struct BVS(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(implicit = 0x18)]
pub struct CLC(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(implicit = 0xD8)]
pub struct CLD(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(implicit = 0x58)]
pub struct CLI(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(implicit = 0xB8)]
pub struct CLV(AddressMode);

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
pub struct CMP(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(immediate = 0xE0, zero = 0xE4, absolute = 0xEC)]
pub struct CPX(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(immediate = 0xC0, zero = 0xC4, absolute = 0xCC)]
pub struct CPY(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(zero = 0xC6, zero_x = 0xD6, absolute = 0xCE, absolute_x = 0xDE)]
pub struct DEC(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(implicit = 0xCA)]
pub struct DEX(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(implicit = 0x88)]
pub struct DEY(AddressMode);

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
pub struct EOR(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(zero = 0xE6, zero_x = 0xF6, absolute = 0xEE, absolute_x = 0xFE)]
pub struct INC(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(implicit = 0xE8)]
pub struct INX(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(implicit = 0xC8)]
pub struct INY(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(absolute = 0x4C, indirect = 0x6C)]
pub struct JMP(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(absolute = 0x20)]
pub struct JSR(AddressMode);

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
pub struct LDA(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(
    immediate = 0xA2,
    zero = 0xA6,
    zero_y = 0xB6,
    absolute = 0xAE,
    absolute_y = 0xBE
)]
pub struct LDX(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(
    immediate = 0xA0,
    zero = 0xA4,
    zero_x = 0xB4,
    absolute = 0xAC,
    absolute_x = 0xBC
)]
pub struct LDY(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(
    accumulator = 0x4A,
    zero = 0x46,
    zero_x = 0x56,
    absolute = 0x4E,
    absolute_x = 0x5E
)]
pub struct LSR(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(implicit = 0xEA)]
pub struct NOP(AddressMode);

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
pub struct ORA(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(implicit = 0x48)]
pub struct PHA(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(implicit = 0x08)]
pub struct PHP(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(implicit = 0x68)]
pub struct PLA(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(implicit = 0x28)]
pub struct PLP(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(
    accumulator = 0x2A,
    zero = 0x26,
    zero_x = 0x36,
    absolute = 0x2E,
    absolute_x = 0x3E
)]
pub struct ROL(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(
    accumulator = 0x6A,
    zero = 0x66,
    zero_x = 0x76,
    absolute = 0x6E,
    absolute_x = 0x7E
)]
pub struct ROR(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(implicit = 0x40)]
pub struct RTI(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(implicit = 0x60)]
pub struct RTS(AddressMode);

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
pub struct SBC(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(implicit = 0x38)]
pub struct SEC(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(implicit = 0xF8)]
pub struct SED(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(implicit = 0x78)]
pub struct SEI(AddressMode);

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
pub struct STA(AddressMode);
#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(zero = 0x86, zero_y = 0x96, absolute = 0x8E)]
pub struct STX(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(zero = 0x84, zero_x = 0x94, absolute = 0x8C)]
pub struct STY(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(implicit = 0xAA)]
pub struct TAX(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(implicit = 0xA8)]
pub struct TAY(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(implicit = 0xBA)]
pub struct TSX(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(implicit = 0x8A)]
pub struct TXA(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(implicit = 0x9A)]
pub struct TXS(AddressMode);

#[derive(Asm6502, Debug, PartialEq)]
#[asm6502(implicit = 0x98)]
pub struct TYA(AddressMode);

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
