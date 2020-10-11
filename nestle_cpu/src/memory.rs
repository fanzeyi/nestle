use anyhow::{anyhow, Result};
use std::cmp::Ordering;
use std::ops::RangeInclusive;

const ADDRESS_SPACE: usize = 0xFFFF;

#[derive(Eq, PartialEq, Clone)]
pub enum SegmentType {
    Mirror(RangeInclusive<usize>),
    Readonly,
    Data(Vec<u8>),
}

#[derive(Eq, PartialEq, Clone)]
pub struct Segment {
    ttype: SegmentType,
    range: RangeInclusive<usize>,
}

impl Segment {
    fn mirror(src: RangeInclusive<usize>, dest: RangeInclusive<usize>) -> Self {
        Segment {
            ttype: SegmentType::Mirror(dest),
            range: src,
        }
    }

    fn readonly(range: RangeInclusive<usize>) -> Self {
        Segment {
            ttype: SegmentType::Readonly,
            range,
        }
    }

    fn data(range: RangeInclusive<usize>, bytes: Vec<u8>) -> Self {
        Segment {
            ttype: SegmentType::Data(bytes),
            range,
        }
    }

    fn into_bytes(self) -> Vec<u8> {
        match self.ttype {
            SegmentType::Data(bytes) => bytes,
            _ => panic!("invalid segment type"),
        }
    }

    fn contains(&self, addr: u16) -> bool {
        self.range.contains(&(addr as usize))
    }

    fn dest_range(&self) -> RangeInclusive<usize> {
        match &self.ttype {
            SegmentType::Mirror(dest) => dest.clone(),
            _ => panic!("inavlid segment type"),
        }
    }

    fn is_mirror(&self) -> bool {
        match self.ttype {
            SegmentType::Mirror { .. } => true,
            _ => false,
        }
    }

    fn is_readonly(&self) -> bool {
        match self.ttype {
            SegmentType::Readonly => true,
            _ => false,
        }
    }
}

impl Ord for Segment {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.range.start().cmp(&other.range.start()) {
            Ordering::Equal => self.range.end().cmp(&other.range.end()),
            x => x,
        }
    }
}

impl PartialOrd for Segment {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct MemoryBuilder {
    segments: Vec<Segment>,
    data: Vec<Segment>,
}

impl MemoryBuilder {
    pub fn new() -> Self {
        MemoryBuilder {
            segments: Vec::new(),
            data: Vec::new(),
        }
    }

    pub fn add_readonly(&mut self, range: RangeInclusive<usize>) {
        self.segments.push(Segment::readonly(range));
    }

    pub fn add_mirror(
        &mut self,
        src: RangeInclusive<usize>,
        dest: RangeInclusive<usize>,
    ) -> Result<()> {
        if src.contains(dest.start()) || src.contains(dest.end()) {
            Err(anyhow!("invalid mirror segment, can't overlap"))
        } else {
            self.segments.push(Segment::mirror(src, dest));
            Ok(())
        }
    }

    pub fn add_data(&mut self, range: RangeInclusive<usize>, bytes: Vec<u8>) -> Result<()> {
        if bytes.len() != ((range.end() - range.start() + 1) as usize) {
            Err(anyhow!(
                "invalid segment, byte length doesn't equal to segment length"
            ))
        } else {
            self.data.push(Segment::data(range, bytes));
            Ok(())
        }
    }

    pub fn build(self) -> Memory {
        let MemoryBuilder { mut segments, data } = self;
        let mut memory: [u8; ADDRESS_SPACE + 1] = [0; ADDRESS_SPACE + 1];

        for datum in data {
            (&mut memory[datum.range.clone()]).copy_from_slice(&datum.into_bytes());
        }

        segments.sort();

        Memory {
            data: memory,
            segments,
        }
    }
}

pub struct Memory {
    // storage
    data: [u8; 0x10000],
    segments: Vec<Segment>,
}

impl Memory {
    fn sync_memory(&mut self, src: usize, dest: usize, offset: usize, length: usize) {
        for i in 0..length {
            log::debug!(
                "syncing {:02x} from ${:04x} to ${:04x}",
                self.data[src + offset + i],
                src + offset + i,
                dest + offset + i
            );
            self.data[dest + offset + i] = self.data[src + offset + i];
        }
    }

