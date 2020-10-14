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
    pub name: *mut libc::c_char,
    pub exports: *mut NeExport,
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

pub struct Ne {
    pub header: NeHeader,
    pub name: *mut libc::c_char,
    pub description: *mut libc::c_char,
    pub nametab: *mut u8,
    pub enttab: *mut NeEntry,
    pub ent_count: libc::c_uint,
    pub imptab: *mut NeImportModule,
    pub segments: *mut NeSegment,
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

pub fn demangle_protection(buffer: &mut String, start: &mut str, prot: &mut str, func: &str) -> usize {
    if start[0] >= 'A' && start[0] <= 'V' {
        if (start[0] - 'A') & 2 {
            buffer.push_str("static ");
        }
        if (start[0] - 'A') & 4 {
            buffer.push_str("virtual ");
        }
        if (start[0] - 'A') & 1 {
            buffer.push_str("near ");
        }
        if ((start[0] - 'A') & 24) == 0 {
            buffer.push_str("private ");
        }
        else if ((start[0] - 'A') & 24) == 8 {
            buffer.push_str("protected ");
        } else if ((start[0] - 'A') & 24) == 16 {
            buffer.push_str("public ");
        }
        prot[0] = start[0];
    } else if start[0] == 'Y' {
        buffer.push_str("near ");
    } else if start[0] == 'Z' {

    } else if start[0] == 'X' {
        prot[0] = 'V';
        return if start[1] >= '0' && start[1] <= '9' {
            buffer.push_str("(X0)");
            buffer[buffer.len() - 3] = start[1];
            2
        } else {
            start.find('@').unwrap() + 1
        }
    } else if start[0] == '_' && start[1] != '$' {
        demangle_protection(buffer, &mut start[1..], prot, func);
        if start[3] >= '0' && start[3] <= '9' {
            buffer.push_str("(_00) ");
            buffer[buffer.len() - 4] = start[2];
            buffer[buffer.len() - 3] = start[3];
            return 4;
        } else {
            return start.find('@').unwrap() + 1;
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

pub fn demangle_type(known_names: Vec<String>, buffer: &mut String, type: &mut str) -> usize {
    
}

