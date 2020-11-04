use libc::c_long;
use std::fmt::Display;
use std::fmt;
use crate::util::{read_word, read_long};
use crate::common::{Endianness};
use std::error::Error;

type WORD = u16;
type LONG = c_long;
type BYTE = u8;
type CHAR = u8;

#[derive(Debug, Clone, Default)]
pub struct ImageDosHeader {
    // DOS .EXE header
    pub e_magic: WORD,     // Magic number
    pub e_cbl_p: WORD,     //: BYTEs on last page of file
    pub e_cp: WORD,        // Pages in file
    pub e_crlc: WORD,      // Relocations
    pub e_cparhdr: WORD,   // Size of header in paragraphs
    pub e_minalloc: WORD,  // Minimum extra paragraphs needed
    pub e_maxalloc: WORD,  // Maximum extra paragraphs needed
    pub e_ss: WORD,        // Initial (relative) SS value
    pub e_sp: WORD,        // Initial SP value
    pub e_csum: WORD,      // Checksum
    pub e_ip: WORD,        // Initial IP value
    pub e_cs: WORD,        // Initial (relative) CS value
    pub e_lfarlc: WORD,    // File address of relocation table
    pub e_ovno: WORD,      // Overlay number
    pub e_res: [u16; 4],   // Reserved: WORDs
    pub e_oemid: WORD,     // OEM identifier (for e_oeminfo)
    pub e_oeminfo: WORD,   // information OEM, e_oemid specific
    pub e_res2: [u16; 10], // Reserved: WORDs
    pub e_lfanew: LONG,    // File address of new exe header
} // PIMAGE_DOS_HEADER IMAGE_DOS_HEADER,

impl Display for ImageDosHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "e_magic={:02x}, e_cbl_p={}, e_cp={}, e_crlc={}, e_cparhdr={}, e_minalloc={}, e_maxalloc={}, e_ss={:02x}, e_sp={:02x}, e_csum={:02x}, e_ip={:02x}, e_cs={:02x}, e_lfarlc={:02x}, e_ovno={}, e_oemid={:02x}, e_oeminfo={:02x}, e_lfanew={}",
            self.e_magic, self.e_cbl_p, self.e_cp, self.e_crlc, self.e_cparhdr, self.e_minalloc, self.e_maxalloc, self.e_ss, self.e_sp, self.e_csum, self.e_ip, self.e_cs, self.e_lfarlc, self.e_ovno, self.e_oemid, self.e_oeminfo, self.e_lfanew
        )
    }
}

impl ImageDosHeader {
    pub fn parse_from_bytes(bytes: &[u8], ptr: &mut usize) -> Result<ImageDosHeader, Box<dyn Error>>{
        let mut hdr: ImageDosHeader = Default::default();
        hdr.e_magic = read_word(bytes, ptr, Endianness::LittleEndian)?;
        *ptr +=2;

        hdr.e_cbl_p = read_word(bytes, ptr, Endianness::LittleEndian)?;
        *ptr +=2;

        hdr.e_cp = read_word(bytes, ptr, Endianness::LittleEndian)?;
        *ptr +=2;

        hdr.e_crlc = read_word(bytes, ptr, Endianness::LittleEndian)?;
        *ptr +=2;

        hdr.e_cparhdr = read_word(bytes, ptr, Endianness::LittleEndian)?;
        *ptr += 2;

        hdr.e_minalloc = read_word(bytes, ptr, Endianness::LittleEndian)?;
        *ptr +=2;

        hdr.e_maxalloc = read_word(bytes, ptr, Endianness::LittleEndian)?;
        *ptr +=2;

        hdr.e_ss = read_word(bytes, ptr, Endianness::LittleEndian)?;
        *ptr +=2;

        hdr.e_sp = read_word(bytes, ptr, Endianness::LittleEndian)?;
        *ptr +=2;

        hdr.e_csum = read_word(bytes, ptr, Endianness::LittleEndian)?;
        *ptr += 2;

        hdr.e_ip = read_word(bytes, ptr, Endianness::LittleEndian)?;
        *ptr += 2;

        hdr.e_cs = read_word(bytes, ptr, Endianness::LittleEndian)?;
        *ptr +=2 ;

        hdr.e_lfarlc = read_word(bytes, ptr, Endianness::LittleEndian)?;
        *ptr +=2 ;

        hdr.e_ovno = read_word(bytes, ptr, Endianness::LittleEndian)?;
        *ptr +=2;

        hdr.e_res = [read_word(bytes, ptr, Endianness::LittleEndian)?,
        read_word(bytes, &(*ptr+2), Endianness::LittleEndian)?,
        read_word(bytes, &(*ptr+4), Endianness::LittleEndian)?,
        read_word(bytes, &(*ptr+6), Endianness::LittleEndian)?];
        *ptr += 8;

        hdr.e_oemid = read_word(bytes, ptr, Endianness::LittleEndian)?;
        *ptr += 2;

        hdr.e_oeminfo = read_word(bytes, ptr, Endianness::LittleEndian)?;
        *ptr += 2;

        hdr.e_res2 = [read_word(bytes, ptr, Endianness::LittleEndian)?,
        read_word(bytes, &(*ptr+2), Endianness::LittleEndian)?,
        read_word(bytes, &(*ptr+4), Endianness::LittleEndian)?,
        read_word(bytes, &(*ptr+6), Endianness::LittleEndian)?,
        read_word(bytes, &(*ptr+8), Endianness::LittleEndian)?,
        read_word(bytes, &(*ptr+10), Endianness::LittleEndian)?,
        read_word(bytes, &(*ptr+12), Endianness::LittleEndian)?,
        read_word(bytes, &(*ptr+14), Endianness::LittleEndian)?,
        read_word(bytes, &(*ptr+16), Endianness::LittleEndian)?,
        read_word(bytes, &(*ptr+18), Endianness::LittleEndian)?,];
        *ptr += 20;

        hdr.e_lfanew = read_long(bytes, ptr, Endianness::LittleEndian)?;
        *ptr +=4;

        Ok(hdr)
    }

    
}

