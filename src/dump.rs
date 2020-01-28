use ::libc;
extern "C" {
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn getopt_long(___argc: libc::c_int, ___argv: *const *mut libc::c_char,
                   __shortopts: *const libc::c_char,
                   __longopts: *const option, __longind: *mut libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    static mut optarg: *mut libc::c_char;
    #[no_mangle]
    static mut optind: libc::c_int;
    #[no_mangle]
    static mut stdout: *mut _IO_FILE;
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fopen(__filename: *const libc::c_char, __modes: *const libc::c_char)
     -> *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn fread(__ptr: *mut libc::c_void, __size: size_t, __n: size_t,
             __stream: *mut FILE) -> size_t;
    #[no_mangle]
    fn fseek(__stream: *mut FILE, __off: libc::c_long, __whence: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn perror(__s: *const libc::c_char);
    /* Whether to print addresses relative to the image base for PE files. */
    #[no_mangle]
    static mut pe_rel_addr: libc::c_int;
    
    
    
}
pub type size_t = libc::c_ulong;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;

#[repr(C)]#[derive(Copy, Clone)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;

#[repr(C)]#[derive(Copy, Clone)]
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

#[repr(C)]#[derive(Copy, Clone)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type FILE = _IO_FILE;
pub type word = uint16_t;
pub type dword = uint32_t;
/* additional options */
pub type C2RustUnnamed = libc::c_uint;
pub const MASM: C2RustUnnamed = 2;
pub const NASM: C2RustUnnamed = 1;
pub const GAS: C2RustUnnamed = 0;
#[no_mangle]
pub static mut f: *mut FILE =  0 as *mut FILE;
#[inline]
unsafe extern "C" fn read_word() -> word {
     let mut w =  0;
    fread(&mut w as *mut word as *mut libc::c_void,
          2u64, 1u64, f);
    return w;
}
#[inline]
unsafe extern "C" fn read_dword() -> dword {
     let mut d =  0;
    fread(&mut d as *mut dword as *mut libc::c_void,
          4u64, 1u64, f);
    return d;
}
#[no_mangle]
pub static mut mode: word = 0;
#[no_mangle]
pub static mut opts: word = 0;
#[no_mangle]
pub static mut asm_syntax: C2RustUnnamed = GAS;
#[no_mangle]
pub static mut resource_filters: *mut *mut libc::c_char =
    
    0 as *mut *mut libc::c_char;
#[no_mangle]
pub static mut resource_filters_count: libc::c_uint = 0;
/*
 * Entry point of the "dump" program
 *
 * Copyright 2017-2019 Zebediah Figura
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
unsafe extern "C" fn dump_file(mut file: *mut libc::c_char) {
    
     let mut offset =  0i64;
    f = fopen(file, b"r\x00" as *const u8 as *const libc::c_char);
    if f.is_null() {
        perror(b"Cannot open %s\x00" as *const u8 as *const libc::c_char);
        return
    }
      let mut magic =   read_word();
    printf(b"File: %s\n\x00" as *const u8 as *const libc::c_char, file);
    if magic as libc::c_int == 0x5a4di32 {
        /* MZ */
        fseek(f, 0x3ci64, 0i32);
        offset = read_dword() as libc::c_long;
        fseek(f, offset, 0i32);
        magic = read_word();
        if magic as libc::c_int == 0x4550i32 {
            crate::src::pe_header::dumppe(offset);
        } else if magic as libc::c_int == 0x454ei32 {
            crate::src::ne_header::dumpne(offset);
        } else { crate::src::mz::dumpmz(); }
    } else {
        fprintf(stderr,
                b"File format not recognized\n\x00" as *const u8 as
                    *const libc::c_char);
    }
    fclose(f);
    fflush(stdout);
}
static mut help_message: [libc::c_char; 1439] =
    [100, 117, 109, 112, 58, 32, 116, 111, 111, 108, 32, 116, 111, 32, 100,
     105, 115, 97, 115, 115, 101, 109, 98, 108, 101, 32, 97, 110, 100, 32,
     112, 114, 105, 110, 116, 32, 105, 110, 102, 111, 114, 109, 97, 116, 105,
     111, 110, 32, 102, 114, 111, 109, 32, 101, 120, 101, 99, 117, 116, 97,
     98, 108, 101, 32, 102, 105, 108, 101, 115, 46, 10, 85, 115, 97, 103, 101,
     58, 32, 100, 117, 109, 112, 32, 91, 111, 112, 116, 105, 111, 110, 115,
     93, 32, 60, 102, 105, 108, 101, 40, 115, 41, 62, 10, 65, 118, 97, 105,
     108, 97, 98, 108, 101, 32, 111, 112, 116, 105, 111, 110, 115, 58, 10, 9,
     45, 97, 44, 32, 45, 45, 114, 101, 115, 111, 117, 114, 99, 101, 91, 61,
     102, 105, 108, 116, 101, 114, 93, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
     32, 32, 32, 32, 80, 114, 105, 110, 116, 32, 101, 109, 98, 101, 100, 100,
     101, 100, 32, 114, 101, 115, 111, 117, 114, 99, 101, 115, 46, 10, 9, 45,
     99, 44, 32, 45, 45, 99, 111, 109, 112, 105, 108, 97, 98, 108, 101, 32,
     32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
     32, 32, 80, 114, 111, 100, 117, 99, 101, 32, 111, 117, 116, 112, 117,
     116, 32, 116, 104, 97, 116, 32, 99, 97, 110, 32, 98, 101, 32, 99, 111,
     109, 112, 105, 108, 101, 100, 46, 10, 9, 45, 67, 44, 32, 45, 45, 100,
     101, 109, 97, 110, 103, 108, 101, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
     32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 68, 101, 109, 97,
     110, 103, 108, 101, 32, 67, 43, 43, 32, 102, 117, 110, 99, 116, 105, 111,
     110, 32, 110, 97, 109, 101, 115, 46, 10, 9, 45, 100, 44, 32, 45, 45, 100,
     105, 115, 97, 115, 115, 101, 109, 98, 108, 101, 32, 32, 32, 32, 32, 32,
     32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 80, 114, 105,
     110, 116, 32, 100, 105, 115, 97, 115, 115, 101, 109, 98, 108, 101, 100,
     32, 109, 97, 99, 104, 105, 110, 101, 32, 99, 111, 100, 101, 46, 10, 9,
     45, 101, 44, 32, 45, 45, 101, 120, 112, 111, 114, 116, 115, 32, 32, 32,
     32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
     32, 32, 32, 80, 114, 105, 110, 116, 32, 101, 120, 112, 111, 114, 116,
     101, 100, 32, 102, 117, 110, 99, 116, 105, 111, 110, 115, 46, 10, 9, 45,
     102, 44, 32, 45, 45, 102, 105, 108, 101, 45, 104, 101, 97, 100, 101, 114,
     115, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
     32, 32, 80, 114, 105, 110, 116, 32, 99, 111, 110, 116, 101, 110, 116,
     115, 32, 111, 102, 32, 116, 104, 101, 32, 102, 105, 108, 101, 32, 104,
     101, 97, 100, 101, 114, 46, 10, 9, 45, 104, 44, 32, 45, 45, 104, 101,
     108, 112, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
     32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 68, 105, 115, 112, 108, 97,
     121, 32, 116, 104, 105, 115, 32, 104, 101, 108, 112, 32, 109, 101, 115,
     115, 97, 103, 101, 46, 10, 9, 45, 105, 44, 32, 45, 45, 105, 109, 112,
     111, 114, 116, 115, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
     32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 80, 114, 105, 110, 116, 32,
     105, 109, 112, 111, 114, 116, 101, 100, 32, 109, 111, 100, 117, 108, 101,
     115, 46, 10, 9, 45, 77, 44, 32, 45, 45, 100, 105, 115, 97, 115, 115, 101,
     109, 98, 108, 101, 114, 45, 111, 112, 116, 105, 111, 110, 115, 61, 91,
     46, 46, 46, 93, 32, 32, 32, 32, 32, 69, 120, 116, 101, 110, 100, 101,
     100, 32, 111, 112, 116, 105, 111, 110, 115, 32, 102, 111, 114, 32, 100,
     105, 115, 97, 115, 115, 101, 109, 98, 108, 121, 46, 10, 9, 9, 97, 116,
     116, 32, 32, 32, 32, 32, 32, 32, 32, 65, 108, 105, 97, 115, 32, 102, 111,
     114, 32, 96, 103, 97, 115, 39, 46, 10, 9, 9, 103, 97, 115, 32, 32, 32,
     32, 32, 32, 32, 32, 85, 115, 101, 32, 71, 65, 83, 32, 115, 121, 110, 116,
     97, 120, 32, 102, 111, 114, 32, 100, 105, 115, 97, 115, 115, 101, 109,
     98, 108, 121, 46, 10, 9, 9, 105, 110, 116, 101, 108, 32, 32, 32, 32, 32,
     32, 65, 108, 105, 97, 115, 32, 102, 111, 114, 32, 96, 109, 97, 115, 109,
     39, 46, 10, 9, 9, 109, 97, 115, 109, 32, 32, 32, 32, 32, 32, 32, 85, 115,
     101, 32, 77, 65, 83, 77, 32, 115, 121, 110, 116, 97, 120, 32, 102, 111,
     114, 32, 100, 105, 115, 97, 115, 115, 101, 109, 98, 108, 121, 46, 10, 9,
     9, 110, 97, 115, 109, 32, 32, 32, 32, 32, 32, 32, 85, 115, 101, 32, 78,
     65, 83, 77, 32, 115, 121, 110, 116, 97, 120, 32, 102, 111, 114, 32, 100,
     105, 115, 97, 115, 115, 101, 109, 98, 108, 121, 46, 10, 9, 45, 111, 44,
     32, 45, 45, 115, 112, 101, 99, 102, 105, 108, 101, 32, 32, 32, 32, 32,
     32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
     67, 114, 101, 97, 116, 101, 32, 97, 32, 115, 112, 101, 99, 102, 105, 108,
     101, 32, 102, 114, 111, 109, 32, 101, 120, 112, 111, 114, 116, 115, 46,
     10, 9, 45, 115, 44, 32, 45, 45, 102, 117, 108, 108, 45, 99, 111, 110,
     116, 101, 110, 116, 115, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
     32, 32, 32, 32, 32, 32, 68, 105, 115, 112, 108, 97, 121, 32, 102, 117,
     108, 108, 32, 99, 111, 110, 116, 101, 110, 116, 115, 32, 111, 102, 32,
     97, 108, 108, 32, 115, 101, 99, 116, 105, 111, 110, 115, 46, 10, 9, 45,
     118, 44, 32, 45, 45, 118, 101, 114, 115, 105, 111, 110, 32, 32, 32, 32,
     32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
     32, 32, 80, 114, 105, 110, 116, 32, 116, 104, 101, 32, 118, 101, 114,
     115, 105, 111, 110, 32, 110, 117, 109, 98, 101, 114, 32, 111, 102, 32,
     115, 101, 109, 98, 108, 97, 110, 99, 101, 46, 10, 9, 45, 120, 44, 32, 45,
     45, 97, 108, 108, 45, 104, 101, 97, 100, 101, 114, 115, 32, 32, 32, 32,
     32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 80, 114,
     105, 110, 116, 32, 97, 108, 108, 32, 104, 101, 97, 100, 101, 114, 115,
     46, 10, 9, 45, 45, 110, 111, 45, 115, 104, 111, 119, 45, 97, 100, 100,
     114, 101, 115, 115, 101, 115, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
     32, 32, 32, 32, 32, 32, 32, 68, 111, 110, 39, 116, 32, 112, 114, 105,
     110, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32,
     97, 100, 100, 114, 101, 115, 115, 101, 115, 46, 10, 9, 45, 45, 110, 111,
     45, 115, 104, 111, 119, 45, 114, 97, 119, 45, 105, 110, 115, 110, 32, 32,
     32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 68,
     111, 110, 39, 116, 32, 112, 114, 105, 110, 116, 32, 114, 97, 119, 32,
     105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 104, 101, 120,
     32, 99, 111, 100, 101, 46, 10, 9, 45, 45, 112, 101, 45, 114, 101, 108,
     45, 97, 100, 100, 114, 61, 91, 121, 47, 110, 93, 32, 32, 32, 32, 32, 32,
     32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 85, 115, 101, 32, 114,
     101, 108, 97, 116, 105, 118, 101, 32, 97, 100, 100, 114, 101, 115, 115,
     101, 115, 32, 102, 111, 114, 32, 80, 69, 32, 102, 105, 108, 101, 115, 46,
     10, 0];
