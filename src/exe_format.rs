use libc::{c_void};

pub const INFINITE: u16 = 0xffff;

pub const IMAGE_DOS_SIGNATURE: u16 = 0x5A4D; // MZ
pub const IMAGE_OS2_SIGNATURE: u16 = 0x454E; // NE
pub const IMAGE_OS2_SIGNATURE_LE: u16 = 0x454C; // LE
pub const IMAGE_NT_SIGNATURE: u32 = 0x00004550; // PE00
pub const IMAGE_EDOS_SIGNATURE: u32 = 0x44454550; // PEED
pub const IMAGE_SIZEOF_FILE_HEADER: usize = 20;
pub const IMAGE_FILE_RELOCS_STRIPPED: u16 = 0x0001; // relocation info stripped from file
pub const IMAGE_FILE_EXECUTABLE_IMAGE: u16 = 0x0002; // file is executable (no unresolved external references)
pub const IMAGE_FILE_LINE_NUMBS_STRIPPED: u16 = 0x0004; // line numbers stripped from file
pub const IMAGE_FILE_LOCAL_SYMS_STRIPPED: u16 = 0x0008; // local symbols stripped from file
pub const IMAGE_FILE_BYTES_REVERSED_LO: u16 = 0x0080; // bytes of machine word are reversed
pub const IMAGE_FILE_32BIT_MACHINES: u16 = 0x0100; // 32 bit word machine
pub const IMAGE_FILE_DEBUG_STRIPPED: u16 = 0x0200; // debugging info stripped from file
pub const IMAGE_FILE_SYSTEM: u16 = 0x1000; // system file
pub const IMAGE_FILE_DLL: u16 = 0x2000; // file is a DLL
pub const IMAGE_FILE_BYTES_REVERSED_HI: u16 = 0x8000; //  bytes of machine word are reversed
pub const IMAGE_FILE_MACHINE_UNKNOWN: u16 = 0x0;
pub const IMAGE_FILE_MACHINE_I386: u16 = 0x14c; // intel 386
pub const IMAGE_FILE_MACHINE_R3000: u16 = 0x162; // MIPS little-endia 0540 big-endian
pub const IMAGE_FILE_MACHINE_R4000: u16 = 0x166; // MIPS little-endian
pub const IMAGE_FILE_MACHINE_ALPHA: u16 = 0x184; // Alpha AXP
pub const IMAGE_FILE_MACHINE_POWERPC: u16 = 0x1F0; // IBM PowerPC
pub const IMAGE_FILE_MACHINE_TAHOE: u16 = 0x7cc; // Intel EM Mmachine
pub const IMAGE_NUMBEROF_DIRECTORY_ENTRIES: usize = 16;
pub const IMAGE_SIZEOF_ROM_OPTIONAL_HEADER: usize = 56;
pub const IMAGE_SIZEOF_STD_OPTIONAL_HEADER: usize = 28;
pub const IMAGE_SIZEOF_NT_OPTIONAL_HEADER: usize = 224;
pub const IMAGE_NT_OPTIONAL_HEADER_MAGIC: u16 = 0x10b;
pub const IMAGE_ROM_OPTIONAL_HDR_MAGIC: u16 = 0x107;

pub const IMAGE_SUBSYSTEM_UNKNOWN: u16 = 0; // unknown subsystem
pub const IMAGE_SUBSYSTEM_NATIVE: u16 = 1; // image doesnt require a subsystem
pub const IMAGE_SUBSYSTEM_WINDOWS_GUI: u16 = 2; // image runs in the windows gui subsystem
pub const IMAGE_SUBSYSTEM_WINDOWS_CUI: u16 = 3; // image runs in the windows character subsystem
pub const IMAGE_SUBSYSTEM_OS2_CUI: u16 = 5; // image runs in the OS/2 character subsystem
pub const IMAGE_SUBSYSTEM_POSIX_CUI: u16 = 7; // image run in the posix character subsystem

// directory entries
pub const IMAGE_DIRECTORY_ENTRY_EXPORT: u16 = 0; // export directory
pub const IMAGE_DIRECTORY_ENTRY_IMPORT: u16 = 1; // import directory
pub const IMAGE_DIRECTORY_ENTRY_RESOURCE: u16 = 2; // resource directory
pub const IMAGE_DIRECTORY_ENTRY_EXCEPTION: u16 = 3; // exception directory
pub const IMAGE_DIRECTORY_ENTRY_SECURITY: u16 = 4; // security directory
pub const IMAGE_DIRECTORY_ENTRY_BASERELOC: u16 = 5; // base relocation table
pub const IMAGE_DIRECTORY_ENTRY_DEBUG: u16 = 6; // debug directory
pub const IMAGE_DIRECTORY_ENTRY_COPYRIGHT: u16 = 7; // description string
pub const IMAGE_DIRECTORY_ENTRY_GLOBALPTR: u16 = 8; // machine value (MIPS GP)
pub const IMAGE_DIRECTORY_ENTRY_TLS: u16 = 9; // tls directory
pub const IMAGE_DIRECTORY_ENTRY_LOAD_CONFIG: u16 = 10; // load configuration directory