#[derive(Debug, Clone, Default)]
pub struct ImageOs2Header {
    // OS/2 .EXE header
    pub ne_magic: WORD,        // Magic number
    pub ne_ver: CHAR,          // Version number
    pub ne_rev: CHAR,          // Revision number
    pub ne_enttab: WORD,       // Offset of Entry Table
    pub ne_cbenttab: WORD,     // Number of: BYTEs in Entry Table
    pub ne_crc: LONG,          // Checksum of whole file
    pub ne_flags: WORD,        // Flag: WORD
    pub ne_autodata: WORD,     // Automatic data segment number
    pub ne_heap: WORD,         // Initial heap allocation
    pub ne_stack: WORD,        // Initial stack allocation
    pub ne_csip: LONG,         // Initial CS:IP setting; initial entry point; index into segment table
    pub ne_sssp: LONG,         // Initial SS:SP setting; initial stack pointer; index into segment table
    pub ne_cseg: WORD,         // Count of file segments
    pub ne_cmod: WORD,         // Entries in Module Reference Table
    pub ne_cbnrestab: WORD,    // Size of non-resident name table
    pub ne_segtab: WORD,       // Offset of Segment Table
    pub ne_rsrctab: WORD,      // Offset of Resource Table
    pub ne_restab: WORD,       // Offset of resident name table
    pub ne_modtab: WORD,       // Offset of Module Reference Table
    pub ne_imptab: WORD,       // Offset of Imported Names Table
    pub ne_nrestab: LONG,      // Offset of Non-resident Names Table
    pub ne_cmovent: WORD,      // Count of movable entries
    pub ne_align: WORD,        // Segment alignment shift count
    pub ne_cres: WORD,         // Count of resource segments
    pub ne_exetyp: BYTE,       // Target Operating system
    pub ne_flagsothers: BYTE,  // Other .EXE flags
    pub ne_pretthunks: WORD,   // offset to return thunks
    pub ne_psegrefbytes: WORD, // offset to segment ref.: BYTEs
    pub ne_swaparea: WORD,     // Minimum code swap area size
    pub ne_expver: WORD,       // Expected Windows version number
} // IMAGE_OS2_HEADER, *PIMAGE_OS2R _HEADE,

impl Display for ImageOs2Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write !(
            f,
            "ne_magic={:02x}, ne_ver={}, ne_rev={}, ne_enttab={}, ne_cbenttab={}, ne_crc={:04x}, ne_flags={:16b}, ne_autodata={:02x}, ne_heap={:02x}, ne_stack={:02x}, ne_csip={:04x}, ne_sssp={:04x}, ne_cseg={}, ne_cmod={}, ne_cbnrestab={}, ne_segtab={}, ne_rsrctab={}, ne_restab={}, ne_modtab={}, ne_imptab={}, ne_nrestab={}, ne_cmovent={}, ne_align={:02x}, ne_cres={}, ne_exetyp={:01x}, ne_flagsothers={:08b}, ne_pretthunks={}, ne_psegrefbytes={}, ne_swaparea={}, ne_exper={:02x}",
            self.ne_magic, self.ne_ver, self.ne_rev, self.ne_enttab, self.ne_cbenttab, self.ne_crc, self.ne_flags, self.ne_autodata, self.ne_heap, self.ne_stack, self.ne_csip, self.ne_sssp, self.ne_cseg, self.ne_cmod, self.ne_cbnrestab, self.ne_segtab, self.ne_rsrctab, self.ne_restab, self.ne_modtab, self.ne_imptab, self.ne_nrestab, self.ne_cmovent, self.ne_align, self.ne_cres, self.ne_exetyp, self.ne_flagsothers, self.ne_pretthunks, self.ne_psegrefbytes, self.ne_swaparea, self.ne_expver
        )
    }
}

