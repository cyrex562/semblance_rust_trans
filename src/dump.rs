use mapr::Mmap;
use mapr::MmapOptions;
use std::{fs::{self, File}, path::Path};
use std::convert::TryFrom;
use std::convert::TryInto;

use crate::multi_error::MultiError;
use semblance_rust::common;


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

#[derive(Default, Copy, Clone)]
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

#[derive(Default, Copy, Clone)]
pub struct OptionalHeaderPep {
    // standard coff fields
    pub magic: u16, // 18
    pub major_linker_version: u8, // 1a,
    pub minor_linker_version: u8, // 1b, 
    pub size_of_code: u32, // 1c,
    pub size_of_initialized_data: u32, // 20
    pub size_of_uninitialized_data: u32, // 24
    pub address_of_entry_point: u32, // 28
    pub base_of_code: u32, // 2c
    // pe fields
    pub image_base: u64, // 30
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
    pub size_of_stack_reserve: u64, // 60
    pub size_of_stack_commit: u64, // 68
    pub size_of_heap_reserve: u64, // 70
    pub size_of_heap_commit: u64, // 78
    pub loader_flags: u32, // 80
    pub number_of_rva_and_sizes: u32, // 84
}

pub struct Section {
    pub name: [u8;8], // 0
    pub min_alloc: u32, // 8
    pub address: u32, // c
    pub length: u32, // 10
    pub offset: u32, // 14
    pub reloc_offset: u32, // 18
    pub lineno_offset: u32, // 1c
    pub reloc_count: u16, // 20
    pub lineno_count: u16, // 22
    pub flags: u32, // 24
    // data
    pub instr_flags: Vec<u8>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union PeOptHeader {
    opt32: OptionalHeader,
    opt64: OptionalHeaderPep
}

pub struct Directory {
    pub address: u32, 
    pub size: u32,
}

pub struct Export {
    pub address: u32, 
    pub ordinal: u16,
    pub name: String,
}

// #[repr(C)]
// pub union NameTableEntryValue {
//     name: String,
//     ordinal: u16,
// }

#[derive(Default, Clone)]
pub struct NameTableEntry {
    // pub value: NameTableEntryValue,
    pub name: String,
    pub ordinal: u16,
    pub is_ordinal: bool,
}

//
// struct import_module {
//     char *name;
//     struct export *exports;
//     unsigned export_count;
// };
pub struct PeImportModule {
    pub module: String,
    pub iat_addr: u32,
    pub nametab: Vec<NameTableEntry>,
    pub count: usize
}

pub struct RelocPe {
    pub offset: u32,
    pub reloc_type: u32, 
}

// #[derive(Default)]
pub struct PE {
    pub magic: u16,
    pub image_base: u64,
    pub file_header: FileHeader,
    pub opt_hdr: PeOptHeader,
    pub dirs: Vec<Directory>,
    pub name: String,
    pub sections: Vec<Section>,
    pub exports: Vec<Export>,
    pub imports: Vec<PeImportModule>,
    pub relocs: Vec<RelocPe>,
}

impl Default for PE {
    fn default() -> Self { PE {
        magic: 0,
        image_base: 0,
        file_header: Default::default(),
        opt_hdr: PeOptHeader{ opt64: Default::default() },
        dirs: Vec::new(),
        name: String::new(),
        sections: Vec::new(),
        exports: Vec::new(),
        imports: Vec::new(),
        relocs: Vec::new(),
    } }
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
    
    let mut magic = common::read_word(&mmap, 0)?;

    if !is_mz_header(magic) {
        log::error!("invalid file header");
        return Err(MultiError{ 
            kind: "InvalidFileHeader".to_string(), 
            message: "Invalid File Header".to_string(),       
        });
    }
    
    let mut offset: usize = common::read_dword(&mmap, 0x3c)? as usize;

