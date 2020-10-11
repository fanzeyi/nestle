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

        // crate::utils::pprint_binaries(&image.prg, 0x0000..=(image.prg.len() - 1));

        println!("prg rom: 0x{:x}", image.prg.len());

        if image.prg.len() == (0xBFFF - 0x8000 + 1) {
            builder.add_data(0x8000..=0xBFFF, image.prg.clone())?;
            builder.add_data(0xC000..=0xFFFF, image.prg)?;
            builder.add_mirror(0xC000..=0xFFFF, 0x8000..=0xBFFF)?;
        } else {
            builder.add_data(0x8000..=0xFFFF, image.prg)?;
        }

        builder.add_readonly(0x8000..=0xFFFF);
        Ok(builder.build())
    }
}
