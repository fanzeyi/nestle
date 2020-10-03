use binread::BinRead;

#[derive(Debug, BinRead)]
pub struct Ines {
    header: InesHeader,

    #[br(if(header.mapper & 0b00100000 != 0), count=512)]
    trainer: Option<Vec<u8>>,

    #[br(count = (header.prg_size as u32) * 16384)]
    prg: Vec<u8>,

    #[br(if(header.chr_size != 0), count = (header.chr_size as u32) * 8192)]
    chr: Option<Vec<u8>>,
    // PlayChoice fields ..
}

impl Ines {
    #[cfg(test)]
    fn dump(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend(&self.header.dump());

        if let Some(trainer) = self.trainer.as_ref() {
            result.extend(trainer);
        }

        result.extend(&self.prg);

        if let Some(chr) = self.chr.as_ref() {
            result.extend(chr);
        }

        result
    }
}

impl Default for Ines {
    fn default() -> Self {
        Ines {
            header: Default::default(),
            trainer: None,
            prg: Vec::new(),
            chr: None,
        }
    }
}

#[derive(Debug, BinRead)]
#[br(magic = b"NES\x1a")]
pub struct InesHeader {
    prg_size: u8, // 4
    chr_size: u8, // 5
    mapper: u8,   // 6
    extra: [u8; 9],
}

impl InesHeader {
    #[cfg(test)]
    fn dump(&self) -> Vec<u8> {
        [
            &b"NES\x1a"[..],
            &[self.prg_size, self.chr_size, self.mapper][..],
            &self.extra[..],
        ]
        .concat()
    }
}

impl Default for InesHeader {
    fn default() -> Self {
        InesHeader {
            prg_size: 1,
            chr_size: 0,
            mapper: 0,
            extra: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use binread::BinReaderExt;
    use std::io::Cursor;

    fn with_header(body: &[u8]) -> Vec<u8> {
        let mut result = InesHeader::default().dump();
        result.extend_from_slice(body);
        result
    }

    #[test]
    fn test_ines_header() {
        let mut reader = Cursor::new(b"NES\x1a\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
        let header: InesHeader = reader.read_le().unwrap();
        assert_eq!(header.prg_size, 1u8);
        assert_eq!(header.chr_size, 0u8);
        assert_eq!(header.mapper, 0u8);
    }

    #[test]
    fn test_ines_parse() {
        let prg = b"\x01".repeat(16384);
        let mut reader = Cursor::new(with_header(&prg));
        let image: Ines = reader.read_le().unwrap();
        assert_eq!(&image.prg, &prg);
    }

    #[test]
    fn test_ines_parse_trainer() {
        let header = InesHeader {
            mapper: 0b0010_0000,
            ..Default::default()
        };
        let trainer = b"trainer\x01".repeat(64);
        let prg = b"\x01".repeat(16384);

        let ines = Ines {
            header,
            trainer: Some(trainer.clone()),
            prg,
            ..Default::default()
        };
        let mut reader = Cursor::new(ines.dump());
        let image: Ines = reader.read_le().unwrap();

        assert!(image.trainer.is_some());
        let img_trainer = image.trainer.unwrap();
        assert_eq!(img_trainer.len(), 512);
        assert_eq!(img_trainer, trainer);
    }

    #[test]
    fn test_ines_pasre_chr() {
        let header = InesHeader {
            chr_size: 2,
            ..Default::default()
        };
        let prg = b"\x01".repeat(16384);
        let chr = b"chrsect\x02".repeat(2048); // 8192 * 2

        let ines = Ines {
            header,
            prg,
            chr: Some(chr.clone()),
            ..Default::default()
        };
        let mut reader = Cursor::new(ines.dump());
        let image: Ines = reader.read_le().unwrap();

        assert!(image.chr.is_some());
        let img_chr = image.chr.unwrap();
        assert_eq!(img_chr.len(), 8192 * 2);
        assert_eq!(img_chr, chr);
    }
}