impl ImageOs2Header{
    pub fn parse_from_bytes(bytes: &[u8], ptr: &mut usize) -> Result<ImageOs2Header, Box<dyn Error>> {
        let mut hdr: ImageOs2Header = Default::default();
        hdr.ne_magic = read_word(bytes, ptr, Endianness::LittleEndian)?;
        *ptr +=2;
        hdr.ne_ver = bytes[2];
        *ptr += 1;
        hdr.ne_rev = bytes[3];
        *ptr += 1;
        hdr.ne_enttab = read_word(bytes, ptr, Endianness::LittleEndian)?;
        *ptr += 2;
        hdr.ne_cbenttab = read_word(bytes, ptr, Endianness::LittleEndian)?;
        *ptr += 2;
        hdr.ne_crc = read_long(bytes, ptr, Endianness::LittleEndian)?;
        *ptr += 4;
        hdr.ne_flags = read_word(bytes, ptr, Endianness::LittleEndian)?;
        *ptr += 2;
        hdr.ne_autodata = read_word(bytes, ptr, Endianness::LittleEndian)?;
        *ptr += 2;
        hdr.ne_heap = read_word(bytes, ptr, Endianness::LittleEndian)?;
        *ptr += 2;
        hdr.ne_stack = read_word(bytes, ptr, Endianness::LittleEndian)?;
        *ptr += 2;
        hdr.ne_csip = read_long(bytes, ptr, Endianness::LittleEndian)?;
        *ptr += 4;
        hdr.ne_sssp = read_long(bytes, ptr, Endianness::LittleEndian)?;
        *ptr += 4;
        hdr.ne_cseg = read_word(bytes, ptr, Endianness::LittleEndian)?;
        *ptr += 2;
        hdr.ne_cmod = read_word(bytes, ptr, Endianness::LittleEndian)?;
        *ptr +=2 ;
        hdr.ne_cbnrestab = read_word(bytes, ptr, Endianness::LittleEndian)?;
        *ptr +=2 ;
        hdr.ne_segtab = read_word(bytes, ptr, Endianness::LittleEndian)?;
        *ptr += 2;
        hdr.ne_rsrctab = read_word(bytes, ptr, Endianness::LittleEndian)?;
        *ptr += 2;
        hdr.ne_restab = read_word(bytes, ptr, Endianness::LittleEndian)?;
        *ptr += 2;
        hdr.ne_modtab = read_word(bytes, ptr, Endianness::LittleEndian)?;
        *ptr += 2;
        hdr.ne_imptab = read_word(bytes, ptr, Endianness::LittleEndian)?;
        *ptr += 2;
        hdr.ne_nrestab = read_long(bytes, ptr, Endianness::LittleEndian)?;
        *ptr +=2;
        hdr.ne_cmovent = read_word(bytes, ptr, Endianness::LittleEndian)?;
        *ptr += 2;
        hdr.ne_align = read_word(bytes, ptr, Endianness::LittleEndian)?;
        *ptr += 2;
        hdr.ne_cres = read_word(bytes, ptr, Endianness::LittleEndian)?;
        *ptr += 2;
        hdr.ne_exetyp = bytes[*ptr];
        *ptr += 1;
        hdr.ne_flagsothers = bytes[*ptr];
        *ptr += 1;
        hdr.ne_pretthunks = read_word(bytes, ptr, Endianness::LittleEndian)?;
        *ptr += 2;
        hdr.ne_psegrefbytes = read_word(bytes, ptr, Endianness::LittleEndian)?;
        *ptr += 2;
        hdr.ne_swaparea = read_word(bytes, ptr, Endianness::LittleEndian)?;
        *ptr += 2;
        hdr.ne_expver = read_word(bytes, ptr, Endianness::LittleEndian)?;
        *ptr += 2;
        Ok(hdr)
    }
}

// ======================================================================
// SEGMENT TABLE
// ======================================================================

// The segment table contains an entry for each segment in the executable
// file. The number of segment table entries are defined in the segmented
// EXE header. The first entry in the segment table is segment number 1.
// The following is the structure of a segment table entry.

//    Size Description
//    ---- -----------

//    DW   Logical-sector offset (n byte) to the contents of the segment
//         data, relative to the beginning of the file. Zero means no
//         file data.

//    DW   Length of the segment in the file, in bytes. Zero means 64K.