pub const IMAGE_SIZEOF_SHORT_NAME: usize = 8;
pub const IMAGE_SIZEOF_SECTION_HEADER: usize = 40;

pub const IMAGE_SCN_TYPE_NO_PAD: u32 = 0x00000008; // reserved
pub const IMAGE_SCN_CNT_CODE: u32 = 0x00000020; // section contains code
pub const IMAGE_SCN_CNT_INITIALIZED_DATA: u32 = 0x00000040; // section contains initialized data
pub const IMAGE_SCN_CNT_UNITIALIZED_DATA: u32 = 0x00000080; // section contains uninitialized data
pub const IMAGE_SCN_LNK_OTHER: u32 = 0x00000100; // reserved
pub const IMAGE_SCN_LNK_INFO: u32 = 0x00000200; // section contains comments or some other type of information
pub const IMAGE_SCN_LNK_REMOVE: u32 = 0x00000800; // section contents will not become part of image
pub const IMAGE_SCN_LNK_COMDAT: u32 = 0x00001000; // section contents comdat
pub const IMAGE_SCN_ALIGN_1BYTES: u32 = 0x00100000; // 
pub const IMAGE_SCN_ALIGN_2BYTES: u32 = 0x00200000; //
pub const IMAGE_SCN_ALIGN_4BYTES: u32 = 0x00300000; //
pub const IMAGE_SCN_ALIGN_8BYTES: u32 = 0x00400000;
pub const IMAGE_SCN_ALIGN_16BYTES: u32 = 0x00500000;
pub const IMAGE_SCN_ALIGN_32BYTES: u32 = 0x00600000;
pub const IMAGE_SCN_ALIGN_64BYTES: u32 = 0x00700000;
pub const IMAGE_SCN_MEM_DISCARDABLE: u32 = 0x02000000; // section can be discarded
pub const IMAGE_SCN_MEM_NOT_CACHED: u32 = 0x04000000; // section is not cacheable
pub const IMAGE_SCN_MEM_NOT_PAGED: u32 = 0x08000000; // section is not pageable
pub const IMAGE_SCN_MEM_SHARED: u32 = 0x10000000; // section is shareable
pub const IMAGE_SCN_MEM_EXECUTE: u32 = 0x20000000; // section is executable
pub const IMAGE_SCN_MEM_READ: u32 = 0x40000000; // section readable
pub const IMAGE_SCN_MEM_WRITE: u32 = 0x80000000; // section is writeable
pub const IMAGE_SIZEOF_SYMBOL: usize = 18; // symbol format

// section values
// symbols have a section number of the section in which they are defined. otherwise, section numbers have the following meanings:
pub const IMAGE_SYM_UNDEFINED: u16 = 0; // symbol is undefined or is common
pub const IMAGE_SYM_ABSOLUTE: u16 = 0xFFFF; // symbol is an absolute value
pub const IMAGE_SYM_DEBUG: u16 = 0xFFFE; // symbol is a special debug item

// type values
pub const IMAGE_SYM_TYPE_NULL: u16 = 0; // no type
pub const IMAGE_SYM_TYPE_VOID: u16 = 1; // 
pub const IMAGE_SYM_TYPE_CHAR: u16 = 2; // type character
pub const IMAGE_SYM_TYPE_SHORT: u16 = 3; // type short integer
pub const IMAGE_SYM_TYPE_INT: u16 = 4; // 
pub const IMAGE_SYM_TYPE_LONG: u16 = 5;
pub const IMAGE_SYM_TYPE_FLOAT: u16 = 6;
pub const IMAGE_SYM_TYPE_DOUBLE: u16 = 7;
pub const IMAGE_SYM_TYPE_STRUCT: u16 = 8;
pub const IMAGE_SYM_TYPE_UNION: u16 = 9;
pub const IMAGE_SYM_TYPE_ENUM: u16 = 10;
pub const IMAGE_SYM_TYPE_MOE: u16 = 11; 
pub const IMAGE_SYM_TYPE_BYTE: u16 = 12;
pub const IMAGE_SYM_TYPE_WORD: u16 = 13;
pub const IMAGE_SYM_TYPE_UINT: u16 = 14;
pub const IMAGE_SYM_TYPE_DWORD: u16 = 15;

