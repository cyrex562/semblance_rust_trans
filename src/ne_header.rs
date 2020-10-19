use crate::multi_error::MultiError;
use crate::app_context::AppContext;
use crate::util::{read_byte, read_word};
use crate::dump::PeImportModule;
use std::fs::File;
use std::io::Read;

pub struct NeHeader {
    pub ne_magic: u16,             /* 00 NE signature 'NE' */
    pub ne_ver: u8,               /* 02 Linker version number */
    pub ne_rev: u8,               /* 03 Linker revision number */
    pub ne_enttab: u16,            /* 04 Offset to entry table */
    pub ne_cbenttab: u16,          /* 06 Length of entry table in bytes */
    pub ne_crc: u32,               /* 08 Checksum */
    pub ne_flags: u16,             /* 0c Flags about segments in this file */
    pub ne_autodata: u8,          /* 0e Automatic data segment number */
    pub ne_unuse: u8,            /* 0f */
    pub ne_heap: u16,              /* 10 Initial size of local heap */
    pub ne_stack: u16,             /* 12 Initial size of stack */
    pub ne_ip: u16,                /* 14 Initial IP */
    pub ne_cs: u16,                /* 16 Initial CS */
    pub ne_sp: u16,                /* 18 Initial SP */
    pub ne_ss: u16,                /* 1a Initial SS */
    pub ne_cseg: u16,              /* 1c # of entries in segment table */
    pub ne_cmod: u16,              /* 1e # of entries in import module table */
    pub ne_cbnrestab: u16,         /* 20 Length of nonresident-name table */
    pub ne_segtab: u16,            /* 22 Offset to segment table */
    pub ne_rsrctab: u16,           /* 24 Offset to resource table */
    pub ne_restab: u16,            /* 26 Offset to resident-name table */
    pub ne_modtab: u16,            /* 28 Offset to import module table */
    pub ne_imptab: u16,            /* 2a Offset to name table */
    pub ne_nrestab: u32,           /* 2c ABSOLUTE Offset to nonresident-name table */
    pub ne_cmovent: u16,           /* 30 # of movable entry points */
    pub ne_align: u16,             /* 32 Logical sector alignment shift count */
    pub ne_cres: u16,              /* 34 # of resource segments */
    pub ne_exetyp: u8,            /* 36 Flags indicating target OS */
    pub ne_flagsothers: u8,       /* 37 Additional information flags */
    pub ne_pretthunks: u16,        /* 38 Offset to return thunks */
    pub ne_psegrefbytes: u16,      /* 3a Offset to segment ref. bytes */
    pub ne_swaparea: u16,          /* 3c Reserved by Microsoft */
    pub ne_expver_min: u8,        /* 3e Expected Windows version number (minor) */
    pub ne_expver_maj: u8,        /* 3f Expected Windows version number (major) */
}


pub struct NeEntry {
    pub flags: u8,
    pub segment: u8,
    pub offset: u16,
    // char *name:;     /* may be NULL */
    pub name: *mut libc::c_char,
}

pub struct NeExport {
    pub ordinal: u16,
    pub name: *mut libc::c_char,
}

pub struct NeImportModule {
    pub name: String,
    pub exports: Vec<NeExport>
}

pub struct NeReloc {
    pub size: u8,
    pub reloc_type: u8,
    pub offset_count: u16,
    pub offsets: *mut u16,
    pub tseg: u16,
    pub toffset: u16,
    pub text: *mut libc::c_char,
}

pub struct NeSegment {
    pub cs: u16,
    pub start: libc::c_long,
    pub seg_length: u16,
    pub flags: u16,
    pub min_alloc: u16,
    pub instr_flags: *mut u8,
    pub reloc_table: *mut NeReloc,
    pub reloc_count: u16,
}

pub struct NeInfo {
    pub header: NeHeader,
    pub name: String,
    pub description: String,
    pub nametab: Vec<String>,
    // pub enttab: *mut NeEntry,
    pub enttab: Vec<NeEntry>,
    // pub imptab: *mut NeImportModule,
    pub imptab: Vec<NeImportModule>,
    // pub segments: *mut NeSegment,
    pub segments: Vec<NeSegment>
}

