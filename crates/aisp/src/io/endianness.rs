#[derive(Clone, Copy)]
pub enum Endianness {
    LittleEndian,
    BigEndian,
}

pub trait EndianAware {
    fn set_endian(&mut self, endian: Endianness);
    fn endian(&self) -> Endianness;
}

impl<T: EndianAware> EndianAware for &mut T {
    fn set_endian(&mut self, endian: Endianness) {
        (**self).set_endian(endian);
    }
    fn endian(&self) -> Endianness {
        (**self).endian()
    }
}
