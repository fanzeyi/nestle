use anyhow::Result;
use std::ops::RangeInclusive;
use std::path::Path;

use asm6502::Opcode;
use nestle_ines::Ines;

use crate::mapper::{Mapper, NROM};
use crate::memory::Memory;
use crate::register::Registers;

pub struct CPU {
    registers: Registers,
    memory: Memory,
}

impl CPU {
    pub fn from_path(path: &Path) -> Result<Self> {
        let ines = Ines::from_path(path)?;
        let memory = NROM::map_image(ines)?;
        let initial_pc = memory.read_u16(0xFFFC);
        let registers = Registers::with_pc(initial_pc);
        Ok(Self { registers, memory })
    }

    pub fn pprint_memory(&self, range: RangeInclusive<usize>) {
        self.memory.pprint_memory(range);
    }

    fn instructions(&self) -> impl Iterator<Item = Opcode> {
        todo!()
    }
}

#[test]
fn test_test() {
    let cpu = CPU::from_path("../fixtures/1.Branch_Basics.nes".as_ref()).unwrap();

    cpu.pprint_memory(0x8000..=0xFFFF);
}