// type (derived) values
pub const IMAGE_SYM_DTYPE_NULL: u16 = 0; // no derived type
pub const IMAGE_SYM_DTYPE_POINTER: u16 = 1; // pointer
pub const IMAGE_SYM_DTYPE_FUNCTION: u16 = 2; // function
pub const IMAGE_SYM_DTYPE_ARRAY: u16 = 3; // array

// storage classes
pub const IMAGE_SYM_CLASS_END_OF_FUNCTION: u8 = 0xff;
pub const IMAGE_SYM_CLASS_NULL: u8 = 0;
pub const IMAGE_SYM_CLASS_AUTOMATIC: u8 = 1;
pub const IMAGE_SYM_CLASS_EXTERNAL: u8 = 2;
pub const IMAGE_SYM_CLASS_STATIC: u8 = 3;
pub const IMAGE_SYM_CLASS_REGISTER: u8 = 4;
pub const IMAGE_SYM_CLASS_EXTERNAL_DEF: u8 = 5;
pub const IMAGE_SYM_CLASS_LABEL: u8 = 6;
pub const IMAGE_SYM_CLASS_UNDEFINED_LABEL: u8 = 7;
pub const IMAGE_SYM_CLASS_MEMBER_OF_STRUCT: u8 = 8;
pub const IMAGE_SYM_CLASS_ARGUMENT: u8 = 9;
pub const IMAGE_SYM_CLASS_STRUCT_TAG: u8 = 10;
pub const IMAGE_SYM_CLASS_MEMBER_OF_UNION: u8 = 11;
pub const IMAGE_SYM_CLASS_UNION_TAG: u8 = 12;
pub const IMAGE_SYM_CLASS_TYPE_DEFINITION: u8 = 13;
pub const IMAGE_SYM_CLASS_UNDEFINED_STATIC: u8 = 14;
pub const IMAGE_SYM_CLASS_ENUM_TAG: u8 = 15;
pub const IMAGE_SYM_CLASS_MEMBER_OF_ENUM: u8 = 16;
pub const IMAGE_SYM_CLASS_REGISTER_PARAM: u8 = 17;
pub const IMAGE_SYM_CLASS_BIT_FIELD: u8 = 18;
pub const IMAGE_SYM_CLASS_BLOCK: u8 = 100;
pub const IMAGE_SYM_CLASS_FUNCTION: u8 = 101;
pub const IMAGE_SYM_CLASS_END_OF_STRUCT: u8 = 102;
pub const IMAGE_SYM_CLASS_FILE: u8 = 103;
pub const IMAGE_SYM_CLASS_SECTION: u8 = 104;
pub const IMAGE_SYM_CLASS_WEAK_EXTERNAL: u8 = 105;

// type packing constants
pub const N_BTMASK: u16 = 017;
pub const N_TMASK: u16 = 060;
pub const N_TMASK1: u16 = 0300;
pub const N_TMASK2: u16 = 0360;
pub const N_BTSHFT: u16 = 4;
pub const N_TSHFIT: u16 = 2;

// communal selection types
pub const IMAGE_COMDAT_SELECT_NODUPLICATES: u8 = 1;
pub const IMAGE_COMDAT_SELECT_ANY: u8 = 2;
pub const IMAGE_COMDAT_SELECT_SAME_SIZE: u8 = 3;
pub const IMAGE_COMADAT_SELECT_EXACT_MATCH: u8 = 4;
pub const IMAGE_COMDAT_SELECT_ASSOCIATIVE: u8 = 5;

pub const IMAGE_WEAK_EXTERN_SEARCH_NOLIBRARY: u8 = 1;
pub const IMAGE_WEAK_EXTERN_SEARCH_LIBRARY: u8 = 2;
pub const IMAGE_WEAK_EXTERN_SEARCH_ALIAS: u8 = 3;

pub const IMAGE_SIZEOF_RELOCATION: usize = 10;

pub const IMAGE_REL_I386_ABSOLUTE: u8 = 0; // reference is absolute no relocation is necessary
pub const IMAGE_REL_I386_DIR16: u8 = 01; // direct 16-bit reference to the symbol's virtual address
pub const IMAGE_REL_I386_REL16: u8 = 02; // pc-relative 16-bit reference to the symbol's virtual address
pub const IMAGE_REL_I386_DIR32: u8 = 06; // direct 32-bit reference to the symbol's virtual address
pub const IMAGE_REL_I386_DIR32NB: u8 = 07; // direct 32-bit reference to the symbol's virtual address, base not included
pub const IMAGE_REL_I386_SEG12: u8 = 011; // direct 16-bit reference to the segment-selector bits of a 32-bit virtual address
pub const IMAGE_REL_I386_SECTION: u8 = 012; 
pub const IMAGE_REL_I386_SECREL: u8 = 013;
pub const IMAGE_REL_I386_REL32: u8 = 024; // pc-relative 32-bit reference to the symbol's virtual address