    magic = common::read_word(&mmap, offset)?;

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

pub fn dumppe(mmap: &Mmap, offset: usize) -> Result<PE, MultiError> {
    // let mut pe: PE = Default::default();

    let pe = read_pe(mmap, offset)?;

    Ok(pe)
}


pub fn read_pe(mmap: &Mmap, offset: usize) -> Result<PE, MultiError> {
    let mut pe: PE = Default::default();

    // copy file header
    pe.file_header.machine = common::read_word(mmap, offset + 4)?;
    pe.file_header.number_of_sections = common::read_word(mmap, offset + 6)?;
    pe.file_header.time_date_stamp = common::read_dword(mmap, offset + 8)?;
    pe.file_header.pointer_to_symbol_table = common::read_dword(mmap, offset + 12)?;
    pe.file_header.number_of_symbols = common::read_dword(mmap, offset + 16)?;
    pe.file_header.size_of_optional_header = common::read_word(mmap, offset + 20)?;
    pe.file_header.characteristics = common::read_word(mmap, offset + 22)?;
    pe.magic = common::read_word(mmap, offset + 24)?;
    if pe.magic == 0x10b {
        pe.opt_hdr.opt32.magic = common::read_word(mmap, offset + 26)?;
        pe.opt_hdr.opt32.major_linker_version = common::read_byte(mmap, offset + 28)?;
        pe.opt_hdr.opt32.minor_linker_version = common::read_byte(mmap, offset + 29)?;
        pe.opt_hdr.opt32.size_of_code = common::read_dword(mmap, offset + 30)?;
        pe.opt_hdr.opt32.size_of_initialized_data = common::read_dword(mmap, offset + 34)?;
        pe.opt_hdr.opt32.size_of_uninitialized_data = common::read_dword(mmap, offset + 38)?;
        pe.opt_hdr.opt32.address_of_entry_point = common::read_dword(mmap, offset + 42)?;
        pe.opt_hdr.opt32.base_of_code = common::read_dword(mmap, offset + 46)?;
        pe.opt_hdr.opt32.base_of_data = common::read_dword(mmap, offset + 50)?;
        pe.opt_hdr.opt32.image_base = common::read_dword(mmap, offset + 54)?;
        pe.opt_hdr.opt32.section_alignment = common::read_dword(mmap, offset + 58)?;
        pe.opt_hdr.opt32.file_alignment = common::read_dword(mmap, offset + 62)?;
        pe.opt_hdr.opt32.major_os_version = common::read_word(mmap, offset + 66)?;
        pe.opt_hdr.opt32.minor_os_version = common::read_word(mmap, offset + 68)?;
        pe.opt_hdr.opt32.major_image_version = common::read_word(mmap, offset + 70)?;
        pe.opt_hdr.opt32.minor_image_version = common::read_word(mmap, offset + 72)?;
        pe.opt_hdr.opt32.win32_version_value = common::read_dword(mmap, offset + 74)?;
        pe.opt_hdr.opt32.size_of_image = common::read_dword(mmap, offset + 78)?;
        pe.opt_hdr.opt32.size_of_headers = common::read_dword(mmap, offset + 82)?;
        pe.opt_hdr.opt32.checksum = common::read_dword(mmap, offset + 86)?;
        pe.opt_hdr.opt32.sub_system = common::read_word(mmap, offset + 88)?;
        pe.opt_hdr.opt32.dll_characteristics = common::read_word(mmap, offset + 90)?;
        pe.opt_hdr.opt32.size_of_stack_reserve = common::read_dword(mmap, offset + 92)?;
        pe.opt_hdr.opt32.size_of_stack_commit = common::read_dword(mmap, offset + 96)?;
        pe.opt_hdr.opt32.size_of_heap_reserve = common::read_dword(mmap, offset + 100)?;
        pe.opt_hdr.opt32.size_of_heap_commit = common::read_dword(mmap, offset + 104)?;
        pe.opt_hdr.opt32.loader_flags = common::read_dword(mmap, offset + 108)?;
        pe.opt_hdr.opt32.number_of_rva_and_sizes = common::read_dword(mmap, offset + 112)?;
    } else if pe.magic == 0x20b {

    } else {
        return Err(MultiError {
            kind: "InvalidImageType".to_string(),
            message: format!("Invalid image type: {:02x}", pe.magic),
        });
    }
    Ok(pe)
}