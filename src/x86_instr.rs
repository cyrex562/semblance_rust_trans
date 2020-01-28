use ::libc;
use ::c2rust_bitfields;
extern "C" {
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
pub type byte = uint8_t;
pub type word = uint16_t;
pub type dword = uint32_t;
pub type qword = uint64_t;
/* additional options */
pub type C2RustUnnamed = libc::c_uint;
pub const MASM: C2RustUnnamed = 2;
pub const NASM: C2RustUnnamed = 1;
pub const GAS: C2RustUnnamed = 0;
pub type argtype = libc::c_uint;
pub const STX: argtype = 53;
pub const ST: argtype = 52;
pub const TR32: argtype = 51;
pub const DR32: argtype = 50;
pub const CR32: argtype = 49;
pub const REG32: argtype = 48;
pub const SEG16: argtype = 47;
pub const XMM: argtype = 46;
pub const MMX: argtype = 45;
pub const REG: argtype = 44;
pub const XMMONLY: argtype = 43;
pub const MMXONLY: argtype = 42;
pub const REGONLY: argtype = 41;
pub const MEM: argtype = 40;
pub const XM: argtype = 39;
pub const MM: argtype = 38;
pub const RM: argtype = 37;
pub const ESDI: argtype = 36;
pub const DSSI: argtype = 35;
pub const DSBX: argtype = 34;
pub const MOFFS16: argtype = 33;
pub const PTR32: argtype = 32;
pub const REL16: argtype = 31;
pub const REL8: argtype = 30;
pub const IMM: argtype = 29;
pub const IMM16: argtype = 28;
pub const IMM8: argtype = 27;
pub const DXS: argtype = 26;
pub const AXS: argtype = 25;
pub const ALS: argtype = 24;
pub const GS: argtype = 23;
pub const FS: argtype = 22;
pub const DS: argtype = 21;
pub const SS: argtype = 20;
pub const CS: argtype = 19;
pub const ES: argtype = 18;
pub const DI: argtype = 17;
pub const SI: argtype = 16;
pub const BP: argtype = 15;
pub const SP: argtype = 14;
pub const BX: argtype = 13;
pub const DX: argtype = 12;
pub const CX: argtype = 11;
pub const AX: argtype = 10;
pub const BH: argtype = 9;
pub const DH: argtype = 8;
pub const CH: argtype = 7;
pub const AH: argtype = 6;
pub const BL: argtype = 5;
pub const DL: argtype = 4;
pub const CL: argtype = 3;
pub const AL: argtype = 2;
pub const ONE: argtype = 1;
pub const NONE: argtype = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct op {
    pub opcode: word,
    pub subcode: byte,
    pub size: libc::c_char,
    pub name: [libc::c_char; 16],
    pub arg0: argtype,
    pub arg1: argtype,
    pub flags: dword,
}
pub type disptype = libc::c_uint;
pub const DISP_REG: disptype = 3;
pub const DISP_16: disptype = 2;
pub const DISP_8: disptype = 1;
pub const DISP_NONE: disptype = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arg {
    pub string: [libc::c_char; 32],
    pub ip: dword,
    pub value: qword,
    pub type_0: argtype,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct instr {
    pub prefix: word,
    pub op: op,
    pub args: [arg; 3],
    pub addrsize: byte,
    pub modrm_disp: disptype,
    pub modrm_reg: libc::c_char,
    pub sib_scale: byte,
    pub sib_index: libc::c_char,
    #[bitfield(name = "usedmem", ty = "libc::c_int", bits = "0..=0")]
    #[bitfield(name = "vex", ty = "libc::c_int", bits = "1..=1")]
    #[bitfield(name = "vex_reg", ty = "libc::c_uint", bits = "2..=4")]
    #[bitfield(name = "vex_256", ty = "libc::c_int", bits = "5..=5")]
    pub usedmem_vex_vex_reg_vex_256: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}
#[no_mangle]
pub static mut f: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub static mut mode: word = 0;
#[no_mangle]
pub static mut opts: word = 0;
#[no_mangle]
pub static mut asm_syntax: C2RustUnnamed = GAS;
#[no_mangle]
pub static mut resource_filters: *mut *mut libc::c_char =
    0 as *const *mut libc::c_char as *mut *mut libc::c_char;
#[no_mangle]
pub static mut resource_filters_count: libc::c_uint = 0;
/*
 * Functions to parse and print x86 instructions
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
/* this is easier than doing bitfields */
static mut instructions: [op; 256] =
    [{
         let mut init =
             op{opcode: 0 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [97, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: REG,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x1 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [97, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: REG,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x2 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [97, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x3 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [97, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x4 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [97, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AL,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x5 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [97, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AX,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x6 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [112, 117, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: ES,
                arg1: NONE,
                flags: 0x100 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x7 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [112, 111, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: ES,
                arg1: NONE,
                flags: 0x100 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x8 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: REG,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x9 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: REG,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xa as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xb as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xc as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AL,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AX,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xe as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [112, 117, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: CS,
                arg1: NONE,
                flags: 0x100 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xf as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x10 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [97, 100, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: REG,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x11 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [97, 100, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: REG,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x12 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [97, 100, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x13 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [97, 100, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x14 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [97, 100, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AL,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x15 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [97, 100, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AX,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x16 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [112, 117, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: SS,
                arg1: NONE,
                flags: 0x100 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x17 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [112, 111, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: SS,
                arg1: NONE,
                flags: 0x100 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x18 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [115, 98, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: REG,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x19 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [115, 98, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: REG,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x1a as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [115, 98, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x1b as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [115, 98, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x1c as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [115, 98, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AL,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x1d as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [115, 98, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AX,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x1e as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [112, 117, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: DS,
                arg1: NONE,
                flags: 0x100 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x2f as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [112, 111, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: DS,
                arg1: NONE,
                flags: 0x100 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x20 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [97, 110, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: REG,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x21 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [97, 110, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: REG,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x22 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [97, 110, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x23 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [97, 110, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x24 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [97, 110, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AL,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x25 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [97, 110, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AX,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x26 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [101, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x27 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [100, 97, 97, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x28 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [115, 117, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: REG,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x29 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [115, 117, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: REG,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x2a as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [115, 117, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x2b as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [115, 117, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x2c as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [115, 117, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AL,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x2d as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [115, 117, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AX,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x2e as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [99, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x2f as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [100, 97, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x30 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [120, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: REG,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x31 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [120, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: REG,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x32 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [120, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x33 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [120, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x34 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [120, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AL,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x35 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [120, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AX,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x36 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [115, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x37 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [97, 97, 97, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [99, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: REG,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x39 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [99, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: REG,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x3a as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [99, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x3b as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [99, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x3c as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [99, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AL,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x3d as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [99, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AX,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x3e as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [100, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x3f as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [97, 97, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x40 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [105, 110, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AX,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x41 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [105, 110, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: CX,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x42 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [105, 110, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: DX,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x43 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [105, 110, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: BX,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x44 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [105, 110, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: SP,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x45 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [105, 110, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: BP,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x46 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [105, 110, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: SI,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x47 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [105, 110, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: DI,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x48 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [100, 101, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AX,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x49 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [100, 101, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: CX,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x4a as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [100, 101, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: DX,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x4b as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [100, 101, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: BX,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x4c as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [100, 101, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: SP,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x4d as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [100, 101, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: BP,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x4e as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [100, 101, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: SI,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x4f as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [100, 101, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: DI,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x50 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [112, 117, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AX,
                arg1: NONE,
                flags: 0x100 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x51 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [112, 117, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: CX,
                arg1: NONE,
                flags: 0x100 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x52 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [112, 117, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: DX,
                arg1: NONE,
                flags: 0x100 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x53 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [112, 117, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: BX,
                arg1: NONE,
                flags: 0x100 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x54 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [112, 117, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: SP,
                arg1: NONE,
                flags: 0x100 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x55 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [112, 117, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: BP,
                arg1: NONE,
                flags: 0x100 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x56 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [112, 117, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: SI,
                arg1: NONE,
                flags: 0x100 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x57 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [112, 117, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: DI,
                arg1: NONE,
                flags: 0x100 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x58 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [112, 111, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AX,
                arg1: NONE,
                flags: 0x100 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x59 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [112, 111, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: CX,
                arg1: NONE,
                flags: 0x100 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x5a as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [112, 111, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: DX,
                arg1: NONE,
                flags: 0x100 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x5b as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [112, 111, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: BX,
                arg1: NONE,
                flags: 0x100 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x5c as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [112, 111, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: SP,
                arg1: NONE,
                flags: 0x100 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x5d as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [112, 111, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: BP,
                arg1: NONE,
                flags: 0x100 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x5e as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [112, 111, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: SI,
                arg1: NONE,
                flags: 0x100 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x5f as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [112, 111, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: DI,
                arg1: NONE,
                flags: 0x100 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x60 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [112, 117, 115, 104, 97, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0x100 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x61 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [112, 111, 112, 97, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0x100 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x62 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [98, 111, 117, 110, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: MEM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x63 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 16 as libc::c_int as libc::c_char,
                name: [97, 114, 112, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: REG,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x64 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [102, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x65 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [103, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x66 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [100, 97, 116, 97, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x67 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [97, 100, 100, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x68 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [112, 117, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: IMM,
                arg1: NONE,
                flags: 0x100 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x69 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [105, 109, 117, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: RM,
                flags: 0x1 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x6a as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [112, 117, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: IMM8,
                arg1: NONE,
                flags: 0x100 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x6b as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [105, 109, 117, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: RM,
                flags: 0x2 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x6c as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [105, 110, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: ESDI,
                arg1: DXS,
                flags:
                    (0x200 as libc::c_int | 0x20 as libc::c_int) as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x6d as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [105, 110, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: ESDI,
                arg1: DXS,
                flags:
                    (0x200 as libc::c_int | 0x20 as libc::c_int) as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x6e as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name:
                    [111, 117, 116, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: DXS,
                arg1: DSSI,
                flags:
                    (0x200 as libc::c_int | 0x20 as libc::c_int) as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x6f as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [111, 117, 116, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: DXS,
                arg1: DSSI,
                flags:
                    (0x200 as libc::c_int | 0x20 as libc::c_int) as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x70 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [106, 111, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL8,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x71 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [106, 110, 111, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL8,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x72 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [106, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL8,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x73 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [106, 97, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL8,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x74 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [106, 122, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL8,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x75 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [106, 110, 122, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL8,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x76 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [106, 98, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL8,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x77 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [106, 97, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL8,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x78 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [106, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL8,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x79 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [106, 110, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL8,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x7a as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [106, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL8,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x7b as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [106, 110, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL8,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x7c as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [106, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL8,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x7d as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [106, 103, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL8,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x7e as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [106, 108, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL8,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x7f as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [106, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL8,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x80 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x81 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x82 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x83 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x84 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name:
                    [116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: REG,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x85 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: REG,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x86 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [120, 99, 104, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: RM,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x87 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [120, 99, 104, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: RM,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x88 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: REG,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x89 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: REG,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x8a as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x8b as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x8c as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: SEG16,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x8d as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [108, 101, 97, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: MEM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x8e as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: SEG16,
                arg1: RM,
                flags: 0x40 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x8f as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x90 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [110, 111, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0x20 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x91 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [120, 99, 104, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AX,
                arg1: CX,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x92 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [120, 99, 104, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AX,
                arg1: DX,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x93 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [120, 99, 104, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AX,
                arg1: BX,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x94 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [120, 99, 104, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AX,
                arg1: SP,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x95 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [120, 99, 104, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AX,
                arg1: BP,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x96 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [120, 99, 104, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AX,
                arg1: SI,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x97 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [120, 99, 104, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AX,
                arg1: DI,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x98 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [99, 98, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x99 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [99, 119, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x9a as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [99, 97, 108, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: PTR32,
                arg1: NONE,
                flags: 0x400 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x9b as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [119, 97, 105, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x9c as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [112, 117, 115, 104, 102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: NONE,
                arg1: NONE,
                flags: 0x100 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x9d as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [112, 111, 112, 102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0x100 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x9e as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [115, 97, 104, 102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x9f as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [108, 97, 104, 102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xa0 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AL,
                arg1: MOFFS16,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xa1 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AX,
                arg1: MOFFS16,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xa2 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MOFFS16,
                arg1: AL,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xa3 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MOFFS16,
                arg1: AX,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xa4 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name:
                    [109, 111, 118, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: DSSI,
                arg1: ESDI,
                flags:
                    (0x200 as libc::c_int | 0x20 as libc::c_int) as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xa5 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [109, 111, 118, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: DSSI,
                arg1: ESDI,
                flags:
                    (0x200 as libc::c_int | 0x20 as libc::c_int) as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xa6 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [99, 109, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: DSSI,
                arg1: ESDI,
                flags:
                    (0x200 as libc::c_int | 0x10 as libc::c_int |
                         0x20 as libc::c_int) as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xa7 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [99, 109, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: DSSI,
                arg1: ESDI,
                flags:
                    (0x200 as libc::c_int | 0x10 as libc::c_int |
                         0x20 as libc::c_int) as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xa8 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name:
                    [116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AL,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xa9 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AX,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xaa as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name:
                    [115, 116, 111, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: ESDI,
                arg1: ALS,
                flags:
                    (0x200 as libc::c_int | 0x20 as libc::c_int) as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xab as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [115, 116, 111, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: ESDI,
                arg1: AXS,
                flags:
                    (0x200 as libc::c_int | 0x20 as libc::c_int) as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xac as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name:
                    [108, 111, 100, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: ALS,
                arg1: DSSI,
                flags:
                    (0x200 as libc::c_int | 0x20 as libc::c_int) as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xad as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [108, 111, 100, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AXS,
                arg1: DSSI,
                flags:
                    (0x200 as libc::c_int | 0x20 as libc::c_int) as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xae as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [115, 99, 97, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: ALS,
                arg1: ESDI,
                flags:
                    (0x200 as libc::c_int | 0x10 as libc::c_int |
                         0x20 as libc::c_int) as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xaf as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [115, 99, 97, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AXS,
                arg1: ESDI,
                flags:
                    (0x200 as libc::c_int | 0x10 as libc::c_int |
                         0x20 as libc::c_int) as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xb0 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AL,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xb1 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: CL,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xb2 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: DL,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xb3 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: BL,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xb4 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AH,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xb5 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: CH,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xb6 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: DH,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xb7 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: BH,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xb8 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AX,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xb9 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: CX,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xba as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: DX,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xbb as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: BX,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xbc as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: SP,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xbd as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: BP,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xbe as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: SI,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xbf as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: DI,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xc0 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xc1 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xc2 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [114, 101, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: IMM16,
                arg1: NONE,
                flags: 0x4000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xc3 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [114, 101, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags:
                    (0x4000 as libc::c_int | 0x20 as libc::c_int |
                         0x10 as libc::c_int) as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xc4 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [108, 101, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: MEM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xc5 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [108, 100, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: MEM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xc6 as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xc7 as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xc8 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [101, 110, 116, 101, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: IMM16,
                arg1: IMM8,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xc9 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [108, 101, 97, 118, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xca as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [114, 101, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: IMM16,
                arg1: NONE,
                flags:
                    (0x4000 as libc::c_int | 0x400 as libc::c_int) as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xcb as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [114, 101, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags:
                    (0x4000 as libc::c_int | 0x400 as libc::c_int) as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xcc as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [105, 110, 116, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0x4000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xcd as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [105, 110, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: IMM8,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xce as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [105, 110, 116, 111, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xcf as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [105, 114, 101, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0x4000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd0 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd1 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd2 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd3 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd4 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [97, 109, 120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: IMM8,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd5 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [97, 100, 120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: IMM8,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd6 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd7 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [120, 108, 97, 116, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: DSBX,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd8 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd9 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xda as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdb as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdc as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdd as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xde as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdf as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xe0 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [108, 111, 111, 112, 110, 122, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: REL8,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xe1 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [108, 111, 111, 112, 122, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: REL8,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xe2 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [108, 111, 111, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL8,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xe3 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [106, 99, 120, 122, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL8,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xe4 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [105, 110, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AL,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xe5 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [105, 110, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AX,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xe6 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [111, 117, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: IMM,
                arg1: AL,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xe7 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [111, 117, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: IMM,
                arg1: AX,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xe8 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [99, 97, 108, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL16,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xe9 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [106, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL16,
                arg1: NONE,
                flags:
                    (0x8000 as libc::c_int | 0x4000 as libc::c_int) as
                        dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xea as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [106, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: PTR32,
                arg1: NONE,
                flags:
                    (0x400 as libc::c_int | 0x4000 as libc::c_int) as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xeb as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [106, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL8,
                arg1: NONE,
                flags:
                    (0x8000 as libc::c_int | 0x4000 as libc::c_int) as
                        dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xec as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [105, 110, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AL,
                arg1: DXS,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xed as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [105, 110, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AX,
                arg1: DXS,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xee as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [111, 117, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: DXS,
                arg1: AL,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xef as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [111, 117, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: DXS,
                arg1: AX,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xf0 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [108, 111, 99, 107, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xf1 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xf2 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [114, 101, 112, 110, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xf3 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [114, 101, 112, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xf4 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [104, 108, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xf5 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [99, 109, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xf6 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xf7 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xf8 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [99, 108, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xf9 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [115, 116, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xfa as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [99, 108, 105, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xfb as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [115, 116, 105, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xfc as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [99, 108, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xfd as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [115, 116, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xfe as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xff as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     }];
static mut instructions64: [op; 256] =
    [{
         let mut init =
             op{opcode: 0 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [97, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: REG,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x1 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [97, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: REG,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x2 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [97, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x3 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [97, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x4 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [97, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AL,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x5 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [97, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AX,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x6 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x7 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x8 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: REG,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x9 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: REG,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xa as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xb as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xc as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AL,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AX,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xe as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xf as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x10 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [97, 100, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: REG,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x11 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [97, 100, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: REG,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x12 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [97, 100, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x13 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [97, 100, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x14 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [97, 100, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AL,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x15 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [97, 100, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AX,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x16 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x17 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x18 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [115, 98, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: REG,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x19 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [115, 98, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: REG,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x1a as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [115, 98, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x1b as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [115, 98, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x1c as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [115, 98, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AL,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x1d as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [115, 98, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AX,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x1e as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x1f as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x20 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [97, 110, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: REG,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x21 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [97, 110, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: REG,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x22 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [97, 110, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x23 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [97, 110, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x24 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [97, 110, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AL,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x25 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [97, 110, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AX,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x26 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x27 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x28 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [115, 117, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: REG,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x29 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [115, 117, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: REG,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x2a as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [115, 117, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x2b as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [115, 117, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x2c as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [115, 117, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AL,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x2d as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [115, 117, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AX,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x2e as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x2f as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x30 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [120, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: REG,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x31 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [120, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: REG,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x32 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [120, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x33 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [120, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x34 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [120, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AL,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x35 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [120, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AX,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x36 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x37 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [99, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: REG,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x39 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [99, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: REG,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x3a as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [99, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x3b as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [99, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x3c as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [99, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AL,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x3d as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [99, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AX,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x3e as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x3f as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x40 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [114, 101, 120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x41 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [114, 101, 120, 46, 66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x42 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [114, 101, 120, 46, 88, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x43 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [114, 101, 120, 46, 88, 66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x44 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [114, 101, 120, 46, 82, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x45 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [114, 101, 120, 46, 82, 66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x46 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [114, 101, 120, 46, 82, 88, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x47 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [114, 101, 120, 46, 82, 88, 66, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x48 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [114, 101, 120, 46, 87, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x49 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [114, 101, 120, 46, 87, 66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x4a as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [114, 101, 120, 46, 87, 88, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x4b as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [114, 101, 120, 46, 87, 88, 66, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x4c as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [114, 101, 120, 46, 87, 82, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x4d as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [114, 101, 120, 46, 87, 82, 66, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x4e as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [114, 101, 120, 46, 87, 82, 88, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x4f as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [114, 101, 120, 46, 87, 82, 88, 66, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x50 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [112, 117, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AX,
                arg1: NONE,
                flags: 0x100 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x51 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [112, 117, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: CX,
                arg1: NONE,
                flags: 0x100 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x52 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [112, 117, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: DX,
                arg1: NONE,
                flags: 0x100 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x53 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [112, 117, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: BX,
                arg1: NONE,
                flags: 0x100 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x54 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [112, 117, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: SP,
                arg1: NONE,
                flags: 0x100 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x55 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [112, 117, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: BP,
                arg1: NONE,
                flags: 0x100 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x56 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [112, 117, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: SI,
                arg1: NONE,
                flags: 0x100 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x57 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [112, 117, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: DI,
                arg1: NONE,
                flags: 0x100 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x58 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [112, 111, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AX,
                arg1: NONE,
                flags: 0x100 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x59 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [112, 111, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: CX,
                arg1: NONE,
                flags: 0x100 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x5a as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [112, 111, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: DX,
                arg1: NONE,
                flags: 0x100 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x5b as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [112, 111, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: BX,
                arg1: NONE,
                flags: 0x100 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x5c as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [112, 111, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: SP,
                arg1: NONE,
                flags: 0x100 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x5d as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [112, 111, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: BP,
                arg1: NONE,
                flags: 0x100 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x5e as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [112, 111, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: SI,
                arg1: NONE,
                flags: 0x100 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x5f as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [112, 111, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: DI,
                arg1: NONE,
                flags: 0x100 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x60 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x61 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x62 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x63 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [109, 111, 118, 115, 120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x64 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [102, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x65 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [103, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x66 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [100, 97, 116, 97, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x67 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [97, 100, 100, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x68 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [112, 117, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: IMM,
                arg1: NONE,
                flags: 0x100 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x69 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [105, 109, 117, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: RM,
                flags: 0x1 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x6a as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [112, 117, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: IMM8,
                arg1: NONE,
                flags: 0x100 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x6b as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [105, 109, 117, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: RM,
                flags: 0x2 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x6c as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [105, 110, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: ESDI,
                arg1: DXS,
                flags:
                    (0x200 as libc::c_int | 0x20 as libc::c_int) as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x6d as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [105, 110, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: ESDI,
                arg1: DXS,
                flags:
                    (0x200 as libc::c_int | 0x20 as libc::c_int) as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x6e as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name:
                    [111, 117, 116, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: DXS,
                arg1: DSSI,
                flags:
                    (0x200 as libc::c_int | 0x20 as libc::c_int) as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x6f as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [111, 117, 116, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: DXS,
                arg1: DSSI,
                flags:
                    (0x200 as libc::c_int | 0x20 as libc::c_int) as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x70 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [106, 111, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL8,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x71 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [106, 110, 111, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL8,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x72 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [106, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL8,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x73 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [106, 97, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL8,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x74 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [106, 122, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL8,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x75 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [106, 110, 122, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL8,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x76 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [106, 98, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL8,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x77 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [106, 97, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL8,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x78 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [106, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL8,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x79 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [106, 110, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL8,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x7a as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [106, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL8,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x7b as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [106, 110, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL8,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x7c as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [106, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL8,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x7d as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [106, 103, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL8,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x7e as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [106, 108, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL8,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x7f as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [106, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL8,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x80 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x81 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x82 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x83 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x84 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name:
                    [116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: REG,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x85 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: REG,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x86 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [120, 99, 104, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: RM,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x87 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [120, 99, 104, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: RM,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x88 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: REG,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x89 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: REG,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x8a as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x8b as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x8c as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: SEG16,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x8d as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [108, 101, 97, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: MEM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x8e as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: SEG16,
                arg1: RM,
                flags: 0x40 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x8f as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x90 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [110, 111, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0x20 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x91 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [120, 99, 104, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AX,
                arg1: CX,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x92 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [120, 99, 104, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AX,
                arg1: DX,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x93 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [120, 99, 104, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AX,
                arg1: BX,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x94 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [120, 99, 104, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AX,
                arg1: SP,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x95 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [120, 99, 104, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AX,
                arg1: BP,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x96 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [120, 99, 104, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AX,
                arg1: SI,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x97 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [120, 99, 104, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AX,
                arg1: DI,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x98 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [99, 98, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x99 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [99, 119, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x9a as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x9b as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [119, 97, 105, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x9c as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [112, 117, 115, 104, 102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: NONE,
                arg1: NONE,
                flags: 0x100 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x9d as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [112, 111, 112, 102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0x100 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x9e as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [115, 97, 104, 102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x9f as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [108, 97, 104, 102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xa0 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AL,
                arg1: MOFFS16,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xa1 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AX,
                arg1: MOFFS16,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xa2 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MOFFS16,
                arg1: AL,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xa3 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MOFFS16,
                arg1: AX,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xa4 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name:
                    [109, 111, 118, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: DSSI,
                arg1: ESDI,
                flags:
                    (0x200 as libc::c_int | 0x20 as libc::c_int) as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xa5 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [109, 111, 118, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: DSSI,
                arg1: ESDI,
                flags:
                    (0x200 as libc::c_int | 0x20 as libc::c_int) as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xa6 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [99, 109, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: DSSI,
                arg1: ESDI,
                flags:
                    (0x200 as libc::c_int | 0x10 as libc::c_int |
                         0x20 as libc::c_int) as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xa7 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [99, 109, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: DSSI,
                arg1: ESDI,
                flags:
                    (0x200 as libc::c_int | 0x10 as libc::c_int |
                         0x20 as libc::c_int) as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xa8 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name:
                    [116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AL,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xa9 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AX,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xaa as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name:
                    [115, 116, 111, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: ESDI,
                arg1: ALS,
                flags:
                    (0x200 as libc::c_int | 0x20 as libc::c_int) as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xab as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [115, 116, 111, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: ESDI,
                arg1: AXS,
                flags:
                    (0x200 as libc::c_int | 0x20 as libc::c_int) as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xac as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name:
                    [108, 111, 100, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: ALS,
                arg1: DSSI,
                flags:
                    (0x200 as libc::c_int | 0x20 as libc::c_int) as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xad as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [108, 111, 100, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AXS,
                arg1: DSSI,
                flags:
                    (0x200 as libc::c_int | 0x20 as libc::c_int) as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xae as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [115, 99, 97, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: ALS,
                arg1: ESDI,
                flags:
                    (0x200 as libc::c_int | 0x10 as libc::c_int |
                         0x20 as libc::c_int) as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xaf as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [115, 99, 97, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AXS,
                arg1: ESDI,
                flags:
                    (0x200 as libc::c_int | 0x10 as libc::c_int |
                         0x20 as libc::c_int) as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xb0 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AL,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xb1 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: CL,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xb2 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: DL,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xb3 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: BL,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xb4 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AH,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xb5 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: CH,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xb6 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: DH,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xb7 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: BH,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xb8 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AX,
                arg1: IMM,
                flags: 0x800 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xb9 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: CX,
                arg1: IMM,
                flags: 0x800 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xba as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: DX,
                arg1: IMM,
                flags: 0x800 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xbb as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: BX,
                arg1: IMM,
                flags: 0x800 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xbc as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: SP,
                arg1: IMM,
                flags: 0x800 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xbd as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: BP,
                arg1: IMM,
                flags: 0x800 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xbe as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: SI,
                arg1: IMM,
                flags: 0x800 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xbf as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: DI,
                arg1: IMM,
                flags: 0x800 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xc0 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xc1 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xc2 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [114, 101, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: IMM16,
                arg1: NONE,
                flags: 0x4000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xc3 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [114, 101, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags:
                    (0x4000 as libc::c_int | 0x20 as libc::c_int |
                         0x10 as libc::c_int) as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xc4 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xc5 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xc6 as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xc7 as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xc8 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [101, 110, 116, 101, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: IMM16,
                arg1: IMM8,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xc9 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [108, 101, 97, 118, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xca as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [114, 101, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: IMM16,
                arg1: NONE,
                flags:
                    (0x4000 as libc::c_int | 0x400 as libc::c_int) as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xcb as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [114, 101, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags:
                    (0x4000 as libc::c_int | 0x400 as libc::c_int) as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xcc as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [105, 110, 116, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0x4000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xcd as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [105, 110, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: IMM8,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xce as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [105, 110, 116, 111, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xcf as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [105, 114, 101, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0x4000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd0 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd1 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd2 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd3 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd4 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd5 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd6 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd7 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [120, 108, 97, 116, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: DSBX,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd8 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd9 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xda as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdb as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdc as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdd as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xde as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdf as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xe0 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [108, 111, 111, 112, 110, 122, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: REL8,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xe1 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [108, 111, 111, 112, 122, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: REL8,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xe2 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [108, 111, 111, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL8,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xe3 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [106, 99, 120, 122, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL8,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xe4 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [105, 110, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AL,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xe5 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [105, 110, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AX,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xe6 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [111, 117, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: IMM,
                arg1: AL,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xe7 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [111, 117, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: IMM,
                arg1: AX,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xe8 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [99, 97, 108, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL16,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xe9 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [106, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL16,
                arg1: NONE,
                flags:
                    (0x8000 as libc::c_int | 0x4000 as libc::c_int) as
                        dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xea as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xeb as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [106, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL8,
                arg1: NONE,
                flags:
                    (0x8000 as libc::c_int | 0x4000 as libc::c_int) as
                        dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xec as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [105, 110, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AL,
                arg1: DXS,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xed as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [105, 110, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AX,
                arg1: DXS,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xee as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [111, 117, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: DXS,
                arg1: AL,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xef as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [111, 117, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: DXS,
                arg1: AX,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xf0 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [108, 111, 99, 107, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xf1 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xf2 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [114, 101, 112, 110, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xf3 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [114, 101, 112, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xf4 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [104, 108, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xf5 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [99, 109, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xf6 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xf7 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xf8 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [99, 108, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xf9 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [115, 116, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xfa as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [99, 108, 105, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xfb as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [115, 116, 105, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xfc as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [99, 108, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xfd as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [115, 116, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xfe as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xff as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     }];
static mut instructions_group: [op; 108] =
    [{
         let mut init =
             op{opcode: 0x80 as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [97, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x80 as libc::c_int as word,
                subcode: 1 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x80 as libc::c_int as word,
                subcode: 2 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [97, 100, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x80 as libc::c_int as word,
                subcode: 3 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [115, 98, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x80 as libc::c_int as word,
                subcode: 4 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [97, 110, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x80 as libc::c_int as word,
                subcode: 5 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [115, 117, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x80 as libc::c_int as word,
                subcode: 6 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [120, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x80 as libc::c_int as word,
                subcode: 7 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [99, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x81 as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [97, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x81 as libc::c_int as word,
                subcode: 1 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x81 as libc::c_int as word,
                subcode: 2 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [97, 100, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x81 as libc::c_int as word,
                subcode: 3 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [115, 98, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x81 as libc::c_int as word,
                subcode: 4 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [97, 110, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x81 as libc::c_int as word,
                subcode: 5 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [115, 117, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x81 as libc::c_int as word,
                subcode: 6 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [120, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x81 as libc::c_int as word,
                subcode: 7 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [99, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x82 as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [97, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM8,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x82 as libc::c_int as word,
                subcode: 1 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM8,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x82 as libc::c_int as word,
                subcode: 2 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [97, 100, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM8,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x82 as libc::c_int as word,
                subcode: 3 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [115, 98, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM8,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x82 as libc::c_int as word,
                subcode: 4 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [97, 110, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM8,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x82 as libc::c_int as word,
                subcode: 5 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [115, 117, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM8,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x82 as libc::c_int as word,
                subcode: 6 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [120, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM8,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x82 as libc::c_int as word,
                subcode: 7 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [99, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM8,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x83 as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [97, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM8,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x83 as libc::c_int as word,
                subcode: 1 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM8,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x83 as libc::c_int as word,
                subcode: 2 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [97, 100, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM8,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x83 as libc::c_int as word,
                subcode: 3 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [115, 98, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM8,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x83 as libc::c_int as word,
                subcode: 4 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [97, 110, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM8,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x83 as libc::c_int as word,
                subcode: 5 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [115, 117, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM8,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x83 as libc::c_int as word,
                subcode: 6 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [120, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM8,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x83 as libc::c_int as word,
                subcode: 7 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [99, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM8,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x8f as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [112, 111, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: NONE,
                flags: 0x100 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xc0 as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [114, 111, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM8,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xc0 as libc::c_int as word,
                subcode: 1 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [114, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM8,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xc0 as libc::c_int as word,
                subcode: 2 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [114, 99, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM8,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xc0 as libc::c_int as word,
                subcode: 3 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [114, 99, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM8,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xc0 as libc::c_int as word,
                subcode: 4 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [115, 104, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM8,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xc0 as libc::c_int as word,
                subcode: 5 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [115, 104, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM8,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xc0 as libc::c_int as word,
                subcode: 6 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [115, 97, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM8,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xc0 as libc::c_int as word,
                subcode: 7 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [115, 97, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM8,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xc1 as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [114, 111, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM8,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xc1 as libc::c_int as word,
                subcode: 1 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [114, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM8,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xc1 as libc::c_int as word,
                subcode: 2 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [114, 99, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM8,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xc1 as libc::c_int as word,
                subcode: 3 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [114, 99, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM8,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xc1 as libc::c_int as word,
                subcode: 4 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [115, 104, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM8,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xc1 as libc::c_int as word,
                subcode: 5 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [115, 104, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM8,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xc1 as libc::c_int as word,
                subcode: 6 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [115, 97, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM8,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xc1 as libc::c_int as word,
                subcode: 7 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [115, 97, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM8,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xc6 as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xc7 as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd0 as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [114, 111, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: ONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd0 as libc::c_int as word,
                subcode: 1 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [114, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: ONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd0 as libc::c_int as word,
                subcode: 2 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [114, 99, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: ONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd0 as libc::c_int as word,
                subcode: 3 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [114, 99, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: ONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd0 as libc::c_int as word,
                subcode: 4 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [115, 104, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: ONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd0 as libc::c_int as word,
                subcode: 5 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [115, 104, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: ONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd0 as libc::c_int as word,
                subcode: 6 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [115, 97, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: ONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd0 as libc::c_int as word,
                subcode: 7 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [115, 97, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: ONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd1 as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [114, 111, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: ONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd1 as libc::c_int as word,
                subcode: 1 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [114, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: ONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd1 as libc::c_int as word,
                subcode: 2 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [114, 99, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: ONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd1 as libc::c_int as word,
                subcode: 3 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [114, 99, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: ONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd1 as libc::c_int as word,
                subcode: 4 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [115, 104, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: ONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd1 as libc::c_int as word,
                subcode: 5 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [115, 104, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: ONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd1 as libc::c_int as word,
                subcode: 6 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [115, 97, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: ONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd1 as libc::c_int as word,
                subcode: 7 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [115, 97, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: ONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd2 as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [114, 111, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: CL,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd2 as libc::c_int as word,
                subcode: 1 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [114, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: CL,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd2 as libc::c_int as word,
                subcode: 2 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [114, 99, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: CL,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd2 as libc::c_int as word,
                subcode: 3 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [114, 99, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: CL,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd2 as libc::c_int as word,
                subcode: 4 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [115, 104, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: CL,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd2 as libc::c_int as word,
                subcode: 5 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [115, 104, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: CL,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd2 as libc::c_int as word,
                subcode: 6 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [115, 97, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: CL,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd2 as libc::c_int as word,
                subcode: 7 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [115, 97, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: CL,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd3 as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [114, 111, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: CL,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd3 as libc::c_int as word,
                subcode: 1 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [114, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: CL,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd3 as libc::c_int as word,
                subcode: 2 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [114, 99, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: CL,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd3 as libc::c_int as word,
                subcode: 3 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [114, 99, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: CL,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd3 as libc::c_int as word,
                subcode: 4 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [115, 104, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: CL,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd3 as libc::c_int as word,
                subcode: 5 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [115, 104, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: CL,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd3 as libc::c_int as word,
                subcode: 6 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [115, 97, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: CL,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd3 as libc::c_int as word,
                subcode: 7 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [115, 97, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: CL,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xf6 as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name:
                    [116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xf6 as libc::c_int as word,
                subcode: 1 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name:
                    [116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xf6 as libc::c_int as word,
                subcode: 2 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [110, 111, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: NONE,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xf6 as libc::c_int as word,
                subcode: 3 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [110, 101, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: NONE,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xf6 as libc::c_int as word,
                subcode: 4 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [109, 117, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xf6 as libc::c_int as word,
                subcode: 5 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name:
                    [105, 109, 117, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xf6 as libc::c_int as word,
                subcode: 6 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [100, 105, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xf6 as libc::c_int as word,
                subcode: 7 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name:
                    [105, 100, 105, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xf7 as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xf7 as libc::c_int as word,
                subcode: 1 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xf7 as libc::c_int as word,
                subcode: 2 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [110, 111, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: NONE,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xf7 as libc::c_int as word,
                subcode: 3 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [110, 101, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: NONE,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xf7 as libc::c_int as word,
                subcode: 4 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [109, 117, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xf7 as libc::c_int as word,
                subcode: 5 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [105, 109, 117, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xf7 as libc::c_int as word,
                subcode: 6 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [100, 105, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xf7 as libc::c_int as word,
                subcode: 7 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [105, 100, 105, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xfe as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [105, 110, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: NONE,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xfe as libc::c_int as word,
                subcode: 1 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [100, 101, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: NONE,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xff as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [105, 110, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: NONE,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xff as libc::c_int as word,
                subcode: 1 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [100, 101, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: NONE,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xff as libc::c_int as word,
                subcode: 2 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [99, 97, 108, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: NONE,
                flags: 0x8 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xff as libc::c_int as word,
                subcode: 3 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [99, 97, 108, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MEM,
                arg1: NONE,
                flags: (0x8 as libc::c_int | 0x400 as libc::c_int) as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xff as libc::c_int as word,
                subcode: 4 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [106, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: NONE,
                flags:
                    (0x8 as libc::c_int | 0x4000 as libc::c_int) as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xff as libc::c_int as word,
                subcode: 5 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [106, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MEM,
                arg1: NONE,
                flags:
                    (0x8 as libc::c_int | 0x4000 as libc::c_int |
                         0x400 as libc::c_int) as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xff as libc::c_int as word,
                subcode: 6 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [112, 117, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: NONE,
                flags: 0x100 as libc::c_int as dword,};
         init
     }];
/* a subcode value of 8 means all subcodes,
 * or the subcode marks the register if there is one present. */
static mut instructions_0F: [op; 133] =
    [{
         let mut init =
             op{opcode: 0 as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [115, 108, 100, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: NONE,
                flags: 0x40 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0 as libc::c_int as word,
                subcode: 1 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [115, 116, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: NONE,
                flags: 0x40 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0 as libc::c_int as word,
                subcode: 2 as libc::c_int as byte,
                size: 16 as libc::c_int as libc::c_char,
                name:
                    [108, 108, 100, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0 as libc::c_int as word,
                subcode: 3 as libc::c_int as byte,
                size: 16 as libc::c_int as libc::c_char,
                name: [108, 116, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0 as libc::c_int as word,
                subcode: 4 as libc::c_int as byte,
                size: 16 as libc::c_int as libc::c_char,
                name:
                    [118, 101, 114, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0 as libc::c_int as word,
                subcode: 5 as libc::c_int as byte,
                size: 16 as libc::c_int as libc::c_char,
                name:
                    [118, 101, 114, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x1 as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [115, 103, 100, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MEM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x1 as libc::c_int as word,
                subcode: 1 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [115, 105, 100, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MEM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x1 as libc::c_int as word,
                subcode: 2 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [108, 103, 100, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MEM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x1 as libc::c_int as word,
                subcode: 3 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [108, 105, 100, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MEM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x1 as libc::c_int as word,
                subcode: 4 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [115, 109, 115, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: NONE,
                flags: 0x40 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x1 as libc::c_int as word,
                subcode: 6 as libc::c_int as byte,
                size: 16 as libc::c_int as libc::c_char,
                name:
                    [108, 109, 115, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x1 as libc::c_int as word,
                subcode: 7 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [105, 110, 118, 108, 112, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MEM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x2 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [108, 97, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: RM,
                flags: 0x40 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x3 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [108, 115, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: RM,
                flags: 0x40 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x5 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [115, 121, 115, 99, 97, 108, 108, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x6 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [99, 108, 116, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x7 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [115, 121, 115, 114, 101, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x8 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [105, 110, 118, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x9 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [119, 98, 105, 110, 118, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [112, 114, 101, 102, 101, 116, 99, 104, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: RM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x18 as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name:
                    [112, 114, 101, 102, 101, 116, 99, 104, 110, 116, 97, 0,
                     0, 0, 0, 0],
                arg0: MEM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x18 as libc::c_int as word,
                subcode: 1 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name:
                    [112, 114, 101, 102, 101, 116, 99, 104, 116, 48, 0, 0, 0,
                     0, 0, 0],
                arg0: MEM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x18 as libc::c_int as word,
                subcode: 2 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name:
                    [112, 114, 101, 102, 101, 116, 99, 104, 116, 49, 0, 0, 0,
                     0, 0, 0],
                arg0: MEM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x18 as libc::c_int as word,
                subcode: 3 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name:
                    [112, 114, 101, 102, 101, 116, 99, 104, 116, 50, 0, 0, 0,
                     0, 0, 0],
                arg0: MEM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x1f as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [110, 111, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x20 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG32,
                arg1: CR32,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x21 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG32,
                arg1: DR32,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x22 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: CR32,
                arg1: REG32,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x23 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: DR32,
                arg1: REG32,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x24 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG32,
                arg1: TR32,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x26 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: TR32,
                arg1: REG32,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x30 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [119, 114, 109, 115, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x31 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [114, 100, 116, 115, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x32 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [114, 100, 109, 115, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x33 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [114, 100, 112, 109, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x34 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [115, 121, 115, 101, 110, 116, 101, 114, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x35 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [115, 121, 115, 101, 120, 105, 116, 0, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x40 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [99, 109, 111, 118, 111, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x41 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [99, 109, 111, 118, 110, 111, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x42 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [99, 109, 111, 118, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x43 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [99, 109, 111, 118, 97, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x44 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [99, 109, 111, 118, 122, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x45 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [99, 109, 111, 118, 110, 122, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x46 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [99, 109, 111, 118, 98, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x47 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [99, 109, 111, 118, 97, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x48 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [99, 109, 111, 118, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x49 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [99, 109, 111, 118, 110, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x4a as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [99, 109, 111, 118, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x4b as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [99, 109, 111, 118, 110, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x4c as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [99, 109, 111, 118, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x4d as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [99, 109, 111, 118, 103, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x4e as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [99, 109, 111, 118, 108, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x4f as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [99, 109, 111, 118, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x80 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [106, 111, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL16,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x81 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [106, 110, 111, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL16,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x82 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [106, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL16,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x83 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [106, 97, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL16,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x84 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [106, 122, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL16,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x85 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [106, 110, 122, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL16,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x86 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [106, 98, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL16,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x87 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [106, 97, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL16,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x88 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [106, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL16,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x89 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [106, 110, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL16,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x8a as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [106, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL16,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x8b as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [106, 110, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL16,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x8c as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [106, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL16,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x8d as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [106, 103, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL16,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x8e as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [106, 108, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL16,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x8f as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [106, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REL16,
                arg1: NONE,
                flags: 0x8000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x90 as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name:
                    [115, 101, 116, 111, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x91 as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name:
                    [115, 101, 116, 110, 111, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: RM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x92 as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [115, 101, 116, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x93 as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name:
                    [115, 101, 116, 97, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x94 as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name:
                    [115, 101, 116, 122, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x95 as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name:
                    [115, 101, 116, 110, 122, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: RM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x96 as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name:
                    [115, 101, 116, 98, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x97 as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [115, 101, 116, 97, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x98 as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name:
                    [115, 101, 116, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x99 as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name:
                    [115, 101, 116, 110, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: RM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x9a as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name:
                    [115, 101, 116, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x9b as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name:
                    [115, 101, 116, 110, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: RM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x9c as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name:
                    [115, 101, 116, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x9d as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name:
                    [115, 101, 116, 103, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: RM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x9e as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name:
                    [115, 101, 116, 108, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: RM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x9f as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name:
                    [115, 101, 116, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xa0 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [112, 117, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: FS,
                arg1: NONE,
                flags: 0x100 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xa1 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [112, 111, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: FS,
                arg1: NONE,
                flags: 0x100 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xa2 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [99, 112, 117, 105, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xa3 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [98, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: REG,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xa4 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [115, 104, 108, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: REG,
                flags: 0x2 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xa5 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [115, 104, 108, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: REG,
                flags: 0x4 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xa8 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [112, 117, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: GS,
                arg1: NONE,
                flags: 0x100 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xa9 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [112, 111, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: GS,
                arg1: NONE,
                flags: 0x100 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xab as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [98, 116, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: REG,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xac as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [115, 104, 114, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: REG,
                flags: 0x2 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xad as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [115, 104, 114, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: REG,
                flags: 0x4 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xae as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 120, 115, 97, 118, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MEM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xae as libc::c_int as word,
                subcode: 1 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 120, 114, 115, 116, 111, 114, 0, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: MEM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xae as libc::c_int as word,
                subcode: 2 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [108, 100, 109, 120, 99, 115, 114, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MEM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xae as libc::c_int as word,
                subcode: 3 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [115, 116, 109, 120, 99, 115, 114, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MEM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xae as libc::c_int as word,
                subcode: 4 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [120, 115, 97, 118, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MEM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xae as libc::c_int as word,
                subcode: 5 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [120, 114, 115, 116, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MEM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xae as libc::c_int as word,
                subcode: 7 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [99, 108, 102, 108, 117, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MEM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xaf as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [105, 109, 117, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xb0 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name:
                    [99, 109, 112, 120, 99, 104, 103, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: RM,
                arg1: REG,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xb1 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [99, 109, 112, 120, 99, 104, 103, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: RM,
                arg1: REG,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xb2 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [108, 115, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: MEM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xb3 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [98, 116, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: REG,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xb4 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [108, 102, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: MEM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xb5 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [108, 103, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: MEM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xb6 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [109, 111, 118, 122, 120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xb7 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [109, 111, 118, 122, 120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xba as libc::c_int as word,
                subcode: 4 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [98, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM8,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xba as libc::c_int as word,
                subcode: 5 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [98, 116, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM8,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xba as libc::c_int as word,
                subcode: 6 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [98, 116, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM8,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xba as libc::c_int as word,
                subcode: 7 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [98, 116, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM8,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xbb as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [98, 116, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: REG,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xbc as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [98, 115, 102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xbd as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [98, 115, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xbe as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [109, 111, 118, 115, 120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xbf as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [109, 111, 118, 115, 120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xc0 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 8 as libc::c_int as libc::c_char,
                name: [120, 97, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: REG,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xc1 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [120, 97, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: REG,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xc7 as libc::c_int as word,
                subcode: 1 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [99, 109, 112, 120, 99, 104, 103, 56, 98, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: MEM,
                arg1: NONE,
                flags: 0x80 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xc8 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [98, 115, 119, 97, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: AX,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xc9 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [98, 115, 119, 97, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: CX,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xca as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [98, 115, 119, 97, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: DX,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xcb as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [98, 115, 119, 97, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: BX,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xcc as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [98, 115, 119, 97, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: SP,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xcd as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [98, 115, 119, 97, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: BP,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xce as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [98, 115, 119, 97, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: SI,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xcf as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name:
                    [98, 115, 119, 97, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: DI,
                arg1: NONE,
                flags: 0,};
         init
     }];
/* mod < 3 (instructions with memory args) */
static mut instructions_fpu_m: [op; 64] =
    [{
         let mut init =
             op{opcode: 0xd8 as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: 32 as libc::c_int as libc::c_char,
                name: [102, 97, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MEM,
                arg1: NONE,
                flags: 0x1000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd8 as libc::c_int as word,
                subcode: 1 as libc::c_int as byte,
                size: 32 as libc::c_int as libc::c_char,
                name:
                    [102, 109, 117, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MEM,
                arg1: NONE,
                flags: 0x1000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd8 as libc::c_int as word,
                subcode: 2 as libc::c_int as byte,
                size: 32 as libc::c_int as libc::c_char,
                name: [102, 99, 111, 109, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MEM,
                arg1: NONE,
                flags: 0x1000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd8 as libc::c_int as word,
                subcode: 3 as libc::c_int as byte,
                size: 32 as libc::c_int as libc::c_char,
                name:
                    [102, 99, 111, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MEM,
                arg1: NONE,
                flags: 0x1000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd8 as libc::c_int as word,
                subcode: 4 as libc::c_int as byte,
                size: 32 as libc::c_int as libc::c_char,
                name: [102, 115, 117, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MEM,
                arg1: NONE,
                flags: 0x1000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd8 as libc::c_int as word,
                subcode: 5 as libc::c_int as byte,
                size: 32 as libc::c_int as libc::c_char,
                name:
                    [102, 115, 117, 98, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MEM,
                arg1: NONE,
                flags: 0x1000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd8 as libc::c_int as word,
                subcode: 6 as libc::c_int as byte,
                size: 32 as libc::c_int as libc::c_char,
                name:
                    [102, 100, 105, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MEM,
                arg1: NONE,
                flags: 0x1000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd8 as libc::c_int as word,
                subcode: 7 as libc::c_int as byte,
                size: 32 as libc::c_int as libc::c_char,
                name:
                    [102, 100, 105, 118, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MEM,
                arg1: NONE,
                flags: 0x1000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd9 as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: 32 as libc::c_int as libc::c_char,
                name: [102, 108, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MEM,
                arg1: NONE,
                flags: 0x1000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd9 as libc::c_int as word,
                subcode: 1 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd9 as libc::c_int as word,
                subcode: 2 as libc::c_int as byte,
                size: 32 as libc::c_int as libc::c_char,
                name: [102, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MEM,
                arg1: NONE,
                flags: 0x1000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd9 as libc::c_int as word,
                subcode: 3 as libc::c_int as byte,
                size: 32 as libc::c_int as libc::c_char,
                name:
                    [102, 115, 116, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MEM,
                arg1: NONE,
                flags: 0x1000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd9 as libc::c_int as word,
                subcode: 4 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 108, 100, 101, 110, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MEM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd9 as libc::c_int as word,
                subcode: 5 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 108, 100, 99, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MEM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd9 as libc::c_int as word,
                subcode: 6 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 110, 115, 116, 101, 110, 118, 0, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: MEM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd9 as libc::c_int as word,
                subcode: 7 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 110, 115, 116, 99, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MEM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xda as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: 32 as libc::c_int as libc::c_char,
                name:
                    [102, 105, 97, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MEM,
                arg1: NONE,
                flags: 0x2000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xda as libc::c_int as word,
                subcode: 1 as libc::c_int as byte,
                size: 32 as libc::c_int as libc::c_char,
                name:
                    [102, 105, 109, 117, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MEM,
                arg1: NONE,
                flags: 0x2000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xda as libc::c_int as word,
                subcode: 2 as libc::c_int as byte,
                size: 32 as libc::c_int as libc::c_char,
                name:
                    [102, 105, 99, 111, 109, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MEM,
                arg1: NONE,
                flags: 0x2000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xda as libc::c_int as word,
                subcode: 3 as libc::c_int as byte,
                size: 32 as libc::c_int as libc::c_char,
                name:
                    [102, 105, 99, 111, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MEM,
                arg1: NONE,
                flags: 0x2000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xda as libc::c_int as word,
                subcode: 4 as libc::c_int as byte,
                size: 32 as libc::c_int as libc::c_char,
                name:
                    [102, 105, 115, 117, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MEM,
                arg1: NONE,
                flags: 0x2000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xda as libc::c_int as word,
                subcode: 5 as libc::c_int as byte,
                size: 32 as libc::c_int as libc::c_char,
                name:
                    [102, 105, 115, 117, 98, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MEM,
                arg1: NONE,
                flags: 0x2000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xda as libc::c_int as word,
                subcode: 6 as libc::c_int as byte,
                size: 32 as libc::c_int as libc::c_char,
                name:
                    [102, 105, 100, 105, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MEM,
                arg1: NONE,
                flags: 0x2000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xda as libc::c_int as word,
                subcode: 7 as libc::c_int as byte,
                size: 32 as libc::c_int as libc::c_char,
                name:
                    [102, 105, 100, 105, 118, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MEM,
                arg1: NONE,
                flags: 0x2000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdb as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: 32 as libc::c_int as libc::c_char,
                name:
                    [102, 105, 108, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MEM,
                arg1: NONE,
                flags: 0x2000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdb as libc::c_int as word,
                subcode: 1 as libc::c_int as byte,
                size: 32 as libc::c_int as libc::c_char,
                name:
                    [102, 105, 115, 116, 116, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MEM,
                arg1: NONE,
                flags: 0x2000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdb as libc::c_int as word,
                subcode: 2 as libc::c_int as byte,
                size: 32 as libc::c_int as libc::c_char,
                name:
                    [102, 105, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MEM,
                arg1: NONE,
                flags: 0x2000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdb as libc::c_int as word,
                subcode: 3 as libc::c_int as byte,
                size: 32 as libc::c_int as libc::c_char,
                name:
                    [102, 105, 115, 116, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MEM,
                arg1: NONE,
                flags: 0x2000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdb as libc::c_int as word,
                subcode: 4 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdb as libc::c_int as word,
                subcode: 5 as libc::c_int as byte,
                size: 80 as libc::c_int as libc::c_char,
                name: [102, 108, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MEM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdb as libc::c_int as word,
                subcode: 6 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdb as libc::c_int as word,
                subcode: 7 as libc::c_int as byte,
                size: 80 as libc::c_int as libc::c_char,
                name:
                    [102, 115, 116, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MEM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdc as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: 64 as libc::c_int as libc::c_char,
                name: [102, 97, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MEM,
                arg1: NONE,
                flags: 0x2000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdc as libc::c_int as word,
                subcode: 1 as libc::c_int as byte,
                size: 64 as libc::c_int as libc::c_char,
                name:
                    [102, 109, 117, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MEM,
                arg1: NONE,
                flags: 0x2000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdc as libc::c_int as word,
                subcode: 2 as libc::c_int as byte,
                size: 64 as libc::c_int as libc::c_char,
                name: [102, 99, 111, 109, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MEM,
                arg1: NONE,
                flags: 0x2000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdc as libc::c_int as word,
                subcode: 3 as libc::c_int as byte,
                size: 64 as libc::c_int as libc::c_char,
                name:
                    [102, 99, 111, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MEM,
                arg1: NONE,
                flags: 0x2000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdc as libc::c_int as word,
                subcode: 4 as libc::c_int as byte,
                size: 64 as libc::c_int as libc::c_char,
                name: [102, 115, 117, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MEM,
                arg1: NONE,
                flags: 0x2000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdc as libc::c_int as word,
                subcode: 5 as libc::c_int as byte,
                size: 64 as libc::c_int as libc::c_char,
                name:
                    [102, 115, 117, 98, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MEM,
                arg1: NONE,
                flags: 0x2000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdc as libc::c_int as word,
                subcode: 6 as libc::c_int as byte,
                size: 64 as libc::c_int as libc::c_char,
                name:
                    [102, 100, 105, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MEM,
                arg1: NONE,
                flags: 0x2000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdc as libc::c_int as word,
                subcode: 7 as libc::c_int as byte,
                size: 64 as libc::c_int as libc::c_char,
                name:
                    [102, 100, 105, 118, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MEM,
                arg1: NONE,
                flags: 0x2000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdd as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: 64 as libc::c_int as libc::c_char,
                name: [102, 108, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MEM,
                arg1: NONE,
                flags: 0x2000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdd as libc::c_int as word,
                subcode: 1 as libc::c_int as byte,
                size: 64 as libc::c_int as libc::c_char,
                name:
                    [102, 105, 115, 116, 116, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MEM,
                arg1: NONE,
                flags: 0x3000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdd as libc::c_int as word,
                subcode: 2 as libc::c_int as byte,
                size: 64 as libc::c_int as libc::c_char,
                name: [102, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MEM,
                arg1: NONE,
                flags: 0x2000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdd as libc::c_int as word,
                subcode: 3 as libc::c_int as byte,
                size: 64 as libc::c_int as libc::c_char,
                name:
                    [102, 115, 116, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MEM,
                arg1: NONE,
                flags: 0x2000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdd as libc::c_int as word,
                subcode: 4 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 114, 115, 116, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MEM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdd as libc::c_int as word,
                subcode: 5 as libc::c_int as byte,
                size: 0,
                name: [0; 16],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdd as libc::c_int as word,
                subcode: 6 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 110, 115, 97, 118, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MEM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdd as libc::c_int as word,
                subcode: 7 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 110, 115, 116, 115, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MEM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xde as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: 16 as libc::c_int as libc::c_char,
                name:
                    [102, 105, 97, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MEM,
                arg1: NONE,
                flags: 0x1000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xde as libc::c_int as word,
                subcode: 1 as libc::c_int as byte,
                size: 16 as libc::c_int as libc::c_char,
                name:
                    [102, 105, 109, 117, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MEM,
                arg1: NONE,
                flags: 0x1000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xde as libc::c_int as word,
                subcode: 2 as libc::c_int as byte,
                size: 16 as libc::c_int as libc::c_char,
                name:
                    [102, 105, 99, 111, 109, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MEM,
                arg1: NONE,
                flags: 0x1000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xde as libc::c_int as word,
                subcode: 3 as libc::c_int as byte,
                size: 16 as libc::c_int as libc::c_char,
                name:
                    [102, 105, 99, 111, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MEM,
                arg1: NONE,
                flags: 0x1000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xde as libc::c_int as word,
                subcode: 4 as libc::c_int as byte,
                size: 16 as libc::c_int as libc::c_char,
                name:
                    [102, 105, 115, 117, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MEM,
                arg1: NONE,
                flags: 0x1000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xde as libc::c_int as word,
                subcode: 5 as libc::c_int as byte,
                size: 16 as libc::c_int as libc::c_char,
                name:
                    [102, 105, 115, 117, 98, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MEM,
                arg1: NONE,
                flags: 0x1000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xde as libc::c_int as word,
                subcode: 6 as libc::c_int as byte,
                size: 16 as libc::c_int as libc::c_char,
                name:
                    [102, 105, 100, 105, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MEM,
                arg1: NONE,
                flags: 0x1000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xde as libc::c_int as word,
                subcode: 7 as libc::c_int as byte,
                size: 16 as libc::c_int as libc::c_char,
                name:
                    [102, 105, 100, 105, 118, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MEM,
                arg1: NONE,
                flags: 0x1000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdf as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: 16 as libc::c_int as libc::c_char,
                name:
                    [102, 105, 108, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MEM,
                arg1: NONE,
                flags: 0x1000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdf as libc::c_int as word,
                subcode: 1 as libc::c_int as byte,
                size: 16 as libc::c_int as libc::c_char,
                name:
                    [102, 105, 115, 116, 116, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MEM,
                arg1: NONE,
                flags: 0x1000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdf as libc::c_int as word,
                subcode: 2 as libc::c_int as byte,
                size: 16 as libc::c_int as libc::c_char,
                name:
                    [102, 105, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MEM,
                arg1: NONE,
                flags: 0x1000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdf as libc::c_int as word,
                subcode: 3 as libc::c_int as byte,
                size: 16 as libc::c_int as libc::c_char,
                name:
                    [102, 105, 115, 116, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MEM,
                arg1: NONE,
                flags: 0x1000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdf as libc::c_int as word,
                subcode: 4 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [102, 98, 108, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MEM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdf as libc::c_int as word,
                subcode: 5 as libc::c_int as byte,
                size: 64 as libc::c_int as libc::c_char,
                name:
                    [102, 105, 108, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MEM,
                arg1: NONE,
                flags: 0x3000 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdf as libc::c_int as word,
                subcode: 6 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 98, 115, 116, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MEM,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdf as libc::c_int as word,
                subcode: 7 as libc::c_int as byte,
                size: 64 as libc::c_int as libc::c_char,
                name:
                    [102, 105, 115, 116, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MEM,
                arg1: NONE,
                flags: 0x3000 as libc::c_int as dword,};
         init
     }];
static mut instructions_fpu_r: [op; 64] =
    [{
         let mut init =
             op{opcode: 0xd8 as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [102, 97, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: ST,
                arg1: STX,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd8 as libc::c_int as word,
                subcode: 1 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 109, 117, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: ST,
                arg1: STX,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd8 as libc::c_int as word,
                subcode: 2 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [102, 99, 111, 109, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: STX,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd8 as libc::c_int as word,
                subcode: 3 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 99, 111, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: STX,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd8 as libc::c_int as word,
                subcode: 4 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [102, 115, 117, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: ST,
                arg1: STX,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd8 as libc::c_int as word,
                subcode: 5 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 115, 117, 98, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: ST,
                arg1: STX,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd8 as libc::c_int as word,
                subcode: 6 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 100, 105, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: ST,
                arg1: STX,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd8 as libc::c_int as word,
                subcode: 7 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 100, 105, 118, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: ST,
                arg1: STX,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd9 as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [102, 108, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: STX,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd9 as libc::c_int as word,
                subcode: 1 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [102, 120, 99, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: STX,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd9 as libc::c_int as word,
                subcode: 2 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [0 as libc::c_int as libc::c_char, 0, 0, 0, 0, 0, 0, 0, 0,
                     0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd9 as libc::c_int as word,
                subcode: 3 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 115, 116, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: STX,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd9 as libc::c_int as word,
                subcode: 4 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [0 as libc::c_int as libc::c_char, 0, 0, 0, 0, 0, 0, 0, 0,
                     0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd9 as libc::c_int as word,
                subcode: 5 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [0 as libc::c_int as libc::c_char, 0, 0, 0, 0, 0, 0, 0, 0,
                     0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd9 as libc::c_int as word,
                subcode: 6 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [0 as libc::c_int as libc::c_char, 0, 0, 0, 0, 0, 0, 0, 0,
                     0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd9 as libc::c_int as word,
                subcode: 7 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [0 as libc::c_int as libc::c_char, 0, 0, 0, 0, 0, 0, 0, 0,
                     0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xda as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 99, 109, 111, 118, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: ST,
                arg1: STX,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xda as libc::c_int as word,
                subcode: 1 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 99, 109, 111, 118, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: ST,
                arg1: STX,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xda as libc::c_int as word,
                subcode: 2 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 99, 109, 111, 118, 98, 101, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: ST,
                arg1: STX,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xda as libc::c_int as word,
                subcode: 3 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 99, 109, 111, 118, 117, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: ST,
                arg1: STX,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xda as libc::c_int as word,
                subcode: 4 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [0 as libc::c_int as libc::c_char, 0, 0, 0, 0, 0, 0, 0, 0,
                     0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xda as libc::c_int as word,
                subcode: 5 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [0 as libc::c_int as libc::c_char, 0, 0, 0, 0, 0, 0, 0, 0,
                     0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xda as libc::c_int as word,
                subcode: 6 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [0 as libc::c_int as libc::c_char, 0, 0, 0, 0, 0, 0, 0, 0,
                     0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xda as libc::c_int as word,
                subcode: 7 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [0 as libc::c_int as libc::c_char, 0, 0, 0, 0, 0, 0, 0, 0,
                     0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdb as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 99, 109, 111, 118, 110, 98, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: ST,
                arg1: STX,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdb as libc::c_int as word,
                subcode: 1 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 99, 109, 111, 118, 110, 101, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: ST,
                arg1: STX,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdb as libc::c_int as word,
                subcode: 2 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 99, 109, 111, 118, 110, 98, 101, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: ST,
                arg1: STX,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdb as libc::c_int as word,
                subcode: 3 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 99, 109, 111, 118, 110, 117, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: ST,
                arg1: STX,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdb as libc::c_int as word,
                subcode: 4 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [0 as libc::c_int as libc::c_char, 0, 0, 0, 0, 0, 0, 0, 0,
                     0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdb as libc::c_int as word,
                subcode: 5 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 117, 99, 111, 109, 105, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: ST,
                arg1: STX,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdb as libc::c_int as word,
                subcode: 6 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 99, 111, 109, 105, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: ST,
                arg1: STX,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdb as libc::c_int as word,
                subcode: 7 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [0 as libc::c_int as libc::c_char, 0, 0, 0, 0, 0, 0, 0, 0,
                     0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdc as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [102, 97, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: STX,
                arg1: ST,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdc as libc::c_int as word,
                subcode: 1 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 109, 117, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: STX,
                arg1: ST,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdc as libc::c_int as word,
                subcode: 2 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [102, 99, 111, 109, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: STX,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdc as libc::c_int as word,
                subcode: 3 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 99, 111, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: STX,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdc as libc::c_int as word,
                subcode: 4 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 115, 117, 98, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: STX,
                arg1: ST,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdc as libc::c_int as word,
                subcode: 5 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [102, 115, 117, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: STX,
                arg1: ST,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdc as libc::c_int as word,
                subcode: 6 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 100, 105, 118, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: STX,
                arg1: ST,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdc as libc::c_int as word,
                subcode: 7 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 100, 105, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: STX,
                arg1: ST,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdd as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 102, 114, 101, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: STX,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdd as libc::c_int as word,
                subcode: 1 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [102, 120, 99, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: STX,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdd as libc::c_int as word,
                subcode: 2 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [102, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: STX,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdd as libc::c_int as word,
                subcode: 3 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 115, 116, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: STX,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdd as libc::c_int as word,
                subcode: 4 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 117, 99, 111, 109, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: STX,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdd as libc::c_int as word,
                subcode: 5 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 117, 99, 111, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: STX,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdd as libc::c_int as word,
                subcode: 6 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [0 as libc::c_int as libc::c_char, 0, 0, 0, 0, 0, 0, 0, 0,
                     0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdd as libc::c_int as word,
                subcode: 7 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [0 as libc::c_int as libc::c_char, 0, 0, 0, 0, 0, 0, 0, 0,
                     0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xde as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 97, 100, 100, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: STX,
                arg1: ST,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xde as libc::c_int as word,
                subcode: 1 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 109, 117, 108, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: STX,
                arg1: ST,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xde as libc::c_int as word,
                subcode: 2 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 99, 111, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: STX,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xde as libc::c_int as word,
                subcode: 3 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [0 as libc::c_int as libc::c_char, 0, 0, 0, 0, 0, 0, 0, 0,
                     0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xde as libc::c_int as word,
                subcode: 4 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 115, 117, 98, 114, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: STX,
                arg1: ST,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xde as libc::c_int as word,
                subcode: 5 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 115, 117, 98, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: STX,
                arg1: ST,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xde as libc::c_int as word,
                subcode: 6 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 100, 105, 118, 114, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: STX,
                arg1: ST,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xde as libc::c_int as word,
                subcode: 7 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 100, 105, 118, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: STX,
                arg1: ST,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdf as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 102, 114, 101, 101, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: STX,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdf as libc::c_int as word,
                subcode: 1 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [102, 120, 99, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: STX,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdf as libc::c_int as word,
                subcode: 2 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 115, 116, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: STX,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdf as libc::c_int as word,
                subcode: 3 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 115, 116, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: STX,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdf as libc::c_int as word,
                subcode: 4 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [0 as libc::c_int as libc::c_char, 0, 0, 0, 0, 0, 0, 0, 0,
                     0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdf as libc::c_int as word,
                subcode: 5 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 117, 99, 111, 109, 105, 112, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: ST,
                arg1: STX,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdf as libc::c_int as word,
                subcode: 6 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 99, 111, 109, 105, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: ST,
                arg1: STX,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdf as libc::c_int as word,
                subcode: 7 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [0 as libc::c_int as libc::c_char, 0, 0, 0, 0, 0, 0, 0, 0,
                     0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     }];
static mut instructions_fpu_single: [op; 36] =
    [{
         let mut init =
             op{opcode: 0xd9 as libc::c_int as word,
                subcode: 0xd0 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 110, 111, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd9 as libc::c_int as word,
                subcode: 0xe0 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [102, 99, 104, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd9 as libc::c_int as word,
                subcode: 0xe1 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [102, 97, 98, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd9 as libc::c_int as word,
                subcode: 0xe4 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 116, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd9 as libc::c_int as word,
                subcode: 0xe5 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [102, 120, 97, 109, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd9 as libc::c_int as word,
                subcode: 0xe8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [102, 108, 100, 49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd9 as libc::c_int as word,
                subcode: 0xe9 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 108, 100, 108, 50, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd9 as libc::c_int as word,
                subcode: 0xea as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 108, 100, 108, 50, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd9 as libc::c_int as word,
                subcode: 0xeb as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 108, 100, 112, 105, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd9 as libc::c_int as word,
                subcode: 0xec as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 108, 100, 108, 103, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd9 as libc::c_int as word,
                subcode: 0xed as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 108, 100, 108, 110, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd9 as libc::c_int as word,
                subcode: 0xee as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 108, 100, 122, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd9 as libc::c_int as word,
                subcode: 0xf0 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 50, 120, 109, 49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd9 as libc::c_int as word,
                subcode: 0xf1 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 121, 108, 50, 120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd9 as libc::c_int as word,
                subcode: 0xf2 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 112, 116, 97, 110, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd9 as libc::c_int as word,
                subcode: 0xf3 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 112, 97, 116, 97, 110, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd9 as libc::c_int as word,
                subcode: 0xf4 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 120, 116, 114, 97, 99, 116, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd9 as libc::c_int as word,
                subcode: 0xf5 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 112, 114, 101, 109, 49, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd9 as libc::c_int as word,
                subcode: 0xf6 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 100, 101, 99, 115, 116, 112, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd9 as libc::c_int as word,
                subcode: 0xf7 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 105, 110, 99, 115, 116, 112, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd9 as libc::c_int as word,
                subcode: 0xf8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 112, 114, 101, 109, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd9 as libc::c_int as word,
                subcode: 0xf9 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 121, 108, 50, 120, 112, 49, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd9 as libc::c_int as word,
                subcode: 0xfa as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 115, 113, 114, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd9 as libc::c_int as word,
                subcode: 0xfb as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 115, 105, 110, 99, 111, 115, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd9 as libc::c_int as word,
                subcode: 0xfc as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 114, 110, 100, 105, 110, 116, 0, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd9 as libc::c_int as word,
                subcode: 0xfd as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 115, 99, 97, 108, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd9 as libc::c_int as word,
                subcode: 0xfe as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 115, 105, 110, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd9 as libc::c_int as word,
                subcode: 0xff as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [102, 99, 111, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xda as libc::c_int as word,
                subcode: 0xe9 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 117, 99, 111, 109, 112, 112, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdb as libc::c_int as word,
                subcode: 0xe0 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 110, 101, 110, 105, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdb as libc::c_int as word,
                subcode: 0xe1 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 110, 100, 105, 115, 105, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdb as libc::c_int as word,
                subcode: 0xe2 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 110, 99, 108, 101, 120, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdb as libc::c_int as word,
                subcode: 0xe3 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 110, 105, 110, 105, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdb as libc::c_int as word,
                subcode: 0xe4 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 110, 115, 101, 116, 112, 109, 0, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xde as libc::c_int as word,
                subcode: 0xd9 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 99, 111, 109, 112, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdf as libc::c_int as word,
                subcode: 0xe0 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [102, 110, 115, 116, 115, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: AX,
                arg1: NONE,
                flags: 0,};
         init
     }];
unsafe extern "C" fn get_fpu_instr(mut p: *const byte, mut op: *mut op)
 -> libc::c_int {
    let mut subcode: byte =
        (*p.offset(1 as libc::c_int as isize) as libc::c_int >>
             3 as libc::c_int & 7 as libc::c_int) as byte;
    let mut index: byte =
        ((*p.offset(0 as libc::c_int as isize) as libc::c_int &
              7 as libc::c_int) * 8 as libc::c_int + subcode as libc::c_int)
            as byte;
    let mut i: libc::c_uint = 0;
    if (*p.offset(1 as libc::c_int as isize) as libc::c_int >>
            6 as libc::c_int) < 3 as libc::c_int {
        if instructions_fpu_m[index as usize].name[0 as libc::c_int as usize]
               != 0 {
            *op = instructions_fpu_m[index as usize]
        }
        return 0 as libc::c_int
    } else {
        if instructions_fpu_r[index as usize].name[0 as libc::c_int as usize]
               != 0 {
            *op = instructions_fpu_r[index as usize];
            return 0 as libc::c_int
        } else {
            /* try the single op list */
            i = 0 as libc::c_int as libc::c_uint;
            while (i as libc::c_ulong) <
                      (::std::mem::size_of::<[op; 36]>() as
                           libc::c_ulong).wrapping_div(::std::mem::size_of::<op>()
                                                           as libc::c_ulong) {
                if *p.offset(0 as libc::c_int as isize) as libc::c_int ==
                       instructions_fpu_single[i as usize].opcode as
                           libc::c_int &&
                       *p.offset(1 as libc::c_int as isize) as libc::c_int ==
                           instructions_fpu_single[i as usize].subcode as
                               libc::c_int {
                    *op = instructions_fpu_single[i as usize];
                    break ;
                } else { i = i.wrapping_add(1) }
            }
        }
        return 1 as libc::c_int
    };
}
static mut instructions_sse: [op; 109] =
    [{
         let mut init =
             op{opcode: 0x10 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [109, 111, 118, 117, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x11 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [109, 111, 118, 117, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XM,
                arg1: XMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x12 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [109, 111, 118, 108, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x13 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [109, 111, 118, 108, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MEM,
                arg1: XMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x14 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [117, 110, 112, 99, 107, 108, 112, 115, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x15 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [117, 110, 112, 99, 107, 104, 112, 115, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x16 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [109, 111, 118, 104, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x17 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [109, 111, 118, 104, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MEM,
                arg1: XMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x28 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [109, 111, 118, 97, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x29 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [109, 111, 118, 97, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XM,
                arg1: XMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x2a as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [99, 118, 116, 112, 105, 50, 112, 115, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: XMM,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x2b as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [109, 111, 118, 110, 116, 112, 115, 0, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: MEM,
                arg1: XMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x2c as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [99, 118, 116, 116, 112, 115, 50, 112, 105, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: MMX,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x2d as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [99, 118, 116, 112, 115, 50, 112, 105, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: MMX,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x2e as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [117, 99, 111, 109, 105, 115, 115, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x2f as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [99, 111, 109, 105, 115, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x50 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [109, 111, 118, 109, 115, 107, 112, 115, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: REGONLY,
                arg1: XMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x51 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [115, 113, 114, 116, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x52 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [114, 115, 113, 114, 116, 112, 115, 0, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x53 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [114, 99, 112, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x54 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [97, 110, 100, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x55 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [97, 110, 100, 110, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x56 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [111, 114, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x57 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [120, 111, 114, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x58 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [97, 100, 100, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x59 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [109, 117, 108, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x5a as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [99, 118, 116, 112, 115, 50, 112, 100, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x5b as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [99, 118, 116, 100, 113, 50, 112, 115, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x5c as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [115, 117, 98, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x5d as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [109, 105, 110, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x5e as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [100, 105, 118, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x5f as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [109, 97, 120, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x60 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 117, 110, 112, 99, 107, 108, 98, 119, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x61 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 117, 110, 112, 99, 107, 108, 119, 100, 0, 0, 0, 0,
                     0, 0, 0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x62 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 117, 110, 112, 99, 107, 108, 100, 113, 0, 0, 0, 0,
                     0, 0, 0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x63 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 97, 99, 107, 115, 115, 119, 98, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x64 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 99, 109, 112, 103, 116, 98, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x65 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 99, 109, 112, 103, 116, 119, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x66 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 99, 109, 112, 103, 116, 100, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x67 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 97, 99, 107, 117, 115, 119, 98, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x68 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 117, 110, 112, 99, 107, 104, 98, 119, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x69 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 117, 110, 112, 99, 107, 104, 119, 100, 0, 0, 0, 0,
                     0, 0, 0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x6a as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 117, 110, 112, 99, 107, 104, 100, 113, 0, 0, 0, 0,
                     0, 0, 0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x6b as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 97, 99, 107, 115, 115, 100, 119, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x6e as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [109, 111, 118, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MMX,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x6f as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [109, 111, 118, 113, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x70 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 104, 117, 102, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MMX,
                arg1: MM,
                flags: 0x2 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x71 as libc::c_int as word,
                subcode: 2 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 114, 108, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MMXONLY,
                arg1: IMM8,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x71 as libc::c_int as word,
                subcode: 4 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 114, 97, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MMXONLY,
                arg1: IMM8,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x71 as libc::c_int as word,
                subcode: 6 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 108, 108, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MMXONLY,
                arg1: IMM8,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x72 as libc::c_int as word,
                subcode: 2 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 114, 108, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MMXONLY,
                arg1: IMM8,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x72 as libc::c_int as word,
                subcode: 4 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 114, 97, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MMXONLY,
                arg1: IMM8,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x72 as libc::c_int as word,
                subcode: 6 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 108, 108, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MMXONLY,
                arg1: IMM8,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x73 as libc::c_int as word,
                subcode: 2 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 114, 108, 113, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MMXONLY,
                arg1: IMM8,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x73 as libc::c_int as word,
                subcode: 6 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 108, 108, 113, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MMXONLY,
                arg1: IMM8,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x74 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 99, 109, 112, 101, 113, 98, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x75 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 99, 109, 112, 101, 113, 119, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x76 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 99, 109, 112, 101, 113, 100, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x77 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [101, 109, 109, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x7e as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [109, 111, 118, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: MMX,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x7f as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [109, 111, 118, 113, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MM,
                arg1: MMX,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xc2 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [99, 109, 112, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0x2 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xc3 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [109, 111, 118, 110, 116, 105, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MEM,
                arg1: REG,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xc4 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 105, 110, 115, 114, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MMX,
                arg1: RM,
                flags: 0x2 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xc5 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 101, 120, 116, 114, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: REGONLY,
                arg1: MMX,
                flags: 0x2 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xc6 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [115, 104, 117, 102, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0x2 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd1 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 114, 108, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd2 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 114, 108, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd3 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 114, 108, 113, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd4 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 97, 100, 100, 113, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd5 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 109, 117, 108, 108, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd7 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 109, 111, 118, 109, 115, 107, 98, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: REGONLY,
                arg1: MMX,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd8 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 117, 98, 117, 115, 98, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd9 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 117, 98, 117, 115, 119, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xda as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 109, 105, 110, 117, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdb as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [112, 97, 110, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdc as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 97, 100, 100, 117, 115, 98, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdd as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 97, 100, 100, 117, 115, 119, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xde as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 109, 97, 120, 117, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdf as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 97, 110, 100, 110, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xe0 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 97, 118, 103, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xe1 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 114, 97, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xe2 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 114, 97, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xe3 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 97, 118, 103, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xe4 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 109, 117, 108, 104, 117, 119, 0, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xe5 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 109, 117, 108, 104, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xe7 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [109, 111, 118, 110, 116, 113, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MEM,
                arg1: MMX,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xe8 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 117, 98, 115, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xe9 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 117, 98, 115, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xea as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 109, 105, 110, 115, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xeb as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [112, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xec as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 97, 100, 100, 115, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xed as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 97, 100, 100, 115, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xee as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 109, 97, 120, 115, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xef as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 120, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xf1 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 108, 108, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xf2 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 108, 108, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xf3 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 108, 108, 113, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xf4 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 109, 117, 108, 117, 100, 113, 0, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xf5 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 109, 97, 100, 100, 119, 100, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xf6 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 97, 100, 98, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xf7 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [109, 97, 115, 107, 109, 111, 118, 113, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: MMX,
                arg1: MMXONLY,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xf8 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 117, 98, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xf9 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 117, 98, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xfa as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 117, 98, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xfb as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 117, 98, 113, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xfc as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 97, 100, 100, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xfd as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 97, 100, 100, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xfe as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 97, 100, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     }];
static mut instructions_sse_op32: [op; 114] =
    [{
         let mut init =
             op{opcode: 0x10 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [109, 111, 118, 117, 112, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x11 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [109, 111, 118, 117, 112, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XM,
                arg1: XMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x12 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [109, 111, 118, 108, 112, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x13 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [109, 111, 118, 108, 112, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MEM,
                arg1: XMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x14 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [117, 110, 112, 99, 107, 108, 112, 100, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x15 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [117, 110, 112, 99, 107, 104, 112, 100, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x16 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [109, 111, 118, 104, 112, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x17 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [109, 111, 118, 104, 112, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MEM,
                arg1: XMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x28 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [109, 111, 118, 97, 112, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x29 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [109, 111, 118, 97, 112, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XM,
                arg1: XMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x2a as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [99, 118, 116, 112, 105, 50, 112, 100, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: XMM,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x2b as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [109, 111, 118, 110, 116, 112, 100, 0, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: MEM,
                arg1: XMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x2c as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [99, 118, 116, 116, 112, 100, 50, 112, 105, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: MMX,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x2d as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [99, 118, 116, 112, 100, 50, 112, 105, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: MMX,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x2e as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [117, 99, 111, 109, 105, 115, 100, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x2f as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [99, 111, 109, 105, 115, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x50 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 32 as libc::c_int as libc::c_char,
                name:
                    [109, 111, 118, 109, 115, 107, 112, 100, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: REGONLY,
                arg1: XMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x51 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [115, 113, 114, 116, 112, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x54 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [97, 110, 100, 112, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x55 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [97, 110, 100, 110, 112, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x56 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [111, 114, 112, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x57 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [120, 111, 114, 112, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x58 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [97, 100, 100, 112, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x59 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [109, 117, 108, 112, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x5a as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [99, 118, 116, 112, 100, 50, 112, 115, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x5b as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [99, 118, 116, 112, 115, 50, 100, 113, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x5c as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [115, 117, 98, 112, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x5d as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [109, 105, 110, 112, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x5e as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [100, 105, 118, 112, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x5f as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [109, 97, 120, 112, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x60 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 117, 110, 112, 99, 107, 108, 98, 119, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x61 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 117, 110, 112, 99, 107, 108, 119, 100, 0, 0, 0, 0,
                     0, 0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x62 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 117, 110, 112, 99, 107, 108, 100, 113, 0, 0, 0, 0,
                     0, 0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x63 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 97, 99, 107, 115, 115, 119, 98, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x64 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 99, 109, 112, 103, 116, 98, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x65 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 99, 109, 112, 103, 116, 119, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x66 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 99, 109, 112, 103, 116, 100, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x67 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 97, 99, 107, 117, 115, 119, 98, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x68 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 117, 110, 112, 99, 107, 104, 98, 119, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x69 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 117, 110, 112, 99, 107, 104, 119, 100, 0, 0, 0, 0,
                     0, 0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x6a as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 117, 110, 112, 99, 107, 104, 100, 113, 0, 0, 0, 0,
                     0, 0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x6b as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 97, 99, 107, 115, 115, 100, 119, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x6c as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 117, 110, 112, 99, 107, 108, 113, 100, 113, 0, 0, 0,
                     0, 0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x6d as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 117, 110, 112, 99, 107, 104, 113, 100, 113, 0, 0, 0,
                     0, 0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x6e as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: XMM,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x6f as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [109, 111, 118, 100, 113, 97, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x70 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 104, 117, 102, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0x2 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x71 as libc::c_int as word,
                subcode: 2 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 114, 108, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMMONLY,
                arg1: IMM8,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x71 as libc::c_int as word,
                subcode: 4 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 114, 97, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: XMMONLY,
                arg1: IMM8,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x71 as libc::c_int as word,
                subcode: 6 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 108, 108, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMMONLY,
                arg1: IMM8,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x72 as libc::c_int as word,
                subcode: 2 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 114, 108, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMMONLY,
                arg1: IMM8,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x72 as libc::c_int as word,
                subcode: 4 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 114, 97, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: XMMONLY,
                arg1: IMM8,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x72 as libc::c_int as word,
                subcode: 6 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 108, 108, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMMONLY,
                arg1: IMM8,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x73 as libc::c_int as word,
                subcode: 2 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 114, 108, 113, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMMONLY,
                arg1: IMM8,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x73 as libc::c_int as word,
                subcode: 3 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 114, 108, 100, 113, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMMONLY,
                arg1: IMM8,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x73 as libc::c_int as word,
                subcode: 6 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 108, 108, 113, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMMONLY,
                arg1: IMM8,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x73 as libc::c_int as word,
                subcode: 7 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 108, 108, 100, 113, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMMONLY,
                arg1: IMM8,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x74 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 99, 109, 112, 101, 113, 98, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x75 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 99, 109, 112, 101, 113, 119, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x76 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 99, 109, 112, 101, 113, 100, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x7c as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [104, 97, 100, 100, 112, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x7d as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [104, 115, 117, 98, 112, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x7e as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: -(1 as libc::c_int) as libc::c_char,
                name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: XMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x7f as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [109, 111, 118, 100, 113, 97, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XM,
                arg1: XMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xc2 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [99, 109, 112, 112, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0x2 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xc4 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 105, 110, 115, 114, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: RM,
                flags: 0x2 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xc5 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 101, 120, 116, 114, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: REGONLY,
                arg1: XMM,
                flags: 0x2 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xc6 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [115, 104, 117, 102, 112, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0x2 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd0 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [97, 100, 100, 115, 117, 98, 112, 100, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd1 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 114, 108, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd2 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 114, 108, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd3 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 114, 108, 113, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd4 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 97, 100, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd5 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 109, 117, 108, 108, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd6 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [109, 111, 118, 113, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: XM,
                arg1: XMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd7 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 32 as libc::c_int as libc::c_char,
                name:
                    [112, 109, 111, 118, 109, 115, 107, 98, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: REGONLY,
                arg1: XMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd8 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 117, 98, 117, 115, 98, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd9 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 117, 98, 117, 115, 119, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xda as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 109, 105, 110, 117, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdb as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [112, 97, 110, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdc as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 97, 100, 100, 117, 115, 98, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdd as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 97, 100, 100, 117, 115, 119, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xde as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 109, 97, 120, 117, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xdf as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 97, 110, 100, 110, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xe0 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 97, 118, 103, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xe1 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 114, 97, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xe2 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 114, 97, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xe3 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 97, 118, 103, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xe4 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 109, 117, 108, 104, 117, 119, 0, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xe5 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 109, 117, 108, 104, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xe6 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [99, 118, 116, 116, 112, 100, 50, 100, 113, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xe7 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [109, 111, 118, 110, 116, 100, 113, 0, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: MEM,
                arg1: XMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xe8 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 117, 98, 115, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xe9 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 117, 98, 115, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xea as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 109, 105, 110, 115, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xeb as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [112, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xec as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 97, 100, 100, 115, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xed as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 97, 100, 100, 115, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xee as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 109, 97, 120, 115, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xef as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 120, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xf1 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 108, 108, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xf2 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 108, 108, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xf3 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 108, 108, 113, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xf4 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 109, 117, 108, 117, 100, 113, 0, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xf5 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 109, 97, 100, 100, 119, 100, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xf6 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 97, 100, 98, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xf7 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [109, 97, 115, 107, 109, 111, 118, 100, 113, 117, 0, 0, 0,
                     0, 0, 0],
                arg0: XMM,
                arg1: XMMONLY,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xf8 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 117, 98, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xf9 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 117, 98, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xfa as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 117, 98, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xfb as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 117, 98, 113, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xfc as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 97, 100, 100, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xfd as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 97, 100, 100, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xfe as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 97, 100, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     }];
static mut instructions_sse_repne: [op; 21] =
    [{
         let mut init =
             op{opcode: 0x10 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [109, 111, 118, 115, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x11 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [109, 111, 118, 115, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XM,
                arg1: XMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x12 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [109, 111, 118, 100, 100, 117, 112, 0, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x2a as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [99, 118, 116, 115, 105, 50, 115, 100, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: XMM,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x2c as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [99, 118, 116, 116, 115, 100, 50, 115, 105, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: REG,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x2d as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [99, 118, 116, 115, 100, 50, 115, 105, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: REG,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x51 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [115, 113, 114, 116, 115, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x58 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [97, 100, 100, 115, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x59 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [109, 117, 108, 115, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x5a as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [99, 118, 116, 115, 100, 50, 115, 115, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x5c as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [115, 117, 98, 115, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x5d as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [109, 105, 110, 115, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x5e as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [100, 105, 118, 115, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x5f as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [109, 97, 120, 115, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x70 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 104, 117, 102, 108, 119, 0, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0x2 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x7c as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [104, 97, 100, 100, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x7d as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [104, 115, 117, 98, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xc2 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [99, 109, 112, 115, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0x2 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xd0 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [97, 100, 100, 115, 117, 98, 112, 115, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xe6 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [99, 118, 116, 112, 100, 50, 100, 113, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xf0 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [108, 100, 100, 113, 117, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: MEM,
                flags: 0,};
         init
     }];
static mut instructions_sse_repe: [op; 25] =
    [{
         let mut init =
             op{opcode: 0x10 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [109, 111, 118, 115, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x11 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [109, 111, 118, 115, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XM,
                arg1: XMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x12 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [109, 111, 118, 115, 108, 100, 117, 112, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x16 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [109, 111, 118, 115, 104, 100, 117, 112, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x2a as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [99, 118, 116, 115, 105, 50, 115, 115, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: XMM,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x2c as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [99, 118, 116, 116, 115, 115, 50, 115, 105, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: REG,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x2d as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [99, 118, 116, 115, 115, 50, 115, 105, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: REG,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x51 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [115, 113, 114, 116, 115, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x52 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [114, 115, 113, 114, 116, 115, 115, 0, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x53 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [114, 99, 112, 115, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x58 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [97, 100, 100, 115, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x59 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [109, 117, 108, 115, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x5a as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [99, 118, 116, 115, 115, 50, 115, 100, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x5b as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [99, 118, 116, 116, 112, 115, 50, 100, 113, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x5c as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [115, 117, 98, 115, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x5d as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [109, 105, 110, 115, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x5e as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [100, 105, 118, 115, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x5f as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [109, 97, 120, 115, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x6f as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [109, 111, 118, 100, 113, 117, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x70 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 104, 117, 102, 104, 119, 0, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0x2 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x7e as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [109, 111, 118, 113, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x7f as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [109, 111, 118, 100, 113, 117, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XM,
                arg1: XMM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xb8 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 16 as libc::c_int as libc::c_char,
                name:
                    [112, 111, 112, 99, 110, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: REG,
                arg1: RM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0xc2 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [99, 109, 112, 115, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0x2 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0xe6 as libc::c_int as word,
                subcode: 8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [99, 118, 116, 100, 113, 50, 112, 100, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     }];
static mut instructions_sse_single: [op; 18] =
    [{
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 104, 117, 102, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0x1 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 104, 97, 100, 100, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0x2 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 104, 97, 100, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0x3 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 104, 97, 100, 100, 115, 119, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0x4 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 109, 97, 100, 100, 117, 98, 115, 119, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0x5 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 104, 115, 117, 98, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0x6 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 104, 115, 117, 98, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0x7 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 104, 115, 117, 98, 115, 119, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0x8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 105, 103, 110, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0x9 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 105, 103, 110, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0xa as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 105, 103, 110, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0xb as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 109, 117, 108, 104, 114, 115, 119, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0x1c as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [112, 97, 98, 115, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0x1d as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 97, 98, 115, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0x1e as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 97, 98, 115, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MMX,
                arg1: MM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0xf0 as libc::c_int as byte,
                size: 16 as libc::c_int as libc::c_char,
                name:
                    [109, 111, 118, 98, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: REG,
                arg1: MEM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0xf1 as libc::c_int as byte,
                size: 16 as libc::c_int as libc::c_char,
                name:
                    [109, 111, 118, 98, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MEM,
                arg1: REG,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x3a as libc::c_int as word,
                subcode: 0xf as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 97, 108, 105, 103, 110, 114, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: MMX,
                arg1: MM,
                flags: 0x2 as libc::c_int as dword,};
         init
     }];
static mut instructions_sse_single_op32: [op; 69] =
    [{
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 104, 117, 102, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0x1 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 104, 97, 100, 100, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0x2 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 104, 97, 100, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0x3 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 104, 97, 100, 100, 115, 119, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0x4 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 109, 97, 100, 100, 117, 98, 115, 119, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0x5 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 104, 115, 117, 98, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0x6 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 104, 115, 117, 98, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0x7 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 104, 115, 117, 98, 115, 119, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0x8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 105, 103, 110, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0x9 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 105, 103, 110, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0xa as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 115, 105, 103, 110, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0xb as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 109, 117, 108, 104, 114, 115, 119, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0x10 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 98, 108, 101, 110, 100, 118, 98, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0x14 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [98, 108, 101, 110, 100, 118, 112, 115, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0x15 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [98, 108, 101, 110, 100, 118, 112, 100, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0x17 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0x1c as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name: [112, 97, 98, 115, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0x1d as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 97, 98, 115, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0x1e as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 97, 98, 115, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0x20 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 109, 111, 118, 115, 120, 98, 119, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0x21 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 109, 111, 118, 115, 120, 98, 100, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0x22 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 109, 111, 118, 115, 120, 98, 113, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0x23 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 109, 111, 118, 115, 120, 119, 100, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0x24 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 109, 111, 118, 115, 120, 119, 113, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0x25 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 109, 111, 118, 115, 120, 100, 113, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0x28 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 109, 117, 108, 100, 113, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0x29 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 99, 109, 112, 101, 113, 113, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0x2a as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [109, 111, 118, 110, 116, 100, 113, 97, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: XMM,
                arg1: MEM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0x2b as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 97, 99, 107, 117, 115, 100, 119, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0x30 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 109, 111, 118, 122, 120, 98, 119, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0x31 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 109, 111, 118, 122, 120, 98, 100, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0x32 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 109, 111, 118, 122, 120, 98, 113, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0x33 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 109, 111, 118, 122, 120, 119, 100, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0x34 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 109, 111, 118, 122, 120, 119, 113, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0x35 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 109, 111, 118, 122, 120, 100, 113, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0x37 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 99, 109, 112, 103, 116, 113, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0x38 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 109, 105, 110, 115, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0x39 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 109, 105, 110, 115, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0x3a as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 109, 105, 110, 117, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0x3b as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 109, 105, 110, 117, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0x3c as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 109, 97, 120, 115, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0x3d as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 109, 97, 120, 115, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0x3e as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 109, 97, 120, 117, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0x3f as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 109, 97, 120, 117, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0x40 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 109, 97, 120, 108, 108, 100, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x38 as libc::c_int as word,
                subcode: 0x41 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 104, 109, 105, 110, 112, 111, 115, 117, 119, 0, 0,
                     0, 0, 0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x3a as libc::c_int as word,
                subcode: 0x8 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [114, 111, 117, 110, 100, 112, 115, 0, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0x2 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x3a as libc::c_int as word,
                subcode: 0x9 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [114, 111, 117, 110, 100, 112, 100, 0, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0x2 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x3a as libc::c_int as word,
                subcode: 0xa as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [114, 111, 117, 110, 100, 115, 115, 0, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0x2 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x3a as libc::c_int as word,
                subcode: 0xb as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [114, 111, 117, 110, 100, 115, 100, 0, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0x2 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x3a as libc::c_int as word,
                subcode: 0xc as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [98, 108, 101, 110, 100, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0x2 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x3a as libc::c_int as word,
                subcode: 0xd as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [98, 108, 101, 110, 100, 112, 100, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0x2 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x3a as libc::c_int as word,
                subcode: 0xe as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 98, 108, 101, 110, 100, 119, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0x2 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x3a as libc::c_int as word,
                subcode: 0xf as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 97, 108, 105, 103, 110, 114, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0x2 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x3a as libc::c_int as word,
                subcode: 0x14 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 101, 120, 116, 114, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: RM,
                arg1: XMM,
                flags: 0x2 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x3a as libc::c_int as word,
                subcode: 0x15 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 101, 120, 116, 114, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: RM,
                arg1: XMM,
                flags: 0x2 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x3a as libc::c_int as word,
                subcode: 0x16 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 101, 120, 116, 114, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: RM,
                arg1: XMM,
                flags: 0x2 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x3a as libc::c_int as word,
                subcode: 0x17 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [101, 120, 116, 114, 97, 99, 116, 112, 115, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: RM,
                arg1: XMM,
                flags: 0x2 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x3a as libc::c_int as word,
                subcode: 0x20 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 105, 110, 115, 114, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: RM,
                flags: 0x2 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x3a as libc::c_int as word,
                subcode: 0x21 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [105, 110, 115, 101, 114, 116, 112, 115, 0, 0, 0, 0, 0, 0,
                     0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0x2 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x3a as libc::c_int as word,
                subcode: 0x22 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 105, 110, 115, 114, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: RM,
                flags: 0x2 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x3a as libc::c_int as word,
                subcode: 0x40 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [100, 112, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x3a as libc::c_int as word,
                subcode: 0x41 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [100, 112, 112, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0,};
         init
     },
     {
         let mut init =
             op{opcode: 0x3a as libc::c_int as word,
                subcode: 0x42 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [109, 112, 115, 113, 100, 98, 119, 0, 0, 0, 0, 0, 0, 0, 0,
                     0],
                arg0: XMM,
                arg1: XM,
                flags: 0x2 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x3a as libc::c_int as word,
                subcode: 0x44 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 99, 108, 109, 117, 108, 113, 100, 113, 0, 0, 0, 0,
                     0, 0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0x2 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x3a as libc::c_int as word,
                subcode: 0x60 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 99, 109, 112, 101, 115, 116, 114, 109, 0, 0, 0, 0,
                     0, 0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0x2 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x3a as libc::c_int as word,
                subcode: 0x61 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 99, 109, 112, 101, 115, 116, 114, 105, 0, 0, 0, 0,
                     0, 0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0x2 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x3a as libc::c_int as word,
                subcode: 0x62 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 99, 109, 112, 105, 115, 116, 114, 109, 0, 0, 0, 0,
                     0, 0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0x2 as libc::c_int as dword,};
         init
     },
     {
         let mut init =
             op{opcode: 0x3a as libc::c_int as word,
                subcode: 0x63 as libc::c_int as byte,
                size: 0 as libc::c_int as libc::c_char,
                name:
                    [112, 99, 109, 112, 105, 115, 116, 114, 105, 0, 0, 0, 0,
                     0, 0, 0],
                arg0: XMM,
                arg1: XM,
                flags: 0x2 as libc::c_int as dword,};
         init
     }];
/* returns the flag if it's a prefix, 0 otherwise */
unsafe extern "C" fn get_prefix(mut opcode: word, mut bits: libc::c_int)
 -> word {
    if bits == 64 as libc::c_int {
        if opcode as libc::c_int & 0xfff0 as libc::c_int ==
               0x40 as libc::c_int {
            return (0x800 as libc::c_int |
                        (opcode as libc::c_int & 0xf as libc::c_int) *
                            0x1000 as libc::c_int) as word
        }
    }
    match opcode as libc::c_int {
        38 => { return 0x1 as libc::c_int as word }
        46 => { return 0x2 as libc::c_int as word }
        54 => { return 0x3 as libc::c_int as word }
        62 => { return 0x4 as libc::c_int as word }
        100 => { return 0x5 as libc::c_int as word }
        101 => { return 0x6 as libc::c_int as word }
        102 => { return 0x8 as libc::c_int as word }
        103 => { return 0x10 as libc::c_int as word }
        155 => { return 0x100 as libc::c_int as word }
        240 => { return 0x20 as libc::c_int as word }
        242 => { return 0x40 as libc::c_int as word }
        243 => { return 0x80 as libc::c_int as word }
        _ => { return 0 as libc::c_int as word }
    };
}
unsafe extern "C" fn instr_matches(opcode: byte, subcode: byte,
                                   mut op: *const op) -> libc::c_int {
    return (opcode as libc::c_int == (*op).opcode as libc::c_int &&
                ((*op).subcode as libc::c_int == 8 as libc::c_int ||
                     subcode as libc::c_int == (*op).subcode as libc::c_int))
               as libc::c_int;
}
/* aka 3 byte opcode */
unsafe extern "C" fn get_sse_single(mut opcode: byte, mut subcode: byte,
                                    mut instr: *mut instr) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if (*instr).prefix as libc::c_int & 0x8 as libc::c_int != 0 {
        i = 0 as libc::c_int;
        while (i as libc::c_ulong) <
                  (::std::mem::size_of::<[op; 69]>() as
                       libc::c_ulong).wrapping_div(::std::mem::size_of::<op>()
                                                       as libc::c_ulong) {
            if instructions_sse_single_op32[i as usize].opcode as libc::c_int
                   == opcode as libc::c_int &&
                   instructions_sse_single_op32[i as usize].subcode as
                       libc::c_int == subcode as libc::c_int {
                (*instr).op = instructions_sse_single_op32[i as usize];
                (*instr).prefix =
                    ((*instr).prefix as libc::c_int & !(0x8 as libc::c_int))
                        as word;
                return 1 as libc::c_int
            }
            i += 1
        }
    } else {
        i = 0 as libc::c_int;
        while (i as libc::c_ulong) <
                  (::std::mem::size_of::<[op; 18]>() as
                       libc::c_ulong).wrapping_div(::std::mem::size_of::<op>()
                                                       as libc::c_ulong) {
            if instructions_sse_single[i as usize].opcode as libc::c_int ==
                   opcode as libc::c_int &&
                   instructions_sse_single[i as usize].subcode as libc::c_int
                       == subcode as libc::c_int {
                (*instr).op = instructions_sse_single[i as usize];
                return 1 as libc::c_int
            }
            i += 1
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn get_sse_instr(mut p: *const byte, mut instr: *mut instr)
 -> libc::c_int {
    let mut subcode: byte =
        (*p.offset(1 as libc::c_int as isize) as libc::c_int >>
             3 as libc::c_int & 7 as libc::c_int) as byte;
    let mut i: libc::c_uint = 0;
    /* Clear the prefix if it matches. This makes the disassembler work right,
     * but it might break things later if we want to interpret these. The
     * solution in that case is probably to modify the size/name instead. */
    if (*instr).prefix as libc::c_int & 0x8 as libc::c_int != 0 {
        i = 0 as libc::c_int as libc::c_uint;
        while (i as libc::c_ulong) <
                  (::std::mem::size_of::<[op; 114]>() as
                       libc::c_ulong).wrapping_div(::std::mem::size_of::<op>()
                                                       as libc::c_ulong) {
            if instr_matches(*p.offset(0 as libc::c_int as isize), subcode,
                             &*instructions_sse_op32.as_ptr().offset(i as
                                                                         isize))
                   != 0 {
                (*instr).op = instructions_sse_op32[i as usize];
                (*instr).prefix =
                    ((*instr).prefix as libc::c_int & !(0x8 as libc::c_int))
                        as word;
                return 0 as libc::c_int
            }
            i = i.wrapping_add(1)
        }
    } else if (*instr).prefix as libc::c_int & 0x40 as libc::c_int != 0 {
        i = 0 as libc::c_int as libc::c_uint;
        while (i as libc::c_ulong) <
                  (::std::mem::size_of::<[op; 21]>() as
                       libc::c_ulong).wrapping_div(::std::mem::size_of::<op>()
                                                       as libc::c_ulong) {
            if instr_matches(*p.offset(0 as libc::c_int as isize), subcode,
                             &*instructions_sse_repne.as_ptr().offset(i as
                                                                          isize))
                   != 0 {
                (*instr).op = instructions_sse_repne[i as usize];
                (*instr).prefix =
                    ((*instr).prefix as libc::c_int & !(0x40 as libc::c_int))
                        as word;
                return 0 as libc::c_int
            }
            i = i.wrapping_add(1)
        }
    } else if (*instr).prefix as libc::c_int & 0x80 as libc::c_int != 0 {
        i = 0 as libc::c_int as libc::c_uint;
        while (i as libc::c_ulong) <
                  (::std::mem::size_of::<[op; 25]>() as
                       libc::c_ulong).wrapping_div(::std::mem::size_of::<op>()
                                                       as libc::c_ulong) {
            if instr_matches(*p.offset(0 as libc::c_int as isize), subcode,
                             &*instructions_sse_repe.as_ptr().offset(i as
                                                                         isize))
                   != 0 {
                (*instr).op = instructions_sse_repe[i as usize];
                (*instr).prefix =
                    ((*instr).prefix as libc::c_int & !(0x80 as libc::c_int))
                        as word;
                return 0 as libc::c_int
            }
            i = i.wrapping_add(1)
        }
    } else {
        i = 0 as libc::c_int as libc::c_uint;
        while (i as libc::c_ulong) <
                  (::std::mem::size_of::<[op; 109]>() as
                       libc::c_ulong).wrapping_div(::std::mem::size_of::<op>()
                                                       as libc::c_ulong) {
            if instr_matches(*p.offset(0 as libc::c_int as isize), subcode,
                             &*instructions_sse.as_ptr().offset(i as isize))
                   != 0 {
                (*instr).op = instructions_sse[i as usize];
                return 0 as libc::c_int
            }
            i = i.wrapping_add(1)
        }
    }
    return get_sse_single(*p.offset(0 as libc::c_int as isize),
                          *p.offset(1 as libc::c_int as isize), instr);
}
unsafe extern "C" fn get_0f_instr(mut p: *const byte, mut instr: *mut instr)
 -> libc::c_int {
    let mut subcode: byte =
        (*p.offset(1 as libc::c_int as isize) as libc::c_int >>
             3 as libc::c_int & 7 as libc::c_int) as byte;
    let mut i: libc::c_uint = 0;
    let mut len: libc::c_int = 0;
    /* a couple of special (read: annoying) cases first */
    if *p.offset(0 as libc::c_int as isize) as libc::c_int ==
           0x1 as libc::c_int &&
           *p.offset(1 as libc::c_int as isize) as libc::c_int >>
               6 as libc::c_int == 3 as libc::c_int {
        (*instr).op.opcode = 0xf01 as libc::c_int as word;
        (*instr).op.subcode = *p.offset(1 as libc::c_int as isize);
        match *p.offset(1 as libc::c_int as isize) as libc::c_int {
            193 => {
                strcpy((*instr).op.name.as_mut_ptr(),
                       b"vmcall\x00" as *const u8 as *const libc::c_char);
            }
            194 => {
                strcpy((*instr).op.name.as_mut_ptr(),
                       b"vmlaunch\x00" as *const u8 as *const libc::c_char);
            }
            195 => {
                strcpy((*instr).op.name.as_mut_ptr(),
                       b"vmresume\x00" as *const u8 as *const libc::c_char);
            }
            196 => {
                strcpy((*instr).op.name.as_mut_ptr(),
                       b"vmcall\x00" as *const u8 as *const libc::c_char);
            }
            200 => {
                strcpy((*instr).op.name.as_mut_ptr(),
                       b"monitor\x00" as *const u8 as *const libc::c_char);
            }
            201 => {
                strcpy((*instr).op.name.as_mut_ptr(),
                       b"mwait\x00" as *const u8 as *const libc::c_char);
            }
            208 => {
                strcpy((*instr).op.name.as_mut_ptr(),
                       b"xgetbv\x00" as *const u8 as *const libc::c_char);
            }
            209 => {
                strcpy((*instr).op.name.as_mut_ptr(),
                       b"xsetbv\x00" as *const u8 as *const libc::c_char);
            }
            249 => {
                strcpy((*instr).op.name.as_mut_ptr(),
                       b"rdtscp\x00" as *const u8 as *const libc::c_char);
            }
            _ => { }
        }
        return 1 as libc::c_int
    } else {
        if *p.offset(0 as libc::c_int as isize) as libc::c_int ==
               0xae as libc::c_int &&
               *p.offset(1 as libc::c_int as isize) as libc::c_int >>
                   6 as libc::c_int == 3 as libc::c_int {
            (*instr).op.opcode = 0xfae as libc::c_int as word;
            (*instr).op.subcode = subcode;
            if subcode as libc::c_int == 0x5 as libc::c_int {
                strcpy((*instr).op.name.as_mut_ptr(),
                       b"lfence\x00" as *const u8 as *const libc::c_char);
            }
            if subcode as libc::c_int == 0x6 as libc::c_int {
                strcpy((*instr).op.name.as_mut_ptr(),
                       b"mfence\x00" as *const u8 as *const libc::c_char);
            }
            if subcode as libc::c_int == 0x7 as libc::c_int {
                strcpy((*instr).op.name.as_mut_ptr(),
                       b"sfence\x00" as *const u8 as *const libc::c_char);
            }
            return 1 as libc::c_int
        }
    }
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) <
              (::std::mem::size_of::<[op; 133]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<op>() as
                                                   libc::c_ulong) {
        if instr_matches(*p.offset(0 as libc::c_int as isize), subcode,
                         &*instructions_0F.as_ptr().offset(i as isize)) != 0 {
            (*instr).op = instructions_0F[i as usize];
            len = 0 as libc::c_int;
            break ;
        } else { i = i.wrapping_add(1) }
    }
    if (*instr).op.name[0 as libc::c_int as usize] == 0 {
        len = get_sse_instr(p, instr)
    }
    (*instr).op.opcode =
        (0xf00 as libc::c_int |
             *p.offset(0 as libc::c_int as isize) as libc::c_int) as word;
    return len;
}
/* Parameters:
 * ip      - [i] NOT current IP, but rather IP of the *argument*. This
 *               is necessary for REL16 to work right.
 * p       - [i] pointer to the current argument to be parsed
 * arg     - [i/o] pointer to the relevant arg struct
 *      ->ip         [o]
 *      ->value      [o]
 *      ->type       [i]
 * instr   - [i/o] pointer to the relevant instr struct
 *      ->prefix     [i]
 *      ->op         [i]
 *      ->modrm_disp [o]
 *      ->modrm_reg  [o]
 * is32    - [i] bitness—REL16 and MOFFS16 are affected by bitness but can't be overridden
 *
 * Returns: number of bytes processed
 *
 * Does not process specific arguments (e.g. registers, DSBX, ONE...)
 * The parameter out is given as a dword but may require additional casting.
 */
unsafe extern "C" fn get_arg(mut ip: dword, mut p: *const byte,
                             mut arg: *mut arg, mut instr: *mut instr,
                             mut bits: libc::c_int) -> libc::c_int {
    (*arg).value = 0 as libc::c_int as qword; /* signed */
    match (*arg).type_0 as libc::c_uint {
        27 => {
            (*arg).ip = ip;
            (*arg).value = *p as qword;
            return 1 as libc::c_int
        }
        28 => {
            (*arg).ip = ip;
            (*arg).value = *(p as *mut word) as qword;
            return 2 as libc::c_int
        }
        29 => {
            (*arg).ip = ip;
            if (*instr).op.size as libc::c_int == 8 as libc::c_int {
                (*arg).value = *p as qword;
                return 1 as libc::c_int
            } else if (*instr).op.size as libc::c_int == 16 as libc::c_int {
                (*arg).value = *(p as *mut word) as qword;
                return 2 as libc::c_int
            } else if (*instr).op.size as libc::c_int == 64 as libc::c_int &&
                          (*instr).op.flags &
                              0x800 as libc::c_int as libc::c_uint != 0 {
                (*arg).value = *(p as *mut qword);
                return 8 as libc::c_int
            } else {
                (*arg).value = *(p as *mut dword) as qword;
                return 4 as libc::c_int
            }
        }
        30 => {
            (*arg).ip = ip;
            (*arg).value =
                ip.wrapping_add(1 as libc::c_int as
                                    libc::c_uint).wrapping_add(*(p as
                                                                     *mut int8_t)
                                                                   as
                                                                   libc::c_uint)
                    as qword;
            return 1 as libc::c_int
        }
        31 => {
            (*arg).ip = ip;
            /* Equivalently signed or unsigned (i.e. clipped) */
            if bits == 16 as libc::c_int {
                (*arg).value =
                    (ip.wrapping_add(2 as libc::c_int as
                                         libc::c_uint).wrapping_add(*(p as
                                                                          *mut word)
                                                                        as
                                                                        libc::c_uint)
                         & 0xffff as libc::c_int as libc::c_uint) as
                        qword; /* I think this should be enough */
                return 2 as libc::c_int
            } else {
                (*arg).value =
                    (ip.wrapping_add(4 as libc::c_int as
                                         libc::c_uint).wrapping_add(*(p as
                                                                          *mut dword))
                         & 0xffffffff as libc::c_uint) as qword;
                return 4 as libc::c_int
            }
        }
        32 => {
            (*arg).ip = ip;
            (*arg).value = *(p as *mut word) as qword;
            return 4 as libc::c_int
        }
        33 => {
            (*arg).ip = ip;
            if bits == 64 as libc::c_int {
                (*arg).value = *(p as *mut qword);
                return 8 as libc::c_int
            } else if bits == 32 as libc::c_int {
                (*arg).value = *(p as *mut dword) as qword;
                return 4 as libc::c_int
            } else {
                (*arg).value = *(p as *mut word) as qword;
                return 2 as libc::c_int
            }
        }
        37 | 40 | 38 | 39 => {
            let mut mod_0: byte =
                (*p as libc::c_int >> 6 as libc::c_int) as byte;
            let mut rm: byte = (*p as libc::c_int & 7 as libc::c_int) as byte;
            let mut ret: libc::c_int = 1 as libc::c_int;
            if mod_0 as libc::c_int == 3 as libc::c_int {
                (*instr).modrm_disp = DISP_REG;
                (*instr).modrm_reg = rm as libc::c_char;
                if (*instr).prefix as libc::c_int & 0x1000 as libc::c_int != 0
                   {
                    (*instr).modrm_reg =
                        ((*instr).modrm_reg as libc::c_int + 8 as libc::c_int)
                            as libc::c_char
                }
                return 1 as libc::c_int
            }
            (*arg).ip = ip;
            if (*instr).addrsize as libc::c_int != 16 as libc::c_int &&
                   rm as libc::c_int == 4 as libc::c_int {
                /* SIB byte */
                p = p.offset(1);
                (*instr).sib_scale =
                    ((1 as libc::c_int) <<
                         (*p as libc::c_int >> 6 as libc::c_int)) as byte;
                (*instr).sib_index =
                    (*p as libc::c_int >> 3 as libc::c_int & 7 as libc::c_int)
                        as libc::c_char;
                if (*instr).prefix as libc::c_int & 0x2000 as libc::c_int != 0
                   {
                    (*instr).sib_index =
                        ((*instr).sib_index as libc::c_int + 8 as libc::c_int)
                            as libc::c_char
                }
                if (*instr).sib_index as libc::c_int == 4 as libc::c_int {
                    (*instr).sib_index = -(1 as libc::c_int) as libc::c_char
                }
                rm = (*p as libc::c_int & 7 as libc::c_int) as byte;
                ret += 1
            }
            if mod_0 as libc::c_int == 0 as libc::c_int &&
                   bits == 64 as libc::c_int &&
                   rm as libc::c_int == 5 as libc::c_int &&
                   (*instr).sib_scale == 0 {
                /* IP-relative addressing... */
                (*arg).value =
                    *(p.offset(1 as libc::c_int as isize) as *mut dword) as
                        qword;
                (*instr).modrm_disp = DISP_16;
                (*instr).modrm_reg = 16 as libc::c_int as libc::c_char;
                ret += 4 as libc::c_int
            } else if mod_0 as libc::c_int == 0 as libc::c_int &&
                          ((*instr).addrsize as libc::c_int ==
                               16 as libc::c_int &&
                               rm as libc::c_int == 6 as libc::c_int ||
                               (*instr).addrsize as libc::c_int !=
                                   16 as libc::c_int &&
                                   rm as libc::c_int == 5 as libc::c_int) {
                if (*instr).addrsize as libc::c_int == 16 as libc::c_int {
                    (*arg).value =
                        *(p.offset(1 as libc::c_int as isize) as *mut word) as
                            qword;
                    ret += 2 as libc::c_int
                } else {
                    (*arg).value =
                        *(p.offset(1 as libc::c_int as isize) as *mut dword)
                            as qword;
                    ret += 4 as libc::c_int
                }
                (*instr).modrm_disp = DISP_16;
                (*instr).modrm_reg = -(1 as libc::c_int) as libc::c_char
            } else if mod_0 as libc::c_int == 0 as libc::c_int {
                (*instr).modrm_disp = DISP_NONE;
                (*instr).modrm_reg = rm as libc::c_char;
                if (*instr).prefix as libc::c_int & 0x1000 as libc::c_int != 0
                   {
                    (*instr).modrm_reg =
                        ((*instr).modrm_reg as libc::c_int + 8 as libc::c_int)
                            as libc::c_char
                }
            } else if mod_0 as libc::c_int == 1 as libc::c_int {
                (*arg).value = *p.offset(1 as libc::c_int as isize) as qword;
                (*instr).modrm_disp = DISP_8;
                (*instr).modrm_reg = rm as libc::c_char;
                if (*instr).prefix as libc::c_int & 0x1000 as libc::c_int != 0
                   {
                    (*instr).modrm_reg =
                        ((*instr).modrm_reg as libc::c_int + 8 as libc::c_int)
                            as libc::c_char
                }
                ret += 1 as libc::c_int
            } else if mod_0 as libc::c_int == 2 as libc::c_int {
                if (*instr).addrsize as libc::c_int == 16 as libc::c_int {
                    (*arg).value =
                        *(p.offset(1 as libc::c_int as isize) as *mut word) as
                            qword;
                    ret += 2 as libc::c_int
                } else {
                    (*arg).value =
                        *(p.offset(1 as libc::c_int as isize) as *mut dword)
                            as qword;
                    ret += 4 as libc::c_int
                }
                (*instr).modrm_disp = DISP_16;
                (*instr).modrm_reg = rm as libc::c_char;
                if (*instr).prefix as libc::c_int & 0x1000 as libc::c_int != 0
                   {
                    (*instr).modrm_reg =
                        ((*instr).modrm_reg as libc::c_int + 8 as libc::c_int)
                            as libc::c_char
                }
            }
            return ret
        }
        44 | 46 | 49 | 50 | 51 => {
            /* doesn't exist in 64-bit mode */
            (*arg).value =
                (*p as libc::c_int >> 3 as libc::c_int & 7 as libc::c_int) as
                    qword;
            if (*instr).prefix as libc::c_int & 0x4000 as libc::c_int != 0 {
                (*arg).value =
                    ((*arg).value as
                         libc::c_ulong).wrapping_add(8 as libc::c_int as
                                                         libc::c_ulong) as
                        qword as qword
            }
            return 0 as libc::c_int
        }
        45 | 47 => {
            (*arg).value =
                (*p as libc::c_int >> 3 as libc::c_int & 7 as libc::c_int) as
                    qword;
            return 0 as libc::c_int
        }
        48 | 53 | 41 | 42 | 43 => {
            (*arg).value = (*p as libc::c_int & 7 as libc::c_int) as qword;
            if (*instr).prefix as libc::c_int & 0x1000 as libc::c_int != 0 {
                (*arg).value =
                    ((*arg).value as
                         libc::c_ulong).wrapping_add(8 as libc::c_int as
                                                         libc::c_ulong) as
                        qword as qword
            }
            return 1 as libc::c_int
        }
        _ => {
            /* all others should be implicit */
            return 0 as libc::c_int
        }
    };
}
#[no_mangle]
pub static mut seg16: [[libc::c_char; 3]; 6] =
    [[101, 115, 0], [99, 115, 0], [115, 115, 0], [100, 115, 0], [102, 115, 0],
     [103, 115, 0]];
static mut reg8: [[libc::c_char; 3]; 8] =
    [[97, 108, 0], [99, 108, 0], [100, 108, 0], [98, 108, 0], [97, 104, 0],
     [99, 104, 0], [100, 104, 0], [98, 104, 0]];
static mut reg8_rex: [[libc::c_char; 5]; 16] =
    [[97, 108, 0, 0, 0], [99, 108, 0, 0, 0], [100, 108, 0, 0, 0],
     [98, 108, 0, 0, 0], [115, 112, 108, 0, 0], [98, 112, 108, 0, 0],
     [115, 105, 108, 0, 0], [100, 105, 108, 0, 0], [114, 56, 98, 0, 0],
     [114, 57, 98, 0, 0], [114, 49, 48, 98, 0], [114, 49, 49, 98, 0],
     [114, 49, 50, 98, 0], [114, 49, 51, 98, 0], [114, 49, 52, 98, 0],
     [114, 49, 53, 98, 0]];
static mut reg16: [[libc::c_char; 5]; 16] =
    [[97, 120, 0, 0, 0], [99, 120, 0, 0, 0], [100, 120, 0, 0, 0],
     [98, 120, 0, 0, 0], [115, 112, 0, 0, 0], [98, 112, 0, 0, 0],
     [115, 105, 0, 0, 0], [100, 105, 0, 0, 0], [114, 56, 119, 0, 0],
     [114, 57, 119, 0, 0], [114, 49, 48, 119, 0], [114, 49, 49, 119, 0],
     [114, 49, 50, 119, 0], [114, 49, 51, 119, 0], [114, 49, 52, 119, 0],
     [114, 49, 53, 119, 0]];
static mut reg32: [[libc::c_char; 5]; 17] =
    [[101, 97, 120, 0, 0], [101, 99, 120, 0, 0], [101, 100, 120, 0, 0],
     [101, 98, 120, 0, 0], [101, 115, 112, 0, 0], [101, 98, 112, 0, 0],
     [101, 115, 105, 0, 0], [101, 100, 105, 0, 0], [114, 56, 100, 0, 0],
     [114, 57, 100, 0, 0], [114, 49, 48, 100, 0], [114, 49, 49, 100, 0],
     [114, 49, 50, 100, 0], [114, 49, 51, 100, 0], [114, 49, 52, 100, 0],
     [114, 49, 53, 100, 0], [101, 105, 112, 0, 0]];
static mut reg64: [[libc::c_char; 4]; 17] =
    [[114, 97, 120, 0], [114, 99, 120, 0], [114, 100, 120, 0],
     [114, 98, 120, 0], [114, 115, 112, 0], [114, 98, 112, 0],
     [114, 115, 105, 0], [114, 100, 105, 0], [114, 56, 0, 0], [114, 57, 0, 0],
     [114, 49, 48, 0], [114, 49, 49, 0], [114, 49, 50, 0], [114, 49, 51, 0],
     [114, 49, 52, 0], [114, 49, 53, 0], [114, 105, 112, 0]];
unsafe extern "C" fn get_seg16(mut out: *mut libc::c_char, mut reg: byte) {
    if asm_syntax as libc::c_uint == GAS as libc::c_int as libc::c_uint {
        strcat(out, b"%\x00" as *const u8 as *const libc::c_char);
    }
    strcat(out, seg16[reg as usize].as_ptr());
}
unsafe extern "C" fn get_reg8(mut out: *mut libc::c_char, mut reg: byte,
                              mut rex: libc::c_int) {
    if asm_syntax as libc::c_uint == GAS as libc::c_int as libc::c_uint {
        strcat(out, b"%\x00" as *const u8 as *const libc::c_char);
    }
    strcat(out,
           if rex != 0 {
               reg8_rex[reg as usize].as_ptr()
           } else { reg8[reg as usize].as_ptr() });
}
unsafe extern "C" fn get_reg16(mut out: *mut libc::c_char, mut reg: byte,
                               mut size: libc::c_int) {
    if reg as libc::c_int != -(1 as libc::c_int) {
        if asm_syntax as libc::c_uint == GAS as libc::c_int as libc::c_uint {
            strcat(out, b"%\x00" as *const u8 as *const libc::c_char);
        }
        if size == 16 as libc::c_int {
            strcat(out, reg16[reg as usize].as_ptr());
        }
        if size == 32 as libc::c_int {
            strcat(out, reg32[reg as usize].as_ptr());
        } else if size == 64 as libc::c_int {
            strcat(out, reg64[reg as usize].as_ptr());
        }
    };
}
unsafe extern "C" fn get_xmm(mut out: *mut libc::c_char, mut reg: byte) {
    if asm_syntax as libc::c_uint == GAS as libc::c_int as libc::c_uint {
        strcat(out, b"%\x00" as *const u8 as *const libc::c_char);
    }
    strcat(out, b"xmm0\x00" as *const u8 as *const libc::c_char);
    *out.offset(strlen(out).wrapping_sub(1 as libc::c_int as libc::c_ulong) as
                    isize) =
        ('0' as i32 + reg as libc::c_int) as libc::c_char;
}
unsafe extern "C" fn get_mmx(mut out: *mut libc::c_char, mut reg: byte) {
    if asm_syntax as libc::c_uint == GAS as libc::c_int as libc::c_uint {
        strcat(out, b"%\x00" as *const u8 as *const libc::c_char);
    }
    strcat(out, b"mm0\x00" as *const u8 as *const libc::c_char);
    *out.offset(strlen(out).wrapping_sub(1 as libc::c_int as libc::c_ulong) as
                    isize) =
        ('0' as i32 + reg as libc::c_int) as libc::c_char;
}
static mut modrm16_gas: [[libc::c_char; 8]; 8] =
    [[37, 98, 120, 44, 37, 115, 105, 0], [37, 98, 120, 44, 37, 100, 105, 0],
     [37, 98, 112, 44, 37, 115, 105, 0], [37, 98, 112, 44, 37, 100, 105, 0],
     [37, 115, 105, 0, 0, 0, 0, 0], [37, 100, 105, 0, 0, 0, 0, 0],
     [37, 98, 112, 0, 0, 0, 0, 0], [37, 98, 120, 0, 0, 0, 0, 0]];
static mut modrm16_masm: [[libc::c_char; 6]; 8] =
    [[98, 120, 43, 115, 105, 0], [98, 120, 43, 100, 105, 0],
     [98, 112, 43, 115, 105, 0], [98, 112, 43, 100, 105, 0],
     [115, 105, 0, 0, 0, 0], [100, 105, 0, 0, 0, 0], [98, 112, 0, 0, 0, 0],
     [98, 120, 0, 0, 0, 0]];
/* Figure out whether it's a register, so we know whether to dispense with size
 * indicators on a memory access. */
unsafe extern "C" fn is_reg(mut arg: argtype) -> libc::c_int {
    return (arg as libc::c_uint >= AL as libc::c_int as libc::c_uint &&
                arg as libc::c_uint <= GS as libc::c_int as libc::c_uint ||
                arg as libc::c_uint >= REG as libc::c_int as libc::c_uint &&
                    arg as libc::c_uint <=
                        TR32 as libc::c_int as libc::c_uint) as libc::c_int;
}
/* With MASM/NASM, use capital letters to help disambiguate them from the following 'h'. */
unsafe extern "C" fn print_arg(mut ip: *mut libc::c_char,
                               mut instr: *mut instr, mut i: libc::c_int,
                               mut bits: libc::c_int) {
    let mut arg: *mut arg =
        &mut *(*instr).args.as_mut_ptr().offset(i as isize) as
            *mut arg; /* someone wants to print something special */
    let mut out: *mut libc::c_char = (*arg).string.as_mut_ptr();
    let mut value: qword = (*arg).value;
    if (*arg).string[0 as libc::c_int as usize] != 0 { return }
    if (*arg).type_0 as libc::c_uint >= AL as libc::c_int as libc::c_uint &&
           (*arg).type_0 as libc::c_uint <= BH as libc::c_int as libc::c_uint
       {
        get_reg8(out,
                 ((*arg).type_0 as
                      libc::c_uint).wrapping_sub(AL as libc::c_int as
                                                     libc::c_uint) as byte,
                 0 as libc::c_int);
    } else if (*arg).type_0 as libc::c_uint >=
                  AX as libc::c_int as libc::c_uint &&
                  (*arg).type_0 as libc::c_uint <=
                      DI as libc::c_int as libc::c_uint {
        get_reg16(out,
                  ((*arg).type_0 as
                       libc::c_uint).wrapping_sub(AX as libc::c_int as
                                                      libc::c_uint).wrapping_add((if (*instr).prefix
                                                                                         as
                                                                                         libc::c_int
                                                                                         &
                                                                                         0x1000
                                                                                             as
                                                                                             libc::c_int
                                                                                         !=
                                                                                         0
                                                                                     {
                                                                                      8
                                                                                          as
                                                                                          libc::c_int
                                                                                  } else {
                                                                                      0
                                                                                          as
                                                                                          libc::c_int
                                                                                  })
                                                                                     as
                                                                                     libc::c_uint)
                      as byte, (*instr).op.size as libc::c_int);
    } else if (*arg).type_0 as libc::c_uint >=
                  ES as libc::c_int as libc::c_uint &&
                  (*arg).type_0 as libc::c_uint <=
                      GS as libc::c_int as libc::c_uint {
        get_seg16(out,
                  ((*arg).type_0 as
                       libc::c_uint).wrapping_sub(ES as libc::c_int as
                                                      libc::c_uint) as byte);
    }
    match (*arg).type_0 as libc::c_uint {
        1 => {
            strcat(out,
                   if asm_syntax as libc::c_uint ==
                          GAS as libc::c_int as libc::c_uint {
                       b"$0x1\x00" as *const u8 as *const libc::c_char
                   } else { b"1h\x00" as *const u8 as *const libc::c_char });
        }
        27 => {
            if (*instr).op.flags & 0x100 as libc::c_int as libc::c_uint != 0 {
                /* 6a */
                if (*instr).op.size as libc::c_int == 64 as libc::c_int {
                    sprintf(out,
                            if asm_syntax as libc::c_uint ==
                                   GAS as libc::c_int as libc::c_uint {
                                b"$0x%016lx\x00" as *const u8 as
                                    *const libc::c_char
                            } else {
                                b"qword %016lxh\x00" as *const u8 as
                                    *const libc::c_char
                            }, value as int8_t as qword);
                } else if (*instr).op.size as libc::c_int == 32 as libc::c_int
                 {
                    sprintf(out,
                            if asm_syntax as libc::c_uint ==
                                   GAS as libc::c_int as libc::c_uint {
                                b"$0x%08x\x00" as *const u8 as
                                    *const libc::c_char
                            } else {
                                b"dword %08Xh\x00" as *const u8 as
                                    *const libc::c_char
                            }, value as int8_t as dword);
                } else {
                    sprintf(out,
                            if asm_syntax as libc::c_uint ==
                                   GAS as libc::c_int as libc::c_uint {
                                b"$0x%04x\x00" as *const u8 as
                                    *const libc::c_char
                            } else {
                                b"word %04Xh\x00" as *const u8 as
                                    *const libc::c_char
                            }, value as int8_t as word as libc::c_int);
                }
            } else {
                sprintf(out,
                        if asm_syntax as libc::c_uint ==
                               GAS as libc::c_int as libc::c_uint {
                            b"$0x%02lx\x00" as *const u8 as
                                *const libc::c_char
                        } else {
                            b"%02lXh\x00" as *const u8 as *const libc::c_char
                        }, value);
            }
        }
        28 => {
            sprintf(out,
                    if asm_syntax as libc::c_uint ==
                           GAS as libc::c_int as libc::c_uint {
                        b"$0x%04lx\x00" as *const u8 as *const libc::c_char
                    } else {
                        b"%04lXh\x00" as *const u8 as *const libc::c_char
                    }, value);
        }
        29 => {
            if (*instr).op.flags & 0x100 as libc::c_int as libc::c_uint != 0 {
                if (*instr).op.size as libc::c_int == 64 as libc::c_int {
                    sprintf(out,
                            if asm_syntax as libc::c_uint ==
                                   GAS as libc::c_int as libc::c_uint {
                                b"$0x%016lx\x00" as *const u8 as
                                    *const libc::c_char
                            } else {
                                b"qword %016lXh\x00" as *const u8 as
                                    *const libc::c_char
                            }, value);
                } else if (*instr).op.size as libc::c_int == 32 as libc::c_int
                 {
                    sprintf(out,
                            if asm_syntax as libc::c_uint ==
                                   GAS as libc::c_int as libc::c_uint {
                                b"$0x%08lx\x00" as *const u8 as
                                    *const libc::c_char
                            } else {
                                b"dword %08lXh\x00" as *const u8 as
                                    *const libc::c_char
                            }, value);
                } else {
                    sprintf(out,
                            if asm_syntax as libc::c_uint ==
                                   GAS as libc::c_int as libc::c_uint {
                                b"$0x%04lx\x00" as *const u8 as
                                    *const libc::c_char
                            } else {
                                b"word %04lXh\x00" as *const u8 as
                                    *const libc::c_char
                            }, value);
                }
            } else if (*instr).op.size as libc::c_int == 8 as libc::c_int {
                sprintf(out,
                        if asm_syntax as libc::c_uint ==
                               GAS as libc::c_int as libc::c_uint {
                            b"$0x%02lx\x00" as *const u8 as
                                *const libc::c_char
                        } else {
                            b"%02lXh\x00" as *const u8 as *const libc::c_char
                        }, value);
            } else if (*instr).op.size as libc::c_int == 16 as libc::c_int {
                sprintf(out,
                        if asm_syntax as libc::c_uint ==
                               GAS as libc::c_int as libc::c_uint {
                            b"$0x%04lx\x00" as *const u8 as
                                *const libc::c_char
                        } else {
                            b"%04lXh\x00" as *const u8 as *const libc::c_char
                        }, value);
            } else if (*instr).op.size as libc::c_int == 64 as libc::c_int &&
                          (*instr).op.flags &
                              0x800 as libc::c_int as libc::c_uint != 0 {
                sprintf(out,
                        if asm_syntax as libc::c_uint ==
                               GAS as libc::c_int as libc::c_uint {
                            b"$0x%016lx\x00" as *const u8 as
                                *const libc::c_char
                        } else {
                            b"%016lXh\x00" as *const u8 as *const libc::c_char
                        }, value);
            } else {
                sprintf(out,
                        if asm_syntax as libc::c_uint ==
                               GAS as libc::c_int as libc::c_uint {
                            b"$0x%08lx\x00" as *const u8 as
                                *const libc::c_char
                        } else {
                            b"%08lXh\x00" as *const u8 as *const libc::c_char
                        }, value);
            }
        }
        30 | 31 => {
            sprintf(out, b"%04lx\x00" as *const u8 as *const libc::c_char,
                    value);
        }
        33 => {
            if asm_syntax as libc::c_uint ==
                   GAS as libc::c_int as libc::c_uint {
                if (*instr).prefix as libc::c_int & 0x7 as libc::c_int != 0 {
                    get_seg16(out,
                              (((*instr).prefix as libc::c_int &
                                    0x7 as libc::c_int) - 1 as libc::c_int) as
                                  byte);
                    strcat(out, b":\x00" as *const u8 as *const libc::c_char);
                }
                sprintf(out.offset(strlen(out) as isize),
                        b"0x%04lx\x00" as *const u8 as *const libc::c_char,
                        value);
            } else {
                *out.offset(0 as libc::c_int as isize) =
                    '[' as i32 as libc::c_char;
                if (*instr).prefix as libc::c_int & 0x7 as libc::c_int != 0 {
                    get_seg16(out,
                              (((*instr).prefix as libc::c_int &
                                    0x7 as libc::c_int) - 1 as libc::c_int) as
                                  byte);
                    strcat(out, b":\x00" as *const u8 as *const libc::c_char);
                }
                sprintf(out.offset(strlen(out) as isize),
                        b"%04lXh]\x00" as *const u8 as *const libc::c_char,
                        value);
            }
            (*instr).set_usedmem(1 as libc::c_int)
        }
        34 | 35 => {
            if asm_syntax as libc::c_uint !=
                   NASM as libc::c_int as libc::c_uint {
                if (*instr).prefix as libc::c_int & 0x7 as libc::c_int != 0 {
                    get_seg16(out,
                              (((*instr).prefix as libc::c_int &
                                    0x7 as libc::c_int) - 1 as libc::c_int) as
                                  byte);
                    strcat(out, b":\x00" as *const u8 as *const libc::c_char);
                }
                strcat(out,
                       if asm_syntax as libc::c_uint ==
                              GAS as libc::c_int as libc::c_uint {
                           b"(\x00" as *const u8 as *const libc::c_char
                       } else {
                           b"[\x00" as *const u8 as *const libc::c_char
                       });
                get_reg16(out,
                          if (*arg).type_0 as libc::c_uint ==
                                 DSBX as libc::c_int as libc::c_uint {
                              3 as libc::c_int
                          } else { 6 as libc::c_int } as byte,
                          (*instr).addrsize as libc::c_int);
                strcat(out,
                       if asm_syntax as libc::c_uint ==
                              GAS as libc::c_int as libc::c_uint {
                           b")\x00" as *const u8 as *const libc::c_char
                       } else {
                           b"]\x00" as *const u8 as *const libc::c_char
                       });
            }
            (*instr).set_usedmem(1 as libc::c_int)
        }
        36 => {
            if asm_syntax as libc::c_uint !=
                   NASM as libc::c_int as libc::c_uint {
                strcat(out,
                       if asm_syntax as libc::c_uint ==
                              GAS as libc::c_int as libc::c_uint {
                           b"%es:(\x00" as *const u8 as *const libc::c_char
                       } else {
                           b"es:[\x00" as *const u8 as *const libc::c_char
                       });
                get_reg16(out, 7 as libc::c_int as byte,
                          (*instr).addrsize as libc::c_int);
                strcat(out,
                       if asm_syntax as libc::c_uint ==
                              GAS as libc::c_int as libc::c_uint {
                           b")\x00" as *const u8 as *const libc::c_char
                       } else {
                           b"]\x00" as *const u8 as *const libc::c_char
                       });
            }
            (*instr).set_usedmem(1 as libc::c_int)
        }
        24 => {
            if asm_syntax as libc::c_uint ==
                   GAS as libc::c_int as libc::c_uint {
                strcpy(out, b"%al\x00" as *const u8 as *const libc::c_char);
            }
        }
        25 => {
            if asm_syntax as libc::c_uint ==
                   GAS as libc::c_int as libc::c_uint {
                strcpy(out, b"%ax\x00" as *const u8 as *const libc::c_char);
            }
        }
        26 => {
            if asm_syntax as libc::c_uint ==
                   GAS as libc::c_int as libc::c_uint {
                strcpy(out, b"(%dx)\x00" as *const u8 as *const libc::c_char);
            } else {
                strcpy(out, b"dx\x00" as *const u8 as *const libc::c_char);
            }
        }
        37 | 40 | 38 | 39 => {
            /* register/memory. this is always the first byte after the opcode,
     * and is always either paired with a simple register or a subcode.
     * there are a few cases where it isn't [namely C6/7 MOV and 8F POP]
     * and we need to warn if we see a value there that isn't 0. */
            if (*instr).modrm_disp as libc::c_uint ==
                   DISP_REG as libc::c_int as libc::c_uint {
                if (*arg).type_0 as libc::c_uint ==
                       XM as libc::c_int as libc::c_uint {
                    get_xmm(out, (*instr).modrm_reg as byte);
                    if (*instr).vex_256() != 0 {
                        *out.offset(if asm_syntax as libc::c_uint ==
                                           GAS as libc::c_int as libc::c_uint
                                       {
                                        1 as libc::c_int
                                    } else { 0 as libc::c_int } as isize) =
                            'y' as i32 as libc::c_char
                    }
                } else if (*arg).type_0 as libc::c_uint ==
                              MM as libc::c_int as libc::c_uint {
                    get_mmx(out, (*instr).modrm_reg as byte);
                } else {
                    if (*arg).type_0 as libc::c_uint ==
                           MEM as libc::c_int as libc::c_uint {
                        fprintf(stderr,
                                b"Warning: %s: \x00" as *const u8 as
                                    *const libc::c_char, ip);
                        fprintf(stderr,
                                b"ModRM byte has mod 3, but opcode only allows accessing memory.\n\x00"
                                    as *const u8 as *const libc::c_char);
                    }
                    if (*instr).op.size as libc::c_int == 8 as libc::c_int ||
                           (*instr).op.opcode as libc::c_int ==
                               0xfb6 as libc::c_int ||
                           (*instr).op.opcode as libc::c_int ==
                               0xfbe as libc::c_int {
                        /* mov*b* */
                        get_reg8(out, (*instr).modrm_reg as byte,
                                 (*instr).prefix as libc::c_int &
                                     0x800 as libc::c_int);
                    } else if (*instr).op.opcode as libc::c_int ==
                                  0xfb7 as libc::c_int ||
                                  (*instr).op.opcode as libc::c_int ==
                                      0xfbf as libc::c_int {
                        /* mov*w* */
                        get_reg16(out, (*instr).modrm_reg as byte,
                                  16 as libc::c_int); /* fixme: 64-bit? */
                    } else {
                        get_reg16(out, (*instr).modrm_reg as byte,
                                  (*instr).op.size as libc::c_int);
                    }
                }
            } else {
                (*instr).set_usedmem(1 as libc::c_int);
                /* NASM: <size>    [<seg>: <reg>+<reg>+/-<offset>h] */
        /* MASM: <size> ptr <seg>:[<reg>+<reg>+/-<offset>h] */
        /* GAS:           *%<seg>:<->0x<offset>(%<reg>,%<reg>) */
                if asm_syntax as libc::c_uint ==
                       GAS as libc::c_int as libc::c_uint {
                    if (*instr).op.opcode as libc::c_int ==
                           0xff as libc::c_int &&
                           (*instr).op.subcode as libc::c_int >=
                               2 as libc::c_int &&
                           (*instr).op.subcode as libc::c_int <=
                               5 as libc::c_int {
                        strcat(out,
                               b"*\x00" as *const u8 as *const libc::c_char);
                    }
                    if (*instr).prefix as libc::c_int & 0x7 as libc::c_int !=
                           0 {
                        get_seg16(out,
                                  (((*instr).prefix as libc::c_int &
                                        0x7 as libc::c_int) -
                                       1 as libc::c_int) as byte);
                        strcat(out,
                               b":\x00" as *const u8 as *const libc::c_char);
                    }
                    /* offset */
                    if (*instr).modrm_disp as libc::c_uint ==
                           DISP_8 as libc::c_int as libc::c_uint {
                        let mut svalue: int8_t =
                            value as int8_t; /* absolute memory is unsigned */
                        if (svalue as libc::c_int) < 0 as libc::c_int {
                            sprintf(out.offset(strlen(out) as isize),
                                    b"-0x%02x\x00" as *const u8 as
                                        *const libc::c_char,
                                    -(svalue as
                                          libc::c_int)); /* absolute memory is unsigned */
                        } else {
                            sprintf(out.offset(strlen(out) as isize),
                                    b"0x%02x\x00" as *const u8 as
                                        *const libc::c_char,
                                    svalue as libc::c_int);
                        }
                    } else if (*instr).modrm_disp as libc::c_uint ==
                                  DISP_16 as libc::c_int as libc::c_uint &&
                                  (*instr).addrsize as libc::c_int ==
                                      16 as libc::c_int {
                        let mut svalue_0: int16_t = value as int16_t;
                        if (*instr).modrm_reg as libc::c_int ==
                               -(1 as libc::c_int) {
                            sprintf(out.offset(strlen(out) as isize),
                                    b"0x%04lx\x00" as *const u8 as
                                        *const libc::c_char, value);
                            return
                        }
                        if (svalue_0 as libc::c_int) < 0 as libc::c_int {
                            sprintf(out.offset(strlen(out) as isize),
                                    b"-0x%04x\x00" as *const u8 as
                                        *const libc::c_char,
                                    -(svalue_0 as libc::c_int));
                        } else {
                            sprintf(out.offset(strlen(out) as isize),
                                    b"0x%04x\x00" as *const u8 as
                                        *const libc::c_char,
                                    svalue_0 as libc::c_int);
                        }
                    } else if (*instr).modrm_disp as libc::c_uint ==
                                  DISP_16 as libc::c_int as libc::c_uint {
                        let mut svalue_1: int32_t = value as int32_t;
                        if (*instr).modrm_reg as libc::c_int ==
                               -(1 as libc::c_int) {
                            sprintf(out.offset(strlen(out) as isize),
                                    b"0x%08lx\x00" as *const u8 as
                                        *const libc::c_char, value);
                            return
                        }
                        if svalue_1 < 0 as libc::c_int {
                            sprintf(out.offset(strlen(out) as isize),
                                    b"-0x%08x\x00" as *const u8 as
                                        *const libc::c_char, -svalue_1);
                        } else {
                            sprintf(out.offset(strlen(out) as isize),
                                    b"0x%08x\x00" as *const u8 as
                                        *const libc::c_char, svalue_1);
                        }
                    }
                    strcat(out, b"(\x00" as *const u8 as *const libc::c_char);
                    if (*instr).addrsize as libc::c_int == 16 as libc::c_int {
                        strcat(out,
                               modrm16_gas[(*instr).modrm_reg as
                                               usize].as_ptr());
                    } else {
                        get_reg16(out, (*instr).modrm_reg as byte,
                                  (*instr).addrsize as libc::c_int);
                        if (*instr).sib_scale as libc::c_int != 0 &&
                               (*instr).sib_index as libc::c_int !=
                                   -(1 as libc::c_int) {
                            strcat(out,
                                   b",\x00" as *const u8 as
                                       *const libc::c_char);
                            get_reg16(out, (*instr).sib_index as byte,
                                      (*instr).addrsize as libc::c_int);
                            strcat(out,
                                   b",0\x00" as *const u8 as
                                       *const libc::c_char);
                            *out.offset(strlen(out).wrapping_sub(1 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_ulong)
                                            as isize) =
                                ('0' as i32 +
                                     (*instr).sib_scale as libc::c_int) as
                                    libc::c_char
                        }
                    }
                    strcat(out, b")\x00" as *const u8 as *const libc::c_char);
                } else {
                    let mut has_sib: libc::c_int =
                        ((*instr).sib_scale as libc::c_int != 0 as libc::c_int
                             &&
                             (*instr).sib_index as libc::c_int !=
                                 -(1 as libc::c_int)) as libc::c_int;
                    if (*instr).op.flags &
                           0x400 as libc::c_int as libc::c_uint != 0 {
                        strcat(out,
                               b"far \x00" as *const u8 as
                                   *const libc::c_char);
                    } else if is_reg((*instr).op.arg0) == 0 &&
                                  is_reg((*instr).op.arg1) == 0 {
                        match (*instr).op.size as libc::c_int {
                            8 => {
                                strcat(out,
                                       b"byte \x00" as *const u8 as
                                           *const libc::c_char);
                            }
                            16 => {
                                strcat(out,
                                       b"word \x00" as *const u8 as
                                           *const libc::c_char);
                            }
                            32 => {
                                strcat(out,
                                       b"dword \x00" as *const u8 as
                                           *const libc::c_char);
                            }
                            64 => {
                                strcat(out,
                                       b"qword \x00" as *const u8 as
                                           *const libc::c_char);
                            }
                            80 => {
                                strcat(out,
                                       b"tword \x00" as *const u8 as
                                           *const libc::c_char);
                            }
                            _ => { }
                        }
                        if asm_syntax as libc::c_uint ==
                               MASM as libc::c_int as libc::c_uint {
                            /* && instr->op.size == 0? */
                            strcat(out,
                                   b"ptr \x00" as *const u8 as
                                       *const libc::c_char);
                        }
                    } else if (*instr).op.opcode as libc::c_int ==
                                  0xfb6 as libc::c_int ||
                                  (*instr).op.opcode as libc::c_int ==
                                      0xfbe as libc::c_int {
                        /* mov*b* */
                        strcat(out,
                               b"byte \x00" as *const u8 as
                                   *const libc::c_char);
                        if asm_syntax as libc::c_uint ==
                               MASM as libc::c_int as libc::c_uint {
                            strcat(out,
                                   b"ptr \x00" as *const u8 as
                                       *const libc::c_char);
                        }
                    } else if (*instr).op.opcode as libc::c_int ==
                                  0xfb7 as libc::c_int ||
                                  (*instr).op.opcode as libc::c_int ==
                                      0xfbf as libc::c_int {
                        /* mov*w* */
                        strcat(out,
                               b"word \x00" as *const u8 as
                                   *const libc::c_char); /* absolute memory is unsigned */
                        if asm_syntax as libc::c_uint ==
                               MASM as libc::c_int as libc::c_uint {
                            strcat(out,
                                   b"ptr \x00" as *const u8 as
                                       *const libc::c_char); /* absolute memory is unsigned */
                        }
                    }
                    if asm_syntax as libc::c_uint ==
                           NASM as libc::c_int as libc::c_uint {
                        strcat(out,
                               b"[\x00" as *const u8 as *const libc::c_char);
                    }
                    if (*instr).prefix as libc::c_int & 0x7 as libc::c_int !=
                           0 {
                        get_seg16(out,
                                  (((*instr).prefix as libc::c_int &
                                        0x7 as libc::c_int) -
                                       1 as libc::c_int) as byte);
                        strcat(out,
                               b":\x00" as *const u8 as *const libc::c_char);
                    }
                    if asm_syntax as libc::c_uint ==
                           MASM as libc::c_int as libc::c_uint {
                        strcat(out,
                               b"[\x00" as *const u8 as *const libc::c_char);
                    }
                    if (*instr).modrm_reg as libc::c_int !=
                           -(1 as libc::c_int) {
                        if (*instr).addrsize as libc::c_int ==
                               16 as libc::c_int {
                            strcat(out,
                                   modrm16_masm[(*instr).modrm_reg as
                                                    usize].as_ptr());
                        } else {
                            get_reg16(out, (*instr).modrm_reg as byte,
                                      (*instr).addrsize as libc::c_int);
                        }
                        if has_sib != 0 {
                            strcat(out,
                                   b"+\x00" as *const u8 as
                                       *const libc::c_char);
                        }
                    }
                    if has_sib != 0 {
                        get_reg16(out, (*instr).sib_index as byte,
                                  (*instr).addrsize as libc::c_int);
                        strcat(out,
                               b"*0\x00" as *const u8 as *const libc::c_char);
                        *out.offset(strlen(out).wrapping_sub(1 as libc::c_int
                                                                 as
                                                                 libc::c_ulong)
                                        as isize) =
                            ('0' as i32 + (*instr).sib_scale as libc::c_int)
                                as libc::c_char
                    }
                    if (*instr).modrm_disp as libc::c_uint ==
                           DISP_8 as libc::c_int as libc::c_uint {
                        let mut svalue_2: int8_t = value as int8_t;
                        if (svalue_2 as libc::c_int) < 0 as libc::c_int {
                            sprintf(out.offset(strlen(out) as isize),
                                    b"-%02Xh\x00" as *const u8 as
                                        *const libc::c_char,
                                    -(svalue_2 as libc::c_int));
                        } else {
                            sprintf(out.offset(strlen(out) as isize),
                                    b"+%02Xh\x00" as *const u8 as
                                        *const libc::c_char,
                                    svalue_2 as libc::c_int);
                        }
                    } else if (*instr).modrm_disp as libc::c_uint ==
                                  DISP_16 as libc::c_int as libc::c_uint &&
                                  (*instr).addrsize as libc::c_int ==
                                      16 as libc::c_int {
                        let mut svalue_3: int16_t = value as int16_t;
                        if (*instr).modrm_reg as libc::c_int ==
                               -(1 as libc::c_int) && has_sib == 0 {
                            sprintf(out.offset(strlen(out) as isize),
                                    b"%04lXh\x00" as *const u8 as
                                        *const libc::c_char, value);
                        } else if (svalue_3 as libc::c_int) < 0 as libc::c_int
                         {
                            sprintf(out.offset(strlen(out) as isize),
                                    b"-%04Xh\x00" as *const u8 as
                                        *const libc::c_char,
                                    -(svalue_3 as libc::c_int));
                        } else {
                            sprintf(out.offset(strlen(out) as isize),
                                    b"+%04Xh\x00" as *const u8 as
                                        *const libc::c_char,
                                    svalue_3 as libc::c_int);
                        }
                    } else if (*instr).modrm_disp as libc::c_uint ==
                                  DISP_16 as libc::c_int as libc::c_uint {
                        let mut svalue_4: int32_t = value as int32_t;
                        if (*instr).modrm_reg as libc::c_int ==
                               -(1 as libc::c_int) && has_sib == 0 {
                            sprintf(out.offset(strlen(out) as isize),
                                    b"%08lXh\x00" as *const u8 as
                                        *const libc::c_char, value);
                        } else if svalue_4 < 0 as libc::c_int {
                            sprintf(out.offset(strlen(out) as isize),
                                    b"-%08Xh\x00" as *const u8 as
                                        *const libc::c_char, -svalue_4);
                        } else {
                            sprintf(out.offset(strlen(out) as isize),
                                    b"+%08Xh\x00" as *const u8 as
                                        *const libc::c_char, svalue_4);
                        }
                    }
                    strcat(out, b"]\x00" as *const u8 as *const libc::c_char);
                }
            }
        }
        44 | 41 => {
            if (*instr).op.size as libc::c_int == 8 as libc::c_int {
                get_reg8(out, value as byte,
                         (*instr).prefix as libc::c_int &
                             0x800 as libc::c_int);
            } else if bits == 64 as libc::c_int &&
                          (*instr).op.opcode as libc::c_int ==
                              0x63 as libc::c_int {
                get_reg16(out, value as byte, 64 as libc::c_int);
            } else {
                get_reg16(out, value as byte,
                          (*instr).op.size as libc::c_int);
            }
        }
        48 => { get_reg16(out, value as byte, bits); }
        47 => {
            if value > 5 as libc::c_int as libc::c_ulong {
                fprintf(stderr,
                        b"Warning: %s: \x00" as *const u8 as
                            *const libc::c_char, ip);
                fprintf(stderr,
                        b"Invalid segment register %ld\n\x00" as *const u8 as
                            *const libc::c_char, value);
            }
            get_seg16(out, value as byte);
        }
        49 => {
            match value {
                0 | 2 | 3 | 4 | 8 => { }
                _ => {
                    fprintf(stderr,
                            b"Warning: %s: \x00" as *const u8 as
                                *const libc::c_char, ip);
                    fprintf(stderr,
                            b"Invalid control register %ld\n\x00" as *const u8
                                as *const libc::c_char, value);
                }
            }
            if asm_syntax as libc::c_uint ==
                   GAS as libc::c_int as libc::c_uint {
                strcat(out, b"%\x00" as *const u8 as *const libc::c_char);
            }
            strcat(out, b"cr0\x00" as *const u8 as *const libc::c_char);
            *out.offset(strlen(out).wrapping_sub(1 as libc::c_int as
                                                     libc::c_ulong) as isize)
                =
                ('0' as i32 as libc::c_ulong).wrapping_add(value) as
                    libc::c_char
        }
        50 => {
            if asm_syntax as libc::c_uint ==
                   GAS as libc::c_int as libc::c_uint {
                strcat(out, b"%\x00" as *const u8 as *const libc::c_char);
            }
            strcat(out, b"dr0\x00" as *const u8 as *const libc::c_char);
            *out.offset(strlen(out).wrapping_sub(1 as libc::c_int as
                                                     libc::c_ulong) as isize)
                =
                ('0' as i32 as libc::c_ulong).wrapping_add(value) as
                    libc::c_char
        }
        51 => {
            if value < 3 as libc::c_int as libc::c_ulong {
                fprintf(stderr,
                        b"Warning: %s: \x00" as *const u8 as
                            *const libc::c_char, ip);
                fprintf(stderr,
                        b"Invalid test register %ld\n\x00" as *const u8 as
                            *const libc::c_char, value);
            }
            if asm_syntax as libc::c_uint ==
                   GAS as libc::c_int as libc::c_uint {
                strcat(out, b"%\x00" as *const u8 as *const libc::c_char);
            }
            strcat(out, b"tr0\x00" as *const u8 as *const libc::c_char);
            *out.offset(strlen(out).wrapping_sub(1 as libc::c_int as
                                                     libc::c_ulong) as isize)
                =
                ('0' as i32 as libc::c_ulong).wrapping_add(value) as
                    libc::c_char
        }
        52 => {
            if asm_syntax as libc::c_uint ==
                   GAS as libc::c_int as libc::c_uint {
                strcat(out, b"%\x00" as *const u8 as *const libc::c_char);
            }
            strcat(out, b"st\x00" as *const u8 as *const libc::c_char);
            if asm_syntax as libc::c_uint ==
                   NASM as libc::c_int as libc::c_uint {
                strcat(out, b"0\x00" as *const u8 as *const libc::c_char);
            }
        }
        53 => {
            if asm_syntax as libc::c_uint ==
                   GAS as libc::c_int as libc::c_uint {
                strcat(out, b"%\x00" as *const u8 as *const libc::c_char);
            }
            strcat(out, b"st\x00" as *const u8 as *const libc::c_char);
            if asm_syntax as libc::c_uint !=
                   NASM as libc::c_int as libc::c_uint {
                strcat(out, b"(\x00" as *const u8 as *const libc::c_char);
            }
            strcat(out, b"0\x00" as *const u8 as *const libc::c_char);
            *out.offset(strlen(out).wrapping_sub(1 as libc::c_int as
                                                     libc::c_ulong) as isize)
                =
                ('0' as i32 as libc::c_ulong).wrapping_add(value) as
                    libc::c_char;
            if asm_syntax as libc::c_uint !=
                   NASM as libc::c_int as libc::c_uint {
                strcat(out, b")\x00" as *const u8 as *const libc::c_char);
            }
        }
        45 | 42 => { get_mmx(out, value as byte); }
        46 | 43 => {
            get_xmm(out, value as byte);
            if (*instr).vex_256() != 0 {
                *out.offset(if asm_syntax as libc::c_uint ==
                                   GAS as libc::c_int as libc::c_uint {
                                1 as libc::c_int
                            } else { 0 as libc::c_int } as isize) =
                    'y' as i32 as libc::c_char
            }
        }
        32 | _ => { }
    };
}
/* helper to tack a length suffix onto a name */
unsafe extern "C" fn suffix_name(mut instr: *mut instr) {
    if (*instr).op.flags & 0x3000 as libc::c_int as libc::c_uint ==
           0x3000 as libc::c_int as libc::c_uint {
        strcat((*instr).op.name.as_mut_ptr(),
               b"ll\x00" as *const u8 as *const libc::c_char);
    } else if (*instr).op.flags & 0x1000 as libc::c_int as libc::c_uint != 0 {
        strcat((*instr).op.name.as_mut_ptr(),
               b"s\x00" as *const u8 as *const libc::c_char);
    } else if (*instr).op.flags & 0x2000 as libc::c_int as libc::c_uint != 0 {
        strcat((*instr).op.name.as_mut_ptr(),
               b"l\x00" as *const u8 as *const libc::c_char);
    } else if (*instr).op.size as libc::c_int == 80 as libc::c_int {
        strcat((*instr).op.name.as_mut_ptr(),
               b"t\x00" as *const u8 as *const libc::c_char);
    } else if (*instr).op.size as libc::c_int == 8 as libc::c_int {
        strcat((*instr).op.name.as_mut_ptr(),
               b"b\x00" as *const u8 as *const libc::c_char);
    } else if (*instr).op.size as libc::c_int == 16 as libc::c_int {
        strcat((*instr).op.name.as_mut_ptr(),
               b"w\x00" as *const u8 as *const libc::c_char);
    } else if (*instr).op.size as libc::c_int == 32 as libc::c_int {
        strcat((*instr).op.name.as_mut_ptr(),
               if asm_syntax as libc::c_uint ==
                      GAS as libc::c_int as libc::c_uint {
                   b"l\x00" as *const u8 as *const libc::c_char
               } else { b"d\x00" as *const u8 as *const libc::c_char });
    } else if (*instr).op.size as libc::c_int == 64 as libc::c_int {
        strcat((*instr).op.name.as_mut_ptr(),
               b"q\x00" as *const u8 as *const libc::c_char);
    };
}
/* Paramters:
 * ip    - current IP (used to calculate relative addresses)
 * p     - pointer to the current instruction to be parsed
 * instr - [output] pointer to an instr_info struct to be filled
 * is32  - bitness
 *
 * Returns: number of bytes processed
 *
 * Note: we don't print warnings here (all warnings should be printed
 * while actually dumping output, both to keep this function agnostic and to
 * ensure they only get printed once), so we will need to watch out for
 * multiple prefixes, invalid instructions, etc.
 */
#[no_mangle]
pub unsafe extern "C" fn get_instr(mut ip: dword, mut p: *const byte,
                                   mut instr: *mut instr,
                                   mut bits: libc::c_int) -> libc::c_int {
    let mut len: libc::c_int = 0 as libc::c_int;
    let mut opcode: byte = 0;
    let mut prefix: word = 0;
    memset(instr as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<instr>() as libc::c_ulong);
    loop  {
        prefix = get_prefix(*p.offset(len as isize) as word, bits);
        if !(prefix != 0) { break ; }
        if (*instr).prefix as libc::c_int & 0x7 as libc::c_int != 0 &&
               prefix as libc::c_int & 0x7 as libc::c_int != 0 {
            (*instr).op = instructions[*p.offset(len as isize) as usize];
            (*instr).prefix =
                ((*instr).prefix as libc::c_int & !(0x7 as libc::c_int)) as
                    word
        } else if !((*instr).prefix as libc::c_int & prefix as libc::c_int &
                        0x8 as libc::c_int != 0) {
            if (*instr).prefix as libc::c_int & prefix as libc::c_int != 0 {
                (*instr).op = instructions[*p.offset(len as isize) as usize];
                (*instr).prefix =
                    ((*instr).prefix as libc::c_int &
                         !(prefix as libc::c_int)) as word;
                return len
            }
        }
        (*instr).prefix =
            ((*instr).prefix as libc::c_int | prefix as libc::c_int) as word;
        len += 1
    }
    opcode = *p.offset(len as isize);
    /* copy the op_info */
    if opcode as libc::c_int == 0xc4 as libc::c_int &&
           *p.offset((len + 1 as libc::c_int) as isize) as libc::c_int >>
               6 as libc::c_int == 3 as libc::c_int &&
           bits != 16 as libc::c_int {
        let mut subcode: byte = 0xcc as libc::c_int as byte;
        len += 1;
        (*instr).set_vex(1 as libc::c_int);
        if *p.offset(len as isize) as libc::c_int & 0x1f as libc::c_int ==
               2 as libc::c_int {
            subcode = 0x38 as libc::c_int as byte
        } else if *p.offset(len as isize) as libc::c_int & 0x1f as libc::c_int
                      == 3 as libc::c_int {
            subcode = 0x3a as libc::c_int as byte
        } else {
            fprintf(stderr,
                    b"Warning: Unhandled subcode %x at %x\n\x00" as *const u8
                        as *const libc::c_char,
                    *p.offset(len as isize) as libc::c_int, ip);
        }
        len += 1;
        (*instr).set_vex_reg(!(*p.offset(len as isize) as libc::c_int >>
                                   3 as libc::c_int & 7 as libc::c_int) as
                                 libc::c_uint);
        (*instr).set_vex_256(if *p.offset(len as isize) as libc::c_int &
                                    4 as libc::c_int != 0 {
                                 1 as libc::c_int
                             } else { 0 as libc::c_int });
        if *p.offset(len as isize) as libc::c_int & 3 as libc::c_int ==
               3 as libc::c_int {
            (*instr).prefix =
                ((*instr).prefix as libc::c_int | 0x40 as libc::c_int) as word
        } else if *p.offset(len as isize) as libc::c_int & 3 as libc::c_int ==
                      2 as libc::c_int {
            (*instr).prefix =
                ((*instr).prefix as libc::c_int | 0x80 as libc::c_int) as word
        } else if *p.offset(len as isize) as libc::c_int & 3 as libc::c_int ==
                      1 as libc::c_int {
            (*instr).prefix =
                ((*instr).prefix as libc::c_int | 0x8 as libc::c_int) as word
        }
        len +=
            get_sse_single(subcode,
                           *p.offset((len + 1 as libc::c_int) as isize),
                           instr)
    } else if opcode as libc::c_int == 0xc5 as libc::c_int &&
                  *p.offset((len + 1 as libc::c_int) as isize) as libc::c_int
                      >> 6 as libc::c_int == 3 as libc::c_int &&
                  bits != 16 as libc::c_int {
        len += 1;
        (*instr).set_vex(1 as libc::c_int);
        (*instr).set_vex_reg(!(*p.offset(len as isize) as libc::c_int >>
                                   3 as libc::c_int & 7 as libc::c_int) as
                                 libc::c_uint);
        (*instr).set_vex_256(if *p.offset(len as isize) as libc::c_int &
                                    4 as libc::c_int != 0 {
                                 1 as libc::c_int
                             } else { 0 as libc::c_int });
        if *p.offset(len as isize) as libc::c_int & 3 as libc::c_int ==
               3 as libc::c_int {
            (*instr).prefix =
                ((*instr).prefix as libc::c_int | 0x40 as libc::c_int) as word
        } else if *p.offset(len as isize) as libc::c_int & 3 as libc::c_int ==
                      2 as libc::c_int {
            (*instr).prefix =
                ((*instr).prefix as libc::c_int | 0x80 as libc::c_int) as word
        } else if *p.offset(len as isize) as libc::c_int & 3 as libc::c_int ==
                      1 as libc::c_int {
            (*instr).prefix =
                ((*instr).prefix as libc::c_int | 0x8 as libc::c_int) as word
        }
        len += 1;
        len += get_0f_instr(p.offset(len as isize), instr)
    } else if bits == 64 as libc::c_int &&
                  instructions64[opcode as
                                     usize].name[0 as libc::c_int as usize] as
                      libc::c_int != 0 {
        (*instr).op = instructions64[opcode as usize]
    } else if bits != 64 as libc::c_int &&
                  instructions[opcode as
                                   usize].name[0 as libc::c_int as usize] as
                      libc::c_int != 0 {
        (*instr).op = instructions[opcode as usize]
    } else {
        let mut subcode_0: byte =
            (*p.offset((len + 1 as libc::c_int) as isize) as libc::c_int >>
                 3 as libc::c_int & 7 as libc::c_int) as byte;
        /* do we have a member of an instruction group? */
        if opcode as libc::c_int == 0xf as libc::c_int {
            len += 1;
            len += get_0f_instr(p.offset(len as isize), instr)
        } else if opcode as libc::c_int >= 0xd8 as libc::c_int &&
                      opcode as libc::c_int <= 0xdf as libc::c_int {
            len += get_fpu_instr(p.offset(len as isize), &mut (*instr).op)
        } else {
            let mut i: libc::c_uint = 0;
            i = 0 as libc::c_int as libc::c_uint;
            while (i as libc::c_ulong) <
                      (::std::mem::size_of::<[op; 108]>() as
                           libc::c_ulong).wrapping_div(::std::mem::size_of::<op>()
                                                           as libc::c_ulong) {
                if opcode as libc::c_int ==
                       instructions_group[i as usize].opcode as libc::c_int &&
                       subcode_0 as libc::c_int ==
                           instructions_group[i as usize].subcode as
                               libc::c_int {
                    (*instr).op = instructions_group[i as usize];
                    break ;
                } else { i = i.wrapping_add(1) }
            }
        }
        /* if we get here and we haven't found a suitable instruction,
         * we ran into something unused (or inadequately documented) */
        if (*instr).op.name[0 as libc::c_int as usize] == 0 {
            /* supply some default values so we can keep parsing */
            strcpy((*instr).op.name.as_mut_ptr(),
                   b"?\x00" as *const u8 as
                       *const libc::c_char); /* less arrogant than objdump's (bad) */
            (*instr).op.subcode = subcode_0;
            (*instr).op.size = 0 as libc::c_int as libc::c_char;
            (*instr).op.arg0 = NONE;
            (*instr).op.arg1 = NONE;
            (*instr).op.flags = 0 as libc::c_int as dword
        }
    }
    len += 1;
    /* resolve the size */
    if (*instr).op.size as libc::c_int == -(1 as libc::c_int) {
        if (*instr).prefix as libc::c_int & 0x8 as libc::c_int != 0 {
            (*instr).op.size =
                if bits == 16 as libc::c_int {
                    32 as libc::c_int
                } else { 16 as libc::c_int } as libc::c_char
        } else if (*instr).prefix as libc::c_int & 0x8000 as libc::c_int != 0
         {
            (*instr).op.size = 64 as libc::c_int as libc::c_char
        } else if (*instr).op.flags &
                      (0x100 as libc::c_int | 0x8 as libc::c_int) as
                          libc::c_uint != 0 {
            (*instr).op.size = bits as libc::c_char
        } else {
            (*instr).op.size =
                if bits == 16 as libc::c_int {
                    16 as libc::c_int
                } else { 32 as libc::c_int } as libc::c_char
        }
    }
    if (*instr).prefix as libc::c_int & 0x10 as libc::c_int != 0 {
        (*instr).addrsize =
            if bits == 32 as libc::c_int {
                16 as libc::c_int
            } else { 32 as libc::c_int } as byte
    } else { (*instr).addrsize = bits as byte }
    /* figure out what arguments we have */
    if (*instr).op.arg0 as u64 != 0 {
        let mut base: libc::c_int = len;
        (*instr).args[0 as libc::c_int as usize].type_0 = (*instr).op.arg0;
        (*instr).args[1 as libc::c_int as usize].type_0 = (*instr).op.arg1;
        /* The convention is that an arg whose value is one or more bytes has
         * IP pointing to that value, but otherwise it points to the beginning
         * of the instruction. This way, we'll never think that e.g. a register
         * value is supposed to be relocated. */
        (*instr).args[2 as libc::c_int as usize].ip = ip;
        (*instr).args[1 as libc::c_int as usize].ip =
            (*instr).args[2 as libc::c_int as usize].ip;
        (*instr).args[0 as libc::c_int as usize].ip =
            (*instr).args[1 as libc::c_int as usize].ip;
        len +=
            get_arg(ip.wrapping_add(len as libc::c_uint),
                    &*p.offset(len as isize),
                    &mut *(*instr).args.as_mut_ptr().offset(0 as libc::c_int
                                                                as isize),
                    instr, bits);
        /* registers that read from the modrm byte, which we might have just processed */
        if (*instr).op.arg1 as libc::c_uint >=
               REG as libc::c_int as libc::c_uint &&
               (*instr).op.arg1 as libc::c_uint <=
                   TR32 as libc::c_int as libc::c_uint {
            len +=
                get_arg(ip.wrapping_add(len as libc::c_uint),
                        &*p.offset(base as isize),
                        &mut *(*instr).args.as_mut_ptr().offset(1 as
                                                                    libc::c_int
                                                                    as isize),
                        instr, bits)
        } else {
            len +=
                get_arg(ip.wrapping_add(len as libc::c_uint),
                        &*p.offset(len as isize),
                        &mut *(*instr).args.as_mut_ptr().offset(1 as
                                                                    libc::c_int
                                                                    as isize),
                        instr, bits)
        }
        /* arg2 */
        if (*instr).op.flags & 0x1 as libc::c_int as libc::c_uint != 0 {
            (*instr).args[2 as libc::c_int as usize].type_0 = IMM
        } else if (*instr).op.flags & 0x2 as libc::c_int as libc::c_uint != 0
         {
            (*instr).args[2 as libc::c_int as usize].type_0 = IMM8
        } else if (*instr).op.flags & 0x4 as libc::c_int as libc::c_uint != 0
         {
            (*instr).args[2 as libc::c_int as usize].type_0 = CL
        }
        len +=
            get_arg(ip.wrapping_add(len as libc::c_uint),
                    &*p.offset(len as isize),
                    &mut *(*instr).args.as_mut_ptr().offset(2 as libc::c_int
                                                                as isize),
                    instr, bits)
    }
    /* modify the instruction name if appropriate */
    if asm_syntax as libc::c_uint == GAS as libc::c_int as libc::c_uint {
        if (*instr).op.opcode as libc::c_int == 0xfb6 as libc::c_int {
            strcpy((*instr).op.name.as_mut_ptr(),
                   b"movzb\x00" as *const u8 as *const libc::c_char);
            suffix_name(instr);
        } else if (*instr).op.opcode as libc::c_int == 0xfb7 as libc::c_int {
            strcpy((*instr).op.name.as_mut_ptr(),
                   b"movzw\x00" as *const u8 as *const libc::c_char);
            suffix_name(instr);
        } else if (*instr).op.opcode as libc::c_int == 0xfbe as libc::c_int {
            strcpy((*instr).op.name.as_mut_ptr(),
                   b"movsb\x00" as *const u8 as *const libc::c_char);
            suffix_name(instr);
        } else if (*instr).op.opcode as libc::c_int == 0xfbf as libc::c_int {
            strcpy((*instr).op.name.as_mut_ptr(),
                   b"movsw\x00" as *const u8 as *const libc::c_char);
            suffix_name(instr);
        } else if (*instr).op.opcode as libc::c_int == 0x63 as libc::c_int &&
                      bits == 64 as libc::c_int {
            strcpy((*instr).op.name.as_mut_ptr(),
                   b"movslq\x00" as *const u8 as *const libc::c_char);
        }
    }
    if (*instr).op.flags & 0x100 as libc::c_int as libc::c_uint != 0 &&
           (*instr).prefix as libc::c_int & 0x8 as libc::c_int != 0 {
        suffix_name(instr);
    } else if (*instr).op.flags & 0x200 as libc::c_int as libc::c_uint != 0 &&
                  asm_syntax as libc::c_uint !=
                      GAS as libc::c_int as libc::c_uint {
        suffix_name(instr);
    } else if (*instr).op.opcode as libc::c_int == 0x98 as libc::c_int {
        strcpy((*instr).op.name.as_mut_ptr(),
               if (*instr).op.size as libc::c_int == 16 as libc::c_int {
                   b"cbw\x00" as *const u8 as *const libc::c_char
               } else if (*instr).op.size as libc::c_int == 32 as libc::c_int
                {
                   b"cwde\x00" as *const u8 as *const libc::c_char
               } else { b"cdqe\x00" as *const u8 as *const libc::c_char });
    } else if (*instr).op.opcode as libc::c_int == 0x99 as libc::c_int {
        strcpy((*instr).op.name.as_mut_ptr(),
               if (*instr).op.size as libc::c_int == 16 as libc::c_int {
                   b"cwd\x00" as *const u8 as *const libc::c_char
               } else if (*instr).op.size as libc::c_int == 32 as libc::c_int
                {
                   b"cdq\x00" as *const u8 as *const libc::c_char
               } else { b"cqo\x00" as *const u8 as *const libc::c_char });
    } else if (*instr).op.opcode as libc::c_int == 0xe3 as libc::c_int &&
                  (*instr).prefix as libc::c_int & 0x10 as libc::c_int != 0 {
        strcpy((*instr).op.name.as_mut_ptr(),
               b"jecxz\x00" as *const u8 as *const libc::c_char);
    } else if (*instr).op.opcode as libc::c_int == 0xd4 as libc::c_int &&
                  (*instr).args[0 as libc::c_int as usize].value ==
                      10 as libc::c_int as libc::c_ulong {
        strcpy((*instr).op.name.as_mut_ptr(),
               b"aam\x00" as *const u8 as *const libc::c_char);
        (*instr).op.arg0 = NONE
    } else if (*instr).op.opcode as libc::c_int == 0xd5 as libc::c_int &&
                  (*instr).args[0 as libc::c_int as usize].value ==
                      10 as libc::c_int as libc::c_ulong {
        strcpy((*instr).op.name.as_mut_ptr(),
               b"aad\x00" as *const u8 as *const libc::c_char);
        (*instr).op.arg0 = NONE
    } else if (*instr).op.opcode as libc::c_int == 0xfc7 as libc::c_int &&
                  (*instr).op.subcode as libc::c_int == 1 as libc::c_int &&
                  (*instr).prefix as libc::c_int & 0x8000 as libc::c_int != 0
     {
        strcpy((*instr).op.name.as_mut_ptr(),
               b"cmpxchg16b\x00" as *const u8 as *const libc::c_char);
    } else if asm_syntax as libc::c_uint == GAS as libc::c_int as libc::c_uint
     {
        if (*instr).op.flags & 0x400 as libc::c_int as libc::c_uint != 0 {
            memmove((*instr).op.name.as_mut_ptr().offset(1 as libc::c_int as
                                                             isize) as
                        *mut libc::c_void,
                    (*instr).op.name.as_mut_ptr() as *const libc::c_void,
                    strlen((*instr).op.name.as_mut_ptr()));
            (*instr).op.name[0 as libc::c_int as usize] =
                'l' as i32 as libc::c_char
        } else if is_reg((*instr).op.arg0) == 0 &&
                      is_reg((*instr).op.arg1) == 0 &&
                      (*instr).modrm_disp as libc::c_uint !=
                          DISP_REG as libc::c_int as libc::c_uint {
            suffix_name(instr);
        }
    } else if asm_syntax as libc::c_uint != GAS as libc::c_int as libc::c_uint
                  &&
                  ((*instr).op.opcode as libc::c_int == 0xca as libc::c_int ||
                       (*instr).op.opcode as libc::c_int ==
                           0xcb as libc::c_int) {
        strcat((*instr).op.name.as_mut_ptr(),
               b"f\x00" as *const u8 as *const libc::c_char);
    }
    return len;
}
/* the literal value 1, used for bit shift ops */
/* specific registers */
/* the same as AL/AX except MASM doesn't print them */
/* the same as DX except GAS puts it in parentheses */
/* absolute or relative numbers, given as 1/2/4 bytes */
/* immediate number */
/* relative to current instruction */
/* absolute instruction, used for far calls/jumps */
/* absolute location in memory, for A0-A3 MOV */
/* specific memory addresses for string operations */
/* to be read from ModRM, appropriately */
/* register/memory */
/* MMX register/memory */
/* SSE register/memory */
/* memory only (using 0x11xxxxxx is invalid) */
/* register only (not using 0x11xxxxxx is invalid) */
/* MMX register only (not using 0x11xxxxxx is invalid) */
/* SSE register only (not using 0x11xxxxxx is invalid) */
/* register */
/* MMX register */
/* SSE register */
/* segment register */
/* 32-bit only register, used for cr/dr/tr */
/* control register */
/* debug register */
/* test register */
/* floating point regs */
/* top of stack aka st(0) */
/* element of stack given by lowest three bytes of "modrm" */
/* opcode flags */
/* has IMM16/32 as third argument */
/* has IMM8 as third argument */
/* has CL as third argument */
/* opcodes which are 64-bit by default (call, jmp), most being 32-bit */
/* repne prefix valid */
/* repe prefix valid */
/* rep prefix valid */
/* operand-size prefix only valid if used with reg */
/* lock prefix valid */
/* only marked for size if overridden */
/* string operations */
/* far operation */
/* IMM argument can be 64-bit */
/* (FPU) op takes -s if GCC */
/* (FPU) op takes -l if GCC */
/* (FPU) op takes -ll if GCC */
/* -t doesn't need to be marked */
/* stop scanning (jmp, ret) */
/* branch to target (jmp, jXX) */
/* 0 if not sized, -1 if size == bitness */
/* usually dest */
/* usually src */
/* arg2 only for imul, shrd, shld */
/* 26 */
/* 2E */
/* 36 */
/* 3E */
/* 64 */
/* 65 */
/* 66 */
/* 67 */
/* F0 */
/* F2 */
/* F3 */
/* 9B */
/* 40 */
/* 41 */
/* 42 */
/* 44 */
/* 48 */
/* no disp, i.e. mod == 0 && m != 6 */
/* one byte */
/* two bytes */
/* register, i.e. mod == 3 */
/* This is a little ugly, but 16 is IP and -1 is none (aka IZ). */
/* used for error checking */
#[no_mangle]
pub unsafe extern "C" fn print_instr(mut ip: *mut libc::c_char,
                                     mut p: *mut byte, mut len: libc::c_int,
                                     mut flags: byte, mut instr: *mut instr,
                                     mut comment: *const libc::c_char,
                                     mut bits: libc::c_int) {
    let mut i: libc::c_int = 0;
    /* FIXME: now that we've had to add bits to this function, get rid of ip_string */
    /* get the arguments */
    print_arg(ip, instr, 0 as libc::c_int, bits);
    print_arg(ip, instr, 1 as libc::c_int, bits);
    print_arg(ip, instr, 2 as libc::c_int, bits);
    /* did we find too many prefixes? */
    if get_prefix((*instr).op.opcode, bits) != 0 {
        if get_prefix((*instr).op.opcode, bits) as libc::c_int &
               0x7 as libc::c_int != 0 {
            fprintf(stderr,
                    b"Warning: %s: \x00" as *const u8 as *const libc::c_char,
                    ip);
            fprintf(stderr,
                    b"Multiple segment prefixes found: %s, %s. Skipping to next instruction.\n\x00"
                        as *const u8 as *const libc::c_char,
                    seg16[(((*instr).prefix as libc::c_int &
                                0x7 as libc::c_int) - 1 as libc::c_int) as
                              usize].as_ptr(), (*instr).op.name.as_mut_ptr());
        } else {
            fprintf(stderr,
                    b"Warning: %s: \x00" as *const u8 as *const libc::c_char,
                    ip);
            fprintf(stderr,
                    b"Prefix specified twice: %s. Skipping to next instruction.\n\x00"
                        as *const u8 as *const libc::c_char,
                    (*instr).op.name.as_mut_ptr());
        }
        (*instr).op.name[0 as libc::c_int as usize] =
            0 as libc::c_int as libc::c_char
    }
    /* check that the instruction exists */
    if (*instr).op.name[0 as libc::c_int as usize] as libc::c_int ==
           '?' as i32 {
        fprintf(stderr,
                b"Warning: %s: \x00" as *const u8 as *const libc::c_char, ip);
        fprintf(stderr,
                b"Unknown opcode 0x%02x (extension %d)\n\x00" as *const u8 as
                    *const libc::c_char, (*instr).op.opcode as libc::c_int,
                (*instr).op.subcode as libc::c_int);
    }
    /* okay, now we begin dumping */
    if flags as libc::c_int & 0x4 as libc::c_int != 0 &&
           opts as libc::c_int & 0x10 as libc::c_int != 0 {
        /* output a label, which is like an address but without the segment prefix */
        /* FIXME: check masm */
        if asm_syntax as libc::c_uint == NASM as libc::c_int as libc::c_uint {
            printf(b".\x00" as *const u8 as *const libc::c_char);
        }
        printf(b"%s:\x00" as *const u8 as *const libc::c_char, ip);
    }
    if opts as libc::c_int & 0x8 as libc::c_int == 0 {
        printf(b"%s:\x00" as *const u8 as *const libc::c_char, ip);
    }
    printf(b"\t\x00" as *const u8 as *const libc::c_char);
    if opts as libc::c_int & 0x4 as libc::c_int == 0 {
        i = 0 as libc::c_int;
        while i < len && i < 7 as libc::c_int {
            printf(b"%02x \x00" as *const u8 as *const libc::c_char,
                   *p.offset(i as isize) as libc::c_int);
            i += 1
        }
        while i < 8 as libc::c_int {
            printf(b"   \x00" as *const u8 as *const libc::c_char);
            i += 1
        }
    }
    /* mark instructions that are jumped to */
    if flags as libc::c_int & 0x4 as libc::c_int != 0 &&
           opts as libc::c_int & 0x10 as libc::c_int == 0 {
        printf(if flags as libc::c_int & 0x10 as libc::c_int != 0 {
                   b">>\x00" as *const u8 as *const libc::c_char
               } else { b" >\x00" as *const u8 as *const libc::c_char });
    } else { printf(b"  \x00" as *const u8 as *const libc::c_char); }
    /* print prefixes, including (fake) prefixes if ours are invalid */
    if (*instr).prefix as libc::c_int & 0x7 as libc::c_int != 0 {
        /* note: is it valid to use overrides with lods and outs? */
        if (*instr).usedmem() == 0 ||
               ((*instr).op.arg0 as libc::c_uint ==
                    ESDI as libc::c_int as libc::c_uint ||
                    (*instr).op.arg1 as libc::c_uint ==
                        ESDI as libc::c_int as libc::c_uint &&
                        (*instr).op.arg0 as libc::c_uint !=
                            DSSI as libc::c_int as libc::c_uint) {
            /* can't be overridden */
            fprintf(stderr,
                    b"Warning: %s: \x00" as *const u8 as *const libc::c_char,
                    ip);
            fprintf(stderr,
                    b"Segment prefix %s used with opcode 0x%02x %s\n\x00" as
                        *const u8 as *const libc::c_char,
                    seg16[(((*instr).prefix as libc::c_int &
                                0x7 as libc::c_int) - 1 as libc::c_int) as
                              usize].as_ptr(),
                    (*instr).op.opcode as libc::c_int,
                    (*instr).op.name.as_mut_ptr());
            printf(b"%s \x00" as *const u8 as *const libc::c_char,
                   seg16[(((*instr).prefix as libc::c_int &
                               0x7 as libc::c_int) - 1 as libc::c_int) as
                             usize].as_ptr());
        }
    }
    if (*instr).prefix as libc::c_int & 0x8 as libc::c_int != 0 &&
           (*instr).op.size as libc::c_int != 16 as libc::c_int &&
           (*instr).op.size as libc::c_int != 32 as libc::c_int {
        fprintf(stderr,
                b"Warning: %s: \x00" as *const u8 as *const libc::c_char, ip);
        fprintf(stderr,
                b"Operand-size override used with opcode 0x%02x %s\n\x00" as
                    *const u8 as *const libc::c_char,
                (*instr).op.opcode as libc::c_int,
                (*instr).op.name.as_mut_ptr());
        printf(if asm_syntax as libc::c_uint ==
                      GAS as libc::c_int as libc::c_uint {
                   b"data32 \x00" as *const u8 as *const libc::c_char
               } else { b"o32 \x00" as *const u8 as *const libc::c_char });
        /* fixme: how should MASM print it? */
    }
    if (*instr).prefix as libc::c_int & 0x10 as libc::c_int != 0 &&
           asm_syntax as libc::c_uint == NASM as libc::c_int as libc::c_uint
           && (*instr).op.flags & 0x200 as libc::c_int as libc::c_uint != 0 {
        printf(b"a32 \x00" as *const u8 as *const libc::c_char);
    } else if (*instr).prefix as libc::c_int & 0x10 as libc::c_int != 0 &&
                  (*instr).usedmem() == 0 &&
                  (*instr).op.opcode as libc::c_int != 0xe3 as libc::c_int {
        /* jecxz */
        fprintf(stderr,
                b"Warning: %s: \x00" as *const u8 as *const libc::c_char, ip);
        fprintf(stderr,
                b"Address-size prefix used with opcode 0x%02x %s\n\x00" as
                    *const u8 as *const libc::c_char,
                (*instr).op.opcode as libc::c_int,
                (*instr).op.name.as_mut_ptr());
        printf(if asm_syntax as libc::c_uint ==
                      GAS as libc::c_int as libc::c_uint {
                   b"addr32 \x00" as *const u8 as *const libc::c_char
               } else { b"a32 \x00" as *const u8 as *const libc::c_char });
        /* fixme: how should MASM print it? */
    }
    if (*instr).prefix as libc::c_int & 0x20 as libc::c_int != 0 {
        if (*instr).op.flags & 0x80 as libc::c_int as libc::c_uint == 0 {
            fprintf(stderr,
                    b"Warning: %s: \x00" as *const u8 as *const libc::c_char,
                    ip);
            fprintf(stderr,
                    b"lock prefix used with opcode 0x%02x %s\n\x00" as
                        *const u8 as *const libc::c_char,
                    (*instr).op.opcode as libc::c_int,
                    (*instr).op.name.as_mut_ptr());
        }
        printf(b"lock \x00" as *const u8 as *const libc::c_char);
    }
    if (*instr).prefix as libc::c_int & 0x40 as libc::c_int != 0 {
        if (*instr).op.flags & 0x10 as libc::c_int as libc::c_uint == 0 {
            fprintf(stderr,
                    b"Warning: %s: \x00" as *const u8 as *const libc::c_char,
                    ip);
            fprintf(stderr,
                    b"repne prefix used with opcode 0x%02x %s\n\x00" as
                        *const u8 as *const libc::c_char,
                    (*instr).op.opcode as libc::c_int,
                    (*instr).op.name.as_mut_ptr());
        }
        printf(b"repne \x00" as *const u8 as *const libc::c_char);
    }
    if (*instr).prefix as libc::c_int & 0x80 as libc::c_int != 0 {
        if (*instr).op.flags & 0x20 as libc::c_int as libc::c_uint == 0 {
            fprintf(stderr,
                    b"Warning: %s: \x00" as *const u8 as *const libc::c_char,
                    ip);
            fprintf(stderr,
                    b"repe prefix used with opcode 0x%02x %s\n\x00" as
                        *const u8 as *const libc::c_char,
                    (*instr).op.opcode as libc::c_int,
                    (*instr).op.name.as_mut_ptr());
        }
        printf(if (*instr).op.flags & 0x10 as libc::c_int as libc::c_uint != 0
                  {
                   b"repe \x00" as *const u8 as *const libc::c_char
               } else { b"rep \x00" as *const u8 as *const libc::c_char });
    }
    if (*instr).prefix as libc::c_int & 0x100 as libc::c_int != 0 {
        printf(b"wait \x00" as *const u8 as *const libc::c_char);
    }
    if (*instr).vex() != 0 {
        printf(b"v\x00" as *const u8 as *const libc::c_char);
    }
    printf(b"%s\x00" as *const u8 as *const libc::c_char,
           (*instr).op.name.as_mut_ptr());
    if (*instr).args[0 as libc::c_int as
                         usize].string[0 as libc::c_int as usize] as
           libc::c_int != 0 ||
           (*instr).args[1 as libc::c_int as
                             usize].string[0 as libc::c_int as usize] as
               libc::c_int != 0 {
        printf(b"\t\x00" as *const u8 as *const libc::c_char);
    }
    if asm_syntax as libc::c_uint == GAS as libc::c_int as libc::c_uint {
        /* fixme: are all of these orderings correct? */
        if (*instr).args[1 as libc::c_int as
                             usize].string[0 as libc::c_int as usize] != 0 {
            printf(b"%s,\x00" as *const u8 as *const libc::c_char,
                   (*instr).args[1 as libc::c_int as
                                     usize].string.as_mut_ptr());
        }
        if (*instr).vex_reg() != 0 {
            printf(b"%%ymm%d, \x00" as *const u8 as *const libc::c_char,
                   (*instr).vex_reg() as libc::c_int);
        }
        if (*instr).args[0 as libc::c_int as
                             usize].string[0 as libc::c_int as usize] != 0 {
            printf(b"%s\x00" as *const u8 as *const libc::c_char,
                   (*instr).args[0 as libc::c_int as
                                     usize].string.as_mut_ptr());
        }
        if (*instr).args[2 as libc::c_int as
                             usize].string[0 as libc::c_int as usize] != 0 {
            printf(b",%s\x00" as *const u8 as *const libc::c_char,
                   (*instr).args[2 as libc::c_int as
                                     usize].string.as_mut_ptr());
        }
    } else {
        if (*instr).args[0 as libc::c_int as
                             usize].string[0 as libc::c_int as usize] != 0 {
            printf(b"%s\x00" as *const u8 as *const libc::c_char,
                   (*instr).args[0 as libc::c_int as
                                     usize].string.as_mut_ptr());
        }
        if (*instr).args[1 as libc::c_int as
                             usize].string[0 as libc::c_int as usize] != 0 {
            printf(b", \x00" as *const u8 as *const libc::c_char);
        }
        if (*instr).vex_reg() != 0 {
            printf(b"ymm%d, \x00" as *const u8 as *const libc::c_char,
                   (*instr).vex_reg() as libc::c_int);
        }
        if (*instr).args[1 as libc::c_int as
                             usize].string[0 as libc::c_int as usize] != 0 {
            printf(b"%s\x00" as *const u8 as *const libc::c_char,
                   (*instr).args[1 as libc::c_int as
                                     usize].string.as_mut_ptr());
        }
        if (*instr).args[2 as libc::c_int as
                             usize].string[0 as libc::c_int as usize] != 0 {
            printf(b", %s\x00" as *const u8 as *const libc::c_char,
                   (*instr).args[2 as libc::c_int as
                                     usize].string.as_mut_ptr());
        }
    }
    if !comment.is_null() {
        printf(if asm_syntax as libc::c_uint ==
                      GAS as libc::c_int as libc::c_uint {
                   b"\t// \x00" as *const u8 as *const libc::c_char
               } else { b"\t;\x00" as *const u8 as *const libc::c_char });
        printf(b" <%s>\x00" as *const u8 as *const libc::c_char, comment);
    }
    /* if we have more than 7 bytes on this line, wrap around */
    if len > 7 as libc::c_int && opts as libc::c_int & 0x4 as libc::c_int == 0
       {
        printf(b"\n\t\t\x00" as *const u8 as *const libc::c_char);
        i = 7 as libc::c_int;
        while i < len {
            printf(b"%02x\x00" as *const u8 as *const libc::c_char,
                   *p.offset(i as isize) as libc::c_int);
            if i < len {
                printf(b" \x00" as *const u8 as *const libc::c_char);
            }
            i += 1
        }
    }
    printf(b"\n\x00" as *const u8 as *const libc::c_char);
}