pub const IMAGE_SIZEOF_BASE_RELOCATION: usize = 8;

// based relocation types
pub const IMAGE_REL_BASED_ABSOLUTE: u8 = 0;
pub const IMAGE_REL_BASED_HIGH: u8 = 1;
pub const IMAGE_REL_BASED_LOW: u8 = 2;
pub const IMAGE_REL_BASED_HIGHLOW: u8 = 3;
pub const IMAGE_REL_BASED_HIGHADJ: u8 = 4;
pub const IMAGE_REL_BASED_MIPS_JMPADDR: u8 = 5;
pub const IMAGE_REL_BASED_IA64_IMM64: u8 = 9;
pub const IMAGE_REL_BASED_DIR64: u8 = 10;

pub const IMAGE_SIZEOF_LINENUMBER: usize = 6;

pub const IMAGE_ARCHIVE_START_SIZE: usize = 8;

pub const IMAGE_ARCHIVE_START: str = "!<arch>\n";
pub const IMAGE_ARCHIVE_END: str = "`\n";
pub const IMAGE_ARCHIVE_PAD: str = "\n";
pub const IMAGE_ARCHIVE_LINKER_MEMBER: str = "/               ";
pub const IMAGE_ARCHIVE_LONGNAMES_MEMBER: str = "//              ";

pub const IMAGE_SIZEOF_ARCHIVE_MEMBER_HDR: usize = 60;

pub const IMAGE_ORDINAL_FLAG: u32 = 0x80000000;

pub const TOOL_HMASTER: usize = 0; // offset to hGlobalHeap in kdata.asm
pub const TOOL_HMODFIRST: usize = 4; // offset to hExeHead in kdata.asm
pub const TOOL_HEADTDB: usize = 14; // offset to headTDB in kdata.asm
pub const TOOL_HMASTLEN: usize = 22; // offset to SelTableLen in kdata.asm
pub const TOOL_HMASTSTART: usize = 24; // offset to SelTableStart in kdata.asm
pub const HI_FIRST: usize = 6; // offset to hi_first in heap header
pub const HI_SIZE: usize = 24; // size of heap info structure
pub const GI_LRUCHAIN: usize = 2; // offset to gi_lruchain in heap header
pub const GI_LRUCOUNT: usize = 4; // offset to gi_lrucount in heap header
pub const GI_FREECOUNT: usize = 16; // offset to gi_free_count in heap header
pub const GA_COUNT: usize = 0; // offset to ga_count in arena header
pub const GA_OWNER386: usize = 18; // offset to "pga_onwer" member in globalareana
pub const GA_OWNER: usize = 1; // offset to "owner" member within Arena
pub const GA_FLAGS: usize = 5; // offset to ga_flags in arena header
pub const GA_NEXT: usize = 9; // offset to ga_next in arena header
pub const GA_HANDLE: usize = 10; // offset to ga_handle in arena header
pub const GA_LRUNEXT: usize = 14; // offset to ga_lrunext
pub const GA_FREENEXT: usize = GA_LRUNEXT; // offset to ga_freenext in arena header
pub const GA_SIZE: usize = 16; // size of the GlobalArena structure

pub const LI_SIG: usize = HI_SIZE + 10; // offset to signature
pub const LI_SIZE: usize = HI_SIZE + 12; // size of LocalInfo structure
pub const LOCALSIG: u16 = 0x4C48; // 'HL' signature

pub const TDB_NEXT: usize = 0; // offset to next TDB in TDB
pub const TDB_PDB: usize = 72; // offset to PDB in TDB

pub const GF_PDB_OWNER: usize = 0x100; // low byte is kernel flags
pub const NE_MAGIC: usize = 0; // offset to NE in module header
pub const NE_USAGE: usize = 2; // offset to usage
pub const NE_CBENTTAB: usize = 6; // offset to cbenttab (really next module pointer)
pub const NE_PATHOFFSET: usize = 10; // offset to file path stuff
pub const NE_CSEG: usize = 28; // offset to cseg, number of segs in module
pub const NE_SEGTAB: usize = 34; // offset to segment table ptr in modhdr
pub const NE_RESTAB: usize = 38; // offset to resident names table ptr in modhdr
pub const NS_HANDLE: usize = 8; // offset to handle in seg table
pub const NEW_SEG1_SIZE: usize = 10; // size of the NS_ stuff

