// use mapr::Mmap;
use netools::multi_error::MultiError;
//
// pub fn read_byte(mmap: &Mmap, offset: usize) -> Result<u8, MultiError> {
//     let mut read_bytes: [u8;1] = [0];
//     read_bytes.clone_from_slice(&mmap[offset..offset+1]);
//     let half_word: u8 = u8::from_le_bytes(read_bytes);
//     Ok(half_word)
// }
//
// pub fn read_word(mmap: &Mmap, offset: usize) -> Result<u16, MultiError> {
//     let mut read_bytes: [u8;2] = [0, 0];
//     read_bytes.clone_from_slice(&mmap[offset..offset+2]);
//     let word: u16 = u16::from_le_bytes(read_bytes);
//     Ok(word)
// }
//
// pub fn read_dword(mmap: &Mmap, offset: usize) -> Result<u32, MultiError> {
//     let mut read_bytes: [u8;4] = [0,0,0,0];
//     read_bytes.clone_from_slice(&mmap[offset..offset+4]);
//     let dword: u32 = u32::from_le_bytes(read_bytes);
//     Ok(dword)
// }
//
// pub fn read_qword(mmap: &Mmap, offset: usize) -> Result<u64, MultiError> {
//     let mut read_bytes: [u8;8] = [0,0,0,0,0,0,0,0];
//     read_bytes.clone_from_slice(&mmap[offset..offset+8]);
//     let dword: u64 = u64::from_le_bytes(read_bytes);
//     Ok(dword)
// }

pub enum Endianness {
    LittleEndian,
    BigEndian
}