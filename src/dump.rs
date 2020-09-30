use mapr::Mmap;
use mapr::MmapOptions;
use std::{fs::{self, File}, path::Path};
use std::convert::TryFrom;
use std::convert::TryInto;

use crate::multi_error::MultiError;


#[derive(Default)]
pub struct FileHeader {
    pub machine: u16, // 04
    pub number_of_sections: u16, // 06
    pub time_date_stamp: u32, // 08
    pub pointer_to_symbol_table: u32, // 0c
    pub number_of_symbols: u32, // 10
    pub size_of_optional_header: u16, // 14
    pub characteristics: u16, // 16
}

#[derive(Default)]
pub struct OptionalHeader {
    // standard COFF fields
    pub magic: u16, // 18
    pub major_linker_version: u8, // 1a
    pub minor_linker_version: u8, // 1b
    pub size_of_code: u32, // 1c
    pub size_of_initialized_data: u32, // 20
    pub size_of_uninitialized_data: u32, // 24
    pub address_of_entry_point: u32, // 28
    pub base_of_code: u32, // 2c
    pub base_of_data: u32, // 30
    // PE fields
    pub image_base: u32, // 34
    pub section_alignment: u32, // 38
    pub file_alignment: u32, // 3c
    pub major_os_version: u16, // 40
    pub minor_os_version: u16, // 42
    pub major_image_version: u16, // 44
    pub minor_image_version: u16, // 46
    pub major_subsystem_version: u16, // 48
    pub minor_subsystem_version: u16, // 4a
    pub win32_version_value: u32, // 4c
    pub size_of_image: u32, // 50
    pub size_of_headers: u32, // 54
    pub checksum: u32, // 58
    pub sub_system: u16, // 5c
    pub dll_characteristics: u16, // 5e
    pub size_of_stack_reserve: u32, // 60
    pub size_of_stack_commit: u32, // 64
    pub size_of_heap_reserve: u32, // 68
    pub size_of_heap_commit: u32, // 6c
    pub loader_flags: u32, // 70
    pub number_of_rva_and_sizes: u32, // 74
}

#[repr(C)]
pub union PeOptHeader {
    opt32: *mut OptionalHeader,
    opt64: *mut OptionalHeaderPep
}

#[derive(Default)]
pub struct PE {
    pub magic: u16,
    pub image_base: u64,
    pub fheader: FileHeader,
    pub opt_hdr: PeOptHeader,
    
}

pub fn read_word(mmap: &Mmap, offset: usize) -> Result<u16, MultiError> {
    let mut read_bytes: [u8;2] = [0, 0]; 
    read_bytes.clone_from_slice(&mmap[offset..offset+2]);
    let word: u16 = u16::from_le_bytes(read_bytes);
    Ok(word)
}

pub fn read_dword(mmap: &Mmap, offset: usize) -> Result<u32, MultiError> {
    let mut read_bytes: [u8;4] = [0,0,0,0];
    read_bytes.clone_from_slice(&mmap[offset..offset+4]);
    let dword: u32 = u32::from_le_bytes(read_bytes);
    Ok(dword)
}

// pub fn is_mz_header(slice: &[u8]) -> bool {
//     if slice[0] == 0x5a && slice[1] == 0x4d {
//         true;
//     }
//     false;
// }

pub fn is_mz_header(magic: u16) -> bool {
    magic == 0x5a4d
}

pub fn dump_file(file: &str) -> Result<(), MultiError> {
    // open file
    let path = Path::new(file);
    let mut file = File::open(&path)?;

    // get file stats
    let stats = fs::metadata(&path)?;

    // memory map the file
    let mmap = unsafe { MmapOptions::new().map(&file)?};
    
    let mut magic = read_word(&mmap, 0)?;

    if !is_mz_header(magic) {
        log::error!("invalid file header");
        return Err(MultiError{ 
            kind: "InvalidFileHeader".to_string(), 
            message: "Invalid File Header".to_string(),       
        });
    }
    
    let mut offset: usize = read_dword(&mmap, 0x3c)? as usize;

    magic = read_word(&mmap, offset)?;

    // PE
    if (magic == 0x4550) {
        dumppe(&mmap, offset)?;
    } 
    // NE
    else if (magic == 0x454e) {
        dumpne(&mmap, offset)?;
    } else {
        dumpmz()
    }

    Ok(())
}

pub fn dumppe(mmap: &Mmap, offset: usize) => Result<(), MultiError> {
    let mut pe: PE = Default::default();
}