pub const SEGTYPE_V86: u8 = 1;
pub const SEGTYPE_PROT: u8 = 2;

// #define HANDLE_NULL ((HANDLE)NULL)
pub const HANDLE_NULL: u32 = 0;

// TODO: define infinite
pub const LONG_TIMEOUT: u32 = INFINITE;

pub static w_kernel_seg: u16 = 0;
pub static dw_offset_th_hook: u32 = 0;
pub static lp_remote_address: u32 = 0; // LPVOID 
pub static lp_remote_block: u32 = 0;
pub static f_kernel_386: bool = false;
pub static dw_ldt_base: u32 = 0;
pub static dw_intel_base: u32 = 0;
pub static lp_nt_vmd_state: u32 = 0; // LPVOID
pub static lp_nt_cpu_info: u32 = 0; // LPVOID
pub static lp_vdm_context: u32 = 0; // LPVOID
pub static lp_vdm_break_points: u32 = 0; // LPVOID


#[derive(Debug, Clone, Default)]
pub struct IMAGE_DOS_HEADER {
    pub e_magic: u16, // magic number
    pub e_cblp: u16, // bytes on laste page of file
    pub e_cp: u16, // pages in file
    pub e_crlc: u16, // relocations
    pub e_cparhdr: u16, // size of header in paragraphs
    pub e_minalloc: u16, // minimum extra paragraphs
    pub e_maxalloc: u16, // maximum extra paragraphs
    pub e_ss: u16, // initial relative SS value
    pub e_sp: u16, // initial SP value
    pub e_csum: u16, // checksum
    pub e_ip: u16, // initial IP value
    pub e_cs: u16, // initial relative CS value
    pub e_lfarlc: u16, // file address of relocation table
    pub e_ovno: u16, // overlay number
    pub e_res: [u16;4], // reserved words
    pub e_oemid: u16, // OEM identifier for e_oeminfo
    pub e_oeminfo: u16, // OEM information; e_oemid specific
    pub e_res2: [u16;10], // reserved words
    pub e_lfanew: u32, // file address of new exe header
}

#[derive(Debug, Clone, Default)]
pub struct IMAGE_OS_HEADER {
    pub ne_magic: u16, // magic number
    pub ne_ver: u8, // version number
    pub ne_rev: u8, // revision number
    pub ne_enttab: u16, // offset of entry table
    pub ne_cbenttab: u16, // number of bytes in entry table
    pub ne_crc: u32, // checksum of whole file
    pub ne_flags: u16, // flag uint16
    pub ne_autodata: u16, // automatic data segment number
    pub ne_heap: u16, // initial heap allocation
    pub ne_stack: u16, // initial stack allocation
    pub ne_csip: u32, // initial CS:IP setting
    pub ne_sssp: u32, // initial SS:SP setting
    pub ne_cseg: u16, // count of file segments
    pub ne_cmod: u16, // entries in module reference table
    pub ne_cbnrestab: u16, // size of non-resident name table
    pub ne_segtab: u16, // offset of segment table
    pub ne_rsrctab: u16, // offset of resource table
    pub ne_restab: u16, // offset of resident name table
    pub ne_modtab: u16, // offset of module reference table
    pub ne_imptab: u16, // offset of imported names table
    pub ne_nrestab: u16, // offset of non-resident names table
    pub ne_cmovent: u16, // count of movable entries
    pub ne_align: u16, // segment alignment shift
    pub ne_cres: u16, // count of resource segments
    pub ne_exetyp: u8, // target operating system
    pub ne_flagsothers: u8, // other EXE flags
    pub ne_pretthunks: u16, // offset to return thunks
    pub ne_psegrefbytes: u16, // offset to segment ref bytes
    pub ne_swaparea: u16, // minimum  code swap area
    pub ne_expver: u16, // expected windows version number
}

#[derive(Debug, Clone, Default)]
pub struct IMAGE_FILE_HEADER {
    pub machine: u16,
    pub number_of_sections: u16,
    pub time_date_stamp: u32,
    pub pointer_to_symbol_table: u32,
    pub number_of_symbols: u32,
    pub size_of_optional_header: u16,
    pub characteristics: u16,
}

#[derive(Debug, Clone, Default)]
pub struct IMAGE_DATA_DIRECTORY {
    pub virtual_address: u32,
    pub size: u32,
}