pub fn parse_flags(flags: &u16) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();

    if (flags & 0x0003) == 0 {
        out.push("no DGROUP".to_string());
    } else if (flags & 0x0003) == 1{
        out.push("single DGROUP".to_string());
    } else if (flags & 0x0003) == 2 {
        out.push("multiple DGROUPs".to_string());
    } else if (flags & 0x0003) == 3 {
        out.push("unknown DGROUP type 3".to_string());
    }

    if flags & 0x0004 {
        out.push("global initialization".to_string());
    }
    if flags & 0x0008 {
        out.push("protected mode only".to_string());
    }
    if flags & 0x0010 {
        out.push("8086".to_string());
    }
    if flags & 0x0020 {
        out.push("80286".to_string());
    }
    if flags & 0x0040 {
        out.push("80386".to_string());
    }
    if flags & 0x0080 {
        out.push("80x87".to_string());
    }

    if (flags & 0x0700) == 0x0100 {
        out.push("fullscreen".to_string());
    } else if (flags & 0x0700) == 0x0200 {
        out.push("console".to_string());
    } else if (flags & 0x0700) == 0x0300 {
        out.push("GUI".to_string());
    } else if (flags & 0x0700) == 0x0 {
        out.push("no subsystem".to_string());
    } else {
        out.push(format!("unknown application type {}", (flags & 0x0700) >> 8));
    }

    if flags & 0x0800 {
        out.push("self-loading".to_string());
    }
    if flags & 0x1000 {
        out.push("unknown flag 0x1000".to_string());
    }
    if flags & 0x2000 {
        out.push("contains linker errors".to_string());
    }
    if flags & 0x4000 {
        out.push("non-conforming program".to_string());
    }
    if flags & 0x8000 {
        out.push("library".to_string());
    }

    out
}

pub const EXE_TYPES: [String;6] = [
    "unknown".to_string(),
    "OS/2".to_string(),
    "Windows (16-bit)".to_string(),
    "Europe Dos 4.x".to_string(),
    "Windows 386 (32-bit)".to_string(),
    "BOSS".to_string(),
];

pub fn demangle_protection(buffer: &mut Vec<u8>, start: &[u8], prot: &u8, func: &[u8]) -> usize {
    if start[0] >= b'A' && start[0] <= b'V' {
        if (start[0] - b'A') & 2 {
            buffer.push_str("static ");
        }
        if (start[0] - b'A') & 4 {
            buffer.push_str("virtual ");
        }
        if (start[0] - b'A') & 1 {
            buffer.push_str("near ");
        }
        if ((start[0] - b'A') & 24) == 0 {
            buffer.push_str("private ");
        }
        else if ((start[0] - b'A') & 24) == 8 {
            buffer.push_str("protected ");
        } else if ((start[0] - b'A') & 24) == 16 {
            buffer.push_str("public ");
        }
        prot[0] = start[0];
    } else if start[0] == b'Y' {
        buffer.push_str("near ");
    } else if start[0] == b'Z' {

    } else if start[0] == b'X' {
        prot[0] = b'V';
        return if start[1] >= b'0' && start[1] <= b'9' {
            buffer.push_str("(X0)");
            buffer[buffer.len() - 3] = start[1];
            2
        } else {
            start.find(b'@').unwrap() + 1
        }
    } else if start[0] == b'_' && start[1] != b'$' {
        demangle_protection(buffer, &start[1..], prot, func);
        if start[3] >= b'0' && start[3] <= b'9' {
            buffer.push_str("(_00) ");
            buffer[buffer.len() - 4] = start[2];
            buffer[buffer.len() - 3] = start[3];
            return 4;
        } else {
            return start.find(b'@').unwrap() + 1;
        }

        return 0;
    } else {
        return 0;
    }
    return 1;
}

pub const INT_TYPES: [String;9] = [
    "signed char".to_string(),
    "char".to_string(),
    "unsigned char".to_string(),
    "short".to_string(),
    "unsigned short".to_string(),
    "int".to_string(),
    "usigned int".to_string(),
    "long".to_string(),
    "unsigned long".to_string(),
];


