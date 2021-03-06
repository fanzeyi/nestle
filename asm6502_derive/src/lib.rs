extern crate proc_macro;

use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[derive(FromDeriveInput, Debug)]
#[darling(attributes(asm6502))]
struct Asm6502 {
    #[darling(default)]
    pub implicit: Option<u8>,
    #[darling(default)]
    pub accumulator: Option<u8>,
    #[darling(default)]
    pub immediate: Option<u8>,
    #[darling(default)]
    pub zero: Option<u8>,
    #[darling(default)]
    pub zero_x: Option<u8>,
    #[darling(default)]
    pub zero_y: Option<u8>,
    #[darling(default)]
    pub relative: Option<u8>,
    #[darling(default)]
    pub absolute: Option<u8>,
    #[darling(default)]
    pub absolute_x: Option<u8>,
    #[darling(default)]
    pub absolute_y: Option<u8>,
    #[darling(default)]
    pub indirect: Option<u8>,
    #[darling(default)]
    pub indirect_x: Option<u8>,
    #[darling(default)]
    pub indirect_y: Option<u8>,
    pub ident: syn::Ident,
}

macro_rules! optype {
    ($result: ident, $field: expr, $($ctor: tt)+) => {
        if let Some(val) = $field {
            $result.push(quote! {
                #val => {
                    bytes.next();
                    Some(Self($($ctor)*))
                }
            });
        }
    };
}

impl Asm6502 {
    fn build_from_peekable(&self) -> proc_macro2::TokenStream {
        let mut branches = Vec::new();

        optype!(branches, self.implicit, AddressMode::Implicit);
        optype!(branches, self.accumulator, AddressMode::Accumulator);
        optype!(
            branches,
            self.immediate,
            AddressMode::Immediate(*bytes.next().unwrap())
        );
        optype!(
            branches,
            self.zero,
            AddressMode::Zero(*bytes.next().unwrap())
        );
        optype!(
            branches,
            self.zero_x,
            AddressMode::ZeroX(*bytes.next().unwrap())
        );
        optype!(
            branches,
            self.zero_y,
            AddressMode::ZeroY(*bytes.next().unwrap())
        );
        optype!(
            branches,
            self.relative,
            AddressMode::Relative(i8::from_le_bytes([*bytes.next().unwrap()]))
        );
        optype!(
            branches,
            self.absolute,
            AddressMode::Absolute(u16::from_le_bytes([
                *bytes.next().unwrap(),
                *bytes.next().unwrap()
            ]))
        );
        optype!(
            branches,
            self.absolute_x,
            AddressMode::AbsoluteX(u16::from_le_bytes([
                *bytes.next().unwrap(),
                *bytes.next().unwrap()
            ]))
        );
        optype!(
            branches,
            self.absolute_y,
            AddressMode::AbsoluteY(u16::from_le_bytes([
                *bytes.next().unwrap(),
                *bytes.next().unwrap()
            ]))
        );
        optype!(
            branches,
            self.indirect,
            AddressMode::Indirect(u16::from_le_bytes([
                *bytes.next().unwrap(),
                *bytes.next().unwrap()
            ]))
        );
        optype!(
            branches,
            self.indirect_x,
            AddressMode::IndirectX(*bytes.next().unwrap())
        );
        optype!(
            branches,
            self.indirect_y,
            AddressMode::IndirectY(*bytes.next().unwrap())
        );

        quote! {
            fn from_peekable<'a, I: Iterator<Item = &'a u8> + 'a>(bytes: &mut std::iter::Peekable<I>) -> Option<Self> {
                if let Some(&next) = bytes.peek() {
                    return match next {
                        #(#branches,)*
                        _ => None,
                    }
                }
                None
            }
        }
    }
}

fn expand_derive(input: DeriveInput) -> Result<proc_macro2::TokenStream, Vec<syn::Error>> {
    let parsed = Asm6502::from_derive_input(&input).unwrap();
    let name = parsed.ident.clone();

    let from_peekable = parsed.build_from_peekable();

    Ok(quote! {
        impl InstructionConstruct for #name {
            #from_peekable

            fn name(&self) -> &'static str {
                stringify!(#name)
            }
        }

        impl Instruction for #name {
            fn size(&self) -> u8 {
                self.0.size()
            }
        }

        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                use AddressMode::*;

                write!(f, stringify!(#name))?;

                match self.0 {
                    Implicit | Accumulator => write!(f, ""),
                    Immediate(a) => write!(f, " #${:02x}", a),
                    Zero(a) => write!(f, " ${:02x}", a),
                    Relative(a) => {
                        if a < 0 {
                            write!(f, " -${:02x}", -a)
                        } else {
                            write!(f, " ${:02x}", a)
                        }
                    },
                    Absolute(a) | Indirect(a) => write!(f, " ${:04x}", a),
                    ZeroX(a) | IndirectX(a) => write!(f, " ${:02x}, X", a),
                    ZeroY(a) | IndirectY(a) => write!(f, " ${:02x}, Y", a),
                    AbsoluteX(a) => write!(f, " ${:04x}, X", a),
                    AbsoluteY(a) => write!(f, " ${:04x}, Y", a),
                }
            }
        }
    })
}

#[proc_macro_derive(Asm6502, attributes(asm6502))]
pub fn asm6502(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    expand_derive(input)
        .unwrap_or_else(to_compile_errors)
        .into()
}

fn to_compile_errors(errors: Vec<syn::Error>) -> proc_macro2::TokenStream {
    let compile_errors = errors.iter().map(syn::Error::to_compile_error);
    quote!(#(#compile_errors)*)
}