#[derive(Debug, Clone, Default)]
pub struct IMAGE_OPTIONAL_HEADER {
    // standard fields
    pub magic: u16,
    pub major_linker_version: u8,
    pub minor_linker_version: u8,
    pub size_of_code: u32,
    pub size_of_initialized_data: u32,
    pub size_of_uninitialized_data: u32,
    pub address_of_entry_point: u32,
    pub base_of_code: u32,
    pub base_of_data: u32,
    // NT additional fields
    pub image_base: u32,
    pub section_alignment: u32,
    pub file_alignment: u32,
    pub major_operating_system_version: u16,
    pub minor_operating_system_version: u16,
    pub major_image_version: u16, 
    pub minor_image_version: u16,
    pub major_sub_system_version: u16,
    pub minor_sub_system_version: u16,
    pub reserved1: u32,
    pub size_of_image: u32,
    pub size_of_headers: u32,
    pub checksum: u32,
    pub sub_system: u16,
    pub dll_characteristics: u16,
    pub size_of_stack_reserve: u32,
    pub size_of_stack_commit: u32,
    pub size_of_heap_reserve: u32,
    pub size_of_heap_commit: u32,
    pub loader_flags: u32,
    pub number_of_rva_and_sizes: u32,
    pub data_directory: [IMAGE_DATA_DIRECTORY;IMAGE_NUMBEROF_DIRECTORY_ENTRIES],
}

#[derive(Debug, Clone, Default)]
pub struct IMAGE_ROM_OPTIONAL_HEADER {
    pub magic: u16,
    pub major_linker_version: u8,
    pub minor_linker_version: u8,
    pub size_of_code: u32,
    pub size_of_initialized_data: u32,
    pub size_of_uninitialized_data: u32,
    pub address_of_entry_point: u32,
    pub base_of_code: u32,
    pub base_of_data: u32,
    pub base_of_bss: u32,
    pub gpr_mask: u32,
    pub cpr_mask: [u32;4],
    pub gp_value: u32,
}

#[derive(Debug, Clone, Default)]
pub struct IMAGE_NT_HEADERS {
    pub signature: u32,
    pub file_header: IMAGE_FILE_HEADER,
    pub optional_header: IMAGE_OPTIONAL_HEADER,
}

#[derive(Debug, Clone, Default)]
pub struct IMAGE_ROM_HEADERS {
    pub file_header: IMAGE_FILE_HEADER,
    pub optional_header: IMAGE_ROM_OPTIONAL_HEADER,
}

#[repr(C)]
pub union image_section_header_misc {
    physical_address: u32,
    virtual_size: u32,
}

#[derive(Debug, Clone, Default)]
pub struct IMAGE_SECTION_HEADER {
    pub name: [u8;IMAGE_SIZEOF_SHORT_NAME],
    pub misc: image_section_header_misc,
    pub virtual_address: u32,
    pub size_of_raw_data: u32,
    pub pointer_to_raw_data: u32,
    pub pointer_to_relocations: u32,
    pub pointer_to_line_numbers: u32,
    pub number_of_relocations: u16,
    pub number_of_line_numbers: u16,
    pub characteristics: u32,
}

#[derive(Debug, Clone, Default)]
pub struct IMAGE_BASE_RELOCATION {
    pub virtual_address: u32,
    pub size_of_block: u32,
}


#[repr(C)]
pub union IMAGE_LINE_NUMBER_TYPE {
    pub symbol_table_index: u32,
    pub virtual_address: u32,
}

#[derive(Debug, Clone, Default)]
pub struct IMAGE_LINENUMBER {
    pub line_num_type: IMAGE_LINE_NUMBER_TYPE,
    pub line_number: u16,
}

#[derive(Debug, Clone, Default)]

pub struct IMAGE_ARCHIVE_MEMBER_HEADER {
    pub name: [u8;16], // file member name `/' terminated
    pub date: [u8;12], // file member date - decimal
    pub user_id: [u8;6], // file member user id - decimal
    pub group_id: [u8;6], // file member group id - decimal
    pub mode: [u8;8], // file member mode - octal
    pub size: [u8;10], // file member size - decimal
    pub end_header: [u8;2], // string to end header
}

#[derive(Debug, Clone, Default)]
pub struct IMAGE_EXPORT_DIRECTORY {
    pub characteristics: u32,
    pub time_date_stamp: u32,
    pub major_version: u16,
    pub minor_version: u16,
    pub name: u32,
    pub base: u32,
    pub number_of_functions: u32,
    pub number_of_names: u32,
    pub address_of_functions: *mut u32,
    pub address_of_names: *mut u32,
    pub address_of_name_ordinals: *mut u32,
}

#[derive(Debug, Clone, Default)]
pub struct IMAGE_IMPORT_BY_NAME {
    pub hint: u16,
    pub name: [u8;1]
}


