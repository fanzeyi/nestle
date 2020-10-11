use crate::Opcode;

pub struct OpcodeIterator<'a, I: Iterator<Item = &'a u8>> {
    inner: std::iter::Peekable<I>,
}

impl<'a, I> Iterator for OpcodeIterator<'a, I>
where
    I: 'a + Iterator<Item = &'a u8>,
{
    type Item = Opcode;

    fn next(&mut self) -> Option<Self::Item> {
        Opcode::from_peekable(&mut self.inner)
    }
}

pub fn opcodes<'a, I: Iterator<Item = &'a u8>>(
    stream: std::iter::Peekable<I>,
) -> OpcodeIterator<'a, I> {
    OpcodeIterator { inner: stream }
}
