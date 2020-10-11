use anyhow::Result;

use crate::memory::{Memory, MemoryBuilder};
use nestle_ines::Ines;

pub trait Mapper {
    fn map_image(image: Ines) -> Result<Memory>;
}

pub struct NROM;

impl Mapper for NROM {
    fn map_image(image: Ines) -> Result<Memory> {
        let mut builder = MemoryBuilder::new();
        if image.prg.len() == (0xBFFF - 0x8000) {
            builder.add_data(0x8000..=0xBFFF, image.prg)?;
            builder.add_mirror(0xC000..=0xFFFF, 0x8000..=0xBFFF)?;
        } else {
            builder.add_data(0x8000..=0xFFFF, image.prg)?;
        }

        builder.add_readonly(0x8000..=0xFFFF);
        Ok(builder.build())
    }
}
