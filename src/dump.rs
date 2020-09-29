use mapr::Mmap;
use mapr::MmapOptions;
use std::{fs::{self, File}, path::Path};

use crate::multi_error::MultiError;


pub fn read_word(mmap: &Mmap, offset: usize) -> Result<[u8;2], MultiError> {
    let read_bytes: [u8;2] = [0, 0]; 
    read_bytes.clone_from_slice(mmap[0..2]);
    Ok(read_bytes)
}


pub fn dump_file(file: &str) -> Result<(), MultiError> {
    // open file
    let path = Path::new(file);
    let mut file = File::open(&path)?;

    // get file stats
    let stats = fs::metadata(&path)?;

    // memory map the file
    let mmap = unsafe { MmapOptions::new().map(&file)?};
    
    let magic = read_word(&mmap, 0)?;
}