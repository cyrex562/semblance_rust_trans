#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Endianness {
    LittleEndian,
    BigEndian,
}

pub const mz_magic: u16 = 0x5a4d; // ZM, e.g. MZ
pub const ne_magic: u16 = 0x454e; // EN, e.g. NE