static mut long_options: [option; 18] =
    [{
         let mut init =
             option{name: b"resource\x00" as *const u8 as *const libc::c_char,
                    has_arg: 2i32,
                    flag:  0 as *mut libc::c_int,
                    val: 'a' as i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"compilable\x00" as *const u8 as *const libc::c_char,
                    has_arg: 0i32,
                    flag:  0 as *mut libc::c_int,
                    val: 'c' as i32,};
         init
     },
     {
         let mut init =
             option{name: b"demangle\x00" as *const u8 as *const libc::c_char,
                    has_arg: 0i32,
                    flag:  0 as *mut libc::c_int,
                    val: 'C' as i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"disassemble\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0i32,
                    flag:  0 as *mut libc::c_int,
                    val: 'd' as i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"disassemble-all\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0i32,
                    flag:  0 as *mut libc::c_int,
                    val: 'D' as i32,};
         init
     },
     {
         let mut init =
             option{name: b"exports\x00" as *const u8 as *const libc::c_char,
                    has_arg: 0i32,
                    flag:  0 as *mut libc::c_int,
                    val: 'e' as i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"file-headers\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0i32,
                    flag:  0 as *mut libc::c_int,
                    val: 'f' as i32,};
         init
     },
     {
         let mut init =
             option{name: b"help\x00" as *const u8 as *const libc::c_char,
                    has_arg: 0i32,
                    flag:  0 as *mut libc::c_int,
                    val: 'h' as i32,};
         init
     },
     {
         let mut init =
             option{name: b"imports\x00" as *const u8 as *const libc::c_char,
                    has_arg: 0i32,
                    flag:  0 as *mut libc::c_int,
                    val: 'i' as i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"disassembler-options\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1i32,
                    flag:  0 as *mut libc::c_int,
                    val: 'M' as i32,};
         init
     },
     {
         let mut init =
             option{name: b"specfile\x00" as *const u8 as *const libc::c_char,
                    has_arg: 0i32,
                    flag:  0 as *mut libc::c_int,
                    val: 'o' as i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"full-contents\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0i32,
                    flag:  0 as *mut libc::c_int,
                    val: 's' as i32,};
         init
     },
     {
         let mut init =
             option{name: b"version\x00" as *const u8 as *const libc::c_char,
                    has_arg: 0i32,
                    flag:  0 as *mut libc::c_int,
                    val: 'v' as i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"all-headers\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0i32,
                    flag:  0 as *mut libc::c_int,
                    val: 'x' as i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"no-show-raw-insn\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0i32,
                    flag:  0 as *mut libc::c_int,
                    val: 0x4i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"no-prefix-addresses\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 0i32,
                    flag:  0 as *mut libc::c_int,
                    val: 0x8i32,};
         init
     },
     {
         let mut init =
             option{name:
                        b"pe-rel-addr\x00" as *const u8 as
                            *const libc::c_char,
                    has_arg: 1i32,
                    flag:  0 as *mut libc::c_int,
                    val: 0x80i32,};
         init
     },
     {
         let mut init =
             option{name: 0 as *const libc::c_char,
                    has_arg: 0,
                    flag:  0 as *mut libc::c_int,
                    val: 0,};
         init
     }];
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
     let mut opt =  0;
    mode = 0u16;
    opts = 0u16;
    asm_syntax = NASM;
    loop  {
        opt =
            getopt_long(argc, argv as *const *mut libc::c_char,
                        b"a::cCdDefhiM:osvx\x00" as *const u8 as
                            *const libc::c_char, long_options.as_ptr(),
                        0 as *mut libc::c_int);
        if !(opt >= 0i32) { break ; }
        match opt {
            4 => { opts = (opts as libc::c_int | 0x4i32) as word }
            8 => { opts = (opts as libc::c_int | 0x8i32) as word }
            97 => {
                
                
                 let mut ret =  0; let mut type_0 =  [0; 256]; let mut i =  0;
                mode = (mode as libc::c_int | 0x2i32) as word;
                if !optarg.is_null() {
                     let mut p =  optarg;
                    while *p as libc::c_int == ' ' as i32 ||
                              *p as libc::c_int == '=' as i32 {
                        p = p.offset(1)
                    }
                    resource_filters =
                        realloc(resource_filters as *mut libc::c_void,
                                (resource_filters_count.wrapping_add(1u32)
                                     as
                                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                                     as
                                                                     libc::c_ulong))
                            as *mut *mut libc::c_char;
                    let fresh0 = resource_filters_count;
                    resource_filters_count =
                        resource_filters_count.wrapping_add(1);
                    let ref mut fresh1 =
                        *resource_filters.offset(fresh0 as isize);
                    *fresh1 = strdup(p)
                }
            }
            99 => {
                /* compilable */
                opts =
                    (opts as libc::c_int |
                         (0x10i32 | 0x8i32 |
                              0x4i32)) as word
            }
            67 => {
                /* demangle */
                opts = (opts as libc::c_int | 0x2i32) as word
            }
            100 => {
                /* disassemble only */
                mode = (mode as libc::c_int | 0x10i32) as word
            }
            68 => {
                /* disassemble all */
                opts = (opts as libc::c_int | 0x1i32) as word
            }
            101 => {
                /* exports */
                mode = (mode as libc::c_int | 0x4i32) as word
            }
            102 => {
                /* dump header only */
                mode = (mode as libc::c_int | 0x1i32) as word
            }
            104 => {
                /* help */
                printf(help_message.as_ptr());
                return 0i32
            }
            105 => {
                /* imports */
                /* FIXME: should also list imported functions (?) */
                mode = (mode as libc::c_int | 0x8i32) as word
            }
            77 => {
                /* additional options */
                if strcmp(optarg,
                          b"att\x00" as *const u8 as *const libc::c_char) == 0
                       ||
                       strcmp(optarg,
                              b"gas\x00" as *const u8 as *const libc::c_char)
                           == 0 {
                    asm_syntax = GAS
                } else if strcmp(optarg,
                                 b"intel\x00" as *const u8 as
                                     *const libc::c_char) == 0 ||
                              strcmp(optarg,
                                     b"masm\x00" as *const u8 as
                                         *const libc::c_char) == 0 {
                    asm_syntax = MASM
                } else if strcmp(optarg,
                                 b"nasm\x00" as *const u8 as
                                     *const libc::c_char) == 0 {
                    asm_syntax = NASM
                } else {
                    fprintf(stderr,
                            b"Unrecognized disassembly option `%s\'.\n\x00" as
                                *const u8 as *const libc::c_char, optarg);
                    return 1i32
                }
            }
            111 => {
                /* make a specfile */
                mode = 0x80u16
            }
            118 => {
                /* version */
                printf(b"semblance version 0.2\n\x00" as *const u8 as
                           *const libc::c_char);
                return 0i32
            }
            115 => {
                /* full contents */
                opts = (opts as libc::c_int | 0x20i32) as word
            }
            120 => {
                /* all headers */
                mode =
                    (mode as libc::c_int |
                         (0x1i32 | 0x4i32 |
                              0x8i32)) as word
            }
            128 => {
                if *optarg.offset(0isize) as libc::c_int ==
                       '1' as i32 ||
                       *optarg.offset(0isize) as
                           libc::c_int == 'y' as i32 ||
                       *optarg.offset(0isize) as
                           libc::c_int == 'Y' as i32 {
                    pe_rel_addr = 1i32
                } else if *optarg.offset(0isize) as
                              libc::c_int == '0' as i32 ||
                              *optarg.offset(0isize) as
                                  libc::c_int == 'n' as i32 ||
                              *optarg.offset(0isize) as
                                  libc::c_int == 'N' as i32 {
                    pe_rel_addr = 0i32
                } else {
                    fprintf(stderr,
                            b"Unrecognized --pe-rel-addr option `%s\'.\n\x00"
                                as *const u8 as *const libc::c_char, optarg);
                    return 1i32
                }
            }
            _ => {
                fprintf(stderr,
                        b"Usage: dumpne [options] <file>\n\x00" as *const u8
                            as *const libc::c_char);
                return 1i32
            }
        }
    }
    if mode as libc::c_int == 0i32 {
        mode = !(0i32) as word
    }
    if optind == argc { printf(help_message.as_ptr()); }
    while optind < argc {
        let fresh2 = optind;
        optind = optind + 1;
        dump_file(*argv.offset(fresh2 as isize));
        if optind < argc {
            printf(b"\n\n\x00" as *const u8 as *const libc::c_char);
        }
    }
    return 0i32;
}
#[main]
pub fn main() {
     let mut args =  Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as libc::c_int,
                                    
                                    args.as_mut_ptr()))
    }
}
