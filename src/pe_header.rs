use ::libc;
use ::c2rust_bitfields;
use crate::src::common::{ Instruction, Operation, Argument, _IO_FILE, size_t, __int8_t,
                          __uint8_t, __int16_t, __uint16_t, __int32_t, __uint32_t,
                          __uint64_t, __off_t, __off64_t, int8_t, int16_t, int32_t,
                          uint8_t, uint16_t, uint32_t, uint64_t, FILE, byte, dword,
                          word, qword, C2RustUnnamed, disptype,
                          DISP_REG, DISP_16, DISP_8, DISP_NONE, argtype, GAS, NASM, MASM,
                          f, asm_syntax, opts, mode};

extern "C" {
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    static mut stdout: *mut _IO_FILE;
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fopen(__filename: *const libc::c_char, __modes: *const libc::c_char)
     -> *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn fread(__ptr: *mut libc::c_void, __size: size_t, __n: size_t,
             __stream: *mut FILE) -> size_t;
    #[no_mangle]
    fn fseek(__stream: *mut FILE, __off: libc::c_long, __whence: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    #[no_mangle]
    fn perror(__s: *const libc::c_char);
    #[no_mangle]
    fn _IO_putc(__c: libc::c_int, __fp: *mut _IO_FILE) -> libc::c_int;
    #[no_mangle]
    fn _IO_getc(__fp: *mut _IO_FILE) -> libc::c_int;
    #[no_mangle]
    static mut pe_rel_addr: libc::c_int;
    
    
    
}


#[repr(C)]#[derive(Copy, Clone)]
pub struct pe {
    pub magic: word,
    pub imagebase: qword,
    pub header: file_header,
    pub c2rust_unnamed: C2RustUnnamed_0,
    pub dirs: *mut directory,
    pub name: *mut libc::c_char,
    pub sections: *mut section,
    pub exports: *mut export,
    pub export_count: libc::c_uint,
    pub imports: *mut import_module,
    pub import_count: libc::c_uint,
    pub reloc_base: dword,
    pub relocs: *mut reloc_pe,
    pub reloc_count: libc::c_uint,
}

#[repr(C, packed)]#[derive(Copy, Clone, BitfieldStruct)]
pub struct reloc_pe {
    #[bitfield(name = "offset", ty = "libc::c_uint", bits = "0..=11")]
    #[bitfield(name = "type_0", ty = "libc::c_uint", bits = "12..=15")]
    pub offset_type_0: [u8; 2],
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct import_module {
    pub module: *mut libc::c_char,
    pub nametab_addr: dword,
    pub nametab: *mut *mut libc::c_char,
    pub count: libc::c_uint,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct export {
    pub address: dword,
    pub ordinal: word,
    pub name: *mut libc::c_char,
}

#[repr(C, packed)]#[derive(Copy, Clone)]
pub struct section {
    pub name: [libc::c_char; 8],
    pub min_alloc: dword,
    pub address: dword,
    pub length: dword,
    pub offset: dword,
    pub reloc_offset: dword,
    pub lineno_offset: dword,
    pub reloc_count: word,
    pub lineno_count: word,
    pub flags: dword,
    pub instr_flags: *mut byte,
}
/* 16 */

#[repr(C, packed)]#[derive(Copy, Clone)]
pub struct directory {
    pub address: dword,
    pub size: dword,
}

#[repr(C)]#[derive(Copy, Clone)]
pub union C2RustUnnamed_0 {
    pub opt32: optional_header,
    pub opt64: optional_header_pep,
}

#[repr(C, packed)]#[derive(Copy, Clone)]
pub struct optional_header_pep {
    pub Magic: word,
    pub MajorLinkerVersion: byte,
    pub MinorLinkerVersion: byte,
    pub SizeOfCode: dword,
    pub SizeOfInitializedData: dword,
    pub SizeOfUninitializedData: dword,
    pub AddressOfEntryPoint: dword,
    pub BaseOfCode: dword,
    pub ImageBase: qword,
    pub SectionAlignment: dword,
    pub FileAlignment: dword,
    pub MajorOperatingSystemVersion: word,
    pub MinorOperatingSystemVersion: word,
    pub MajorImageVersion: word,
    pub MinorImageVersion: word,
    pub MajorSubsystemVersion: word,
    pub MinorSubsystemVersion: word,
    pub Win32VersionValue: dword,
    pub SizeOfImage: dword,
    pub SizeOfHeaders: dword,
    pub CheckSum: dword,
    pub Subsystem: word,
    pub DllCharacteristics: word,
    pub SizeOfStackReserve: qword,
    pub SizeOfStackCommit: qword,
    pub SizeOfHeapReserve: qword,
    pub SizeOfHeapCommit: qword,
    pub LoaderFlags: dword,
    pub NumberOfRvaAndSizes: dword,
}

#[repr(C, packed)]#[derive(Copy, Clone)]
pub struct optional_header {
    pub Magic: word,
    pub MajorLinkerVersion: byte,
    pub MinorLinkerVersion: byte,
    pub SizeOfCode: dword,
    pub SizeOfInitializedData: dword,
    pub SizeOfUninitializedData: dword,
    pub AddressOfEntryPoint: dword,
    pub BaseOfCode: dword,
    pub BaseOfData: dword,
    pub ImageBase: dword,
    pub SectionAlignment: dword,
    pub FileAlignment: dword,
    pub MajorOperatingSystemVersion: word,
    pub MinorOperatingSystemVersion: word,
    pub MajorImageVersion: word,
    pub MinorImageVersion: word,
    pub MajorSubsystemVersion: word,
    pub MinorSubsystemVersion: word,
    pub Win32VersionValue: dword,
    pub SizeOfImage: dword,
    pub SizeOfHeaders: dword,
    pub CheckSum: dword,
    pub Subsystem: word,
    pub DllCharacteristics: word,
    pub SizeOfStackReserve: dword,
    pub SizeOfStackCommit: dword,
    pub SizeOfHeapReserve: dword,
    pub SizeOfHeapCommit: dword,
    pub LoaderFlags: dword,
    pub NumberOfRvaAndSizes: dword,
}

#[repr(C, packed)]#[derive(Copy, Clone)]
pub struct file_header {
    pub Machine: word,
    pub NumberOfSections: word,
    pub TimeDateStamp: dword,
    pub PointerToSymbolTable: dword,
    pub NumberOfSymbols: dword,
    pub SizeOfOptionalHeader: word,
    pub Characteristics: word,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct export_header {
    pub flags: dword,
    pub timestamp: dword,
    pub ver_major: word,
    pub ver_minor: word,
    pub module_name_addr: dword,
    pub ordinal_base: dword,
    pub addr_table_count: dword,
    pub export_count: dword,
    pub addr_table_addr: dword,
    pub name_table_addr: dword,
    pub ord_table_addr: dword,
}
#[inline]
unsafe extern "C" fn putchar(mut __c: libc::c_int) -> libc::c_int {
    return _IO_putc(__c, stdout);
}

#[inline]
unsafe extern "C" fn read_byte() -> byte { return _IO_getc(f) as byte; }

#[inline]
unsafe extern "C" fn read_dword() -> dword {
     let mut d =  0;
    fread(&mut d as *mut dword as *mut libc::c_void,
          4u64, 1u64, f);
    return d;
}
#[inline]
unsafe extern "C" fn read_qword() -> dword {
     let mut q =  0;
    fread(&mut q as *mut qword as *mut libc::c_void,
          8u64, 1u64, f);
    return q as dword;
}

/*
 * Functions for parsing the PE header
 *
 * Copyright 2017-2018 Zebediah Figura
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
unsafe extern "C" fn print_flags(mut flags: word) {
     let mut buffer =
    
        *::std::mem::transmute::<&[u8; 1024],
                                 &mut [libc::c_char; 1024]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"); /* deprecated and reserved */
    if flags as libc::c_int & 0x1i32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", relocations stripped\x00" as *const u8 as
                   *const libc::c_char); /* 44 */
    } /* 1a */
    if flags as libc::c_int & 0x2i32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", executable\x00" as *const u8 as *const libc::c_char);
    }
    if flags as libc::c_int & 0x4i32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", line numbers stripped\x00" as *const u8 as
                   *const libc::c_char);
    }
    if flags as libc::c_int & 0x8i32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", local symbols stripped\x00" as *const u8 as
                   *const libc::c_char);
    }
    if flags as libc::c_int & 0x10i32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", aggressively trimmed\x00" as *const u8 as
                   *const libc::c_char);
    }
    if flags as libc::c_int & 0x20i32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", large address aware\x00" as *const u8 as
                   *const libc::c_char);
    }
    if flags as libc::c_int & 0x40i32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", 16-bit\x00" as *const u8 as *const libc::c_char);
    }
    if flags as libc::c_int & 0x80i32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", little-endian\x00" as *const u8 as *const libc::c_char);
    }
    if flags as libc::c_int & 0x100i32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", 32-bit\x00" as *const u8 as *const libc::c_char);
    }
    if flags as libc::c_int & 0x200i32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", debug info stripped\x00" as *const u8 as
                   *const libc::c_char);
    }
    if flags as libc::c_int & 0x400i32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP\x00" as *const u8 as
                   *const libc::c_char);
    }
    if flags as libc::c_int & 0x800i32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", IMAGE_FILE_NET_RUN_FROM_SWAP\x00" as *const u8 as
                   *const libc::c_char);
    }
    if flags as libc::c_int & 0x1000i32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", system file\x00" as *const u8 as *const libc::c_char);
    }
    if flags as libc::c_int & 0x2000i32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", DLL\x00" as *const u8 as *const libc::c_char);
    }
    if flags as libc::c_int & 0x4000i32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", uniprocessor\x00" as *const u8 as *const libc::c_char);
    }
    if flags as libc::c_int & 0x8000i32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", big-endian\x00" as *const u8 as *const libc::c_char);
    }
    printf(b"Flags: 0x%04x (%s)\n\x00" as *const u8 as *const libc::c_char,
           flags as libc::c_int,
           buffer.as_mut_ptr().offset(2isize));
}
unsafe extern "C" fn print_dll_flags(mut flags: word) {
     let mut buffer =
    
        *::std::mem::transmute::<&[u8; 1024],
                                 &mut [libc::c_char; 1024]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
    if flags as libc::c_int & 0x1i32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", per-process initialization\x00" as *const u8 as
                   *const libc::c_char);
    }
    if flags as libc::c_int & 0x2i32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", per-process termination\x00" as *const u8 as
                   *const libc::c_char);
    }
    if flags as libc::c_int & 0x4i32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", per-thread initialization\x00" as *const u8 as
                   *const libc::c_char);
    }
    if flags as libc::c_int & 0x8i32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", per-thread termination\x00" as *const u8 as
                   *const libc::c_char);
    }
    if flags as libc::c_int & 0x40i32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", dynamic base\x00" as *const u8 as *const libc::c_char);
    }
    if flags as libc::c_int & 0x80i32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", force integrity\x00" as *const u8 as *const libc::c_char);
    }
    if flags as libc::c_int & 0x100i32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", DEP compatible\x00" as *const u8 as *const libc::c_char);
    }
    if flags as libc::c_int & 0x200i32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", no isolation\x00" as *const u8 as *const libc::c_char);
    }
    if flags as libc::c_int & 0x400i32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", no SEH\x00" as *const u8 as *const libc::c_char);
    }
    if flags as libc::c_int & 0x800i32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", no bind\x00" as *const u8 as *const libc::c_char);
    }
    if flags as libc::c_int & 0x2000i32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", WDM driver\x00" as *const u8 as *const libc::c_char);
    }
    if flags as libc::c_int & 0x8000i32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", terminal server aware\x00" as *const u8 as
                   *const libc::c_char);
    }
    if flags as libc::c_int & 0x5030i32 != 0 {
        sprintf(buffer.as_mut_ptr().offset(strlen(buffer.as_mut_ptr()) as
                                               isize),
                b", (unknown flags 0x%04x)\x00" as *const u8 as
                    *const libc::c_char,
                flags as libc::c_int & 0x5030i32);
    }
    printf(b"DLL flags: 0x%04x (%s)\n\x00" as *const u8 as
               *const libc::c_char, flags as libc::c_int,
           buffer.as_mut_ptr().offset(2isize));
}
static mut subsystems: [*const libc::c_char; 18] =
    [b"unknown\x00" as *const u8 as *const libc::c_char,
     b"native\x00" as *const u8 as *const libc::c_char,
     b"GUI\x00" as *const u8 as *const libc::c_char,
     b"CUI\x00" as *const u8 as *const libc::c_char,
     b"(unknown value 4)\x00" as *const u8 as *const libc::c_char,
     b"OS/2 CUI\x00" as *const u8 as *const libc::c_char,
     b"(unknown value 6)\x00" as *const u8 as *const libc::c_char,
     b"POSIX CUI\x00" as *const u8 as *const libc::c_char,
     b"(unknown value 8)\x00" as *const u8 as *const libc::c_char,
     b"CE\x00" as *const u8 as *const libc::c_char,
     b"EFI\x00" as *const u8 as *const libc::c_char,
     b"EFI with boot services\x00" as *const u8 as *const libc::c_char,
     b"EFI with runtime services\x00" as *const u8 as *const libc::c_char,
     b"EFI ROM image\x00" as *const u8 as *const libc::c_char,
     b"Xbox\x00" as *const u8 as *const libc::c_char,
     b"(unknown value 15)\x00" as *const u8 as *const libc::c_char,
     b"boot\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char];
unsafe extern "C" fn print_opt32(mut opt: *mut optional_header) {
    printf(b"File version: %d.%d\n\x00" as *const u8 as *const libc::c_char,
           (*opt).MajorImageVersion as libc::c_int,
           (*opt).MinorImageVersion as libc::c_int);
    printf(b"Linker version: %d.%d\n\x00" as *const u8 as *const libc::c_char,
           (*opt).MajorLinkerVersion as libc::c_int,
           (*opt).MinorLinkerVersion as libc::c_int);
    if (*opt).AddressOfEntryPoint != 0 {
         let mut address =  (*opt).AddressOfEntryPoint;
        if pe_rel_addr == 0 {
            address =
                
                (address).wrapping_add((*opt).ImageBase)
        }
        printf(b"Program entry point: 0x%x\n\x00" as *const u8 as
                   *const libc::c_char, address);
        /* 28 */
    } /* 2c */
    printf(b"Base of code section: 0x%x\n\x00" as *const u8 as
               *const libc::c_char, (*opt).BaseOfCode); /* 30 */
    printf(b"Base of data section: 0x%x\n\x00" as *const u8 as
               *const libc::c_char, (*opt).BaseOfData); /* 34 */
    printf(b"Preferred base address: 0x%x\n\x00" as *const u8 as
               *const libc::c_char, (*opt).ImageBase); /* 40 */
    printf(b"Required OS version: %d.%d\n\x00" as *const u8 as
               *const libc::c_char,
           (*opt).MajorOperatingSystemVersion as libc::c_int,
           (*opt).MinorOperatingSystemVersion as libc::c_int); /* 4c */
    if (*opt).Win32VersionValue != 0u32 {
        fprintf(stderr,
                b"Warning: Win32VersionValue is %d (expected 0)\n\x00" as
                    *const u8 as *const libc::c_char,
                (*opt).Win32VersionValue);
    }
    if (*opt).Subsystem as libc::c_int <= 16i32 {
        /* 5c */
        printf(b"Subsystem: %s\n\x00" as *const u8 as *const libc::c_char,
               subsystems[(*opt).Subsystem as usize]); /* 48 */
    } else {
        printf(b"Subsystem: (unknown value %d)\n\x00" as *const u8 as
                   *const libc::c_char,
               (*opt).Subsystem as libc::c_int); /* 5e */
    } /* 60 */
    printf(b"Subsystem version: %d.%d\n\x00" as *const u8 as
               *const libc::c_char,
           (*opt).MajorSubsystemVersion as libc::c_int,
           (*opt).MinorSubsystemVersion as libc::c_int); /* 64 */
    print_dll_flags((*opt).DllCharacteristics); /* 68 */
    printf(b"Stack size (reserve): %d bytes\n\x00" as *const u8 as
               *const libc::c_char, (*opt).SizeOfStackReserve); /* 6c */
    printf(b"Stack size (commit): %d bytes\n\x00" as *const u8 as
               *const libc::c_char, (*opt).SizeOfStackCommit);
    printf(b"Heap size (reserve): %d bytes\n\x00" as *const u8 as
               *const libc::c_char, (*opt).SizeOfHeapReserve);
    printf(b"Heap size (commit): %d bytes\n\x00" as *const u8 as
               *const libc::c_char, (*opt).SizeOfHeapCommit);
    if (*opt).LoaderFlags != 0u32 {
        fprintf(stderr,
                b"Warning: LoaderFlags is 0x%x (expected 0)\n\x00" as
                    *const u8 as *const libc::c_char, (*opt).LoaderFlags);
    };
    /* 70 */
}
unsafe extern "C" fn print_opt64(mut opt: *mut optional_header_pep) {
    printf(b"File version: %d.%d\n\x00" as *const u8 as *const libc::c_char,
           (*opt).MajorImageVersion as libc::c_int,
           (*opt).MinorImageVersion as libc::c_int); /* 44 */
    printf(b"Linker version: %d.%d\n\x00" as *const u8 as *const libc::c_char,
           (*opt).MajorLinkerVersion as libc::c_int,
           (*opt).MinorLinkerVersion as libc::c_int); /* 1a */
    if (*opt).AddressOfEntryPoint != 0 {
         let mut address =  (*opt).AddressOfEntryPoint;
        if pe_rel_addr == 0 {
            address =
                
                (address as libc::c_ulong).wrapping_add((*opt).ImageBase) as dword
        }
        printf(b"Program entry point: 0x%x\n\x00" as *const u8 as
                   *const libc::c_char, address);
        /* 28 */
    } /* 2c */
    printf(b"Base of code section: 0x%x\n\x00" as *const u8 as
               *const libc::c_char, (*opt).BaseOfCode); /* 30 */
    printf(b"Preferred base address: 0x%lx\n\x00" as *const u8 as
               *const libc::c_char, (*opt).ImageBase); /* 40 */
    printf(b"Required OS version: %d.%d\n\x00" as *const u8 as
               *const libc::c_char,
           (*opt).MajorOperatingSystemVersion as libc::c_int,
           (*opt).MinorOperatingSystemVersion as libc::c_int); /* 4c */
    if (*opt).Win32VersionValue != 0u32 {
        fprintf(stderr,
                b"Warning: Win32VersionValue is %d (expected 0)\n\x00" as
                    *const u8 as *const libc::c_char,
                (*opt).Win32VersionValue);
    }
    if (*opt).Subsystem as libc::c_int <= 16i32 {
        /* 5c */
        printf(b"Subsystem: %s\n\x00" as *const u8 as *const libc::c_char,
               subsystems[(*opt).Subsystem as usize]); /* 48 */
    } else {
        printf(b"Subsystem: (unknown value %d)\n\x00" as *const u8 as
                   *const libc::c_char,
               (*opt).Subsystem as libc::c_int); /* 5e */
    } /* 60 */
    printf(b"Subsystem version: %d.%d\n\x00" as *const u8 as
               *const libc::c_char,
           (*opt).MajorSubsystemVersion as libc::c_int,
           (*opt).MinorSubsystemVersion as libc::c_int); /* 68 */
    print_dll_flags((*opt).DllCharacteristics); /* 70 */
    printf(b"Stack size (reserve): %ld bytes\n\x00" as *const u8 as
               *const libc::c_char, (*opt).SizeOfStackReserve); /* 78 */
    printf(b"Stack size (commit): %ld bytes\n\x00" as *const u8 as
               *const libc::c_char, (*opt).SizeOfStackCommit);
    printf(b"Heap size (reserve): %ld bytes\n\x00" as *const u8 as
               *const libc::c_char, (*opt).SizeOfHeapReserve);
    printf(b"Heap size (commit): %ld bytes\n\x00" as *const u8 as
               *const libc::c_char, (*opt).SizeOfHeapCommit);
    if (*opt).LoaderFlags != 0u32 {
        fprintf(stderr,
                b"Warning: LoaderFlags is 0x%x (expected 0)\n\x00" as
                    *const u8 as *const libc::c_char, (*opt).LoaderFlags);
    };
    /* 80 */
}
unsafe extern "C" fn print_header(mut pe: *mut pe) {
    putchar('\n' as i32); /* 16 */
    if (*pe).header.SizeOfOptionalHeader == 0 {
        printf(b"No optional header\n\x00" as *const u8 as
                   *const libc::c_char);
        return
    } else {
        if ((*pe).header.SizeOfOptionalHeader as libc::c_ulong) <
               ::std::mem::size_of::<optional_header>() as libc::c_ulong {
            fprintf(stderr,
                    b"Warning: Size of optional header is %u (expected at least %lu).\n\x00"
                        as *const u8 as *const libc::c_char,
                    (*pe).header.SizeOfOptionalHeader as libc::c_int,
                    ::std::mem::size_of::<optional_header>() as
                        libc::c_ulong);
        }
    }
    print_flags((*pe).header.Characteristics);
    if (*pe).magic as libc::c_int == 0x10bi32 {
        printf(b"Image type: 32-bit\n\x00" as *const u8 as
                   *const libc::c_char);
        print_opt32(&mut (*pe).c2rust_unnamed.opt32);
    } else if (*pe).magic as libc::c_int == 0x20bi32 {
        printf(b"Image type: 64-bit\n\x00" as *const u8 as
                   *const libc::c_char);
        print_opt64(&mut (*pe).c2rust_unnamed.opt64);
    };
}
unsafe extern "C" fn print_specfile(mut pe: *mut pe) {
    
    
     let mut spec_name =
    
        malloc(strlen((*pe).name).wrapping_add(4u64)) as
            *mut libc::c_char;
    sprintf(spec_name, b"%s.ord\x00" as *const u8 as *const libc::c_char,
            (*pe).name);
      let mut specfile =
    
     fopen(spec_name, b"w\x00" as *const u8 as *const libc::c_char);
    if specfile.is_null() {
        perror(b"Couldn\'t open %s\x00" as *const u8 as *const libc::c_char);
        return
    }
    fprintf(specfile,
            b"#Generated by dump -o\n\x00" as *const u8 as
                *const libc::c_char);
      let mut i =   0i32;
    while (i as libc::c_uint) < (*pe).export_count {
        fprintf(specfile, b"%d\t%s\n\x00" as *const u8 as *const libc::c_char,
                (*(*pe).exports.offset(i as isize)).ordinal as libc::c_int,
                (*(*pe).exports.offset(i as isize)).name);
        i += 1
    }
    fclose(specfile);
}
unsafe extern "C" fn fstrdup(mut offset: libc::c_long) -> *mut libc::c_char {
     let mut cursor =  ftell(f);
    
    
    fseek(f, offset, 0i32);
    while read_byte() != 0 { }
      let mut len = 
    
        (ftell(f) - offset + 1i64) as libc::c_int;
    fseek(f, offset, 0i32);
      let mut ret = 
     malloc(len as libc::c_ulong) as *mut libc::c_char;
    fread(ret as *mut libc::c_void,
          ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
          len as size_t, f);
    fseek(f, cursor, 0i32);
    return ret;
}
unsafe extern "C" fn get_export_table(mut pe: *mut pe) {
    
     let mut header =
    
        export_header{flags: 0,
                      timestamp: 0,
                      ver_major: 0,
                      ver_minor: 0,
                      module_name_addr: 0,
                      ordinal_base: 0,
                      addr_table_count: 0,
                      export_count: 0,
                      addr_table_addr: 0,
                      name_table_addr: 0,
                      ord_table_addr: 0,}; let mut offset =
    
        crate::src::pe_section::addr2offset((*(*pe).dirs.offset(0isize)).address,
                    pe);
    
    /* More headers. It's like a PE file is nothing but headers.
     * Do we really need to print any of this? No, not really. Just use the data. */
    fseek(f, offset, 0i32);
    fread(&mut header as *mut export_header as *mut libc::c_void,
          ::std::mem::size_of::<export_header>() as libc::c_ulong,
          1u64, f);
    /* Grab the name. */
    
    /* Grab the exports. */
     *pe =
    crate::src::pe_header::pe{name:
                                  
                               fstrdup(crate::src::pe_section::addr2offset(header.module_name_addr, pe)),
                              exports:
                                  
                              
        malloc((header.addr_table_count as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<export>()
                                                    as libc::c_ulong)) as
            *mut export, ..*pe};
    /* If addr_table_count exceeds export_count, this means that some exports
     * are nameless (and thus exported by ordinal). */
    fseek(f, crate::src::pe_section::addr2offset(header.addr_table_addr, pe), 0i32);
      let mut i =   0i32;
    while (i as libc::c_uint) < header.addr_table_count {
        
         *(*pe).exports.offset(i as isize) =
    crate::src::pe_header::export{ordinal:
                                      
                                  
            (i as libc::c_uint).wrapping_add(header.ordinal_base) as word,
                                  address:
                                      
                                   read_dword(),
                                                      ..*(*pe).exports.offset(i as isize)};
        let ref mut fresh0 = (*(*pe).exports.offset(i as isize)).name;
        *fresh0 = 0 as *mut libc::c_char;
        i += 1
    }
    /* Why? WHY? */
    i = 0i32;
    while (i as libc::c_uint) < header.export_count {
        
        fseek(f,
              (crate::src::pe_section::addr2offset(header.ord_table_addr, pe) as
                   libc::c_ulong).wrapping_add((i as
                                                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<word>()
                                                                                    as
                                                                                    libc::c_ulong))
                  as libc::c_long, 0i32);
          let mut index =   read_word();
        fseek(f,
              (crate::src::pe_section::addr2offset(header.name_table_addr, pe) as
                   libc::c_ulong).wrapping_add((i as
                                                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<dword>()
                                                                                    as
                                                                                    libc::c_ulong))
                  as libc::c_long, 0i32);
        let ref mut fresh1 = (*(*pe).exports.offset(index as isize)).name;
        *fresh1 = fstrdup(crate::src::pe_section::addr2offset(read_dword(), pe));
        i += 1
    }
     *pe = crate::src::pe_header::pe{export_count:   header.addr_table_count, ..*pe};
}
unsafe extern "C" fn get_import_name_table(mut module: *mut import_module,
                                           mut pe: *mut pe) {
    
     let mut offset =
     crate::src::pe_section::addr2offset((*module).nametab_addr, pe); let mut cursor =  ftell(f);
    
    
    fseek(f, offset, 0i32);
      let mut count =   0u32;
    if (*pe).magic as libc::c_int == 0x10bi32 {
        while read_dword() != 0 { count = count.wrapping_add(1) }
    } else { while read_qword() != 0 { count = count.wrapping_add(1) } }
     *module =
    crate::src::pe_header::import_module{nametab:
                                             
                                         
        malloc((count as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut *mut libc::c_char,
                                                                           ..*module};
    fseek(f, offset, 0i32);
      let mut i =   0u32;
    while i < count {
         let mut address =
    
            if (*pe).magic as libc::c_int == 0x10bi32 {
                read_dword()
            } else { read_qword() } as qword;
        if address & 0x80000000u64 != 0 {
            address &= 0x7fffffffu64;
            let ref mut fresh2 = *(*module).nametab.offset(i as isize);
            *fresh2 =
                malloc(snprintf(0 as *mut libc::c_char,
                                0u64,
                                b"%s.%lu\x00" as *const u8 as
                                    *const libc::c_char, (*module).module,
                                address) as libc::c_ulong) as
                    *mut libc::c_char;
            sprintf(*(*module).nametab.offset(i as isize),
                    b"%s.%lu\x00" as *const u8 as *const libc::c_char,
                    (*module).module, address);
        } else {
            let ref mut fresh3 = *(*module).nametab.offset(i as isize);
            *fresh3 =
                fstrdup(crate::src::pe_section::addr2offset(address as dword, pe) +
                            2i64)
        }
        i = i.wrapping_add(1)
        /* skip hint */
    }
     *module = crate::src::pe_header::import_module{count, ..*module};
    fseek(f, cursor, 0i32);
}
unsafe extern "C" fn get_import_module_table(mut pe: *mut pe) {
     let mut offset =
    
        crate::src::pe_section::addr2offset((*(*pe).dirs.offset(1isize)).address,
                    pe);
    static mut zeroes: [dword; 5] = [0u32, 0, 0, 0, 0];
     let mut entry =  [0; 5];
    
    fseek(f, offset, 0i32);
     *pe = crate::src::pe_header::pe{import_count:   0u32, ..*pe};
    while fread(entry.as_mut_ptr() as *mut libc::c_void,
                ::std::mem::size_of::<dword>() as libc::c_ulong,
                5u64, f) ==
              5u64 &&
              memcmp(entry.as_mut_ptr() as *const libc::c_void,
                     zeroes.as_ptr() as *const libc::c_void,
                     ::std::mem::size_of::<[dword; 5]>() as libc::c_ulong) !=
                  0 {
        *pe = crate::src::pe_header::pe{import_count:
                               (*pe).import_count.wrapping_add(1), ..*pe}
    }
     *pe =
    crate::src::pe_header::pe{imports:
                                  
                              
        malloc(((*pe).import_count as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<import_module>()
                                                    as libc::c_ulong)) as
            *mut import_module, ..*pe};
    fseek(f, offset, 0i32);
      let mut i =   0i32;
    while (i as libc::c_uint) < (*pe).import_count {
        fseek(f,
              (3u64).wrapping_mul(::std::mem::size_of::<dword>()
                                                   as libc::c_ulong) as
                  libc::c_long, 1i32);
        let ref mut fresh4 = (*(*pe).imports.offset(i as isize)).module;
        *fresh4 = fstrdup(crate::src::pe_section::addr2offset(read_dword(), pe));
         *(*pe).imports.offset(i as isize) =
    crate::src::pe_header::import_module{nametab_addr:
                                             
                                          read_dword(),
                                                             ..*(*pe).imports.offset(i as isize)};
        /* grab the imports themselves */
        get_import_name_table(&mut *(*pe).imports.offset(i as isize), pe);
        i += 1
    };
}
unsafe extern "C" fn get_reloc_table(mut pe: *mut pe) {
     let mut offset =
    
        crate::src::pe_section::addr2offset((*(*pe).dirs.offset(5isize)).address,
                    pe);
    fseek(f, offset, 0i32);
    
    
     *pe =
    crate::src::pe_header::pe{reloc_base:   read_dword(),
                              reloc_count:
                                  
                              
        read_dword().wrapping_sub(8u32).wrapping_div(2u32),
                              relocs:
                                  
                              
        malloc(((*pe).reloc_count as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<reloc_pe>()
                                                    as libc::c_ulong)) as
            *mut reloc_pe, ..*pe};
    fread((*pe).relocs as *mut libc::c_void,
          ::std::mem::size_of::<reloc_pe>() as libc::c_ulong,
          (*pe).reloc_count as size_t, f);
}
#[no_mangle]
pub unsafe extern "C" fn readpe(mut offset_pe: libc::c_long,
                                mut pe: *mut pe) {
    
     let mut cdirs =  0;
    fseek(f,
          (offset_pe as
               libc::c_ulong).wrapping_add(::std::mem::size_of::<dword>() as
                                               libc::c_ulong) as libc::c_long,
          0i32);
    fread(&mut (*pe).header as *mut file_header as *mut libc::c_void,
          ::std::mem::size_of::<file_header>() as libc::c_ulong,
          1u64, f);
     *pe = crate::src::pe_header::pe{magic:   read_word(), ..*pe};
    fseek(f,
          (::std::mem::size_of::<word>() as libc::c_ulong).wrapping_neg() as
              libc::c_long, 1i32);
    if (*pe).magic as libc::c_int == 0x10bi32 {
        fread(&mut (*pe).c2rust_unnamed.opt32 as *mut optional_header as
                  *mut libc::c_void,
              ::std::mem::size_of::<optional_header>() as libc::c_ulong,
              1u64, f);
         *pe =
    crate::src::pe_header::pe{imagebase:
                                  
                               (*pe).c2rust_unnamed.opt32.ImageBase as qword, ..*pe};
        cdirs = (*pe).c2rust_unnamed.opt32.NumberOfRvaAndSizes as libc::c_int
    } else if (*pe).magic as libc::c_int == 0x20bi32 {
        fread(&mut (*pe).c2rust_unnamed.opt64 as *mut optional_header_pep as
                  *mut libc::c_void,
              ::std::mem::size_of::<optional_header_pep>() as libc::c_ulong,
              1u64, f);
         *pe =
    crate::src::pe_header::pe{imagebase:
                                  
                               (*pe).c2rust_unnamed.opt64.ImageBase,
                                                                          ..*pe};
        cdirs = (*pe).c2rust_unnamed.opt64.NumberOfRvaAndSizes as libc::c_int
    } else {
        fprintf(stderr,
                b"Warning: Don\'t know how to read image type %#x\n\x00" as
                    *const u8 as *const libc::c_char,
                (*pe).magic as libc::c_int);
        exit(1i32);
    }
     *pe =
    crate::src::pe_header::pe{dirs:
                                  
                              
        malloc((cdirs as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<directory>()
                                                    as libc::c_ulong)) as
            *mut directory, ..*pe};
    fread((*pe).dirs as *mut libc::c_void,
          ::std::mem::size_of::<directory>() as libc::c_ulong,
          cdirs as size_t, f);
    /* read the section table */
     *pe =
    crate::src::pe_header::pe{sections:
                                  
                              
        malloc(((*pe).header.NumberOfSections as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<section>()
                                                    as libc::c_ulong)) as
            *mut section, ..*pe};
      let mut i =   0i32;
    while i < (*pe).header.NumberOfSections as libc::c_int {
        fread(&mut *(*pe).sections.offset(i as isize) as *mut section as
                  *mut libc::c_void, 0x28u64,
              1u64, f);
        /* allocate zeroes, but only if it's a code section */
        /* in theory nobody will ever try to jump into a data section.
         * VirtualProtect() be damned */
        if (*(*pe).sections.offset(i as isize)).flags &
               0x20u32 != 0 {
            let ref mut fresh5 =
                (*(*pe).sections.offset(i as isize)).instr_flags;
            *fresh5 =
                calloc((*(*pe).sections.offset(i as isize)).min_alloc as
                           libc::c_ulong,
                       ::std::mem::size_of::<byte>() as libc::c_ulong) as
                    *mut byte
        } else {
            let ref mut fresh6 =
                (*(*pe).sections.offset(i as isize)).instr_flags;
            *fresh6 = 0 as *mut byte
        }
        i += 1
    }
    /* Read the Data Directories.
     * PE is bizarre. It tries to make all of these things generic by putting
     * them in separate "directories". But the order of these seems to be fixed
     * anyway, so why bother? */
    if cdirs >= 1i32 &&
           (*(*pe).dirs.offset(0isize)).size != 0 {
        get_export_table(pe);
    }
    if cdirs >= 2i32 &&
           (*(*pe).dirs.offset(1isize)).size != 0 {
        get_import_module_table(pe);
    }
    if cdirs >= 6i32 &&
           (*(*pe).dirs.offset(5isize)).size != 0 {
        get_reloc_table(pe);
    }
    /* Read the code. */
    crate::src::pe_section::read_sections(pe);
}
#[no_mangle]
pub unsafe extern "C" fn freepe(mut pe: *mut pe) {
    
     let mut j =  0;
    free((*pe).dirs as *mut libc::c_void);
      let mut i =   0i32;
    while i < (*pe).header.NumberOfSections as libc::c_int {
        free((*(*pe).sections.offset(i as isize)).instr_flags as
                 *mut libc::c_void);
        i += 1
    }
    free((*pe).sections as *mut libc::c_void);
    free((*pe).name as *mut libc::c_void);
    i = 0i32;
    while (i as libc::c_uint) < (*pe).export_count {
        free((*(*pe).exports.offset(i as isize)).name as *mut libc::c_void);
        i += 1
    }
    free((*pe).exports as *mut libc::c_void);
    i = 0i32;
    while (i as libc::c_uint) < (*pe).import_count {
        j = 0i32;
        while (j as libc::c_uint) < (*(*pe).imports.offset(i as isize)).count
              {
            free(*(*(*pe).imports.offset(i as
                                             isize)).nametab.offset(j as
                                                                        isize)
                     as *mut libc::c_void);
            j += 1
        }
        free((*(*pe).imports.offset(i as isize)).nametab as
                 *mut libc::c_void);
        free((*(*pe).imports.offset(i as isize)).module as *mut libc::c_void);
        i += 1
    }
    free((*pe).imports as *mut libc::c_void);
    free((*pe).relocs as *mut libc::c_void);
}
/* Common functions */
/* Common globals */
/* what to dump */
/* additional options */
/* Whether to print addresses relative to the image base for PE files. */
/* Entry points */
#[no_mangle]
pub unsafe extern "C" fn dumppe(mut offset_pe: libc::c_long) {
    
     let mut pe =
    
        {
            let mut init =
                pe{magic: 0u16,
                   imagebase: 0,
                   header:
                       file_header{Machine: 0,
                                   NumberOfSections: 0,
                                   TimeDateStamp: 0,
                                   PointerToSymbolTable: 0,
                                   NumberOfSymbols: 0,
                                   SizeOfOptionalHeader: 0,
                                   Characteristics: 0,},
                   c2rust_unnamed:
                       C2RustUnnamed_0{opt32:
                                           optional_header{Magic: 0,
                                                           MajorLinkerVersion:
                                                               0,
                                                           MinorLinkerVersion:
                                                               0,
                                                           SizeOfCode: 0,
                                                           SizeOfInitializedData:
                                                               0,
                                                           SizeOfUninitializedData:
                                                               0,
                                                           AddressOfEntryPoint:
                                                               0,
                                                           BaseOfCode: 0,
                                                           BaseOfData: 0,
                                                           ImageBase: 0,
                                                           SectionAlignment:
                                                               0,
                                                           FileAlignment: 0,
                                                           MajorOperatingSystemVersion:
                                                               0,
                                                           MinorOperatingSystemVersion:
                                                               0,
                                                           MajorImageVersion:
                                                               0,
                                                           MinorImageVersion:
                                                               0,
                                                           MajorSubsystemVersion:
                                                               0,
                                                           MinorSubsystemVersion:
                                                               0,
                                                           Win32VersionValue:
                                                               0,
                                                           SizeOfImage: 0,
                                                           SizeOfHeaders: 0,
                                                           CheckSum: 0,
                                                           Subsystem: 0,
                                                           DllCharacteristics:
                                                               0,
                                                           SizeOfStackReserve:
                                                               0,
                                                           SizeOfStackCommit:
                                                               0,
                                                           SizeOfHeapReserve:
                                                               0,
                                                           SizeOfHeapCommit:
                                                               0,
                                                           LoaderFlags: 0,
                                                           NumberOfRvaAndSizes:
                                                               0,},},
                   dirs: 0 as *mut directory,
                   name: 0 as *mut libc::c_char,
                   sections: 0 as *mut section,
                   exports: 0 as *mut export,
                   export_count: 0,
                   imports: 0 as *mut import_module,
                   import_count: 0,
                   reloc_base: 0,
                   relocs: 0 as *mut reloc_pe,
                   reloc_count: 0,};
            init
        }; let mut i =  0;
    readpe(offset_pe, &mut pe);
    if mode as libc::c_int == 0x80i32 {
        print_specfile(&mut pe);
        freepe(&mut pe);
        return
    }
    /* objdump always applies the image base to addresses. This makes sense for
     * EXEs, which can always be loaded at their preferred address, but for DLLs
     * it just makes debugging more annoying, since you have to subtract the
     * image base and *then* add the address the DLL was actually loaded at.
     * In theory PE provides us with everything we need to fix up a DLL
     * (relocations etc.) so that we only ever print the *relative* addresses.
     * But we can't do the same for an EXE, and we probably don't want to either.
     * Is the discrepancy going to be confusing? Probably not that much.
     *
     * Anyway, offer the user the option. Default is to enable relative addressing
     * for DLLs but disable it for EXEs. Note that if they manually enable it,
     * we won't be able to fix up everything. Caveat emptor.
     *
     * Internally we want to use relative IPs everywhere possible. The only place
     * that we can't is in arg->value. */
    if pe_rel_addr == -(1i32) {
        pe_rel_addr =
            pe.header.Characteristics as libc::c_int & 0x2000i32
    }
    printf(b"Module type: PE (Portable Executable)\n\x00" as *const u8 as
               *const libc::c_char);
    if !pe.name.is_null() {
        printf(b"Module name: %s\n\x00" as *const u8 as *const libc::c_char,
               pe.name);
    }
    if mode as libc::c_int & 0x1i32 != 0 {
        print_header(&mut pe);
    }
    if mode as libc::c_int & 0x4i32 != 0 {
        putchar('\n' as i32);
        if !pe.exports.is_null() {
            printf(b"Exports:\n\x00" as *const u8 as *const libc::c_char);
            i = 0i32;
            while (i as libc::c_uint) < pe.export_count {
                 let mut address = 
                    (*pe.exports.offset(i as isize)).address;
                if pe_rel_addr == 0 {
                    address =
                        
                        (address as libc::c_ulong).wrapping_add(pe.imagebase) as dword
                }
                printf(b"\t%5d\t%#8x\t%s\x00" as *const u8 as
                           *const libc::c_char,
                       (*pe.exports.offset(i as isize)).ordinal as
                           libc::c_int, address,
                       if !(*pe.exports.offset(i as isize)).name.is_null() {
                           (*pe.exports.offset(i as isize)).name
                       } else {
                           b"<no name>\x00" as *const u8 as
                               *const libc::c_char
                       });
                if (*pe.exports.offset(i as isize)).address >=
                       (*pe.dirs.offset(0isize)).address &&
                       (*pe.exports.offset(i as isize)).address <
                           (*pe.dirs.offset(0isize)).address.wrapping_add((*pe.dirs.offset(0isize)).size)
                   {
                     let mut c =  0;
                    printf(b" -> \x00" as *const u8 as *const libc::c_char);
                    fseek(f,
                          crate::src::pe_section::addr2offset((*pe.exports.offset(i as
                                                              isize)).address,
                                      &mut pe), 0i32);
                    loop  {
                        c = read_byte() as libc::c_char;
                        if !(c != 0) { break ; }
                        putchar(c as libc::c_int);
                    }
                }
                putchar('\n' as i32);
                i += 1
            }
        } else {
            printf(b"No export table\n\x00" as *const u8 as
                       *const libc::c_char);
        }
    }
    if mode as libc::c_int & 0x8i32 != 0 {
        putchar('\n' as i32);
        if !pe.imports.is_null() {
            printf(b"Imported modules:\n\x00" as *const u8 as
                       *const libc::c_char);
            i = 0i32;
            while (i as libc::c_uint) < pe.import_count {
                printf(b"\t%s\n\x00" as *const u8 as *const libc::c_char,
                       (*pe.imports.offset(i as isize)).module);
                i += 1
            }
        } else {
            printf(b"No imported module table\n\x00" as *const u8 as
                       *const libc::c_char);
        }
    }
    if mode as libc::c_int & 0x10i32 != 0 {
        crate::src::pe_section::print_sections(&mut pe);
    }
    freepe(&mut pe);
}
