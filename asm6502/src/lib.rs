mod instructions;

pub use crate::instructions::{Instruction, Opcode};

pub fn dump(data: Vec<u8>) -> Vec<Opcode> {
    let mut peekable = data.iter().peekable();
    let mut result = Vec::new();

    while let Some(op) = Opcode::from_peekable(&mut peekable) {
        result.push(op);
    }

    result
}

pub fn prettyprint(opcodes: Vec<Opcode>) {
    let mut addr = 0;
    for op in opcodes {
        println!("${:04x} {}", addr, op);

        addr += op.size();
    }
}

#[test]
fn test_simple() {
    let input = b"\xa2\x08\xca\x8e\x00\x02\xe0\x03\xd0\xf8\x8e\x01\x02\x00";
    prettyprint(dump(input.to_vec()));
}