    fn sync(&mut self, addr: usize, length: usize) {
        for seg in self.segments.clone() {
            if seg.is_mirror() {
                // range check
                if seg.range.contains(&addr) {
                    // from src to dest
                    let src = *seg.range.start();
                    let offset = addr - src;
                    self.sync_memory(src, *seg.dest_range().start(), offset, length);
                } else if seg.dest_range().contains(&addr) {
                    // from dest to src
                    let src = *seg.dest_range().start();
                    let offset = addr - src;
                    self.sync_memory(src, *seg.range.start(), offset, length);
                }
            }
        }
    }

    fn readonly(&self, addr: usize) -> bool {
        for seg in &self.segments {
            if seg.is_readonly() && seg.range.contains(&addr) {
                return true;
            }
        }
        false
    }

    pub fn read_u8(&self, addr: usize) -> u8 {
        self.data[addr]
    }

    pub fn read_u16(&self, addr: usize) -> u16 {
        u16::from_le_bytes([self.data[addr], self.data[addr + 1]])
    }

    pub fn write_u8(&mut self, addr: usize, byte: u8) -> Result<()> {
        if self.readonly(addr) {
            Err(anyhow!("Memory address is readonly."))
        } else {
            self.data[addr] = byte;
            self.sync(addr, 1);
            Ok(())
        }
    }

    pub fn write_u16(&mut self, addr: usize, byte: u16) -> Result<()> {
        if self.readonly(addr) {
            Err(anyhow!("Memory address is readonly."))
        } else {
            let le_bytes = byte.to_le_bytes();
            self.data[addr] = le_bytes[0];
            self.data[addr + 1] = le_bytes[1];
            self.sync(addr, 2);
            Ok(())
        }
    }

    pub fn iter_range(&self, range: RangeInclusive<usize>) -> impl Iterator<Item = &u8> {
        self.data[range].iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mirror() {
        let mut builder = MemoryBuilder::new();
        builder
            .add_mirror(0x0000..=0x7FFF, 0x8000..=0xFFFF)
            .unwrap();
        let mut memory = builder.build();

        memory.write_u8(0x3FFF, 47u8).unwrap();
        assert_eq!(memory.read_u8(0x3FFF), memory.read_u8(0x8000 + 0x3FFF));

        memory.write_u8(0x8FFF, 74u8).unwrap();
        assert_eq!(memory.read_u8(0x0FFF), memory.read_u8(0x8FFF));

        memory.write_u8(0x0000, 42u8).unwrap();
        assert_eq!(memory.read_u8(0x0000), memory.read_u8(0x8000));

        memory.write_u8(0x7FFF, 24u8).unwrap();
        assert_eq!(memory.read_u8(0x7FFF), memory.read_u8(0xFFFF));

        memory.write_u16(0x7FFE, 0xABCDu16).unwrap();
        assert_eq!(memory.read_u16(0x7FFE), memory.read_u16(0xFFFE));
    }

    #[test]
    fn test_readonly() {
        let mut builder = MemoryBuilder::new();
        builder.add_readonly(0x0000..=0x7FFF);
        let mut memory = builder.build();

        assert!(memory.write_u8(0x0000, 42).is_err());
        assert!(memory.write_u8(0x8000, 42).is_ok());
        assert!(memory.write_u8(0x7FFF, 42).is_err());
    }

    #[test]
    fn test_memory_builder_data() {
        let mut builder = MemoryBuilder::new();
        builder.add_data(0x0000..=0x0001, vec![0xAD, 0xDE]).unwrap();
        let memory = builder.build();

        assert_eq!(memory.read_u16(0x0000), 0xDEAD);
    }
}