#[repr(C)]
pub union image_thunk_data_u1 {
    function: u32,
    orginal: u32,
    address_of_data: *mut IMAGE_IMPORT_BY_NAME,
}

#[derive(Debug, Clone, Default)]
pub struct IMAGE_THUNK_DATA {
    pub u1: image_thunk_data_u1,
}

#[derive(Debug, Clone, Default)]
pub struct IMAGE_IMPORT_DESCRIPTOR {
    pub characteristics: u32,
    pub time_data_stamp: u32,
    pub forwarder_chain: u32,
    pub name: u32,
    pub first_thunk: *mut IMAGE_THUNK_DATA,
}

#[derive(Debug, Clone, Default)]
pub struct GLOBALENTRY16 {
    pub dw_size: u32,
    pub dw_address: u32,
    pub dw_block_size: u32,
    pub h_block: u16,
    pub wc_lock: u16,
    pub wc_page_lock: u16,
    pub w_flags: u16,
    pub w_heap_present: u16,
    pub h_owner: u16,
    pub w_type: u16,
    pub w_data: u16,
    pub dw_next: u32,
    pub dw_next_alt: u32,
}

// TODO: find def for MAX_MODULE_NAME
// TODO: find def ro MAX_PATH_16
#[derive(Debug, Clone, Default)]
pub struct MODULEENTRY16 {
    pub dw_size: u32,
    pub sz_module: [u8;MAX_MODULE_NAME],
    pub h_module: u16,
    pub wc_usage: u16,
    pub sz_exe_path: [u8;MAX_PATH_16],
    pub w_next: u16,
}

#[derive(Debug, Clone, Default)]
pub struct seg_entry {
    // struct _segentry *Next,
    pub seg_entry_type: i32,
    pub sz_exe_path: [u8;MAX_PATH_16],
    pub sz_module: [u8;MAX_MODULE_NAME],
    pub selector: u16,
    pub segment: u16,
    pub length: u32,
}

#[derive(Debug, Clone, Default)]
pub struct GNODE32 {
    pub pga_next: u32, // next arena entry, last points to self
    pub pga_prev: u32, // previous areana entry (first points to self)
    pub pga_address: u32, // 32-bit linear address of memory
    pub pga_size: u32, // 32_bit size in bytes
    pub pga_handle: u16, // back link to handle table entry
    pub pga_owner: u16, // owner field
    pub pga_count: u8, // lock count for movable segments
    pub pga_pglock: u8, // # times page locked
    pub pga_flags: u8, // 1 word available for flags
    pub pga_selcount: u8, // number of selectors allocated
    pub pga_lruprev: u32, // previous entry in LRU chain
    pub pga_lrunext: u32, // next entry in LRU chain
}

#[derive(Debug, Clone, Default)]
pub struct GHI32 {
    pub hi_check: u16, // arena check word non-zero enables heap checking
    pub hi_freeze: u16, // arena frozen word; non-zero prevents compaction
    pub hi_count: u16, // # of entries in arena
    pub hi_first: u16, // first arena entry (sentinel, always busy)
    pub hi_res1: u16, // reserved
    pub hi_last: u16, // last arena entry (sentinel, always busy)
    pub hi_res2: u16, // reserved
    pub hi_ncompact: u8, // #compactions done so far (max of 3)
    pub hi_dislevel: u8, // current discard level
    pub hi_distotal: u32, // total amount discarded so far
    pub hi_htable: u16, // head of handle table list
    pub hi_hfree: u16, // head of free handle table list
    pub hi_hdelta: u16, // handles to allocate each time
    pub hi_hexpand: u16, // address of near procedure to expand handles for this arena
    pub hi_pstats: u16, // address of statistics table or zero
}

#[derive(Debug, Clone, Default)]
pub struct HEAPENTRY {
    pub gnode: GNODE32,
    pub current_entry: u32,
    pub next_entry: u32,
    pub selector: u16,
    pub segment_number: i32,
    pub owner_name: [u8;9],
    pub file_name: [u8;9],
    pub module_arg: [u8;9],
}