//    DW   Flag word.
//         0007h = TYPE_MASK  Segment-type field.
//         0000h = CODE       Code-segment type.
//         0001h = DATA       Data-segment type.
//         0010h = MOVEABLE   Segment is not fixed.
//         0040h = PRELOAD    Segment will be preloaded; read-only if
//                            this is a data segment.
//         0100h = RELOCINFO  Set if segment has relocation records.
//         F000h = DISCARD    Discard priority.

//    DW   Minimum allocation size of the segment, in bytes. Total size
//         of the segment. Zero means 64K.

#[derive(Debug, Clone, PartialEq)]
pub enum NeSegmentFlag {
    CODE = 0x0000, // code segment type
    DATA = 0x0001, // data segment type
    TYPE_MASK = 0x0007, // segment type field
    MOVEABLE = 0x0010, // segment is not fixed
    PRELOAD = 0x0040, // segment will be preloaded; read only if this is a data segment
    RELOCINFO = 0x0100, // set if segment has relocation records
    DISCARD = 0xF000, // discard priority
}

impl Display for NeSegmentFlag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Default)]
pub struct NeSegmentTableEntry {
    pub logical_sector_offset: WORD,
    pub segment_length: WORD,
    pub flags: WORD,
    pub min_seg_alloc_sz: WORD
}

pub const NE_SEG_TBL_ENTRY_SZ: usize = 8;

impl Display for NeSegmentTableEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write !(
            f,
            "logical sector offset={}, segment length={}, flags={} ({:02x}), min seg alloc sz={}",
            self.logical_sector_offset,
            self.segment_length,
            self.get_flags_as_str(),
            self.flags,
            self.min_seg_alloc_sz
        )
    }
}

impl NeSegmentTableEntry {
    pub fn parse_from_bytes(bytes: &[u8], ptr: &mut usize) -> Result<NeSegmentTableEntry, Box<dyn Error>> {
        let mut entry: NeSegmentTableEntry = Default::default();
        entry.logical_sector_offset = read_word(bytes, ptr, Endianness::LittleEndian)?;
        *ptr +=2;
        entry.segment_length = read_word(bytes, ptr, Endianness::LittleEndian)?;
        *ptr +=2;
        entry.flags = read_word(bytes, ptr, Endianness::LittleEndian)?;
        *ptr +=2;
        entry.min_seg_alloc_sz = read_word(bytes, ptr, Endianness::LittleEndian)?;
        *ptr +=2;
        Ok(entry)
    }

    pub fn get_flags(&self) -> Vec<NeSegmentFlag> {
        let mut flags: Vec<NeSegmentFlag> = Vec::new();
        let type_part: u16 = self.flags & (NeSegmentFlag::TYPE_MASK as u16);

        if type_part & (NeSegmentFlag::DATA as u16) != 0 {
            flags.push(NeSegmentFlag::DATA);
        }
        if type_part == 0 {
            flags.push(NeSegmentFlag::CODE);
        }

        if self.flags & (NeSegmentFlag::MOVEABLE as u16) != 0 {
            flags.push(NeSegmentFlag::MOVEABLE);
        }
        if self.flags & (NeSegmentFlag::PRELOAD as u16) != 0 {
            flags.push(NeSegmentFlag::PRELOAD);
        }
        if self.flags & (NeSegmentFlag::RELOCINFO as u16) != 0 {
            flags.push(NeSegmentFlag::RELOCINFO);
        }
        if self.flags & (NeSegmentFlag::DISCARD as u16) != 0 {
            flags.push(NeSegmentFlag::DISCARD);
        }
        flags
    }

    pub fn get_flags_as_str(&self) -> String {
        let flags = self.get_flags();
        let mut out = String::new();
        for f in flags {
            out.push_str(format!("{}, ", f).as_str());
        }
        out
    }
}

#[derive(Debug, Clone, Default)]
pub struct NeSegmentTable {
    pub offset: usize,
    pub count: usize,
    pub entries: Vec<NeSegmentTableEntry>
}

impl Display for NeSegmentTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut entries = String::new();
        for e in &self.entries[..] {
            entries.push_str(format!("{{{}}}, ", e).as_str());
        }
        write!(
            f,
            "table offset={}, entries=[{}]", self.offset, entries
        )
    }
}

impl NeSegmentTable {
    pub fn parse_from_bytes(bytes: &[u8], ptr: &mut usize, seg_cnt: usize) -> Result<NeSegmentTable, Box<dyn Error>> {
        let mut table: NeSegmentTable = Default::default();
        table.offset = *ptr;
        table.count = seg_cnt;

        for _i in 0..seg_cnt {
            let entry = NeSegmentTableEntry::parse_from_bytes(bytes, ptr)?;
            table.entries.push(entry);
        }

        Ok(table)
    }
}