pub fn demangle_type(known_names: &mut Vec<String>, buffer: &mut Vec<u8>, tbuf: &[u8]) -> usize {
    if tbuf[0] >= b'C' && tbuf[0] <= b'K' {
        buffer.push_str(INT_TYPES[tbuf[0] - b'C'].to_str());
        buffer.push_str(" ");
        return 1;
    }

    return match tbuf[0] {
        b'A' | b'P' => {
            let mut ret = 0;
            if (tbuf[1] - b'A') & 1 {
                buffer.push_str("const ");
            }
            if (tbuf[1] - b'A') & 2 {
                buffer.push_str("volatile ");
            }
            ret = demangle_type(known_names, buffer, &tbuf[2..]);
            if !((tbuf[1] - b'A') & 4) {
                buffer.push_str("near ");
            }
            if tbuf[0] == b'A' {
                buffer.push_str("&");
            } else {
                buffer.push_str("*");
            }
            ret + 2
        }
        b'M' => {
            buffer.push_str("float ");
            1
        },
        b'N' => {
            buffer.push_str("double ");
            1
        },
        b'U' | b'V' => {
            let i: u32 = 0;
            let mut end: usize = 0;

            if tbuf[1] >= b'0' && tbuf[1] <= b'9' {
                buffer.push_str(known_names[type_str[1] - b'0']);
                buffer.push_str(" ");
                return 3;
            }

            end = tbuf.find(b"@").unwrap();
            end += 1;
            end = tbuf[end..].find(b"@").unwrap();
            if tbuf[end - 1] == b"@" {
                buffer.push_str(type_str[1..end - 2]);
            } else {
                buffer.push_str(type_str[1..end - 1]);
            }

            // todo: check if buffer already in known names
            known_names.push(buffer.clone());
            buffer.push_str(" ");
            end + 1
        },
        b'X' => {
            buffer.push_str("void ");
            1
        },
        _ => {
            0
        }
    };
}

pub fn demangle(func: &[u8]) -> Result<Vec<u8>, MultiError> {
    let mut out: Vec<u8> = Vec::new();
    let mut known_names: Vec<String> = Vec::new();
    let mut known_types: Vec<String> = Vec::new();
    let mut known_type_idx = 0;
    let mut known_name_idx = 0;
    let mut buffer: Vec<u8> = Vec::with_capacity(1024);
    let mut start: usize = 0;
    let mut end: usize = 0;
    let mut ptr: &[u8];
    let mut len = 0;
    let mut prot: u8 = 0;

    if func[1] == b'?' {
        let func_str = func.to_str()?;
        out.push_str(func_str);
        return Ok(out);
    }

    // populate the known names up to the function name
    ptr = func;
    while ptr[0] != b'@' && known_name_idx < 10 {
        let copy_start = ptr.find(b'@').unwrap();
        let known_name = String::from(ptr[copy_start..]);
        known_names.push(known_name);
        known_name_idx += 1;
        let next = ptr.find(b'@').unwrap() + 1;
        ptr = &ptr[next..];
    }

    // figure out the modifiers and calling convention
    buffer[0] = 0;
    let mut idx = func.find("@@").unwrap() + 2;
    ptr = &func[idx..];
    len = demangle_protection(&mut buffer, ptr, &prot, func);
    if !len {
        out.push_str(func.to_str()?);
        return Ok(out);
    }
    ptr = &ptr[len..];

    // the next one always seems to be E or F
    if prot >= b'A' && prot <= b'V' && ((prot - b'A') & 2) != 0 {
        if ptr[0] != b'E' && ptr[0] != b'F' {
            log::warn!("Unknown modifier {} for function {}", ptr[0], func);
        }
        ptr = &ptr[1..];
    }

    // this should mark the calling convention. Always seems to be A, but this corroborates the
    // function body which uses CDECL.
    if ptr[0] == b'A' {
        buffer.push_str("__cdecl ");
    } else if ptr[0] == b'C' {
        buffer.push_str("__pascal ");
    } else {
        log::warn!("unknown calling convention {} for function {}", ptr[0], func);
    }

    // this marks the return value
    ptr = &ptr[1..];
    len = demangle_type(&mut known_names, &mut buffer, ptr);
    if !len {
        log::warn!("unknown return type {} for function {}", ptr[0], func);
    }
    ptr = &ptr[len..];

    // get the classname. This is in reverse order, so find the first @@ and work backwards.
    start = func.find("@@").unwrap();
    end = start;

    loop {
        start -= 1;
        while func[start] != b'?' && func[start] != b'@' {
            start -= 1;
        }
        buffer.push_str(func[start+1..start+1+end]);
        if func[start] == b'?' {
            break;
        }
        buffer.push_str("::");
        end = start;
    }

    // print the arguments
    if ptr[0] == b'X' {
        buffer.push_str("(void)");
    } else {
        buffer.push_str("(");
        while ptr[0] != b'@' {
            if ptr[0] >= b'0' && p[tr0] <= b'9' {
                buffer.push_str(known_types[ptr[0] - b'0']);
                ptr = &ptr[1..]
            } else {
                let arg_type: String = String::from(&buffer[buffer.len()..]);
                len = demangle_type(&mut known_names, &mut buffer, ptr);
                if buffer[buffer.len()-1] == b" " {
                    known_types.push_str(&arg_type.as_str());
                    known_type_idx += 1;
                } else if (!len) {
                    log::warn!("unknown argument type {} for function {}", ptr[0], func);
                    len = 1;
                }
                ptr = &ptr[len..];
            }
            buffer.push_str(", ");
        }
        buffer.push_str(")");
    }

    out.push_str(buffer.to_str()?);
    Ok(out)
}

