use binread::BinRead;

#[derive(Debug, BinRead)]
pub struct Ines {
    header: InesHeader,

    #[br(if(header.mapper & 0b00100000 == 1), count=512)]
    trainer: Option<Vec<u8>>,

    #[br(count = (header.prg_size as u32) * 16384)]
    prg: Vec<u8>,

    #[br(if(header.chr_size != 0))]
    chr: Option<Vec<u8>>,
    // PlayChoice fields ..
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
}