#[derive(Debug, Clone, Default)]
pub struct NEHEADER {
    pub ne_magic: u16,
    pub ne_ver: u8,
    pub ne_rev: u8,
    pub ne_enttab: u16,
    pub ne_cbenttab: u16,
    pub ne_crc : u32,
    pub ne_flgas: u16,
    pub ne_autodata: u16,
    pub ne_heap: u16,
    pub ne_stack: u16,
    pub ne_csip: u32,
    pub ne_sssp: u32,
    pub ne_cseg: u16,
    pub ne_cmod: u16,
    pub ne_cbnrestab: u16,
    pub ne_segtab: u16,
    pub ne_rsrctab: u16,
    pub ne_restab: u16,
    pub ne_modtab: u16,
    pub ne_imptab: u16,
    pub ne_nrestab: u32,
    pub ne_cmovent: u16,
    pub ne_align: u16,
    pub ne_cres: u16,
    pub ne_exetyp: u8,
    pub ne_flagsothers: u8,
    pub ne_pretthunks: u16,
    pub ne_psegrefbytes: u16,
    pub ne_swaparea: u16,
    pub ne_expver: u16
}

#[derive(Debug, Clone, Default)]
pub struct NT_CPU_REG {
    pub nano_reg: *mut u32, // where the nano CPU keeps the register
    pub reg: *mut u32, // whree the light compiler keeps the register
    pub saved_reg: *mut u32, // where currently unused bits are kept
    pub universe_8bit_mask: u32, // is register in 8-bit form?
    pub universe_16bit_mask: u32, // is register in 16-bit form?
}

#[derive(Debug, Clone)]
pub struct NT_CPU_INFO {
    // variables for deciding which mode we're in
    pub in_nano_cpu: *mut bool, // is the nano cpu executing?
    pub universe: *mut u32, // the mode that the CPU is in
    // general purpose register pointers
    pub eax: NT_CPU_REG,
    pub ebx: NT_CPU_REG,
    pub ecx: NT_CPU_REG,
    pub edx: NT_CPU_REG,
    pub esi: NT_CPU_REG,
    pub edi: NT_CPU_REG,
    pub ebp: NT_CPU_REG,
    // variables for getting SP or ESP
    pub stack_is_big: *mut bool, // is the stack 32-bit?
    pub nano_esp: *mut u32, // where the nano cpu keeps esp
    pub host_sp: *mut *mut u8, // pointer to variable holding stack pointer as a host address
    pub ss_base: *mut *mut u8, // pointer to variables holding base of SS as a host address
    pub esp_sanctuary: *mut u32, // top 16 bits of ESP if we're now using SP
    pub eip: *mut u32,
    // segment registers
    pub cs: *mut u16,
    pub ds: *mut u16,
    pub es: *mut u16,
    pub fs: *mut u16,
    pub gs: *mut u16,
    pub ss: *mut u16,

    pub flags: *mut u32,

    // cr0, mainly to let us figure out if we're in real or protect mode
    pub cr0: *mut u32

}

/*

#define IMAGE_FIRST_SECTION( ntheader ) ((PIMAGE_SECTION_HEADER)        \
    ((UINT32)ntheader +                                                  \
     FIELD_OFFSET( IMAGE_NT_HEADERS, OptionalHeader ) +                 \
     ((PIMAGE_NT_HEADERS)(ntheader))->FileHeader.SizeOfOptionalHeader   \
    ))

*/
pub fn image_first_section() {

}

/*

#define IMAGE_SNAP_BY_ORDINAL(Ordinal) ((Ordinal & IMAGE_ORDINAL_FLAG) != 0)

*/
pub fn image_snap_by_ordinal() {

}

/*

#define IMAGE_ORDINAL(Ordinal) (Ordinal & 0xffff)

*/
pub fn image_ordinal() {

}


/*
#define READ_FIXED_ITEM(seg,offset,item)  \
    if ( ReadItem(hProcess,seg,offset,&item,sizeof(item)) ) goto punt;
*/
pub fn read_fixed_item() {

}

/*
#define WRITE_FIXED_ITEM(seg,offset,item)  \
    if ( WriteItem(hProcess,seg,offset,&item,sizeof(item)) ) goto punt;
*/
pub fn write_fixed_item() {

}

/*
#define LOAD_FIXED_ITEM(seg,offset,item)  \
    ReadItem(hProcess,seg,offset,&item,sizeof(item))
*/
pub fn load_fixed_item() {

}

/*
#define READ_SIZED_ITEM(seg,offset,item,size)  \
    if ( ReadItem(hProcess,seg,offset,item,size) ) goto punt;
*/
pub fn read_sized_item() {

}


/*
#define WRITE_SIZED_ITEM(seg,offset,item,size)  \
    if ( WriteItem(hProcess,seg,offset,item,size) ) goto punt;
*/
pub fn write_sized_item() {

}

/*
#define MALLOC(cb) HeapAlloc(GetProcessHeap(), HEAP_GENERATE_EXCEPTIONS, cb)
*/
pub fn malloc() {

}

/*
#define FREE(addr) HeapFree(GetProcessHeap(), 0, addr)
*/
pub fn free() {

}