pub fn read_res_name_table(app_ctx: &AppContext, start: usize, entry_table: &mut Vec<NeEntry>) -> Vec<u8> {
    let mut cursor = start;
    let mut length_byte: u8 = 0;
    let mut length: usize = 0;
    let mut first: Vec<u8> = Vec::new();

    length_byte = read_byte(&app_ctx, cursor);
    length = usize::from(length_byte);
    cursor += 1;

    first.clone_from_slice(&app_ctx.file_buf[cursor..cursor+length]);
    cursor += length + 2;

    while length = usize::from(read_byte(&app_ctx, cursor)) {
        cursor +=1;

        let mut name: Vec<u8> = Vec::new();
        name.clone_from_slice(&app_ctx.file_buf[cursor..cursor+length]);
        cursor += length;

        if name[0] == b'?' {
            name = demangle(&name.as_slice())?;
        }

        entry_table[read_word(&app_ctx, cursor, Endianness::LittleEndian) - 1].name = name;
        cursor += 2;
    }

    first
}

pub fn get_entry_table(app_ctx: &AppContext, start: usize, ne: &mut NeInfo) -> Result<(), MultiError> {
    let mut len_byte: u8 = 0;
    let mut index: u8 = 0;
    let mut count: usize = 0;
    let mut i = 0;
    let mut word: u16 = 0;
    let mut cursor: usize = 0;

    // get a count
    cursor = start;
    while len_byte = read_byte(&app_ctx, cursor) {
        cursor += 1;
        index = read_byte(&app_ctx, cursor);
        cursor += 1;
        count += len_byte;
        if index != 0 {
            if index == 0xff {
                cursor += 6 * length;
            } else {
                cursor += 3 * length;
            }

        }
    }

    cursor = start;
    while len_byte = read_byte(&app_ctx, cursor) {
        cursor += 1;
        index = read_byte(&app_ctx, cursor);
        cursor += 1;
        for i in 0..length {
            let mut entry: NeEntry = Default::default();
            if index == 0xff {
                entry.flags = read_byte(&app_ctx, cursor);
                word = read_word(&app_ctx, cursor + 1, Endianness::LittleEndian);
                if word != 0x3fcd {
                        log::warn!("entry {} has interrupt bytes {:02x}{:02x} (expected 3fcd)", count+1, word & 0xff, word >> 16);
                }
                entry.segment = read_byte(&app_ctx, cursor + 3);
                entry.offset = read_word(&app_ctx, cursor + 4, Endianness::LittlEndian);
                cursor += 6;
            } else if index == 0x00 {
                // no entries here
            } else {
                entry.flags = read_byte(&app_ctx, cursor);
                entry.segment = index;
                entry.offset = read_word(&app_ctx, cursor + 1, Endianness::LittleEndian);
                cursor += 3;
            }
            count += 1;
            ne.enttab.push(entry);
        }
    }

    ne.ent_count = count;

    Ok(())

}

pub fn get_import_module_table(app_ctx: &AppContext, start: usize, ne: &mut NeInfo) -> Result<(), MultiError> {
    let offset: u16;
    let length: u8;
    let i: u32 = 0;

    for i in 0..ne.header.ne_cmod {
        offset = read_word(app_ctx, start + i * 2, Endianness::LittleEndian);
        length = ne.nametab[offset];
        let import = NeImportModule{
            name: nametab[offset+1],
            exports: vec![]
        };
        ne.imptab[i].
    }

    Ok(())

}