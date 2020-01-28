use ::libc;
extern "C" {
    #[no_mangle]
    static mut stdout: *mut _IO_FILE;
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn _IO_getc(__fp: *mut _IO_FILE) -> libc::c_int;
    #[no_mangle]
    fn _IO_putc(__c: libc::c_int, __fp: *mut _IO_FILE) -> libc::c_int;
    #[no_mangle]
    fn fopen(__filename: *const libc::c_char, __modes: *const libc::c_char)
     -> *mut FILE;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn fgets(__s: *mut libc::c_char, __n: libc::c_int, __stream: *mut FILE)
     -> *mut libc::c_char;
    #[no_mangle]
    fn fread(__ptr: *mut libc::c_void, __size: size_t, __n: size_t,
             __stream: *mut FILE) -> size_t;
    #[no_mangle]
    fn fseek(__stream: *mut FILE, __off: libc::c_long, __whence: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn perror(__s: *const libc::c_char);
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strncat(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strndup(_: *const libc::c_char, _: libc::c_ulong) -> *mut libc::c_char;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strstr(_: *const libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    /* in ne_resource.c */
    #[no_mangle]
    fn print_rsrc(start: libc::c_long);
    /* in ne_segment.c */
    #[no_mangle]
    fn read_segments(start: libc::c_long, ne: *mut ne);
    #[no_mangle]
    fn free_segments(ne: *mut ne);
    #[no_mangle]
    fn print_segments(ne: *mut ne);
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type FILE = _IO_FILE;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
/* Common functions */
pub type byte = uint8_t;
pub type word = uint16_t;
pub type dword = uint32_t;
/* additional options */
pub type C2RustUnnamed = libc::c_uint;
pub const MASM: C2RustUnnamed = 2;
pub const NASM: C2RustUnnamed = 1;
pub const GAS: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ne {
    pub header: header_ne,
    pub name: *mut libc::c_char,
    pub description: *mut libc::c_char,
    pub nametab: *mut byte,
    pub enttab: *mut entry,
    pub entcount: libc::c_uint,
    pub imptab: *mut import_module,
    pub segments: *mut segment,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct segment {
    pub cs: word,
    pub start: libc::c_long,
    pub length: word,
    pub flags: word,
    pub min_alloc: word,
    pub instr_flags: *mut byte,
    pub reloc_table: *mut reloc,
    pub reloc_count: word,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct reloc {
    pub size: byte,
    pub type_0: byte,
    pub offset_count: word,
    pub offsets: *mut word,
    pub tseg: word,
    pub toffset: word,
    pub text: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct import_module {
    pub name: *mut libc::c_char,
    pub exports: *mut export,
    pub export_count: libc::c_uint,
}
/* may be NULL */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct export {
    pub ordinal: word,
    pub name: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct entry {
    pub flags: byte,
    pub segment: byte,
    pub offset: word,
    pub name: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct header_ne {
    pub ne_magic: word,
    pub ne_ver: byte,
    pub ne_rev: byte,
    pub ne_enttab: word,
    pub ne_cbenttab: word,
    pub ne_crc: dword,
    pub ne_flags: word,
    pub ne_autodata: byte,
    pub ne_unused: byte,
    pub ne_heap: word,
    pub ne_stack: word,
    pub ne_ip: word,
    pub ne_cs: word,
    pub ne_sp: word,
    pub ne_ss: word,
    pub ne_cseg: word,
    pub ne_cmod: word,
    pub ne_cbnrestab: word,
    pub ne_segtab: word,
    pub ne_rsrctab: word,
    pub ne_restab: word,
    pub ne_modtab: word,
    pub ne_imptab: word,
    pub ne_nrestab: dword,
    pub ne_cmovent: word,
    pub ne_align: word,
    pub ne_cres: word,
    pub ne_exetyp: byte,
    pub ne_flagsothers: byte,
    pub ne_pretthunks: word,
    pub ne_psegrefbytes: word,
    pub ne_swaparea: word,
    pub ne_expver_min: byte,
    pub ne_expver_maj: byte,
}
#[inline]
unsafe extern "C" fn putchar(mut __c: libc::c_int) -> libc::c_int {
    return _IO_putc(__c, stdout);
}
#[no_mangle]
pub static mut f: *mut FILE = 0 as *const FILE as *mut FILE;
#[inline]
unsafe extern "C" fn read_byte() -> byte { return _IO_getc(f) as byte; }
#[inline]
unsafe extern "C" fn read_word() -> word {
    let mut w: word = 0;
    fread(&mut w as *mut word as *mut libc::c_void,
          2 as libc::c_int as size_t, 1 as libc::c_int as size_t, f);
    return w;
}
#[no_mangle]
pub static mut mode: word = 0;
#[no_mangle]
pub static mut opts: word = 0;
#[no_mangle]
pub static mut asm_syntax: C2RustUnnamed = GAS;
#[no_mangle]
pub static mut resource_filters_count: libc::c_uint = 0;
#[no_mangle]
pub static mut resource_filters: *mut *mut libc::c_char =
    0 as *const *mut libc::c_char as *mut *mut libc::c_char;
/*
 * Functions for parsing the NE header
 *
 * Copyright 2017-2018,2020 Zebediah Figura
 *
 * This file is part of Semblance.
 *
 * Semblance is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Semblance is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Semblance; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA 02110-1301, USA
 */
//#include <strings.h>
unsafe extern "C" fn print_flags(mut flags: word) {
    let mut buffer: [libc::c_char; 1024] = [0; 1024]; /* FRAMEBUF */
    if flags as libc::c_int & 0x3 as libc::c_int == 0 as libc::c_int {
        strcpy(buffer.as_mut_ptr(),
               b"no DGROUP\x00" as *const u8 as
                   *const libc::c_char); /* API compatible */
    } else if flags as libc::c_int & 0x3 as libc::c_int == 1 as libc::c_int {
        strcpy(buffer.as_mut_ptr(),
               b"single DGROUP\x00" as *const u8 as
                   *const libc::c_char); /* uses API */
    } else if flags as libc::c_int & 0x3 as libc::c_int == 2 as libc::c_int {
        strcpy(buffer.as_mut_ptr(),
               b"multiple DGROUPs\x00" as *const u8 as
                   *const libc::c_char); /* none? */
    } else if flags as libc::c_int & 0x3 as libc::c_int == 3 as libc::c_int {
        strcpy(buffer.as_mut_ptr(),
               b"(unknown DGROUP type 3)\x00" as *const u8 as
                   *const libc::c_char); /* OS/2 family */
    } /* gangload */
    if flags as libc::c_int & 0x4 as libc::c_int != 0 {
        strcat(buffer.as_mut_ptr(),
               b", global initialization\x00" as *const u8 as
                   *const libc::c_char);
    }
    if flags as libc::c_int & 0x8 as libc::c_int != 0 {
        strcat(buffer.as_mut_ptr(),
               b", protected mode only\x00" as *const u8 as
                   *const libc::c_char);
    }
    if flags as libc::c_int & 0x10 as libc::c_int != 0 {
        strcat(buffer.as_mut_ptr(),
               b", 8086\x00" as *const u8 as *const libc::c_char);
    }
    if flags as libc::c_int & 0x20 as libc::c_int != 0 {
        strcat(buffer.as_mut_ptr(),
               b", 80286\x00" as *const u8 as *const libc::c_char);
    }
    if flags as libc::c_int & 0x40 as libc::c_int != 0 {
        strcat(buffer.as_mut_ptr(),
               b", 80386\x00" as *const u8 as *const libc::c_char);
    }
    if flags as libc::c_int & 0x80 as libc::c_int != 0 {
        strcat(buffer.as_mut_ptr(),
               b", 80x87\x00" as *const u8 as *const libc::c_char);
    }
    if flags as libc::c_int & 0x700 as libc::c_int == 0x100 as libc::c_int {
        strcat(buffer.as_mut_ptr(),
               b", fullscreen\x00" as *const u8 as *const libc::c_char);
    } else if flags as libc::c_int & 0x700 as libc::c_int ==
                  0x200 as libc::c_int {
        strcat(buffer.as_mut_ptr(),
               b", console\x00" as *const u8 as *const libc::c_char);
    } else if flags as libc::c_int & 0x700 as libc::c_int ==
                  0x300 as libc::c_int {
        strcat(buffer.as_mut_ptr(),
               b", GUI\x00" as *const u8 as *const libc::c_char);
    } else if flags as libc::c_int & 0x700 as libc::c_int == 0 as libc::c_int
     {
        strcat(buffer.as_mut_ptr(),
               b", (no subsystem)\x00" as *const u8 as *const libc::c_char);
    } else {
        sprintf(buffer.as_mut_ptr().offset(strlen(buffer.as_mut_ptr()) as
                                               isize),
                b", (unknown application type %d)\x00" as *const u8 as
                    *const libc::c_char,
                (flags as libc::c_int & 0x700 as libc::c_int) >>
                    8 as libc::c_int);
    }
    if flags as libc::c_int & 0x800 as libc::c_int != 0 {
        strcat(buffer.as_mut_ptr(),
               b", self-loading\x00" as *const u8 as *const libc::c_char);
    }
    if flags as libc::c_int & 0x1000 as libc::c_int != 0 {
        strcat(buffer.as_mut_ptr(),
               b", (unknown flag 0x1000)\x00" as *const u8 as
                   *const libc::c_char);
    }
    if flags as libc::c_int & 0x2000 as libc::c_int != 0 {
        strcat(buffer.as_mut_ptr(),
               b", contains linker errors\x00" as *const u8 as
                   *const libc::c_char);
    }
    if flags as libc::c_int & 0x4000 as libc::c_int != 0 {
        strcat(buffer.as_mut_ptr(),
               b", non-conforming program\x00" as *const u8 as
                   *const libc::c_char);
    }
    if flags as libc::c_int & 0x8000 as libc::c_int != 0 {
        strcat(buffer.as_mut_ptr(),
               b", library\x00" as *const u8 as *const libc::c_char);
    }
    printf(b"Flags: 0x%04x (%s)\n\x00" as *const u8 as *const libc::c_char,
           flags as libc::c_int, buffer.as_mut_ptr());
}
unsafe extern "C" fn print_os2flags(mut flags: word) {
    let mut buffer: [libc::c_char; 1024] = [0; 1024];
    buffer[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    if flags as libc::c_int & 0x1 as libc::c_int != 0 {
        strcat(buffer.as_mut_ptr(),
               b", long filename support\x00" as *const u8 as
                   *const libc::c_char);
    }
    if flags as libc::c_int & 0x2 as libc::c_int != 0 {
        strcat(buffer.as_mut_ptr(),
               b", 2.x protected mode\x00" as *const u8 as
                   *const libc::c_char);
    }
    if flags as libc::c_int & 0x4 as libc::c_int != 0 {
        strcat(buffer.as_mut_ptr(),
               b", 2.x proportional fonts\x00" as *const u8 as
                   *const libc::c_char);
    }
    if flags as libc::c_int & 0x8 as libc::c_int != 0 {
        strcat(buffer.as_mut_ptr(),
               b", fast-load area\x00" as *const u8 as *const libc::c_char);
    }
    if flags as libc::c_int & 0xfff0 as libc::c_int != 0 {
        sprintf(buffer.as_mut_ptr().offset(strlen(buffer.as_mut_ptr()) as
                                               isize),
                b", (unknown flags 0x%04x)\x00" as *const u8 as
                    *const libc::c_char,
                flags as libc::c_int & 0xfff0 as libc::c_int);
    }
    if buffer[0 as libc::c_int as usize] != 0 {
        printf(b"OS/2 flags: 0x%04x (%s)\n\x00" as *const u8 as
                   *const libc::c_char, flags as libc::c_int,
               buffer.as_mut_ptr().offset(2 as libc::c_int as isize));
    } else {
        printf(b"OS/2 flags: 0x0000\n\x00" as *const u8 as
                   *const libc::c_char);
    };
}
static mut exetypes: [*const libc::c_char; 7] =
    [b"unknown\x00" as *const u8 as *const libc::c_char,
     b"OS/2\x00" as *const u8 as *const libc::c_char,
     b"Windows (16-bit)\x00" as *const u8 as *const libc::c_char,
     b"European Dos 4.x\x00" as *const u8 as *const libc::c_char,
     b"Windows 386 (32-bit)\x00" as *const u8 as *const libc::c_char,
     b"BOSS\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char];
unsafe extern "C" fn print_header(mut header: *mut header_ne) {
    /* Still need to deal with:
     *
     * 34 - number of resource segments (all of my testcases return 0)
     * 38 - offset to return thunks (have testcases)
     * 3a - offset to segment ref. bytes (same)
     */
    putchar('\n' as i32); /* 02 */
    printf(b"Linker version: %d.%d\n\x00" as *const u8 as *const libc::c_char,
           (*header).ne_ver as libc::c_int,
           (*header).ne_rev as libc::c_int); /* 08 */
    printf(b"Checksum: %08x\n\x00" as *const u8 as *const libc::c_char,
           (*header).ne_crc); /* 0c */
    print_flags((*header).ne_flags); /* 10 */
    printf(b"Automatic data segment: %d\n\x00" as *const u8 as
               *const libc::c_char,
           (*header).ne_autodata as libc::c_int); /* 12 */
    if (*header).ne_unused as libc::c_int != 0 as libc::c_int {
        fprintf(stderr,
                b"Warning: Header byte at position 0f has value 0x%02x.\n\x00"
                    as *const u8 as *const libc::c_char,
                (*header).ne_unused as libc::c_int); /* 14 */
    } /* 18 */
    printf(b"Heap size: %d bytes\n\x00" as *const u8 as *const libc::c_char,
           (*header).ne_heap as libc::c_int);
    printf(b"Stack size: %d bytes\n\x00" as *const u8 as *const libc::c_char,
           (*header).ne_stack as libc::c_int);
    printf(b"Program entry point: %d:%04x\n\x00" as *const u8 as
               *const libc::c_char, (*header).ne_cs as libc::c_int,
           (*header).ne_ip as libc::c_int);
    printf(b"Initial stack location: %d:%04x\n\x00" as *const u8 as
               *const libc::c_char, (*header).ne_ss as libc::c_int,
           (*header).ne_sp as libc::c_int);
    if (*header).ne_exetyp as libc::c_int <= 5 as libc::c_int {
        /* 36 */
        printf(b"Target OS: %s\n\x00" as *const u8 as *const libc::c_char,
               exetypes[(*header).ne_exetyp as usize]); /* 37 */
    } else {
        printf(b"Target OS: (unknown value %d)\n\x00" as *const u8 as
                   *const libc::c_char,
               (*header).ne_exetyp as libc::c_int); /* 3c */
    }
    print_os2flags((*header).ne_flagsothers as word);
    printf(b"Swap area: %d\n\x00" as *const u8 as *const libc::c_char,
           (*header).ne_swaparea as libc::c_int);
    printf(b"Expected Windows version: %d.%d\n\x00" as *const u8 as
               *const libc::c_char, (*header).ne_expver_maj as libc::c_int,
           (*header).ne_expver_min as libc::c_int);
}
unsafe extern "C" fn print_export(mut ne: *mut ne) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while (i as libc::c_uint) < (*ne).entcount {
        if (*(*ne).enttab.offset(i as isize)).segment as libc::c_int ==
               0xfe as libc::c_int {
            /* absolute value */
            printf(b"\t%5d\t   %04x\t%s\n\x00" as *const u8 as
                       *const libc::c_char, i + 1 as libc::c_int,
                   (*(*ne).enttab.offset(i as isize)).offset as libc::c_int,
                   if !(*(*ne).enttab.offset(i as isize)).name.is_null() {
                       (*(*ne).enttab.offset(i as isize)).name
                   } else {
                       b"<no name>\x00" as *const u8 as *const libc::c_char
                   });
        } else if (*(*ne).enttab.offset(i as isize)).segment != 0 {
            printf(b"\t%5d\t%2d:%04x\t%s\n\x00" as *const u8 as
                       *const libc::c_char, i + 1 as libc::c_int,
                   (*(*ne).enttab.offset(i as isize)).segment as libc::c_int,
                   (*(*ne).enttab.offset(i as isize)).offset as libc::c_int,
                   if !(*(*ne).enttab.offset(i as isize)).name.is_null() {
                       (*(*ne).enttab.offset(i as isize)).name
                   } else {
                       b"<no name>\x00" as *const u8 as *const libc::c_char
                   });
        }
        i += 1
    }
    putchar('\n' as i32);
}
unsafe extern "C" fn print_specfile(mut ne: *mut ne) {
    let mut i: libc::c_int = 0;
    let mut specfile: *mut FILE = 0 as *mut FILE;
    let mut spec_name: [libc::c_char; 13] = [0; 13];
    sprintf(spec_name.as_mut_ptr(),
            b"%.8s.ORD\x00" as *const u8 as *const libc::c_char, (*ne).name);
    specfile =
        fopen(spec_name.as_mut_ptr(),
              b"w\x00" as *const u8 as *const libc::c_char);
    if specfile.is_null() {
        perror(b"Couldn\'t open %s\x00" as *const u8 as *const libc::c_char);
        return
    }
    fprintf(specfile,
            b"# Generated by dump -o\n\x00" as *const u8 as
                *const libc::c_char);
    i = 0 as libc::c_int;
    while (i as libc::c_uint) < (*ne).entcount {
        if !(*(*ne).enttab.offset(i as isize)).name.is_null() {
            fprintf(specfile,
                    b"%d\t%s\n\x00" as *const u8 as *const libc::c_char,
                    i + 1 as libc::c_int,
                    (*(*ne).enttab.offset(i as isize)).name);
        } else if (*(*ne).enttab.offset(i as isize)).segment != 0 {
            fprintf(specfile, b"%d\n\x00" as *const u8 as *const libc::c_char,
                    i + 1 as libc::c_int);
        }
        i += 1
    }
    fclose(specfile);
}
unsafe extern "C" fn demangle_protection(mut buffer: *mut libc::c_char,
                                         mut start: *mut libc::c_char,
                                         mut prot: *mut libc::c_char,
                                         mut func: *mut libc::c_char)
 -> libc::c_int {
    if *start as libc::c_int >= 'A' as i32 &&
           *start as libc::c_int <= 'V' as i32 {
        if *start as libc::c_int - 'A' as i32 & 2 as libc::c_int != 0 {
            strcat(buffer,
                   b"static \x00" as *const u8 as *const libc::c_char);
        }
        if *start as libc::c_int - 'A' as i32 & 4 as libc::c_int != 0 {
            strcat(buffer,
                   b"virtual \x00" as *const u8 as *const libc::c_char);
        }
        if *start as libc::c_int - 'A' as i32 & 1 as libc::c_int == 0 {
            strcat(buffer, b"near \x00" as *const u8 as *const libc::c_char);
        }
        if *start as libc::c_int - 'A' as i32 & 24 as libc::c_int ==
               0 as libc::c_int {
            strcat(buffer,
                   b"private \x00" as *const u8 as *const libc::c_char);
        } else if *start as libc::c_int - 'A' as i32 & 24 as libc::c_int ==
                      8 as libc::c_int {
            strcat(buffer,
                   b"protected \x00" as *const u8 as *const libc::c_char);
        } else if *start as libc::c_int - 'A' as i32 & 24 as libc::c_int ==
                      16 as libc::c_int {
            strcat(buffer,
                   b"public \x00" as *const u8 as *const libc::c_char);
        }
        *prot = *start
    } else if *start as libc::c_int == 'Y' as i32 {
        strcat(buffer, b"near \x00" as *const u8 as *const libc::c_char);
    } else if *start as libc::c_int == 'Z' as i32 {
    } else if *start as libc::c_int == 'X' as i32 {
        /* It's not clear what this means, but it always seems to be
         * followed by either a number, or a string of text and then @. */
        *prot = 'V' as i32 as libc::c_char; /* just pretend that for now */
        if *start.offset(1 as libc::c_int as isize) as libc::c_int >=
               '0' as i32 &&
               *start.offset(1 as libc::c_int as isize) as libc::c_int <=
                   '9' as i32 {
            strcat(buffer, b"(X0) \x00" as *const u8 as *const libc::c_char);
            *buffer.offset(strlen(buffer).wrapping_sub(3 as libc::c_int as
                                                           libc::c_ulong) as
                               isize) =
                *start.offset(1 as libc::c_int as isize);
            return 2 as libc::c_int
        } else {
            return strchr(start,
                          '@' as
                              i32).offset(1 as libc::c_int as
                                              isize).wrapping_offset_from(start)
                       as libc::c_long as libc::c_int
        }
    } else if *start as libc::c_int == '_' as i32 &&
                  *start.offset(1 as libc::c_int as isize) as libc::c_int !=
                      '$' as i32 {
        /* Same as above, but there is an extra character first (which
         * is often V, so is likely to be the protection/etc), and then
         * a number (often 7 or 3). */
        demangle_protection(buffer, start.offset(1 as libc::c_int as isize),
                            prot, func);
        if *start.offset(3 as libc::c_int as isize) as libc::c_int >=
               '0' as i32 &&
               *start.offset(3 as libc::c_int as isize) as libc::c_int <=
                   '9' as i32 {
            strcat(buffer, b"(_00) \x00" as *const u8 as *const libc::c_char);
            *buffer.offset(strlen(buffer).wrapping_sub(4 as libc::c_int as
                                                           libc::c_ulong) as
                               isize) =
                *start.offset(2 as libc::c_int as isize);
            *buffer.offset(strlen(buffer).wrapping_sub(3 as libc::c_int as
                                                           libc::c_ulong) as
                               isize) =
                *start.offset(3 as libc::c_int as isize);
            return 4 as libc::c_int
        } else {
            return strchr(start,
                          '@' as
                              i32).offset(1 as libc::c_int as
                                              isize).wrapping_offset_from(start)
                       as libc::c_long as libc::c_int
        }
    } else {
        fprintf(stderr,
                b"Warning: Unknown modifier %c for function %s\n\x00" as
                    *const u8 as *const libc::c_char, *start as libc::c_int,
                func);
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
static mut int_types: [*const libc::c_char; 9] =
    [b"signed char\x00" as *const u8 as *const libc::c_char,
     b"char\x00" as *const u8 as *const libc::c_char,
     b"unsigned char\x00" as *const u8 as *const libc::c_char,
     b"short\x00" as *const u8 as *const libc::c_char,
     b"unsigned short\x00" as *const u8 as *const libc::c_char,
     b"int\x00" as *const u8 as *const libc::c_char,
     b"unsigned int\x00" as *const u8 as *const libc::c_char,
     b"long\x00" as *const u8 as *const libc::c_char,
     b"unsigned long\x00" as *const u8 as *const libc::c_char];
/* Returns the number of characters processed. */
unsafe extern "C" fn demangle_type(mut known_names: *mut *mut libc::c_char,
                                   mut buffer: *mut libc::c_char,
                                   mut type_0: *mut libc::c_char)
 -> libc::c_int {
    if *type_0 as libc::c_int >= 'C' as i32 &&
           *type_0 as libc::c_int <= 'K' as i32 {
        strcat(buffer,
               int_types[(*type_0 as libc::c_int - 'C' as i32) as usize]);
        strcat(buffer, b" \x00" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int
    }
    match *type_0 as libc::c_int {
        65 | 80 => {
            let mut ret: libc::c_int = 0;
            if *type_0.offset(1 as libc::c_int as isize) as libc::c_int -
                   'A' as i32 & 1 as libc::c_int != 0 {
                strcat(buffer,
                       b"const \x00" as *const u8 as *const libc::c_char);
            }
            if *type_0.offset(1 as libc::c_int as isize) as libc::c_int -
                   'A' as i32 & 2 as libc::c_int != 0 {
                strcat(buffer,
                       b"volatile \x00" as *const u8 as *const libc::c_char);
            }
            ret =
                demangle_type(known_names, buffer,
                              type_0.offset(2 as libc::c_int as isize));
            if *type_0.offset(1 as libc::c_int as isize) as libc::c_int -
                   'A' as i32 & 4 as libc::c_int == 0 {
                strcat(buffer,
                       b"near \x00" as *const u8 as *const libc::c_char);
            }
            strcat(buffer,
                   if *type_0 as libc::c_int == 'A' as i32 {
                       b"&\x00" as *const u8 as *const libc::c_char
                   } else { b"*\x00" as *const u8 as *const libc::c_char });
            return ret + 2 as libc::c_int
        }
        77 => {
            strcat(buffer, b"float \x00" as *const u8 as *const libc::c_char);
            return 1 as libc::c_int
        }
        78 => {
            strcat(buffer,
                   b"double \x00" as *const u8 as *const libc::c_char);
            return 1 as libc::c_int
        }
        85 | 86 => {
            let mut p: *const libc::c_char = buffer;
            let mut end: *const libc::c_char = 0 as *const libc::c_char;
            let mut i: libc::c_uint = 0;
            if *type_0.offset(1 as libc::c_int as isize) as libc::c_int >=
                   '0' as i32 &&
                   *type_0.offset(1 as libc::c_int as isize) as libc::c_int <=
                       '9' as i32 {
                strcat(buffer,
                       *known_names.offset((*type_0.offset(1 as libc::c_int as
                                                               isize) as
                                                libc::c_int - '0' as i32) as
                                               isize));
                strcat(buffer, b" \x00" as *const u8 as *const libc::c_char);
                return 3 as libc::c_int
            }
            /* These represent structs (U) or types (V), but the name given
         * doesn't seem to need a qualifier. */
        /* something can go between the at signs, but what does it mean? */
            end =
                strchr(strchr(type_0,
                              '@' as i32).offset(1 as libc::c_int as isize),
                       '@' as i32);
            if *end.offset(-(1 as libc::c_int) as isize) as libc::c_int ==
                   '@' as i32 {
                strncat(buffer, type_0.offset(1 as libc::c_int as isize),
                        end.offset(-(1 as libc::c_int as
                                         isize)).wrapping_offset_from(type_0.offset(1
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize))
                            as libc::c_long as libc::c_ulong);
            } else {
                strncat(buffer, type_0.offset(1 as libc::c_int as isize),
                        end.wrapping_offset_from(type_0.offset(1 as
                                                                   libc::c_int
                                                                   as isize))
                            as libc::c_long as libc::c_ulong);
            }
            i = 0 as libc::c_int as libc::c_uint;
            while i < 10 as libc::c_int as libc::c_uint {
                if (*known_names.offset(i as isize)).is_null() {
                    let ref mut fresh0 = *known_names.offset(i as isize);
                    *fresh0 = strdup(p);
                    break ;
                } else { i = i.wrapping_add(1) }
            }
            strcat(buffer, b" \x00" as *const u8 as *const libc::c_char);
            return end.offset(1 as libc::c_int as
                                  isize).wrapping_offset_from(type_0) as
                       libc::c_long as libc::c_int
        }
        88 => {
            strcat(buffer, b"void \x00" as *const u8 as *const libc::c_char);
            return 1 as libc::c_int
        }
        _ => { return 0 as libc::c_int }
    };
}
/* Demangle a C++ function name. The scheme used seems to be mostly older
 * than any documented, but I was able to find documentation that is at
 * least close in Agner Fog's manual. */
unsafe extern "C" fn demangle(mut func: *mut libc::c_char)
 -> *mut libc::c_char {
    let mut known_types: [*mut libc::c_char; 10] =
        [0 as *mut libc::c_char, 0 as *mut libc::c_char,
         0 as *mut libc::c_char, 0 as *mut libc::c_char,
         0 as *mut libc::c_char, 0 as *mut libc::c_char,
         0 as *mut libc::c_char, 0 as *mut libc::c_char,
         0 as *mut libc::c_char, 0 as *mut libc::c_char];
    let mut known_names: [*mut libc::c_char; 10] =
        [0 as *mut libc::c_char, 0 as *mut libc::c_char,
         0 as *mut libc::c_char, 0 as *mut libc::c_char,
         0 as *mut libc::c_char, 0 as *mut libc::c_char,
         0 as *mut libc::c_char, 0 as *mut libc::c_char,
         0 as *mut libc::c_char, 0 as *mut libc::c_char];
    let mut known_type_idx: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut known_name_idx: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut buffer: [libc::c_char; 1024] = [0; 1024];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut prot: libc::c_char = 0 as libc::c_int as libc::c_char;
    let mut len: libc::c_int = 0;
    if *func.offset(1 as libc::c_int as isize) as libc::c_int == '?' as i32 {
        /* TODO: constructor/destructor */
        return func
    }
    /* First populate the known names up to the function name. */
    p = func;
    while *p as libc::c_int != '@' as i32 &&
              known_name_idx < 10 as libc::c_int as libc::c_uint {
        let fresh1 = known_name_idx;
        known_name_idx = known_name_idx.wrapping_add(1);
        known_names[fresh1 as usize] =
            strndup(p,
                    strchr(p, '@' as i32).wrapping_offset_from(p) as
                        libc::c_long as libc::c_ulong);
        p = strchr(p, '@' as i32).offset(1 as libc::c_int as isize)
    }
    /* Figure out the modifiers and calling convention. */
    buffer[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    p =
        strstr(func,
               b"@@\x00" as *const u8 as
                   *const libc::c_char).offset(2 as libc::c_int as isize);
    len = demangle_protection(buffer.as_mut_ptr(), p, &mut prot, func);
    if len == 0 { return func }
    p = p.offset(len as isize);
    /* The next one seems to always be E or F. No idea why. */
    if prot as libc::c_int >= 'A' as i32 && prot as libc::c_int <= 'V' as i32
           && prot as libc::c_int - 'A' as i32 & 2 as libc::c_int == 0 {
        if *p as libc::c_int != 'E' as i32 && *p as libc::c_int != 'F' as i32
           {
            fprintf(stderr,
                    b"Warning: Unknown modifier %c for function %s\n\x00" as
                        *const u8 as *const libc::c_char, *p as libc::c_int,
                    func);
        }
        p = p.offset(1)
    }
    /* This should mark the calling convention. Always seems to be A,
     * but this corroborates the function body which uses CDECL. */
    if !(*p as libc::c_int == 'A' as i32) {
        if *p as libc::c_int == 'C' as i32 {
            strcat(buffer.as_mut_ptr(),
                   b"__pascal \x00" as *const u8 as *const libc::c_char);
        } else {
            fprintf(stderr,
                    b"Warning: Unknown calling convention %c for function %s\n\x00"
                        as *const u8 as *const libc::c_char,
                    *p as libc::c_int, func);
        }
    }
    /* This marks the return value. */
    p = p.offset(1);
    len = demangle_type(known_names.as_mut_ptr(), buffer.as_mut_ptr(), p);
    if len == 0 {
        fprintf(stderr,
                b"Warning: Unknown return type %c for function %s\n\x00" as
                    *const u8 as *const libc::c_char, *p as libc::c_int,
                func);
        len = 1 as libc::c_int
    }
    p = p.offset(len as isize);
    /* Get the classname. This is in reverse order, so
     * find the first @@ and work backwards from there. */
    end = strstr(func, b"@@\x00" as *const u8 as *const libc::c_char);
    start = end;
    loop  {
        start = start.offset(-1);
        while *start as libc::c_int != '?' as i32 &&
                  *start as libc::c_int != '@' as i32 {
            start = start.offset(-1)
        }
        strncat(buffer.as_mut_ptr(), start.offset(1 as libc::c_int as isize),
                end.wrapping_offset_from(start.offset(1 as libc::c_int as
                                                          isize)) as
                    libc::c_long as libc::c_ulong);
        if *start as libc::c_int == '?' as i32 { break ; }
        strcat(buffer.as_mut_ptr(),
               b"::\x00" as *const u8 as *const libc::c_char);
        end = start
    }
    /* Print the arguments. */
    if *p as libc::c_int == 'X' as i32 {
        strcat(buffer.as_mut_ptr(),
               b"(void)\x00" as *const u8 as *const libc::c_char);
    } else {
        strcat(buffer.as_mut_ptr(),
               b"(\x00" as *const u8 as *const libc::c_char);
        while *p as libc::c_int != '@' as i32 {
            if *p as libc::c_int >= '0' as i32 &&
                   *p as libc::c_int <= '9' as i32 {
                strcat(buffer.as_mut_ptr(),
                       known_types[(*p as libc::c_int - '0' as i32) as
                                       usize]);
                p = p.offset(1)
            } else {
                let mut type_0: *const libc::c_char =
                    buffer.as_mut_ptr().offset(strlen(buffer.as_mut_ptr()) as
                                                   isize);
                len =
                    demangle_type(known_names.as_mut_ptr(),
                                  buffer.as_mut_ptr(), p);
                if buffer[strlen(buffer.as_mut_ptr()).wrapping_sub(1 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_ulong)
                              as usize] as libc::c_int == ' ' as i32 {
                    buffer[strlen(buffer.as_mut_ptr()).wrapping_sub(1 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_ulong)
                               as usize] = 0 as libc::c_int as libc::c_char
                }
                if len > 1 as libc::c_int &&
                       known_type_idx < 10 as libc::c_int as libc::c_uint {
                    let fresh2 = known_type_idx;
                    known_type_idx = known_type_idx.wrapping_add(1);
                    known_types[fresh2 as usize] = strdup(type_0)
                } else if len == 0 {
                    fprintf(stderr,
                            b"Warning: Unknown argument type %c for function %s\n\x00"
                                as *const u8 as *const libc::c_char,
                            *p as libc::c_int, func);
                    len = 1 as libc::c_int
                }
                p = p.offset(len as isize)
            }
            strcat(buffer.as_mut_ptr(),
                   b", \x00" as *const u8 as *const libc::c_char);
        }
        buffer[strlen(buffer.as_mut_ptr()).wrapping_sub(2 as libc::c_int as
                                                            libc::c_ulong) as
                   usize] = ')' as i32 as libc::c_char;
        buffer[strlen(buffer.as_mut_ptr()).wrapping_sub(1 as libc::c_int as
                                                            libc::c_ulong) as
                   usize] = 0 as libc::c_int as libc::c_char
    }
    func =
        realloc(func as *mut libc::c_void,
                strlen(buffer.as_mut_ptr()).wrapping_add(1 as libc::c_int as
                                                             libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                                                             as
                                                                                             libc::c_ulong))
            as *mut libc::c_char;
    strcpy(func, buffer.as_mut_ptr());
    while known_type_idx != 0 {
        known_type_idx = known_type_idx.wrapping_sub(1);
        free(known_types[known_type_idx as usize] as *mut libc::c_void);
    }
    while known_name_idx != 0 {
        known_name_idx = known_name_idx.wrapping_sub(1);
        free(known_names[known_name_idx as usize] as *mut libc::c_void);
    }
    return func;
}
/* return the first entry (module name/desc) */
unsafe extern "C" fn read_res_name_table(mut start: libc::c_long,
                                         mut entry_table: *mut entry)
 -> *mut libc::c_char {
    /* reads (non)resident names into our entry table */
    let mut length: byte = 0; /* null term */
    let mut first: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ordinal: word = 0;
    fseek(f, start, 0 as libc::c_int);
    length = read_byte();
    first =
        malloc(((length as libc::c_int + 1 as libc::c_int) as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut libc::c_char;
    fread(first as *mut libc::c_void,
          ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
          length as size_t, f);
    *first.offset(length as isize) = 0 as libc::c_int as libc::c_char;
    fseek(f, ::std::mem::size_of::<word>() as libc::c_ulong as libc::c_long,
          1 as libc::c_int);
    loop  {
        length = read_byte();
        if !(length != 0) { break ; }
        name =
            malloc(((length as libc::c_int + 1 as libc::c_int) as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                        as libc::c_ulong)) as
                *mut libc::c_char;
        fread(name as *mut libc::c_void,
              ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
              length as size_t, f);
        *name.offset(length as isize) = 0 as libc::c_int as libc::c_char;
        ordinal = read_word();
        if opts as libc::c_int & 0x2 as libc::c_int != 0 &&
               *name.offset(0 as libc::c_int as isize) as libc::c_int ==
                   '?' as i32 {
            name = demangle(name)
        }
        let ref mut fresh3 =
            (*entry_table.offset((ordinal as libc::c_int - 1 as libc::c_int)
                                     as isize)).name;
        *fresh3 = name
    }
    return first;
}
unsafe extern "C" fn get_entry_table(mut start: libc::c_long,
                                     mut ne: *mut ne) {
    let mut length: byte = 0;
    let mut index: byte = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_uint = 0;
    let mut w: word = 0;
    /* get a count */
    fseek(f, start, 0 as libc::c_int);
    loop  {
        length = read_byte();
        if !(length != 0) { break ; }
        index = read_byte();
        count += length as libc::c_int;
        if index as libc::c_int != 0 as libc::c_int {
            fseek(f,
                  ((if index as libc::c_int == 0xff as libc::c_int {
                        6 as libc::c_int
                    } else { 3 as libc::c_int }) * length as libc::c_int) as
                      libc::c_long, 1 as libc::c_int);
        }
    }
    (*ne).enttab =
        calloc(::std::mem::size_of::<entry>() as libc::c_ulong,
               count as libc::c_ulong) as *mut entry;
    fseek(f, start, 0 as libc::c_int);
    count = 0 as libc::c_int;
    loop  {
        length = read_byte();
        if !(length != 0) { break ; }
        index = read_byte();
        i = 0 as libc::c_int as libc::c_uint;
        while i < length as libc::c_uint {
            if index as libc::c_int == 0xff as libc::c_int {
                (*(*ne).enttab.offset(count as isize)).flags = read_byte();
                w = read_word();
                if w as libc::c_int != 0x3fcd as libc::c_int {
                    fprintf(stderr,
                            b"Warning: Entry %d has interrupt bytes %02x %02x (expected 3f cd).\n\x00"
                                as *const u8 as *const libc::c_char,
                            count + 1 as libc::c_int,
                            w as libc::c_int & 0xff as libc::c_int,
                            w as libc::c_int >> 16 as libc::c_int);
                }
                (*(*ne).enttab.offset(count as isize)).segment = read_byte();
                (*(*ne).enttab.offset(count as isize)).offset = read_word()
            } else if !(index as libc::c_int == 0 as libc::c_int) {
                (*(*ne).enttab.offset(count as isize)).flags = read_byte();
                (*(*ne).enttab.offset(count as isize)).segment = index;
                (*(*ne).enttab.offset(count as isize)).offset = read_word()
            }
            count += 1;
            i = i.wrapping_add(1)
        }
    }
    (*ne).entcount = count as libc::c_uint;
}
unsafe extern "C" fn load_exports(mut module: *mut import_module) {
    let mut specfile: *mut FILE = 0 as *mut FILE;
    let mut spec_name: [libc::c_char; 18] = [0; 18];
    let mut line: [libc::c_char; 300] = [0; 300];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut count: libc::c_int = 0;
    let mut ordinal: word = 0;
    sprintf(spec_name.as_mut_ptr(),
            b"%.8s.ORD\x00" as *const u8 as *const libc::c_char,
            (*module).name);
    specfile =
        fopen(spec_name.as_mut_ptr(),
              b"r\x00" as *const u8 as *const libc::c_char);
    if specfile.is_null() {
        sprintf(spec_name.as_mut_ptr(),
                b"spec/%.8s.ORD\x00" as *const u8 as *const libc::c_char,
                (*module).name);
        specfile =
            fopen(spec_name.as_mut_ptr(),
                  b"r\x00" as *const u8 as *const libc::c_char);
        if specfile.is_null() {
            fprintf(stderr,
                    b"Note: couldn\'t find specfile for module %s; exported names won\'t be given.\n\x00"
                        as *const u8 as *const libc::c_char, (*module).name);
            fprintf(stderr,
                    b"      To create a specfile, run `dumpne -o <module.dll>\'.\n\x00"
                        as *const u8 as *const libc::c_char);
            (*module).exports = 0 as *mut export;
            (*module).export_count = 0 as libc::c_int as libc::c_uint;
            return
        }
    }
    /* first grab a count */
    count = 0 as libc::c_int; /* kill final newline */
    while !fgets(line.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 300]>() as libc::c_ulong
                     as libc::c_int, specfile).is_null() {
        if line[0 as libc::c_int as usize] as libc::c_int == '#' as i32 ||
               line[0 as libc::c_int as usize] as libc::c_int == '\n' as i32 {
            continue ;
        }
        count += 1
    }
    (*module).exports =
        malloc((count as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<export>()
                                                    as libc::c_ulong)) as
            *mut export;
    fseek(specfile, 0 as libc::c_int as libc::c_long, 0 as libc::c_int);
    count = 0 as libc::c_int;
    while !fgets(line.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 300]>() as libc::c_ulong
                     as libc::c_int, specfile).is_null() {
        if line[0 as libc::c_int as usize] as libc::c_int == '#' as i32 ||
               line[0 as libc::c_int as usize] as libc::c_int == '\n' as i32 {
            continue ;
        }
        p = strchr(line.as_mut_ptr(), '\n' as i32);
        if !p.is_null() { *p = 0 as libc::c_int as libc::c_char }
        if sscanf(line.as_mut_ptr(),
                  b"%hu\x00" as *const u8 as *const libc::c_char,
                  &mut ordinal as *mut word) != 1 as libc::c_int {
            fprintf(stderr,
                    b"Error reading specfile near line: `%s\'\n\x00" as
                        *const u8 as *const libc::c_char, line.as_mut_ptr());
        } else {
            (*(*module).exports.offset(count as isize)).ordinal = ordinal;
            p = strchr(line.as_mut_ptr(), '\t' as i32);
            if !p.is_null() {
                p = p.offset(1);
                let ref mut fresh4 =
                    (*(*module).exports.offset(count as isize)).name;
                *fresh4 = strdup(p);
                if opts as libc::c_int & 0x2 as libc::c_int != 0 &&
                       *(*(*module).exports.offset(count as
                                                       isize)).name.offset(0
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize)
                           as libc::c_int == '?' as i32 {
                    let ref mut fresh5 =
                        (*(*module).exports.offset(count as isize)).name;
                    *fresh5 =
                        demangle((*(*module).exports.offset(count as
                                                                isize)).name)
                }
            } else {
                let ref mut fresh6 =
                    (*(*module).exports.offset(count as isize)).name;
                *fresh6 = 0 as *mut libc::c_char
            }
            count += 1
        }
    }
    (*module).export_count = count as libc::c_uint;
    fclose(specfile);
}
unsafe extern "C" fn get_import_module_table(mut start: libc::c_long,
                                             mut ne: *mut ne) {
    let mut offset: word = 0;
    let mut length: byte = 0;
    let mut i: libc::c_uint = 0;
    fseek(f, start, 0 as libc::c_int);
    (*ne).imptab =
        malloc(((*ne).header.ne_cmod as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<import_module>()
                                                    as libc::c_ulong)) as
            *mut import_module;
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*ne).header.ne_cmod as libc::c_uint {
        offset = read_word();
        length = *(*ne).nametab.offset(offset as isize);
        let ref mut fresh7 = (*(*ne).imptab.offset(i as isize)).name;
        *fresh7 =
            malloc(((length as libc::c_int + 1 as libc::c_int) as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                        as libc::c_ulong)) as
                *mut libc::c_char;
        memcpy((*(*ne).imptab.offset(i as isize)).name as *mut libc::c_void,
               &mut *(*ne).nametab.offset((offset as libc::c_int +
                                               1 as libc::c_int) as isize) as
                   *mut byte as *const libc::c_void, length as libc::c_ulong);
        *(*(*ne).imptab.offset(i as isize)).name.offset(length as isize) =
            0 as libc::c_int as libc::c_char;
        if mode as libc::c_int & 0x10 as libc::c_int != 0 {
            load_exports(&mut *(*ne).imptab.offset(i as isize));
        } else {
            let ref mut fresh8 = (*(*ne).imptab.offset(i as isize)).exports;
            *fresh8 = 0 as *mut export;
            (*(*ne).imptab.offset(i as isize)).export_count =
                0 as libc::c_int as libc::c_uint
        }
        i = i.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn readne(mut offset_ne: libc::c_long,
                                mut ne: *mut ne) {
    fseek(f, offset_ne, 0 as libc::c_int);
    fread(&mut (*ne).header as *mut header_ne as *mut libc::c_void,
          ::std::mem::size_of::<header_ne>() as libc::c_ulong,
          1 as libc::c_int as size_t, f);
    /* read our various tables */
    get_entry_table(offset_ne + (*ne).header.ne_enttab as libc::c_long, ne);
    (*ne).name =
        read_res_name_table(offset_ne +
                                (*ne).header.ne_restab as libc::c_long,
                            (*ne).enttab);
    (*ne).description =
        read_res_name_table((*ne).header.ne_nrestab as libc::c_long,
                            (*ne).enttab);
    fseek(f, offset_ne + (*ne).header.ne_imptab as libc::c_long,
          0 as libc::c_int);
    (*ne).nametab =
        malloc(((*ne).header.ne_enttab as libc::c_int -
                    (*ne).header.ne_imptab as libc::c_int) as libc::c_ulong)
            as *mut byte;
    fread((*ne).nametab as *mut libc::c_void,
          ::std::mem::size_of::<byte>() as libc::c_ulong,
          ((*ne).header.ne_enttab as libc::c_int -
               (*ne).header.ne_imptab as libc::c_int) as size_t, f);
    get_import_module_table(offset_ne +
                                (*ne).header.ne_modtab as libc::c_long, ne);
    read_segments(offset_ne + (*ne).header.ne_segtab as libc::c_long, ne);
}
#[no_mangle]
pub unsafe extern "C" fn freene(mut ne: *mut ne) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    free((*ne).nametab as *mut libc::c_void);
    free((*ne).name as *mut libc::c_void);
    free((*ne).description as *mut libc::c_void);
    /* free the entry table */
    if !(*ne).enttab.is_null() {
        i = 0 as libc::c_int;
        while (i as libc::c_uint) < (*ne).entcount {
            free((*(*ne).enttab.offset(i as isize)).name as
                     *mut libc::c_void);
            i += 1
        }
        free((*ne).enttab as *mut libc::c_void);
    }
    /* free the import module table */
    if !(*ne).imptab.is_null() {
        i = 0 as libc::c_int;
        while i < (*ne).header.ne_cmod as libc::c_int {
            free((*(*ne).imptab.offset(i as isize)).name as
                     *mut libc::c_void);
            j = 0 as libc::c_int;
            while (j as libc::c_uint) <
                      (*(*ne).imptab.offset(i as isize)).export_count {
                free((*(*(*ne).imptab.offset(i as
                                                 isize)).exports.offset(j as
                                                                            isize)).name
                         as *mut libc::c_void);
                j += 1
            }
            free((*(*ne).imptab.offset(i as isize)).exports as
                     *mut libc::c_void);
            i += 1
        }
        free((*ne).imptab as *mut libc::c_void);
    }
    free_segments(ne);
}
/* Whether to print addresses relative to the image base for PE files. */
/* Entry points */
#[no_mangle]
pub unsafe extern "C" fn dumpne(mut offset_ne: libc::c_long) {
    let mut ne: ne =
        ne{header:
               header_ne{ne_magic: 0,
                         ne_ver: 0,
                         ne_rev: 0,
                         ne_enttab: 0,
                         ne_cbenttab: 0,
                         ne_crc: 0,
                         ne_flags: 0,
                         ne_autodata: 0,
                         ne_unused: 0,
                         ne_heap: 0,
                         ne_stack: 0,
                         ne_ip: 0,
                         ne_cs: 0,
                         ne_sp: 0,
                         ne_ss: 0,
                         ne_cseg: 0,
                         ne_cmod: 0,
                         ne_cbnrestab: 0,
                         ne_segtab: 0,
                         ne_rsrctab: 0,
                         ne_restab: 0,
                         ne_modtab: 0,
                         ne_imptab: 0,
                         ne_nrestab: 0,
                         ne_cmovent: 0,
                         ne_align: 0,
                         ne_cres: 0,
                         ne_exetyp: 0,
                         ne_flagsothers: 0,
                         ne_pretthunks: 0,
                         ne_psegrefbytes: 0,
                         ne_swaparea: 0,
                         ne_expver_min: 0,
                         ne_expver_maj: 0,},
           name: 0 as *mut libc::c_char,
           description: 0 as *mut libc::c_char,
           nametab: 0 as *mut byte,
           enttab: 0 as *mut entry,
           entcount: 0,
           imptab: 0 as *mut import_module,
           segments: 0 as *mut segment,};
    let mut i: libc::c_int = 0;
    readne(offset_ne, &mut ne);
    if mode as libc::c_int == 0x80 as libc::c_int {
        print_specfile(&mut ne);
        freene(&mut ne);
        return
    }
    printf(b"Module type: NE (New Executable)\n\x00" as *const u8 as
               *const libc::c_char);
    printf(b"Module name: %s\n\x00" as *const u8 as *const libc::c_char,
           ne.name);
    printf(b"Module description: %s\n\x00" as *const u8 as
               *const libc::c_char, ne.description);
    if mode as libc::c_int & 0x1 as libc::c_int != 0 {
        print_header(&mut ne.header);
    }
    if mode as libc::c_int & 0x4 as libc::c_int != 0 {
        putchar('\n' as i32);
        printf(b"Exports:\n\x00" as *const u8 as *const libc::c_char);
        print_export(&mut ne);
    }
    if mode as libc::c_int & 0x8 as libc::c_int != 0 {
        putchar('\n' as i32);
        printf(b"Imported modules:\n\x00" as *const u8 as
                   *const libc::c_char);
        i = 0 as libc::c_int;
        while i < ne.header.ne_cmod as libc::c_int {
            printf(b"\t%s\n\x00" as *const u8 as *const libc::c_char,
                   (*ne.imptab.offset(i as isize)).name);
            i += 1
        }
    }
    if mode as libc::c_int & 0x10 as libc::c_int != 0 {
        print_segments(&mut ne);
    }
    if mode as libc::c_int & 0x2 as libc::c_int != 0 {
        fseek(f, offset_ne + ne.header.ne_rsrctab as libc::c_long,
              0 as libc::c_int);
        if ne.header.ne_rsrctab as libc::c_int !=
               ne.header.ne_restab as libc::c_int {
            print_rsrc(offset_ne + ne.header.ne_rsrctab as libc::c_long);
        } else {
            printf(b"No resource table\n\x00" as *const u8 as
                       *const libc::c_char);
        }
    }
    freene(&mut ne);
}
