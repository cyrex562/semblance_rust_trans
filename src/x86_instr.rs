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

#[repr(C)]
#[derive(Copy, Clone)]
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

#[repr(C)]
#[derive(Copy, Clone)]
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Operation {
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Argument {
    pub string: [libc::c_char; 32],
    pub ip: dword,
    pub value: qword,
    pub type_0: argtype,
}

#[repr(C)]
#[derive(Copy, Clone, BitfieldStruct)]
pub struct Instruction {
    pub prefix: word,
    pub op: Operation,
    pub args: [Argument; 3],
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
pub static mut f: *mut FILE = 0 as *mut FILE;
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
static mut instructions: [Operation; 256] =
    [{
        let mut init =
            Operation {
                opcode: 0u16,
                subcode: 8u8,
                size: 8i8,
                name: [97, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: REG,
                flags: 0x80u32,
            };
        init
    },
        {
            let mut init =
                Operation {
                    opcode: 0x1u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [97, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: REG,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x2u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [97, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x3u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [97, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x4u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [97, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AL,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x5u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [97, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AX,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x6u16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [112, 117, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: ES,
                    arg1: NONE,
                    flags: 0x100u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x7u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [112, 111, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: ES,
                    arg1: NONE,
                    flags: 0x100u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x8u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: REG,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x9u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: REG,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xau16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xbu16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xcu16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AL,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdu16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AX,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xeu16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [112, 117, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: CS,
                    arg1: NONE,
                    flags: 0x100u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xfu16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x10u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [97, 100, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: REG,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x11u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [97, 100, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: REG,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x12u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [97, 100, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x13u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [97, 100, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x14u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [97, 100, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AL,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x15u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [97, 100, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AX,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x16u16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [112, 117, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: SS,
                    arg1: NONE,
                    flags: 0x100u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x17u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [112, 111, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: SS,
                    arg1: NONE,
                    flags: 0x100u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x18u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [115, 98, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: REG,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x19u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [115, 98, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: REG,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x1au16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [115, 98, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x1bu16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [115, 98, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x1cu16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [115, 98, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AL,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x1du16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [115, 98, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AX,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x1eu16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [112, 117, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: DS,
                    arg1: NONE,
                    flags: 0x100u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x2fu16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [112, 111, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: DS,
                    arg1: NONE,
                    flags: 0x100u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x20u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [97, 110, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: REG,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x21u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [97, 110, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: REG,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x22u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [97, 110, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x23u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [97, 110, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x24u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [97, 110, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AL,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x25u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [97, 110, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AX,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x26u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [101, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x27u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [100, 97, 97, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x28u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [115, 117, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: REG,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x29u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [115, 117, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: REG,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x2au16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [115, 117, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x2bu16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [115, 117, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x2cu16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [115, 117, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AL,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x2du16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [115, 117, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AX,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x2eu16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [99, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x2fu16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [100, 97, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x30u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [120, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: REG,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x31u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [120, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: REG,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x32u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [120, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x33u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [120, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x34u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [120, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AL,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x35u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [120, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AX,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x36u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [115, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x37u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [97, 97, 97, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [99, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: REG,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x39u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [99, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: REG,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x3au16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [99, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x3bu16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [99, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x3cu16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [99, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AL,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x3du16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [99, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AX,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x3eu16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [100, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x3fu16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [97, 97, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x40u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [105, 110, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AX,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x41u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [105, 110, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: CX,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x42u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [105, 110, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: DX,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x43u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [105, 110, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: BX,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x44u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [105, 110, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: SP,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x45u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [105, 110, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: BP,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x46u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [105, 110, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: SI,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x47u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [105, 110, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: DI,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x48u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [100, 101, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AX,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x49u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [100, 101, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: CX,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x4au16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [100, 101, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: DX,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x4bu16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [100, 101, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: BX,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x4cu16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [100, 101, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: SP,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x4du16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [100, 101, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: BP,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x4eu16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [100, 101, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: SI,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x4fu16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [100, 101, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: DI,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x50u16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [112, 117, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AX,
                    arg1: NONE,
                    flags: 0x100u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x51u16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [112, 117, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: CX,
                    arg1: NONE,
                    flags: 0x100u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x52u16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [112, 117, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: DX,
                    arg1: NONE,
                    flags: 0x100u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x53u16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [112, 117, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: BX,
                    arg1: NONE,
                    flags: 0x100u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x54u16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [112, 117, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: SP,
                    arg1: NONE,
                    flags: 0x100u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x55u16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [112, 117, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: BP,
                    arg1: NONE,
                    flags: 0x100u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x56u16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [112, 117, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: SI,
                    arg1: NONE,
                    flags: 0x100u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x57u16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [112, 117, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: DI,
                    arg1: NONE,
                    flags: 0x100u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x58u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [112, 111, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AX,
                    arg1: NONE,
                    flags: 0x100u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x59u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [112, 111, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: CX,
                    arg1: NONE,
                    flags: 0x100u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x5au16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [112, 111, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: DX,
                    arg1: NONE,
                    flags: 0x100u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x5bu16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [112, 111, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: BX,
                    arg1: NONE,
                    flags: 0x100u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x5cu16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [112, 111, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: SP,
                    arg1: NONE,
                    flags: 0x100u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x5du16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [112, 111, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: BP,
                    arg1: NONE,
                    flags: 0x100u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x5eu16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [112, 111, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: SI,
                    arg1: NONE,
                    flags: 0x100u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x5fu16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [112, 111, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: DI,
                    arg1: NONE,
                    flags: 0x100u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x60u16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [112, 117, 115, 104, 97, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0x100u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x61u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [112, 111, 112, 97, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0x100u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x62u16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [98, 111, 117, 110, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: MEM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x63u16,
                    subcode: 8u8,
                    size: 16i8,
                    name: [97, 114, 112, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: REG,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x64u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [102, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x65u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [103, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x66u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [100, 97, 116, 97, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x67u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [97, 100, 100, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x68u16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [112, 117, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: IMM,
                    arg1: NONE,
                    flags: 0x100u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x69u16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [105, 109, 117, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0x1u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x6au16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [112, 117, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: IMM8,
                    arg1: NONE,
                    flags: 0x100u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x6bu16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [105, 109, 117, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0x2u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x6cu16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [105, 110, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: ESDI,
                    arg1: DXS,
                    flags:
                    (0x200i32 | 0x20i32) as dword,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x6du16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [105, 110, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: ESDI,
                    arg1: DXS,
                    flags:
                    (0x200i32 | 0x20i32) as dword,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x6eu16,
                    subcode: 8u8,
                    size: 8i8,
                    name:
                    [111, 117, 116, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: DXS,
                    arg1: DSSI,
                    flags:
                    (0x200i32 | 0x20i32) as dword,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x6fu16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [111, 117, 116, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: DXS,
                    arg1: DSSI,
                    flags:
                    (0x200i32 | 0x20i32) as dword,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x70u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [106, 111, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL8,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x71u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [106, 110, 111, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL8,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x72u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [106, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL8,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x73u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [106, 97, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL8,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x74u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [106, 122, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL8,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x75u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [106, 110, 122, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL8,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x76u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [106, 98, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL8,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x77u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [106, 97, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL8,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x78u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [106, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL8,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x79u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [106, 110, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL8,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x7au16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [106, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL8,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x7bu16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [106, 110, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL8,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x7cu16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [106, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL8,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x7du16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [106, 103, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL8,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x7eu16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [106, 108, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL8,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x7fu16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [106, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL8,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x80u16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x81u16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x82u16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x83u16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x84u16,
                    subcode: 8u8,
                    size: 8i8,
                    name:
                    [116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: REG,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x85u16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: REG,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x86u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [120, 99, 104, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x87u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [120, 99, 104, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x88u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: REG,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x89u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: REG,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x8au16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x8bu16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x8cu16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: SEG16,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x8du16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [108, 101, 97, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: MEM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x8eu16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: SEG16,
                    arg1: RM,
                    flags: 0x40u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x8fu16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x90u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [110, 111, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0x20u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x91u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [120, 99, 104, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AX,
                    arg1: CX,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x92u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [120, 99, 104, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AX,
                    arg1: DX,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x93u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [120, 99, 104, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AX,
                    arg1: BX,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x94u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [120, 99, 104, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AX,
                    arg1: SP,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x95u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [120, 99, 104, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AX,
                    arg1: BP,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x96u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [120, 99, 104, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AX,
                    arg1: SI,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x97u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [120, 99, 104, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AX,
                    arg1: DI,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x98u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [99, 98, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x99u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [99, 119, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x9au16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [99, 97, 108, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: PTR32,
                    arg1: NONE,
                    flags: 0x400u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x9bu16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [119, 97, 105, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x9cu16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [112, 117, 115, 104, 102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0x100u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x9du16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [112, 111, 112, 102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0x100u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x9eu16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [115, 97, 104, 102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x9fu16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [108, 97, 104, 102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xa0u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AL,
                    arg1: MOFFS16,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xa1u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AX,
                    arg1: MOFFS16,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xa2u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MOFFS16,
                    arg1: AL,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xa3u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MOFFS16,
                    arg1: AX,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xa4u16,
                    subcode: 8u8,
                    size: 8i8,
                    name:
                    [109, 111, 118, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: DSSI,
                    arg1: ESDI,
                    flags:
                    (0x200i32 | 0x20i32) as dword,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xa5u16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [109, 111, 118, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: DSSI,
                    arg1: ESDI,
                    flags:
                    (0x200i32 | 0x20i32) as dword,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xa6u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [99, 109, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: DSSI,
                    arg1: ESDI,
                    flags:
                    (0x200i32 | 0x10i32 |
                        0x20i32) as dword,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xa7u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [99, 109, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: DSSI,
                    arg1: ESDI,
                    flags:
                    (0x200i32 | 0x10i32 |
                        0x20i32) as dword,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xa8u16,
                    subcode: 8u8,
                    size: 8i8,
                    name:
                    [116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AL,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xa9u16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AX,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xaau16,
                    subcode: 8u8,
                    size: 8i8,
                    name:
                    [115, 116, 111, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: ESDI,
                    arg1: ALS,
                    flags:
                    (0x200i32 | 0x20i32) as dword,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xabu16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [115, 116, 111, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: ESDI,
                    arg1: AXS,
                    flags:
                    (0x200i32 | 0x20i32) as dword,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xacu16,
                    subcode: 8u8,
                    size: 8i8,
                    name:
                    [108, 111, 100, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: ALS,
                    arg1: DSSI,
                    flags:
                    (0x200i32 | 0x20i32) as dword,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xadu16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [108, 111, 100, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AXS,
                    arg1: DSSI,
                    flags:
                    (0x200i32 | 0x20i32) as dword,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xaeu16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [115, 99, 97, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: ALS,
                    arg1: ESDI,
                    flags:
                    (0x200i32 | 0x10i32 |
                        0x20i32) as dword,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xafu16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [115, 99, 97, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AXS,
                    arg1: ESDI,
                    flags:
                    (0x200i32 | 0x10i32 |
                        0x20i32) as dword,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xb0u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AL,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xb1u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: CL,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xb2u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: DL,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xb3u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: BL,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xb4u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AH,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xb5u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: CH,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xb6u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: DH,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xb7u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: BH,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xb8u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AX,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xb9u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: CX,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xbau16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: DX,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xbbu16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: BX,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xbcu16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: SP,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xbdu16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: BP,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xbeu16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: SI,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xbfu16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: DI,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xc0u16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xc1u16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xc2u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [114, 101, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: IMM16,
                    arg1: NONE,
                    flags: 0x4000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xc3u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [114, 101, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags:
                    (0x4000i32 | 0x20i32 |
                        0x10i32) as dword,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xc4u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [108, 101, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: MEM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xc5u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [108, 100, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: MEM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xc6u16,
                    subcode: 0u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xc7u16,
                    subcode: 0u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xc8u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [101, 110, 116, 101, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: IMM16,
                    arg1: IMM8,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xc9u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [108, 101, 97, 118, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xcau16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [114, 101, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: IMM16,
                    arg1: NONE,
                    flags:
                    (0x4000i32 | 0x400i32) as dword,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xcbu16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [114, 101, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags:
                    (0x4000i32 | 0x400i32) as dword,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xccu16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [105, 110, 116, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0x4000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xcdu16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [105, 110, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: IMM8,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xceu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [105, 110, 116, 111, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xcfu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [105, 114, 101, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0x4000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd0u16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd1u16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd2u16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd3u16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd4u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [97, 109, 120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: IMM8,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd5u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [97, 100, 120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: IMM8,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd6u16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd7u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [120, 108, 97, 116, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: DSBX,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd8u16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd9u16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdau16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdbu16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdcu16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xddu16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdeu16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdfu16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xe0u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [108, 111, 111, 112, 110, 122, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: REL8,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xe1u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [108, 111, 111, 112, 122, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: REL8,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xe2u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [108, 111, 111, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL8,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xe3u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [106, 99, 120, 122, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL8,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xe4u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [105, 110, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AL,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xe5u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [105, 110, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AX,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xe6u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [111, 117, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: IMM,
                    arg1: AL,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xe7u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [111, 117, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: IMM,
                    arg1: AX,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xe8u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [99, 97, 108, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL16,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xe9u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [106, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL16,
                    arg1: NONE,
                    flags:
                    (0x8000i32 | 0x4000i32) as
                        dword,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xeau16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [106, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: PTR32,
                    arg1: NONE,
                    flags:
                    (0x400i32 | 0x4000i32) as dword,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xebu16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [106, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL8,
                    arg1: NONE,
                    flags:
                    (0x8000i32 | 0x4000i32) as
                        dword,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xecu16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [105, 110, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AL,
                    arg1: DXS,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xedu16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [105, 110, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AX,
                    arg1: DXS,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xeeu16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [111, 117, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: DXS,
                    arg1: AL,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xefu16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [111, 117, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: DXS,
                    arg1: AX,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xf0u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [108, 111, 99, 107, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xf1u16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xf2u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [114, 101, 112, 110, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xf3u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [114, 101, 112, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xf4u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [104, 108, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xf5u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [99, 109, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xf6u16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xf7u16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xf8u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [99, 108, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xf9u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [115, 116, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xfau16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [99, 108, 105, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xfbu16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [115, 116, 105, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xfcu16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [99, 108, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xfdu16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [115, 116, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xfeu16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xffu16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        }];
static mut instructions64: [Operation; 256] =
    [{
        let mut init =
            Operation {
                opcode: 0u16,
                subcode: 8u8,
                size: 8i8,
                name: [97, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: REG,
                flags: 0x80u32,
            };
        init
    },
        {
            let mut init =
                Operation {
                    opcode: 0x1u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [97, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: REG,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x2u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [97, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x3u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [97, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x4u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [97, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AL,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x5u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [97, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AX,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x6u16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x7u16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x8u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: REG,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x9u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: REG,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xau16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xbu16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xcu16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AL,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdu16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AX,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xeu16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xfu16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x10u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [97, 100, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: REG,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x11u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [97, 100, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: REG,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x12u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [97, 100, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x13u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [97, 100, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x14u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [97, 100, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AL,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x15u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [97, 100, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AX,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x16u16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x17u16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x18u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [115, 98, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: REG,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x19u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [115, 98, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: REG,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x1au16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [115, 98, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x1bu16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [115, 98, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x1cu16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [115, 98, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AL,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x1du16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [115, 98, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AX,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x1eu16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x1fu16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x20u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [97, 110, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: REG,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x21u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [97, 110, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: REG,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x22u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [97, 110, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x23u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [97, 110, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x24u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [97, 110, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AL,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x25u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [97, 110, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AX,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x26u16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x27u16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x28u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [115, 117, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: REG,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x29u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [115, 117, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: REG,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x2au16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [115, 117, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x2bu16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [115, 117, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x2cu16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [115, 117, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AL,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x2du16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [115, 117, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AX,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x2eu16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x2fu16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x30u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [120, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: REG,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x31u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [120, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: REG,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x32u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [120, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x33u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [120, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x34u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [120, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AL,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x35u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [120, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AX,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x36u16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x37u16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [99, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: REG,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x39u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [99, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: REG,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x3au16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [99, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x3bu16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [99, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x3cu16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [99, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AL,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x3du16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [99, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AX,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x3eu16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x3fu16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x40u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [114, 101, 120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x41u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [114, 101, 120, 46, 66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x42u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [114, 101, 120, 46, 88, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x43u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [114, 101, 120, 46, 88, 66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x44u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [114, 101, 120, 46, 82, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x45u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [114, 101, 120, 46, 82, 66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x46u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [114, 101, 120, 46, 82, 88, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x47u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [114, 101, 120, 46, 82, 88, 66, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x48u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [114, 101, 120, 46, 87, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x49u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [114, 101, 120, 46, 87, 66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x4au16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [114, 101, 120, 46, 87, 88, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x4bu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [114, 101, 120, 46, 87, 88, 66, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x4cu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [114, 101, 120, 46, 87, 82, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x4du16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [114, 101, 120, 46, 87, 82, 66, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x4eu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [114, 101, 120, 46, 87, 82, 88, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x4fu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [114, 101, 120, 46, 87, 82, 88, 66, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x50u16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [112, 117, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AX,
                    arg1: NONE,
                    flags: 0x100u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x51u16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [112, 117, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: CX,
                    arg1: NONE,
                    flags: 0x100u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x52u16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [112, 117, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: DX,
                    arg1: NONE,
                    flags: 0x100u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x53u16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [112, 117, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: BX,
                    arg1: NONE,
                    flags: 0x100u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x54u16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [112, 117, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: SP,
                    arg1: NONE,
                    flags: 0x100u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x55u16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [112, 117, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: BP,
                    arg1: NONE,
                    flags: 0x100u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x56u16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [112, 117, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: SI,
                    arg1: NONE,
                    flags: 0x100u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x57u16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [112, 117, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: DI,
                    arg1: NONE,
                    flags: 0x100u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x58u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [112, 111, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AX,
                    arg1: NONE,
                    flags: 0x100u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x59u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [112, 111, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: CX,
                    arg1: NONE,
                    flags: 0x100u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x5au16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [112, 111, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: DX,
                    arg1: NONE,
                    flags: 0x100u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x5bu16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [112, 111, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: BX,
                    arg1: NONE,
                    flags: 0x100u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x5cu16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [112, 111, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: SP,
                    arg1: NONE,
                    flags: 0x100u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x5du16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [112, 111, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: BP,
                    arg1: NONE,
                    flags: 0x100u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x5eu16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [112, 111, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: SI,
                    arg1: NONE,
                    flags: 0x100u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x5fu16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [112, 111, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: DI,
                    arg1: NONE,
                    flags: 0x100u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x60u16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x61u16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x62u16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x63u16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [109, 111, 118, 115, 120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x64u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [102, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x65u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [103, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x66u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [100, 97, 116, 97, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x67u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [97, 100, 100, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x68u16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [112, 117, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: IMM,
                    arg1: NONE,
                    flags: 0x100u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x69u16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [105, 109, 117, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0x1u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x6au16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [112, 117, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: IMM8,
                    arg1: NONE,
                    flags: 0x100u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x6bu16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [105, 109, 117, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0x2u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x6cu16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [105, 110, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: ESDI,
                    arg1: DXS,
                    flags:
                    (0x200i32 | 0x20i32) as dword,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x6du16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [105, 110, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: ESDI,
                    arg1: DXS,
                    flags:
                    (0x200i32 | 0x20i32) as dword,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x6eu16,
                    subcode: 8u8,
                    size: 8i8,
                    name:
                    [111, 117, 116, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: DXS,
                    arg1: DSSI,
                    flags:
                    (0x200i32 | 0x20i32) as dword,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x6fu16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [111, 117, 116, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: DXS,
                    arg1: DSSI,
                    flags:
                    (0x200i32 | 0x20i32) as dword,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x70u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [106, 111, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL8,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x71u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [106, 110, 111, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL8,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x72u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [106, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL8,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x73u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [106, 97, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL8,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x74u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [106, 122, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL8,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x75u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [106, 110, 122, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL8,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x76u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [106, 98, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL8,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x77u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [106, 97, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL8,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x78u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [106, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL8,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x79u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [106, 110, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL8,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x7au16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [106, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL8,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x7bu16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [106, 110, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL8,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x7cu16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [106, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL8,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x7du16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [106, 103, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL8,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x7eu16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [106, 108, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL8,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x7fu16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [106, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL8,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x80u16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x81u16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x82u16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x83u16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x84u16,
                    subcode: 8u8,
                    size: 8i8,
                    name:
                    [116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: REG,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x85u16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: REG,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x86u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [120, 99, 104, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x87u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [120, 99, 104, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x88u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: REG,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x89u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: REG,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x8au16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x8bu16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x8cu16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: SEG16,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x8du16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [108, 101, 97, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: MEM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x8eu16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: SEG16,
                    arg1: RM,
                    flags: 0x40u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x8fu16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x90u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [110, 111, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0x20u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x91u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [120, 99, 104, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AX,
                    arg1: CX,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x92u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [120, 99, 104, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AX,
                    arg1: DX,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x93u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [120, 99, 104, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AX,
                    arg1: BX,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x94u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [120, 99, 104, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AX,
                    arg1: SP,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x95u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [120, 99, 104, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AX,
                    arg1: BP,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x96u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [120, 99, 104, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AX,
                    arg1: SI,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x97u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [120, 99, 104, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AX,
                    arg1: DI,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x98u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [99, 98, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x99u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [99, 119, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x9au16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x9bu16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [119, 97, 105, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x9cu16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [112, 117, 115, 104, 102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0x100u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x9du16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [112, 111, 112, 102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0x100u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x9eu16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [115, 97, 104, 102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x9fu16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [108, 97, 104, 102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xa0u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AL,
                    arg1: MOFFS16,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xa1u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AX,
                    arg1: MOFFS16,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xa2u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MOFFS16,
                    arg1: AL,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xa3u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MOFFS16,
                    arg1: AX,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xa4u16,
                    subcode: 8u8,
                    size: 8i8,
                    name:
                    [109, 111, 118, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: DSSI,
                    arg1: ESDI,
                    flags:
                    (0x200i32 | 0x20i32) as dword,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xa5u16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [109, 111, 118, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: DSSI,
                    arg1: ESDI,
                    flags:
                    (0x200i32 | 0x20i32) as dword,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xa6u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [99, 109, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: DSSI,
                    arg1: ESDI,
                    flags:
                    (0x200i32 | 0x10i32 |
                        0x20i32) as dword,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xa7u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [99, 109, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: DSSI,
                    arg1: ESDI,
                    flags:
                    (0x200i32 | 0x10i32 |
                        0x20i32) as dword,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xa8u16,
                    subcode: 8u8,
                    size: 8i8,
                    name:
                    [116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AL,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xa9u16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AX,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xaau16,
                    subcode: 8u8,
                    size: 8i8,
                    name:
                    [115, 116, 111, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: ESDI,
                    arg1: ALS,
                    flags:
                    (0x200i32 | 0x20i32) as dword,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xabu16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [115, 116, 111, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: ESDI,
                    arg1: AXS,
                    flags:
                    (0x200i32 | 0x20i32) as dword,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xacu16,
                    subcode: 8u8,
                    size: 8i8,
                    name:
                    [108, 111, 100, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: ALS,
                    arg1: DSSI,
                    flags:
                    (0x200i32 | 0x20i32) as dword,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xadu16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [108, 111, 100, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AXS,
                    arg1: DSSI,
                    flags:
                    (0x200i32 | 0x20i32) as dword,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xaeu16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [115, 99, 97, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: ALS,
                    arg1: ESDI,
                    flags:
                    (0x200i32 | 0x10i32 |
                        0x20i32) as dword,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xafu16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [115, 99, 97, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AXS,
                    arg1: ESDI,
                    flags:
                    (0x200i32 | 0x10i32 |
                        0x20i32) as dword,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xb0u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AL,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xb1u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: CL,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xb2u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: DL,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xb3u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: BL,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xb4u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AH,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xb5u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: CH,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xb6u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: DH,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xb7u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: BH,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xb8u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AX,
                    arg1: IMM,
                    flags: 0x800u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xb9u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: CX,
                    arg1: IMM,
                    flags: 0x800u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xbau16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: DX,
                    arg1: IMM,
                    flags: 0x800u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xbbu16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: BX,
                    arg1: IMM,
                    flags: 0x800u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xbcu16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: SP,
                    arg1: IMM,
                    flags: 0x800u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xbdu16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: BP,
                    arg1: IMM,
                    flags: 0x800u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xbeu16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: SI,
                    arg1: IMM,
                    flags: 0x800u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xbfu16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: DI,
                    arg1: IMM,
                    flags: 0x800u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xc0u16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xc1u16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xc2u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [114, 101, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: IMM16,
                    arg1: NONE,
                    flags: 0x4000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xc3u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [114, 101, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags:
                    (0x4000i32 | 0x20i32 |
                        0x10i32) as dword,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xc4u16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xc5u16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xc6u16,
                    subcode: 0u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xc7u16,
                    subcode: 0u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xc8u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [101, 110, 116, 101, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: IMM16,
                    arg1: IMM8,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xc9u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [108, 101, 97, 118, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xcau16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [114, 101, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: IMM16,
                    arg1: NONE,
                    flags:
                    (0x4000i32 | 0x400i32) as dword,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xcbu16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [114, 101, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags:
                    (0x4000i32 | 0x400i32) as dword,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xccu16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [105, 110, 116, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0x4000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xcdu16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [105, 110, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: IMM8,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xceu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [105, 110, 116, 111, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xcfu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [105, 114, 101, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0x4000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd0u16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd1u16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd2u16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd3u16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd4u16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd5u16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd6u16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd7u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [120, 108, 97, 116, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: DSBX,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd8u16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd9u16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdau16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdbu16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdcu16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xddu16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdeu16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdfu16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xe0u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [108, 111, 111, 112, 110, 122, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: REL8,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xe1u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [108, 111, 111, 112, 122, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: REL8,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xe2u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [108, 111, 111, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL8,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xe3u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [106, 99, 120, 122, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL8,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xe4u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [105, 110, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AL,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xe5u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [105, 110, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AX,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xe6u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [111, 117, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: IMM,
                    arg1: AL,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xe7u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [111, 117, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: IMM,
                    arg1: AX,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xe8u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [99, 97, 108, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL16,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xe9u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [106, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL16,
                    arg1: NONE,
                    flags:
                    (0x8000i32 | 0x4000i32) as
                        dword,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xeau16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xebu16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [106, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL8,
                    arg1: NONE,
                    flags:
                    (0x8000i32 | 0x4000i32) as
                        dword,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xecu16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [105, 110, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AL,
                    arg1: DXS,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xedu16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [105, 110, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AX,
                    arg1: DXS,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xeeu16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [111, 117, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: DXS,
                    arg1: AL,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xefu16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [111, 117, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: DXS,
                    arg1: AX,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xf0u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [108, 111, 99, 107, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xf1u16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xf2u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [114, 101, 112, 110, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xf3u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [114, 101, 112, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xf4u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [104, 108, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xf5u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [99, 109, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xf6u16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xf7u16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xf8u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [99, 108, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xf9u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [115, 116, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xfau16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [99, 108, 105, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xfbu16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [115, 116, 105, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xfcu16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [99, 108, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xfdu16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [115, 116, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xfeu16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xffu16,
                    subcode: 8u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        }];
static mut instructions_group: [Operation; 108] =
    [{
        let mut init =
            Operation {
                opcode: 0x80u16,
                subcode: 0u8,
                size: 8i8,
                name: [97, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: IMM,
                flags: 0x80u32,
            };
        init
    },
        {
            let mut init =
                Operation {
                    opcode: 0x80u16,
                    subcode: 1u8,
                    size: 8i8,
                    name: [111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: IMM,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x80u16,
                    subcode: 2u8,
                    size: 8i8,
                    name: [97, 100, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: IMM,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x80u16,
                    subcode: 3u8,
                    size: 8i8,
                    name: [115, 98, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: IMM,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x80u16,
                    subcode: 4u8,
                    size: 8i8,
                    name: [97, 110, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: IMM,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x80u16,
                    subcode: 5u8,
                    size: 8i8,
                    name: [115, 117, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: IMM,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x80u16,
                    subcode: 6u8,
                    size: 8i8,
                    name: [120, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: IMM,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x80u16,
                    subcode: 7u8,
                    size: 8i8,
                    name: [99, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x81u16,
                    subcode: 0u8,
                    size: -1i8,
                    name: [97, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: IMM,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x81u16,
                    subcode: 1u8,
                    size: -1i8,
                    name: [111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: IMM,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x81u16,
                    subcode: 2u8,
                    size: -1i8,
                    name: [97, 100, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: IMM,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x81u16,
                    subcode: 3u8,
                    size: -1i8,
                    name: [115, 98, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: IMM,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x81u16,
                    subcode: 4u8,
                    size: -1i8,
                    name: [97, 110, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: IMM,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x81u16,
                    subcode: 5u8,
                    size: -1i8,
                    name: [115, 117, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: IMM,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x81u16,
                    subcode: 6u8,
                    size: -1i8,
                    name: [120, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: IMM,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x81u16,
                    subcode: 7u8,
                    size: -1i8,
                    name: [99, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x82u16,
                    subcode: 0u8,
                    size: 8i8,
                    name: [97, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: IMM8,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x82u16,
                    subcode: 1u8,
                    size: 8i8,
                    name: [111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: IMM8,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x82u16,
                    subcode: 2u8,
                    size: 8i8,
                    name: [97, 100, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: IMM8,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x82u16,
                    subcode: 3u8,
                    size: 8i8,
                    name: [115, 98, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: IMM8,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x82u16,
                    subcode: 4u8,
                    size: 8i8,
                    name: [97, 110, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: IMM8,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x82u16,
                    subcode: 5u8,
                    size: 8i8,
                    name: [115, 117, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: IMM8,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x82u16,
                    subcode: 6u8,
                    size: 8i8,
                    name: [120, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: IMM8,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x82u16,
                    subcode: 7u8,
                    size: 8i8,
                    name: [99, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: IMM8,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x83u16,
                    subcode: 0u8,
                    size: -1i8,
                    name: [97, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: IMM8,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x83u16,
                    subcode: 1u8,
                    size: -1i8,
                    name: [111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: IMM8,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x83u16,
                    subcode: 2u8,
                    size: -1i8,
                    name: [97, 100, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: IMM8,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x83u16,
                    subcode: 3u8,
                    size: -1i8,
                    name: [115, 98, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: IMM8,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x83u16,
                    subcode: 4u8,
                    size: -1i8,
                    name: [97, 110, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: IMM8,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x83u16,
                    subcode: 5u8,
                    size: -1i8,
                    name: [115, 117, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: IMM8,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x83u16,
                    subcode: 6u8,
                    size: -1i8,
                    name: [120, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: IMM8,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x83u16,
                    subcode: 7u8,
                    size: -1i8,
                    name: [99, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: IMM8,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x8fu16,
                    subcode: 0u8,
                    size: -1i8,
                    name: [112, 111, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: NONE,
                    flags: 0x100u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xc0u16,
                    subcode: 0u8,
                    size: 8i8,
                    name: [114, 111, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: IMM8,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xc0u16,
                    subcode: 1u8,
                    size: 8i8,
                    name: [114, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: IMM8,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xc0u16,
                    subcode: 2u8,
                    size: 8i8,
                    name: [114, 99, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: IMM8,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xc0u16,
                    subcode: 3u8,
                    size: 8i8,
                    name: [114, 99, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: IMM8,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xc0u16,
                    subcode: 4u8,
                    size: 8i8,
                    name: [115, 104, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: IMM8,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xc0u16,
                    subcode: 5u8,
                    size: 8i8,
                    name: [115, 104, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: IMM8,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xc0u16,
                    subcode: 6u8,
                    size: 8i8,
                    name: [115, 97, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: IMM8,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xc0u16,
                    subcode: 7u8,
                    size: 8i8,
                    name: [115, 97, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: IMM8,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xc1u16,
                    subcode: 0u8,
                    size: -1i8,
                    name: [114, 111, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: IMM8,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xc1u16,
                    subcode: 1u8,
                    size: -1i8,
                    name: [114, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: IMM8,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xc1u16,
                    subcode: 2u8,
                    size: -1i8,
                    name: [114, 99, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: IMM8,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xc1u16,
                    subcode: 3u8,
                    size: -1i8,
                    name: [114, 99, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: IMM8,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xc1u16,
                    subcode: 4u8,
                    size: -1i8,
                    name: [115, 104, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: IMM8,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xc1u16,
                    subcode: 5u8,
                    size: -1i8,
                    name: [115, 104, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: IMM8,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xc1u16,
                    subcode: 6u8,
                    size: -1i8,
                    name: [115, 97, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: IMM8,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xc1u16,
                    subcode: 7u8,
                    size: -1i8,
                    name: [115, 97, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: IMM8,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xc6u16,
                    subcode: 0u8,
                    size: 8i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xc7u16,
                    subcode: 0u8,
                    size: -1i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd0u16,
                    subcode: 0u8,
                    size: 8i8,
                    name: [114, 111, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: ONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd0u16,
                    subcode: 1u8,
                    size: 8i8,
                    name: [114, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: ONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd0u16,
                    subcode: 2u8,
                    size: 8i8,
                    name: [114, 99, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: ONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd0u16,
                    subcode: 3u8,
                    size: 8i8,
                    name: [114, 99, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: ONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd0u16,
                    subcode: 4u8,
                    size: 8i8,
                    name: [115, 104, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: ONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd0u16,
                    subcode: 5u8,
                    size: 8i8,
                    name: [115, 104, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: ONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd0u16,
                    subcode: 6u8,
                    size: 8i8,
                    name: [115, 97, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: ONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd0u16,
                    subcode: 7u8,
                    size: 8i8,
                    name: [115, 97, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: ONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd1u16,
                    subcode: 0u8,
                    size: -1i8,
                    name: [114, 111, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: ONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd1u16,
                    subcode: 1u8,
                    size: -1i8,
                    name: [114, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: ONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd1u16,
                    subcode: 2u8,
                    size: -1i8,
                    name: [114, 99, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: ONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd1u16,
                    subcode: 3u8,
                    size: -1i8,
                    name: [114, 99, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: ONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd1u16,
                    subcode: 4u8,
                    size: -1i8,
                    name: [115, 104, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: ONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd1u16,
                    subcode: 5u8,
                    size: -1i8,
                    name: [115, 104, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: ONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd1u16,
                    subcode: 6u8,
                    size: -1i8,
                    name: [115, 97, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: ONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd1u16,
                    subcode: 7u8,
                    size: -1i8,
                    name: [115, 97, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: ONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd2u16,
                    subcode: 0u8,
                    size: 8i8,
                    name: [114, 111, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: CL,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd2u16,
                    subcode: 1u8,
                    size: 8i8,
                    name: [114, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: CL,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd2u16,
                    subcode: 2u8,
                    size: 8i8,
                    name: [114, 99, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: CL,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd2u16,
                    subcode: 3u8,
                    size: 8i8,
                    name: [114, 99, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: CL,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd2u16,
                    subcode: 4u8,
                    size: 8i8,
                    name: [115, 104, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: CL,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd2u16,
                    subcode: 5u8,
                    size: 8i8,
                    name: [115, 104, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: CL,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd2u16,
                    subcode: 6u8,
                    size: 8i8,
                    name: [115, 97, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: CL,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd2u16,
                    subcode: 7u8,
                    size: 8i8,
                    name: [115, 97, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: CL,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd3u16,
                    subcode: 0u8,
                    size: -1i8,
                    name: [114, 111, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: CL,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd3u16,
                    subcode: 1u8,
                    size: -1i8,
                    name: [114, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: CL,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd3u16,
                    subcode: 2u8,
                    size: -1i8,
                    name: [114, 99, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: CL,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd3u16,
                    subcode: 3u8,
                    size: -1i8,
                    name: [114, 99, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: CL,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd3u16,
                    subcode: 4u8,
                    size: -1i8,
                    name: [115, 104, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: CL,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd3u16,
                    subcode: 5u8,
                    size: -1i8,
                    name: [115, 104, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: CL,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd3u16,
                    subcode: 6u8,
                    size: -1i8,
                    name: [115, 97, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: CL,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd3u16,
                    subcode: 7u8,
                    size: -1i8,
                    name: [115, 97, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: CL,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xf6u16,
                    subcode: 0u8,
                    size: 8i8,
                    name:
                    [116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xf6u16,
                    subcode: 1u8,
                    size: 8i8,
                    name:
                    [116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xf6u16,
                    subcode: 2u8,
                    size: 8i8,
                    name: [110, 111, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: NONE,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xf6u16,
                    subcode: 3u8,
                    size: 8i8,
                    name: [110, 101, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: NONE,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xf6u16,
                    subcode: 4u8,
                    size: 8i8,
                    name: [109, 117, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xf6u16,
                    subcode: 5u8,
                    size: 8i8,
                    name:
                    [105, 109, 117, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xf6u16,
                    subcode: 6u8,
                    size: 8i8,
                    name: [100, 105, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xf6u16,
                    subcode: 7u8,
                    size: 8i8,
                    name:
                    [105, 100, 105, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xf7u16,
                    subcode: 0u8,
                    size: -1i8,
                    name:
                    [116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xf7u16,
                    subcode: 1u8,
                    size: -1i8,
                    name:
                    [116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: IMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xf7u16,
                    subcode: 2u8,
                    size: -1i8,
                    name: [110, 111, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: NONE,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xf7u16,
                    subcode: 3u8,
                    size: -1i8,
                    name: [110, 101, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: NONE,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xf7u16,
                    subcode: 4u8,
                    size: -1i8,
                    name: [109, 117, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xf7u16,
                    subcode: 5u8,
                    size: -1i8,
                    name:
                    [105, 109, 117, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xf7u16,
                    subcode: 6u8,
                    size: -1i8,
                    name: [100, 105, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xf7u16,
                    subcode: 7u8,
                    size: -1i8,
                    name:
                    [105, 100, 105, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xfeu16,
                    subcode: 0u8,
                    size: 8i8,
                    name: [105, 110, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: NONE,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xfeu16,
                    subcode: 1u8,
                    size: 8i8,
                    name: [100, 101, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: NONE,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xffu16,
                    subcode: 0u8,
                    size: -1i8,
                    name: [105, 110, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: NONE,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xffu16,
                    subcode: 1u8,
                    size: -1i8,
                    name: [100, 101, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: NONE,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xffu16,
                    subcode: 2u8,
                    size: -1i8,
                    name: [99, 97, 108, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: NONE,
                    flags: 0x8u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xffu16,
                    subcode: 3u8,
                    size: -1i8,
                    name: [99, 97, 108, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: (0x8i32 | 0x400i32) as dword,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xffu16,
                    subcode: 4u8,
                    size: -1i8,
                    name: [106, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: NONE,
                    flags:
                    (0x8i32 | 0x4000i32) as dword,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xffu16,
                    subcode: 5u8,
                    size: -1i8,
                    name: [106, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MEM,
                    arg1: NONE,
                    flags:
                    (0x8i32 | 0x4000i32 |
                        0x400i32) as dword,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xffu16,
                    subcode: 6u8,
                    size: -1i8,
                    name:
                    [112, 117, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: NONE,
                    flags: 0x100u32,
                };
            init
        }];
/* a subcode value of 8 means all subcodes,
 * or the subcode marks the register if there is one present. */
static mut instructions_0F: [Operation; 133] =
    [{
        let mut init =
            Operation {
                opcode: 0u16,
                subcode: 0u8,
                size: -1i8,
                name:
                [115, 108, 100, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: RM,
                arg1: NONE,
                flags: 0x40u32,
            };
        init
    },
        {
            let mut init =
                Operation {
                    opcode: 0u16,
                    subcode: 1u8,
                    size: -1i8,
                    name: [115, 116, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: NONE,
                    flags: 0x40u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0u16,
                    subcode: 2u8,
                    size: 16i8,
                    name:
                    [108, 108, 100, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0u16,
                    subcode: 3u8,
                    size: 16i8,
                    name: [108, 116, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0u16,
                    subcode: 4u8,
                    size: 16i8,
                    name:
                    [118, 101, 114, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0u16,
                    subcode: 5u8,
                    size: 16i8,
                    name:
                    [118, 101, 114, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x1u16,
                    subcode: 0u8,
                    size: 0i8,
                    name:
                    [115, 103, 100, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x1u16,
                    subcode: 1u8,
                    size: 0i8,
                    name:
                    [115, 105, 100, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x1u16,
                    subcode: 2u8,
                    size: 0i8,
                    name:
                    [108, 103, 100, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x1u16,
                    subcode: 3u8,
                    size: 0i8,
                    name:
                    [108, 105, 100, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x1u16,
                    subcode: 4u8,
                    size: -1i8,
                    name:
                    [115, 109, 115, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: NONE,
                    flags: 0x40u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x1u16,
                    subcode: 6u8,
                    size: 16i8,
                    name:
                    [108, 109, 115, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x1u16,
                    subcode: 7u8,
                    size: 0i8,
                    name:
                    [105, 110, 118, 108, 112, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x2u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [108, 97, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0x40u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x3u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [108, 115, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0x40u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x5u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [115, 121, 115, 99, 97, 108, 108, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x6u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [99, 108, 116, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x7u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [115, 121, 115, 114, 101, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x8u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [105, 110, 118, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x9u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [119, 98, 105, 110, 118, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdu16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [112, 114, 101, 102, 101, 116, 99, 104, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: RM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x18u16,
                    subcode: 0u8,
                    size: 8i8,
                    name:
                    [112, 114, 101, 102, 101, 116, 99, 104, 110, 116, 97, 0,
                        0, 0, 0, 0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x18u16,
                    subcode: 1u8,
                    size: 8i8,
                    name:
                    [112, 114, 101, 102, 101, 116, 99, 104, 116, 48, 0, 0, 0,
                        0, 0, 0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x18u16,
                    subcode: 2u8,
                    size: 8i8,
                    name:
                    [112, 114, 101, 102, 101, 116, 99, 104, 116, 49, 0, 0, 0,
                        0, 0, 0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x18u16,
                    subcode: 3u8,
                    size: 8i8,
                    name:
                    [112, 114, 101, 102, 101, 116, 99, 104, 116, 50, 0, 0, 0,
                        0, 0, 0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x1fu16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [110, 111, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x20u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG32,
                    arg1: CR32,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x21u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG32,
                    arg1: DR32,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x22u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: CR32,
                    arg1: REG32,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x23u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: DR32,
                    arg1: REG32,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x24u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG32,
                    arg1: TR32,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x26u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: TR32,
                    arg1: REG32,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x30u16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [119, 114, 109, 115, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x31u16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [114, 100, 116, 115, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x32u16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [114, 100, 109, 115, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x33u16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [114, 100, 112, 109, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x34u16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [115, 121, 115, 101, 110, 116, 101, 114, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x35u16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [115, 121, 115, 101, 120, 105, 116, 0, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x40u16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [99, 109, 111, 118, 111, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x41u16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [99, 109, 111, 118, 110, 111, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x42u16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [99, 109, 111, 118, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x43u16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [99, 109, 111, 118, 97, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x44u16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [99, 109, 111, 118, 122, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x45u16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [99, 109, 111, 118, 110, 122, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x46u16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [99, 109, 111, 118, 98, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x47u16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [99, 109, 111, 118, 97, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x48u16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [99, 109, 111, 118, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x49u16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [99, 109, 111, 118, 110, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x4au16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [99, 109, 111, 118, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x4bu16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [99, 109, 111, 118, 110, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x4cu16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [99, 109, 111, 118, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x4du16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [99, 109, 111, 118, 103, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x4eu16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [99, 109, 111, 118, 108, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x4fu16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [99, 109, 111, 118, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x80u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [106, 111, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL16,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x81u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [106, 110, 111, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL16,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x82u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [106, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL16,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x83u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [106, 97, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL16,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x84u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [106, 122, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL16,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x85u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [106, 110, 122, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL16,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x86u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [106, 98, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL16,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x87u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [106, 97, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL16,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x88u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [106, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL16,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x89u16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [106, 110, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL16,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x8au16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [106, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL16,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x8bu16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [106, 110, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL16,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x8cu16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [106, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL16,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x8du16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [106, 103, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL16,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x8eu16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [106, 108, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL16,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x8fu16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [106, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REL16,
                    arg1: NONE,
                    flags: 0x8000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x90u16,
                    subcode: 0u8,
                    size: 8i8,
                    name:
                    [115, 101, 116, 111, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x91u16,
                    subcode: 0u8,
                    size: 8i8,
                    name:
                    [115, 101, 116, 110, 111, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: RM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x92u16,
                    subcode: 0u8,
                    size: 8i8,
                    name: [115, 101, 116, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x93u16,
                    subcode: 0u8,
                    size: 8i8,
                    name:
                    [115, 101, 116, 97, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x94u16,
                    subcode: 0u8,
                    size: 8i8,
                    name:
                    [115, 101, 116, 122, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x95u16,
                    subcode: 0u8,
                    size: 8i8,
                    name:
                    [115, 101, 116, 110, 122, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: RM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x96u16,
                    subcode: 0u8,
                    size: 8i8,
                    name:
                    [115, 101, 116, 98, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x97u16,
                    subcode: 0u8,
                    size: 8i8,
                    name: [115, 101, 116, 97, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x98u16,
                    subcode: 0u8,
                    size: 8i8,
                    name:
                    [115, 101, 116, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x99u16,
                    subcode: 0u8,
                    size: 8i8,
                    name:
                    [115, 101, 116, 110, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: RM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x9au16,
                    subcode: 0u8,
                    size: 8i8,
                    name:
                    [115, 101, 116, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x9bu16,
                    subcode: 0u8,
                    size: 8i8,
                    name:
                    [115, 101, 116, 110, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: RM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x9cu16,
                    subcode: 0u8,
                    size: 8i8,
                    name:
                    [115, 101, 116, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x9du16,
                    subcode: 0u8,
                    size: 8i8,
                    name:
                    [115, 101, 116, 103, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: RM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x9eu16,
                    subcode: 0u8,
                    size: 8i8,
                    name:
                    [115, 101, 116, 108, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: RM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x9fu16,
                    subcode: 0u8,
                    size: 8i8,
                    name:
                    [115, 101, 116, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xa0u16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [112, 117, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: FS,
                    arg1: NONE,
                    flags: 0x100u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xa1u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [112, 111, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: FS,
                    arg1: NONE,
                    flags: 0x100u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xa2u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [99, 112, 117, 105, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xa3u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [98, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: REG,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xa4u16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [115, 104, 108, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: REG,
                    flags: 0x2u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xa5u16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [115, 104, 108, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: REG,
                    flags: 0x4u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xa8u16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [112, 117, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: GS,
                    arg1: NONE,
                    flags: 0x100u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xa9u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [112, 111, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: GS,
                    arg1: NONE,
                    flags: 0x100u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xabu16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [98, 116, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: REG,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xacu16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [115, 104, 114, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: REG,
                    flags: 0x2u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xadu16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [115, 104, 114, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: REG,
                    flags: 0x4u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xaeu16,
                    subcode: 0u8,
                    size: 0i8,
                    name:
                    [102, 120, 115, 97, 118, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xaeu16,
                    subcode: 1u8,
                    size: 0i8,
                    name:
                    [102, 120, 114, 115, 116, 111, 114, 0, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xaeu16,
                    subcode: 2u8,
                    size: 0i8,
                    name:
                    [108, 100, 109, 120, 99, 115, 114, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xaeu16,
                    subcode: 3u8,
                    size: 0i8,
                    name:
                    [115, 116, 109, 120, 99, 115, 114, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xaeu16,
                    subcode: 4u8,
                    size: 0i8,
                    name:
                    [120, 115, 97, 118, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xaeu16,
                    subcode: 5u8,
                    size: 0i8,
                    name:
                    [120, 114, 115, 116, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xaeu16,
                    subcode: 7u8,
                    size: 0i8,
                    name:
                    [99, 108, 102, 108, 117, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xafu16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [105, 109, 117, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xb0u16,
                    subcode: 8u8,
                    size: 8i8,
                    name:
                    [99, 109, 112, 120, 99, 104, 103, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: RM,
                    arg1: REG,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xb1u16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [99, 109, 112, 120, 99, 104, 103, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: RM,
                    arg1: REG,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xb2u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [108, 115, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: MEM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xb3u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [98, 116, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: REG,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xb4u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [108, 102, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: MEM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xb5u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [108, 103, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: MEM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xb6u16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [109, 111, 118, 122, 120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xb7u16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [109, 111, 118, 122, 120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xbau16,
                    subcode: 4u8,
                    size: -1i8,
                    name: [98, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: IMM8,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xbau16,
                    subcode: 5u8,
                    size: -1i8,
                    name: [98, 116, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: IMM8,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xbau16,
                    subcode: 6u8,
                    size: -1i8,
                    name: [98, 116, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: IMM8,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xbau16,
                    subcode: 7u8,
                    size: -1i8,
                    name: [98, 116, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: IMM8,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xbbu16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [98, 116, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: REG,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xbcu16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [98, 115, 102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xbdu16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [98, 115, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xbeu16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [109, 111, 118, 115, 120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xbfu16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [109, 111, 118, 115, 120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xc0u16,
                    subcode: 8u8,
                    size: 8i8,
                    name: [120, 97, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: REG,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xc1u16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [120, 97, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: REG,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xc7u16,
                    subcode: 1u8,
                    size: 0i8,
                    name:
                    [99, 109, 112, 120, 99, 104, 103, 56, 98, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0x80u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xc8u16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [98, 115, 119, 97, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: AX,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xc9u16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [98, 115, 119, 97, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: CX,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xcau16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [98, 115, 119, 97, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: DX,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xcbu16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [98, 115, 119, 97, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: BX,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xccu16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [98, 115, 119, 97, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: SP,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xcdu16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [98, 115, 119, 97, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: BP,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xceu16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [98, 115, 119, 97, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: SI,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xcfu16,
                    subcode: 8u8,
                    size: -1i8,
                    name:
                    [98, 115, 119, 97, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: DI,
                    arg1: NONE,
                    flags: 0,
                };
            init
        }];
/* mod < 3 (instructions with memory args) */
static mut instructions_fpu_m: [Operation; 64] =
    [{
        let mut init =
            Operation {
                opcode: 0xd8u16,
                subcode: 0u8,
                size: 32i8,
                name: [102, 97, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: MEM,
                arg1: NONE,
                flags: 0x1000u32,
            };
        init
    },
        {
            let mut init =
                Operation {
                    opcode: 0xd8u16,
                    subcode: 1u8,
                    size: 32i8,
                    name:
                    [102, 109, 117, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0x1000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd8u16,
                    subcode: 2u8,
                    size: 32i8,
                    name: [102, 99, 111, 109, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0x1000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd8u16,
                    subcode: 3u8,
                    size: 32i8,
                    name:
                    [102, 99, 111, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0x1000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd8u16,
                    subcode: 4u8,
                    size: 32i8,
                    name: [102, 115, 117, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0x1000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd8u16,
                    subcode: 5u8,
                    size: 32i8,
                    name:
                    [102, 115, 117, 98, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0x1000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd8u16,
                    subcode: 6u8,
                    size: 32i8,
                    name:
                    [102, 100, 105, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0x1000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd8u16,
                    subcode: 7u8,
                    size: 32i8,
                    name:
                    [102, 100, 105, 118, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0x1000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd9u16,
                    subcode: 0u8,
                    size: 32i8,
                    name: [102, 108, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0x1000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd9u16,
                    subcode: 1u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd9u16,
                    subcode: 2u8,
                    size: 32i8,
                    name: [102, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0x1000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd9u16,
                    subcode: 3u8,
                    size: 32i8,
                    name:
                    [102, 115, 116, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0x1000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd9u16,
                    subcode: 4u8,
                    size: 0i8,
                    name:
                    [102, 108, 100, 101, 110, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd9u16,
                    subcode: 5u8,
                    size: 0i8,
                    name:
                    [102, 108, 100, 99, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd9u16,
                    subcode: 6u8,
                    size: 0i8,
                    name:
                    [102, 110, 115, 116, 101, 110, 118, 0, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd9u16,
                    subcode: 7u8,
                    size: 0i8,
                    name:
                    [102, 110, 115, 116, 99, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdau16,
                    subcode: 0u8,
                    size: 32i8,
                    name:
                    [102, 105, 97, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0x2000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdau16,
                    subcode: 1u8,
                    size: 32i8,
                    name:
                    [102, 105, 109, 117, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0x2000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdau16,
                    subcode: 2u8,
                    size: 32i8,
                    name:
                    [102, 105, 99, 111, 109, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0x2000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdau16,
                    subcode: 3u8,
                    size: 32i8,
                    name:
                    [102, 105, 99, 111, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0x2000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdau16,
                    subcode: 4u8,
                    size: 32i8,
                    name:
                    [102, 105, 115, 117, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0x2000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdau16,
                    subcode: 5u8,
                    size: 32i8,
                    name:
                    [102, 105, 115, 117, 98, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0x2000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdau16,
                    subcode: 6u8,
                    size: 32i8,
                    name:
                    [102, 105, 100, 105, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0x2000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdau16,
                    subcode: 7u8,
                    size: 32i8,
                    name:
                    [102, 105, 100, 105, 118, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0x2000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdbu16,
                    subcode: 0u8,
                    size: 32i8,
                    name:
                    [102, 105, 108, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0x2000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdbu16,
                    subcode: 1u8,
                    size: 32i8,
                    name:
                    [102, 105, 115, 116, 116, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0x2000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdbu16,
                    subcode: 2u8,
                    size: 32i8,
                    name:
                    [102, 105, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0x2000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdbu16,
                    subcode: 3u8,
                    size: 32i8,
                    name:
                    [102, 105, 115, 116, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0x2000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdbu16,
                    subcode: 4u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdbu16,
                    subcode: 5u8,
                    size: 80i8,
                    name: [102, 108, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdbu16,
                    subcode: 6u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdbu16,
                    subcode: 7u8,
                    size: 80i8,
                    name:
                    [102, 115, 116, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdcu16,
                    subcode: 0u8,
                    size: 64i8,
                    name: [102, 97, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0x2000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdcu16,
                    subcode: 1u8,
                    size: 64i8,
                    name:
                    [102, 109, 117, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0x2000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdcu16,
                    subcode: 2u8,
                    size: 64i8,
                    name: [102, 99, 111, 109, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0x2000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdcu16,
                    subcode: 3u8,
                    size: 64i8,
                    name:
                    [102, 99, 111, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0x2000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdcu16,
                    subcode: 4u8,
                    size: 64i8,
                    name: [102, 115, 117, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0x2000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdcu16,
                    subcode: 5u8,
                    size: 64i8,
                    name:
                    [102, 115, 117, 98, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0x2000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdcu16,
                    subcode: 6u8,
                    size: 64i8,
                    name:
                    [102, 100, 105, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0x2000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdcu16,
                    subcode: 7u8,
                    size: 64i8,
                    name:
                    [102, 100, 105, 118, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0x2000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xddu16,
                    subcode: 0u8,
                    size: 64i8,
                    name: [102, 108, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0x2000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xddu16,
                    subcode: 1u8,
                    size: 64i8,
                    name:
                    [102, 105, 115, 116, 116, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0x3000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xddu16,
                    subcode: 2u8,
                    size: 64i8,
                    name: [102, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0x2000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xddu16,
                    subcode: 3u8,
                    size: 64i8,
                    name:
                    [102, 115, 116, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0x2000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xddu16,
                    subcode: 4u8,
                    size: 0i8,
                    name:
                    [102, 114, 115, 116, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xddu16,
                    subcode: 5u8,
                    size: 0,
                    name: [0; 16],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xddu16,
                    subcode: 6u8,
                    size: 0i8,
                    name:
                    [102, 110, 115, 97, 118, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xddu16,
                    subcode: 7u8,
                    size: 0i8,
                    name:
                    [102, 110, 115, 116, 115, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdeu16,
                    subcode: 0u8,
                    size: 16i8,
                    name:
                    [102, 105, 97, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0x1000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdeu16,
                    subcode: 1u8,
                    size: 16i8,
                    name:
                    [102, 105, 109, 117, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0x1000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdeu16,
                    subcode: 2u8,
                    size: 16i8,
                    name:
                    [102, 105, 99, 111, 109, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0x1000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdeu16,
                    subcode: 3u8,
                    size: 16i8,
                    name:
                    [102, 105, 99, 111, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0x1000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdeu16,
                    subcode: 4u8,
                    size: 16i8,
                    name:
                    [102, 105, 115, 117, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0x1000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdeu16,
                    subcode: 5u8,
                    size: 16i8,
                    name:
                    [102, 105, 115, 117, 98, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0x1000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdeu16,
                    subcode: 6u8,
                    size: 16i8,
                    name:
                    [102, 105, 100, 105, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0x1000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdeu16,
                    subcode: 7u8,
                    size: 16i8,
                    name:
                    [102, 105, 100, 105, 118, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0x1000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdfu16,
                    subcode: 0u8,
                    size: 16i8,
                    name:
                    [102, 105, 108, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0x1000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdfu16,
                    subcode: 1u8,
                    size: 16i8,
                    name:
                    [102, 105, 115, 116, 116, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0x1000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdfu16,
                    subcode: 2u8,
                    size: 16i8,
                    name:
                    [102, 105, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0x1000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdfu16,
                    subcode: 3u8,
                    size: 16i8,
                    name:
                    [102, 105, 115, 116, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0x1000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdfu16,
                    subcode: 4u8,
                    size: 0i8,
                    name: [102, 98, 108, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdfu16,
                    subcode: 5u8,
                    size: 64i8,
                    name:
                    [102, 105, 108, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0x3000u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdfu16,
                    subcode: 6u8,
                    size: 0i8,
                    name:
                    [102, 98, 115, 116, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdfu16,
                    subcode: 7u8,
                    size: 64i8,
                    name:
                    [102, 105, 115, 116, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MEM,
                    arg1: NONE,
                    flags: 0x3000u32,
                };
            init
        }];
static mut instructions_fpu_r: [Operation; 64] =
    [{
        let mut init =
            Operation {
                opcode: 0xd8u16,
                subcode: 0u8,
                size: 0i8,
                name: [102, 97, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: ST,
                arg1: STX,
                flags: 0,
            };
        init
    },
        {
            let mut init =
                Operation {
                    opcode: 0xd8u16,
                    subcode: 1u8,
                    size: 0i8,
                    name:
                    [102, 109, 117, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: ST,
                    arg1: STX,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd8u16,
                    subcode: 2u8,
                    size: 0i8,
                    name: [102, 99, 111, 109, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: STX,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd8u16,
                    subcode: 3u8,
                    size: 0i8,
                    name:
                    [102, 99, 111, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: STX,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd8u16,
                    subcode: 4u8,
                    size: 0i8,
                    name: [102, 115, 117, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: ST,
                    arg1: STX,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd8u16,
                    subcode: 5u8,
                    size: 0i8,
                    name:
                    [102, 115, 117, 98, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: ST,
                    arg1: STX,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd8u16,
                    subcode: 6u8,
                    size: 0i8,
                    name:
                    [102, 100, 105, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: ST,
                    arg1: STX,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd8u16,
                    subcode: 7u8,
                    size: 0i8,
                    name:
                    [102, 100, 105, 118, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: ST,
                    arg1: STX,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd9u16,
                    subcode: 0u8,
                    size: 0i8,
                    name: [102, 108, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: STX,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd9u16,
                    subcode: 1u8,
                    size: 0i8,
                    name: [102, 120, 99, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: STX,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd9u16,
                    subcode: 2u8,
                    size: 0i8,
                    name:
                    [0i8, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd9u16,
                    subcode: 3u8,
                    size: 0i8,
                    name:
                    [102, 115, 116, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: STX,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd9u16,
                    subcode: 4u8,
                    size: 0i8,
                    name:
                    [0i8, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd9u16,
                    subcode: 5u8,
                    size: 0i8,
                    name:
                    [0i8, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd9u16,
                    subcode: 6u8,
                    size: 0i8,
                    name:
                    [0i8, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd9u16,
                    subcode: 7u8,
                    size: 0i8,
                    name:
                    [0i8, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdau16,
                    subcode: 0u8,
                    size: 0i8,
                    name:
                    [102, 99, 109, 111, 118, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: ST,
                    arg1: STX,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdau16,
                    subcode: 1u8,
                    size: 0i8,
                    name:
                    [102, 99, 109, 111, 118, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: ST,
                    arg1: STX,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdau16,
                    subcode: 2u8,
                    size: 0i8,
                    name:
                    [102, 99, 109, 111, 118, 98, 101, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: ST,
                    arg1: STX,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdau16,
                    subcode: 3u8,
                    size: 0i8,
                    name:
                    [102, 99, 109, 111, 118, 117, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: ST,
                    arg1: STX,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdau16,
                    subcode: 4u8,
                    size: 0i8,
                    name:
                    [0i8, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdau16,
                    subcode: 5u8,
                    size: 0i8,
                    name:
                    [0i8, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdau16,
                    subcode: 6u8,
                    size: 0i8,
                    name:
                    [0i8, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdau16,
                    subcode: 7u8,
                    size: 0i8,
                    name:
                    [0i8, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdbu16,
                    subcode: 0u8,
                    size: 0i8,
                    name:
                    [102, 99, 109, 111, 118, 110, 98, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: ST,
                    arg1: STX,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdbu16,
                    subcode: 1u8,
                    size: 0i8,
                    name:
                    [102, 99, 109, 111, 118, 110, 101, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: ST,
                    arg1: STX,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdbu16,
                    subcode: 2u8,
                    size: 0i8,
                    name:
                    [102, 99, 109, 111, 118, 110, 98, 101, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: ST,
                    arg1: STX,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdbu16,
                    subcode: 3u8,
                    size: 0i8,
                    name:
                    [102, 99, 109, 111, 118, 110, 117, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: ST,
                    arg1: STX,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdbu16,
                    subcode: 4u8,
                    size: 0i8,
                    name:
                    [0i8, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdbu16,
                    subcode: 5u8,
                    size: 0i8,
                    name:
                    [102, 117, 99, 111, 109, 105, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: ST,
                    arg1: STX,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdbu16,
                    subcode: 6u8,
                    size: 0i8,
                    name:
                    [102, 99, 111, 109, 105, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: ST,
                    arg1: STX,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdbu16,
                    subcode: 7u8,
                    size: 0i8,
                    name:
                    [0i8, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdcu16,
                    subcode: 0u8,
                    size: 0i8,
                    name: [102, 97, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: STX,
                    arg1: ST,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdcu16,
                    subcode: 1u8,
                    size: 0i8,
                    name:
                    [102, 109, 117, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: STX,
                    arg1: ST,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdcu16,
                    subcode: 2u8,
                    size: 0i8,
                    name: [102, 99, 111, 109, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: STX,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdcu16,
                    subcode: 3u8,
                    size: 0i8,
                    name:
                    [102, 99, 111, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: STX,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdcu16,
                    subcode: 4u8,
                    size: 0i8,
                    name:
                    [102, 115, 117, 98, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: STX,
                    arg1: ST,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdcu16,
                    subcode: 5u8,
                    size: 0i8,
                    name: [102, 115, 117, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: STX,
                    arg1: ST,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdcu16,
                    subcode: 6u8,
                    size: 0i8,
                    name:
                    [102, 100, 105, 118, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: STX,
                    arg1: ST,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdcu16,
                    subcode: 7u8,
                    size: 0i8,
                    name:
                    [102, 100, 105, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: STX,
                    arg1: ST,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xddu16,
                    subcode: 0u8,
                    size: 0i8,
                    name:
                    [102, 102, 114, 101, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: STX,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xddu16,
                    subcode: 1u8,
                    size: 0i8,
                    name: [102, 120, 99, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: STX,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xddu16,
                    subcode: 2u8,
                    size: 0i8,
                    name: [102, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: STX,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xddu16,
                    subcode: 3u8,
                    size: 0i8,
                    name:
                    [102, 115, 116, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: STX,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xddu16,
                    subcode: 4u8,
                    size: 0i8,
                    name:
                    [102, 117, 99, 111, 109, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: STX,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xddu16,
                    subcode: 5u8,
                    size: 0i8,
                    name:
                    [102, 117, 99, 111, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: STX,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xddu16,
                    subcode: 6u8,
                    size: 0i8,
                    name:
                    [0i8, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xddu16,
                    subcode: 7u8,
                    size: 0i8,
                    name:
                    [0i8, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdeu16,
                    subcode: 0u8,
                    size: 0i8,
                    name:
                    [102, 97, 100, 100, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: STX,
                    arg1: ST,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdeu16,
                    subcode: 1u8,
                    size: 0i8,
                    name:
                    [102, 109, 117, 108, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: STX,
                    arg1: ST,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdeu16,
                    subcode: 2u8,
                    size: 0i8,
                    name:
                    [102, 99, 111, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: STX,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdeu16,
                    subcode: 3u8,
                    size: 0i8,
                    name:
                    [0i8, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdeu16,
                    subcode: 4u8,
                    size: 0i8,
                    name:
                    [102, 115, 117, 98, 114, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: STX,
                    arg1: ST,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdeu16,
                    subcode: 5u8,
                    size: 0i8,
                    name:
                    [102, 115, 117, 98, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: STX,
                    arg1: ST,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdeu16,
                    subcode: 6u8,
                    size: 0i8,
                    name:
                    [102, 100, 105, 118, 114, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: STX,
                    arg1: ST,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdeu16,
                    subcode: 7u8,
                    size: 0i8,
                    name:
                    [102, 100, 105, 118, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: STX,
                    arg1: ST,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdfu16,
                    subcode: 0u8,
                    size: 0i8,
                    name:
                    [102, 102, 114, 101, 101, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: STX,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdfu16,
                    subcode: 1u8,
                    size: 0i8,
                    name: [102, 120, 99, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: STX,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdfu16,
                    subcode: 2u8,
                    size: 0i8,
                    name:
                    [102, 115, 116, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: STX,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdfu16,
                    subcode: 3u8,
                    size: 0i8,
                    name:
                    [102, 115, 116, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: STX,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdfu16,
                    subcode: 4u8,
                    size: 0i8,
                    name:
                    [0i8, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdfu16,
                    subcode: 5u8,
                    size: 0i8,
                    name:
                    [102, 117, 99, 111, 109, 105, 112, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: ST,
                    arg1: STX,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdfu16,
                    subcode: 6u8,
                    size: 0i8,
                    name:
                    [102, 99, 111, 109, 105, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: ST,
                    arg1: STX,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdfu16,
                    subcode: 7u8,
                    size: 0i8,
                    name:
                    [0i8, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        }];
static mut instructions_fpu_single: [Operation; 36] =
    [{
        let mut init =
            Operation {
                opcode: 0xd9u16,
                subcode: 0xd0u8,
                size: 0i8,
                name:
                [102, 110, 111, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                arg0: NONE,
                arg1: NONE,
                flags: 0,
            };
        init
    },
        {
            let mut init =
                Operation {
                    opcode: 0xd9u16,
                    subcode: 0xe0u8,
                    size: 0i8,
                    name: [102, 99, 104, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd9u16,
                    subcode: 0xe1u8,
                    size: 0i8,
                    name: [102, 97, 98, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd9u16,
                    subcode: 0xe4u8,
                    size: 0i8,
                    name:
                    [102, 116, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd9u16,
                    subcode: 0xe5u8,
                    size: 0i8,
                    name: [102, 120, 97, 109, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd9u16,
                    subcode: 0xe8u8,
                    size: 0i8,
                    name: [102, 108, 100, 49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd9u16,
                    subcode: 0xe9u8,
                    size: 0i8,
                    name:
                    [102, 108, 100, 108, 50, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd9u16,
                    subcode: 0xeau8,
                    size: 0i8,
                    name:
                    [102, 108, 100, 108, 50, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd9u16,
                    subcode: 0xebu8,
                    size: 0i8,
                    name:
                    [102, 108, 100, 112, 105, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd9u16,
                    subcode: 0xecu8,
                    size: 0i8,
                    name:
                    [102, 108, 100, 108, 103, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd9u16,
                    subcode: 0xedu8,
                    size: 0i8,
                    name:
                    [102, 108, 100, 108, 110, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd9u16,
                    subcode: 0xeeu8,
                    size: 0i8,
                    name:
                    [102, 108, 100, 122, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd9u16,
                    subcode: 0xf0u8,
                    size: 0i8,
                    name:
                    [102, 50, 120, 109, 49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd9u16,
                    subcode: 0xf1u8,
                    size: 0i8,
                    name:
                    [102, 121, 108, 50, 120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd9u16,
                    subcode: 0xf2u8,
                    size: 0i8,
                    name:
                    [102, 112, 116, 97, 110, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd9u16,
                    subcode: 0xf3u8,
                    size: 0i8,
                    name:
                    [102, 112, 97, 116, 97, 110, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd9u16,
                    subcode: 0xf4u8,
                    size: 0i8,
                    name:
                    [102, 120, 116, 114, 97, 99, 116, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd9u16,
                    subcode: 0xf5u8,
                    size: 0i8,
                    name:
                    [102, 112, 114, 101, 109, 49, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd9u16,
                    subcode: 0xf6u8,
                    size: 0i8,
                    name:
                    [102, 100, 101, 99, 115, 116, 112, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd9u16,
                    subcode: 0xf7u8,
                    size: 0i8,
                    name:
                    [102, 105, 110, 99, 115, 116, 112, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd9u16,
                    subcode: 0xf8u8,
                    size: 0i8,
                    name:
                    [102, 112, 114, 101, 109, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd9u16,
                    subcode: 0xf9u8,
                    size: 0i8,
                    name:
                    [102, 121, 108, 50, 120, 112, 49, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd9u16,
                    subcode: 0xfau8,
                    size: 0i8,
                    name:
                    [102, 115, 113, 114, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd9u16,
                    subcode: 0xfbu8,
                    size: 0i8,
                    name:
                    [102, 115, 105, 110, 99, 111, 115, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd9u16,
                    subcode: 0xfcu8,
                    size: 0i8,
                    name:
                    [102, 114, 110, 100, 105, 110, 116, 0, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd9u16,
                    subcode: 0xfdu8,
                    size: 0i8,
                    name:
                    [102, 115, 99, 97, 108, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd9u16,
                    subcode: 0xfeu8,
                    size: 0i8,
                    name:
                    [102, 115, 105, 110, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd9u16,
                    subcode: 0xffu8,
                    size: 0i8,
                    name: [102, 99, 111, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdau16,
                    subcode: 0xe9u8,
                    size: 0i8,
                    name:
                    [102, 117, 99, 111, 109, 112, 112, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdbu16,
                    subcode: 0xe0u8,
                    size: 0i8,
                    name:
                    [102, 110, 101, 110, 105, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdbu16,
                    subcode: 0xe1u8,
                    size: 0i8,
                    name:
                    [102, 110, 100, 105, 115, 105, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdbu16,
                    subcode: 0xe2u8,
                    size: 0i8,
                    name:
                    [102, 110, 99, 108, 101, 120, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdbu16,
                    subcode: 0xe3u8,
                    size: 0i8,
                    name:
                    [102, 110, 105, 110, 105, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdbu16,
                    subcode: 0xe4u8,
                    size: 0i8,
                    name:
                    [102, 110, 115, 101, 116, 112, 109, 0, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdeu16,
                    subcode: 0xd9u8,
                    size: 0i8,
                    name:
                    [102, 99, 111, 109, 112, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdfu16,
                    subcode: 0xe0u8,
                    size: 0i8,
                    name:
                    [102, 110, 115, 116, 115, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: AX,
                    arg1: NONE,
                    flags: 0,
                };
            init
        }];

unsafe extern "C" fn get_fpu_instr(mut p: *const byte, mut op: *mut Operation)
                                   -> libc::c_int {
    let mut subcode =
        (*p.offset(1isize) as libc::c_int >>
            3i32 & 7i32) as byte;
    let mut index =

        ((*p.offset(0isize) as libc::c_int &
            7i32) * 8i32 + subcode as libc::c_int)
            as byte;
    let mut i = 0;
    return if (*p.offset(1isize) as libc::c_int >>
        6i32) < 3i32 {
        if instructions_fpu_m[index as usize].name[0usize]
            != 0 {
            *op = instructions_fpu_m[index as usize]
        }
        0i32
    } else {
        if instructions_fpu_r[index as usize].name[0usize]
            != 0 {
            *op = instructions_fpu_r[index as usize];
            return 0i32;
        } else {
            /* try the single op list */
            i = 0u32;
            while (i as libc::c_ulong) <
                (::std::mem::size_of::<[Operation; 36]>() as
                    libc::c_ulong).wrapping_div(::std::mem::size_of::<Operation>()
                    as libc::c_ulong) {
                if *p.offset(0isize) as libc::c_int ==
                    instructions_fpu_single[i as usize].opcode as
                        libc::c_int &&
                    *p.offset(1isize) as libc::c_int ==
                        instructions_fpu_single[i as usize].subcode as
                            libc::c_int {
                    *op = instructions_fpu_single[i as usize];
                    break;
                } else { i = i.wrapping_add(1) }
            }
        }
        1i32
    };
}

static mut instructions_sse: [Operation; 109] =
    [{
        let mut init =
            Operation {
                opcode: 0x10u16,
                subcode: 8u8,
                size: 0i8,
                name:
                [109, 111, 118, 117, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0],
                arg0: XMM,
                arg1: XM,
                flags: 0,
            };
        init
    },
        {
            let mut init =
                Operation {
                    opcode: 0x11u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [109, 111, 118, 117, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XM,
                    arg1: XMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x12u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [109, 111, 118, 108, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x13u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [109, 111, 118, 108, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MEM,
                    arg1: XMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x14u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [117, 110, 112, 99, 107, 108, 112, 115, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x15u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [117, 110, 112, 99, 107, 104, 112, 115, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x16u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [109, 111, 118, 104, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x17u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [109, 111, 118, 104, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MEM,
                    arg1: XMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x28u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [109, 111, 118, 97, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x29u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [109, 111, 118, 97, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XM,
                    arg1: XMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x2au16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [99, 118, 116, 112, 105, 50, 112, 115, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: XMM,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x2bu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [109, 111, 118, 110, 116, 112, 115, 0, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: MEM,
                    arg1: XMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x2cu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [99, 118, 116, 116, 112, 115, 50, 112, 105, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: MMX,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x2du16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [99, 118, 116, 112, 115, 50, 112, 105, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: MMX,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x2eu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [117, 99, 111, 109, 105, 115, 115, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x2fu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [99, 111, 109, 105, 115, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x50u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [109, 111, 118, 109, 115, 107, 112, 115, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: REGONLY,
                    arg1: XMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x51u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [115, 113, 114, 116, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x52u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [114, 115, 113, 114, 116, 112, 115, 0, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x53u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [114, 99, 112, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x54u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [97, 110, 100, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x55u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [97, 110, 100, 110, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x56u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [111, 114, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x57u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [120, 111, 114, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x58u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [97, 100, 100, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x59u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [109, 117, 108, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x5au16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [99, 118, 116, 112, 115, 50, 112, 100, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x5bu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [99, 118, 116, 100, 113, 50, 112, 115, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x5cu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [115, 117, 98, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x5du16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [109, 105, 110, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x5eu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [100, 105, 118, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x5fu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [109, 97, 120, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x60u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 117, 110, 112, 99, 107, 108, 98, 119, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x61u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 117, 110, 112, 99, 107, 108, 119, 100, 0, 0, 0, 0,
                        0, 0, 0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x62u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 117, 110, 112, 99, 107, 108, 100, 113, 0, 0, 0, 0,
                        0, 0, 0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x63u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 97, 99, 107, 115, 115, 119, 98, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x64u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 99, 109, 112, 103, 116, 98, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x65u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 99, 109, 112, 103, 116, 119, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x66u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 99, 109, 112, 103, 116, 100, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x67u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 97, 99, 107, 117, 115, 119, 98, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x68u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 117, 110, 112, 99, 107, 104, 98, 119, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x69u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 117, 110, 112, 99, 107, 104, 119, 100, 0, 0, 0, 0,
                        0, 0, 0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x6au16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 117, 110, 112, 99, 107, 104, 100, 113, 0, 0, 0, 0,
                        0, 0, 0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x6bu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 97, 99, 107, 115, 115, 100, 119, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x6eu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [109, 111, 118, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MMX,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x6fu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [109, 111, 118, 113, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x70u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 115, 104, 117, 102, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0x2u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x71u16,
                    subcode: 2u8,
                    size: 0i8,
                    name:
                    [112, 115, 114, 108, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MMXONLY,
                    arg1: IMM8,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x71u16,
                    subcode: 4u8,
                    size: 0i8,
                    name:
                    [112, 115, 114, 97, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MMXONLY,
                    arg1: IMM8,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x71u16,
                    subcode: 6u8,
                    size: 0i8,
                    name:
                    [112, 115, 108, 108, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MMXONLY,
                    arg1: IMM8,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x72u16,
                    subcode: 2u8,
                    size: 0i8,
                    name:
                    [112, 115, 114, 108, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MMXONLY,
                    arg1: IMM8,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x72u16,
                    subcode: 4u8,
                    size: 0i8,
                    name:
                    [112, 115, 114, 97, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MMXONLY,
                    arg1: IMM8,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x72u16,
                    subcode: 6u8,
                    size: 0i8,
                    name:
                    [112, 115, 108, 108, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MMXONLY,
                    arg1: IMM8,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x73u16,
                    subcode: 2u8,
                    size: 0i8,
                    name:
                    [112, 115, 114, 108, 113, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MMXONLY,
                    arg1: IMM8,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x73u16,
                    subcode: 6u8,
                    size: 0i8,
                    name:
                    [112, 115, 108, 108, 113, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MMXONLY,
                    arg1: IMM8,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x74u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 99, 109, 112, 101, 113, 98, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x75u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 99, 109, 112, 101, 113, 119, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x76u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 99, 109, 112, 101, 113, 100, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x77u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [101, 109, 109, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: NONE,
                    arg1: NONE,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x7eu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [109, 111, 118, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: MMX,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x7fu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [109, 111, 118, 113, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MM,
                    arg1: MMX,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xc2u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [99, 109, 112, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0x2u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xc3u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [109, 111, 118, 110, 116, 105, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MEM,
                    arg1: REG,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xc4u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 105, 110, 115, 114, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MMX,
                    arg1: RM,
                    flags: 0x2u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xc5u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 101, 120, 116, 114, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: REGONLY,
                    arg1: MMX,
                    flags: 0x2u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xc6u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [115, 104, 117, 102, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0x2u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd1u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 115, 114, 108, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd2u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 115, 114, 108, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd3u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 115, 114, 108, 113, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd4u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 97, 100, 100, 113, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd5u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 109, 117, 108, 108, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd7u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 109, 111, 118, 109, 115, 107, 98, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: REGONLY,
                    arg1: MMX,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd8u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 115, 117, 98, 117, 115, 98, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd9u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 115, 117, 98, 117, 115, 119, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdau16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 109, 105, 110, 117, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdbu16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [112, 97, 110, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdcu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 97, 100, 100, 117, 115, 98, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xddu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 97, 100, 100, 117, 115, 119, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdeu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 109, 97, 120, 117, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdfu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 97, 110, 100, 110, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xe0u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 97, 118, 103, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xe1u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 115, 114, 97, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xe2u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 115, 114, 97, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xe3u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 97, 118, 103, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xe4u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 109, 117, 108, 104, 117, 119, 0, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xe5u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 109, 117, 108, 104, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xe7u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [109, 111, 118, 110, 116, 113, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MEM,
                    arg1: MMX,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xe8u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 115, 117, 98, 115, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xe9u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 115, 117, 98, 115, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xeau16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 109, 105, 110, 115, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xebu16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [112, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xecu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 97, 100, 100, 115, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xedu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 97, 100, 100, 115, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xeeu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 109, 97, 120, 115, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xefu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 120, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xf1u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 115, 108, 108, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xf2u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 115, 108, 108, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xf3u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 115, 108, 108, 113, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xf4u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 109, 117, 108, 117, 100, 113, 0, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xf5u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 109, 97, 100, 100, 119, 100, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xf6u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 115, 97, 100, 98, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xf7u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [109, 97, 115, 107, 109, 111, 118, 113, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: MMX,
                    arg1: MMXONLY,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xf8u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 115, 117, 98, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xf9u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 115, 117, 98, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xfau16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 115, 117, 98, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xfbu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 115, 117, 98, 113, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xfcu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 97, 100, 100, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xfdu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 97, 100, 100, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xfeu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 97, 100, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        }];
static mut instructions_sse_op32: [Operation; 114] =
    [{
        let mut init =
            Operation {
                opcode: 0x10u16,
                subcode: 8u8,
                size: 0i8,
                name:
                [109, 111, 118, 117, 112, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0],
                arg0: XMM,
                arg1: XM,
                flags: 0,
            };
        init
    },
        {
            let mut init =
                Operation {
                    opcode: 0x11u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [109, 111, 118, 117, 112, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XM,
                    arg1: XMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x12u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [109, 111, 118, 108, 112, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x13u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [109, 111, 118, 108, 112, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MEM,
                    arg1: XMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x14u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [117, 110, 112, 99, 107, 108, 112, 100, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x15u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [117, 110, 112, 99, 107, 104, 112, 100, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x16u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [109, 111, 118, 104, 112, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x17u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [109, 111, 118, 104, 112, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MEM,
                    arg1: XMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x28u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [109, 111, 118, 97, 112, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x29u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [109, 111, 118, 97, 112, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XM,
                    arg1: XMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x2au16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [99, 118, 116, 112, 105, 50, 112, 100, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: XMM,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x2bu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [109, 111, 118, 110, 116, 112, 100, 0, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: MEM,
                    arg1: XMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x2cu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [99, 118, 116, 116, 112, 100, 50, 112, 105, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: MMX,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x2du16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [99, 118, 116, 112, 100, 50, 112, 105, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: MMX,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x2eu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [117, 99, 111, 109, 105, 115, 100, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x2fu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [99, 111, 109, 105, 115, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x50u16,
                    subcode: 8u8,
                    size: 32i8,
                    name:
                    [109, 111, 118, 109, 115, 107, 112, 100, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: REGONLY,
                    arg1: XMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x51u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [115, 113, 114, 116, 112, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x54u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [97, 110, 100, 112, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x55u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [97, 110, 100, 110, 112, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x56u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [111, 114, 112, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x57u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [120, 111, 114, 112, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x58u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [97, 100, 100, 112, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x59u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [109, 117, 108, 112, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x5au16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [99, 118, 116, 112, 100, 50, 112, 115, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x5bu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [99, 118, 116, 112, 115, 50, 100, 113, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x5cu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [115, 117, 98, 112, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x5du16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [109, 105, 110, 112, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x5eu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [100, 105, 118, 112, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x5fu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [109, 97, 120, 112, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x60u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 117, 110, 112, 99, 107, 108, 98, 119, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x61u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 117, 110, 112, 99, 107, 108, 119, 100, 0, 0, 0, 0,
                        0, 0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x62u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 117, 110, 112, 99, 107, 108, 100, 113, 0, 0, 0, 0,
                        0, 0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x63u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 97, 99, 107, 115, 115, 119, 98, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x64u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 99, 109, 112, 103, 116, 98, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x65u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 99, 109, 112, 103, 116, 119, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x66u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 99, 109, 112, 103, 116, 100, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x67u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 97, 99, 107, 117, 115, 119, 98, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x68u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 117, 110, 112, 99, 107, 104, 98, 119, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x69u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 117, 110, 112, 99, 107, 104, 119, 100, 0, 0, 0, 0,
                        0, 0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x6au16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 117, 110, 112, 99, 107, 104, 100, 113, 0, 0, 0, 0,
                        0, 0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x6bu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 97, 99, 107, 115, 115, 100, 119, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x6cu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 117, 110, 112, 99, 107, 108, 113, 100, 113, 0, 0, 0,
                        0, 0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x6du16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 117, 110, 112, 99, 107, 104, 113, 100, 113, 0, 0, 0,
                        0, 0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x6eu16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: XMM,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x6fu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [109, 111, 118, 100, 113, 97, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x70u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 115, 104, 117, 102, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0x2u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x71u16,
                    subcode: 2u8,
                    size: 0i8,
                    name:
                    [112, 115, 114, 108, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMMONLY,
                    arg1: IMM8,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x71u16,
                    subcode: 4u8,
                    size: 0i8,
                    name:
                    [112, 115, 114, 97, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: XMMONLY,
                    arg1: IMM8,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x71u16,
                    subcode: 6u8,
                    size: 0i8,
                    name:
                    [112, 115, 108, 108, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMMONLY,
                    arg1: IMM8,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x72u16,
                    subcode: 2u8,
                    size: 0i8,
                    name:
                    [112, 115, 114, 108, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMMONLY,
                    arg1: IMM8,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x72u16,
                    subcode: 4u8,
                    size: 0i8,
                    name:
                    [112, 115, 114, 97, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: XMMONLY,
                    arg1: IMM8,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x72u16,
                    subcode: 6u8,
                    size: 0i8,
                    name:
                    [112, 115, 108, 108, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMMONLY,
                    arg1: IMM8,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x73u16,
                    subcode: 2u8,
                    size: 0i8,
                    name:
                    [112, 115, 114, 108, 113, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMMONLY,
                    arg1: IMM8,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x73u16,
                    subcode: 3u8,
                    size: 0i8,
                    name:
                    [112, 115, 114, 108, 100, 113, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMMONLY,
                    arg1: IMM8,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x73u16,
                    subcode: 6u8,
                    size: 0i8,
                    name:
                    [112, 115, 108, 108, 113, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMMONLY,
                    arg1: IMM8,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x73u16,
                    subcode: 7u8,
                    size: 0i8,
                    name:
                    [112, 115, 108, 108, 100, 113, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMMONLY,
                    arg1: IMM8,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x74u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 99, 109, 112, 101, 113, 98, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x75u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 99, 109, 112, 101, 113, 119, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x76u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 99, 109, 112, 101, 113, 100, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x7cu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [104, 97, 100, 100, 112, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x7du16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [104, 115, 117, 98, 112, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x7eu16,
                    subcode: 8u8,
                    size: -1i8,
                    name: [109, 111, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: RM,
                    arg1: XMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x7fu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [109, 111, 118, 100, 113, 97, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XM,
                    arg1: XMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xc2u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [99, 109, 112, 112, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0x2u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xc4u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 105, 110, 115, 114, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: RM,
                    flags: 0x2u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xc5u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 101, 120, 116, 114, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: REGONLY,
                    arg1: XMM,
                    flags: 0x2u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xc6u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [115, 104, 117, 102, 112, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0x2u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd0u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [97, 100, 100, 115, 117, 98, 112, 100, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd1u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 115, 114, 108, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd2u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 115, 114, 108, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd3u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 115, 114, 108, 113, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd4u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 97, 100, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd5u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 109, 117, 108, 108, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd6u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [109, 111, 118, 113, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: XM,
                    arg1: XMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd7u16,
                    subcode: 8u8,
                    size: 32i8,
                    name:
                    [112, 109, 111, 118, 109, 115, 107, 98, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: REGONLY,
                    arg1: XMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd8u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 115, 117, 98, 117, 115, 98, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd9u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 115, 117, 98, 117, 115, 119, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdau16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 109, 105, 110, 117, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdbu16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [112, 97, 110, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdcu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 97, 100, 100, 117, 115, 98, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xddu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 97, 100, 100, 117, 115, 119, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdeu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 109, 97, 120, 117, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xdfu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 97, 110, 100, 110, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xe0u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 97, 118, 103, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xe1u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 115, 114, 97, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xe2u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 115, 114, 97, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xe3u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 97, 118, 103, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xe4u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 109, 117, 108, 104, 117, 119, 0, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xe5u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 109, 117, 108, 104, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xe6u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [99, 118, 116, 116, 112, 100, 50, 100, 113, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xe7u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [109, 111, 118, 110, 116, 100, 113, 0, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: MEM,
                    arg1: XMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xe8u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 115, 117, 98, 115, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xe9u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 115, 117, 98, 115, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xeau16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 109, 105, 110, 115, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xebu16,
                    subcode: 8u8,
                    size: 0i8,
                    name: [112, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xecu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 97, 100, 100, 115, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xedu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 97, 100, 100, 115, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xeeu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 109, 97, 120, 115, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xefu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 120, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xf1u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 115, 108, 108, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xf2u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 115, 108, 108, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xf3u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 115, 108, 108, 113, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xf4u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 109, 117, 108, 117, 100, 113, 0, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xf5u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 109, 97, 100, 100, 119, 100, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xf6u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 115, 97, 100, 98, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xf7u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [109, 97, 115, 107, 109, 111, 118, 100, 113, 117, 0, 0, 0,
                        0, 0, 0],
                    arg0: XMM,
                    arg1: XMMONLY,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xf8u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 115, 117, 98, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xf9u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 115, 117, 98, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xfau16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 115, 117, 98, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xfbu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 115, 117, 98, 113, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xfcu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 97, 100, 100, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xfdu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 97, 100, 100, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xfeu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 97, 100, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        }];
static mut instructions_sse_repne: [Operation; 21] =
    [{
        let mut init =
            Operation {
                opcode: 0x10u16,
                subcode: 8u8,
                size: 0i8,
                name:
                [109, 111, 118, 115, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0],
                arg0: XMM,
                arg1: XM,
                flags: 0,
            };
        init
    },
        {
            let mut init =
                Operation {
                    opcode: 0x11u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [109, 111, 118, 115, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XM,
                    arg1: XMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x12u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [109, 111, 118, 100, 100, 117, 112, 0, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x2au16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [99, 118, 116, 115, 105, 50, 115, 100, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: XMM,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x2cu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [99, 118, 116, 116, 115, 100, 50, 115, 105, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: REG,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x2du16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [99, 118, 116, 115, 100, 50, 115, 105, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: REG,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x51u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [115, 113, 114, 116, 115, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x58u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [97, 100, 100, 115, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x59u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [109, 117, 108, 115, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x5au16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [99, 118, 116, 115, 100, 50, 115, 115, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x5cu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [115, 117, 98, 115, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x5du16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [109, 105, 110, 115, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x5eu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [100, 105, 118, 115, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x5fu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [109, 97, 120, 115, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x70u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 115, 104, 117, 102, 108, 119, 0, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0x2u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x7cu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [104, 97, 100, 100, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x7du16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [104, 115, 117, 98, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xc2u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [99, 109, 112, 115, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0x2u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xd0u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [97, 100, 100, 115, 117, 98, 112, 115, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xe6u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [99, 118, 116, 112, 100, 50, 100, 113, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xf0u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [108, 100, 100, 113, 117, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: MEM,
                    flags: 0,
                };
            init
        }];
static mut instructions_sse_repe: [Operation; 25] =
    [{
        let mut init =
            Operation {
                opcode: 0x10u16,
                subcode: 8u8,
                size: 0i8,
                name:
                [109, 111, 118, 115, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0],
                arg0: XMM,
                arg1: XM,
                flags: 0,
            };
        init
    },
        {
            let mut init =
                Operation {
                    opcode: 0x11u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [109, 111, 118, 115, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XM,
                    arg1: XMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x12u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [109, 111, 118, 115, 108, 100, 117, 112, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x16u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [109, 111, 118, 115, 104, 100, 117, 112, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x2au16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [99, 118, 116, 115, 105, 50, 115, 115, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: XMM,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x2cu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [99, 118, 116, 116, 115, 115, 50, 115, 105, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: REG,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x2du16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [99, 118, 116, 115, 115, 50, 115, 105, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: REG,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x51u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [115, 113, 114, 116, 115, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x52u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [114, 115, 113, 114, 116, 115, 115, 0, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x53u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [114, 99, 112, 115, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x58u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [97, 100, 100, 115, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x59u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [109, 117, 108, 115, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x5au16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [99, 118, 116, 115, 115, 50, 115, 100, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x5bu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [99, 118, 116, 116, 112, 115, 50, 100, 113, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x5cu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [115, 117, 98, 115, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x5du16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [109, 105, 110, 115, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x5eu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [100, 105, 118, 115, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x5fu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [109, 97, 120, 115, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x6fu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [109, 111, 118, 100, 113, 117, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x70u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [112, 115, 104, 117, 102, 104, 119, 0, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0x2u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x7eu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [109, 111, 118, 113, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x7fu16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [109, 111, 118, 100, 113, 117, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XM,
                    arg1: XMM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xb8u16,
                    subcode: 8u8,
                    size: 16i8,
                    name:
                    [112, 111, 112, 99, 110, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: REG,
                    arg1: RM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xc2u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [99, 109, 112, 115, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0x2u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0xe6u16,
                    subcode: 8u8,
                    size: 0i8,
                    name:
                    [99, 118, 116, 100, 113, 50, 112, 100, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        }];
static mut instructions_sse_single: [Operation; 18] =
    [{
        let mut init =
            Operation {
                opcode: 0x38u16,
                subcode: 0u8,
                size: 0i8,
                name:
                [112, 115, 104, 117, 102, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0],
                arg0: MMX,
                arg1: MM,
                flags: 0,
            };
        init
    },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0x1u8,
                    size: 0i8,
                    name:
                    [112, 104, 97, 100, 100, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0x2u8,
                    size: 0i8,
                    name:
                    [112, 104, 97, 100, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0x3u8,
                    size: 0i8,
                    name:
                    [112, 104, 97, 100, 100, 115, 119, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0x4u8,
                    size: 0i8,
                    name:
                    [112, 109, 97, 100, 100, 117, 98, 115, 119, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0x5u8,
                    size: 0i8,
                    name:
                    [112, 104, 115, 117, 98, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0x6u8,
                    size: 0i8,
                    name:
                    [112, 104, 115, 117, 98, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0x7u8,
                    size: 0i8,
                    name:
                    [112, 104, 115, 117, 98, 115, 119, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0x8u8,
                    size: 0i8,
                    name:
                    [112, 115, 105, 103, 110, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0x9u8,
                    size: 0i8,
                    name:
                    [112, 115, 105, 103, 110, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0xau8,
                    size: 0i8,
                    name:
                    [112, 115, 105, 103, 110, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0xbu8,
                    size: 0i8,
                    name:
                    [112, 109, 117, 108, 104, 114, 115, 119, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0x1cu8,
                    size: 0i8,
                    name: [112, 97, 98, 115, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0x1du8,
                    size: 0i8,
                    name:
                    [112, 97, 98, 115, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0x1eu8,
                    size: 0i8,
                    name:
                    [112, 97, 98, 115, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0xf0u8,
                    size: 16i8,
                    name:
                    [109, 111, 118, 98, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: REG,
                    arg1: MEM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0xf1u8,
                    size: 16i8,
                    name:
                    [109, 111, 118, 98, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: MEM,
                    arg1: REG,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x3au16,
                    subcode: 0xfu8,
                    size: 0i8,
                    name:
                    [112, 97, 108, 105, 103, 110, 114, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: MMX,
                    arg1: MM,
                    flags: 0x2u32,
                };
            init
        }];
static mut instructions_sse_single_op32: [Operation; 69] =
    [{
        let mut init =
            Operation {
                opcode: 0x38u16,
                subcode: 0u8,
                size: 0i8,
                name:
                [112, 115, 104, 117, 102, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0],
                arg0: XMM,
                arg1: XM,
                flags: 0,
            };
        init
    },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0x1u8,
                    size: 0i8,
                    name:
                    [112, 104, 97, 100, 100, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0x2u8,
                    size: 0i8,
                    name:
                    [112, 104, 97, 100, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0x3u8,
                    size: 0i8,
                    name:
                    [112, 104, 97, 100, 100, 115, 119, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0x4u8,
                    size: 0i8,
                    name:
                    [112, 109, 97, 100, 100, 117, 98, 115, 119, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0x5u8,
                    size: 0i8,
                    name:
                    [112, 104, 115, 117, 98, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0x6u8,
                    size: 0i8,
                    name:
                    [112, 104, 115, 117, 98, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0x7u8,
                    size: 0i8,
                    name:
                    [112, 104, 115, 117, 98, 115, 119, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0x8u8,
                    size: 0i8,
                    name:
                    [112, 115, 105, 103, 110, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0x9u8,
                    size: 0i8,
                    name:
                    [112, 115, 105, 103, 110, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0xau8,
                    size: 0i8,
                    name:
                    [112, 115, 105, 103, 110, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0xbu8,
                    size: 0i8,
                    name:
                    [112, 109, 117, 108, 104, 114, 115, 119, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0x10u8,
                    size: 0i8,
                    name:
                    [112, 98, 108, 101, 110, 100, 118, 98, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0x14u8,
                    size: 0i8,
                    name:
                    [98, 108, 101, 110, 100, 118, 112, 115, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0x15u8,
                    size: 0i8,
                    name:
                    [98, 108, 101, 110, 100, 118, 112, 100, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0x17u8,
                    size: 0i8,
                    name:
                    [112, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0x1cu8,
                    size: 0i8,
                    name: [112, 97, 98, 115, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0x1du8,
                    size: 0i8,
                    name:
                    [112, 97, 98, 115, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0x1eu8,
                    size: 0i8,
                    name:
                    [112, 97, 98, 115, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0x20u8,
                    size: 0i8,
                    name:
                    [112, 109, 111, 118, 115, 120, 98, 119, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0x21u8,
                    size: 0i8,
                    name:
                    [112, 109, 111, 118, 115, 120, 98, 100, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0x22u8,
                    size: 0i8,
                    name:
                    [112, 109, 111, 118, 115, 120, 98, 113, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0x23u8,
                    size: 0i8,
                    name:
                    [112, 109, 111, 118, 115, 120, 119, 100, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0x24u8,
                    size: 0i8,
                    name:
                    [112, 109, 111, 118, 115, 120, 119, 113, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0x25u8,
                    size: 0i8,
                    name:
                    [112, 109, 111, 118, 115, 120, 100, 113, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0x28u8,
                    size: 0i8,
                    name:
                    [112, 109, 117, 108, 100, 113, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0x29u8,
                    size: 0i8,
                    name:
                    [112, 99, 109, 112, 101, 113, 113, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0x2au8,
                    size: 0i8,
                    name:
                    [109, 111, 118, 110, 116, 100, 113, 97, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: XMM,
                    arg1: MEM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0x2bu8,
                    size: 0i8,
                    name:
                    [112, 97, 99, 107, 117, 115, 100, 119, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0x30u8,
                    size: 0i8,
                    name:
                    [112, 109, 111, 118, 122, 120, 98, 119, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0x31u8,
                    size: 0i8,
                    name:
                    [112, 109, 111, 118, 122, 120, 98, 100, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0x32u8,
                    size: 0i8,
                    name:
                    [112, 109, 111, 118, 122, 120, 98, 113, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0x33u8,
                    size: 0i8,
                    name:
                    [112, 109, 111, 118, 122, 120, 119, 100, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0x34u8,
                    size: 0i8,
                    name:
                    [112, 109, 111, 118, 122, 120, 119, 113, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0x35u8,
                    size: 0i8,
                    name:
                    [112, 109, 111, 118, 122, 120, 100, 113, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0x37u8,
                    size: 0i8,
                    name:
                    [112, 99, 109, 112, 103, 116, 113, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0x38u8,
                    size: 0i8,
                    name:
                    [112, 109, 105, 110, 115, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0x39u8,
                    size: 0i8,
                    name:
                    [112, 109, 105, 110, 115, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0x3au8,
                    size: 0i8,
                    name:
                    [112, 109, 105, 110, 117, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0x3bu8,
                    size: 0i8,
                    name:
                    [112, 109, 105, 110, 117, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0x3cu8,
                    size: 0i8,
                    name:
                    [112, 109, 97, 120, 115, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0x3du8,
                    size: 0i8,
                    name:
                    [112, 109, 97, 120, 115, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0x3eu8,
                    size: 0i8,
                    name:
                    [112, 109, 97, 120, 117, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0x3fu8,
                    size: 0i8,
                    name:
                    [112, 109, 97, 120, 117, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0x40u8,
                    size: 0i8,
                    name:
                    [112, 109, 97, 120, 108, 108, 100, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x38u16,
                    subcode: 0x41u8,
                    size: 0i8,
                    name:
                    [112, 104, 109, 105, 110, 112, 111, 115, 117, 119, 0, 0,
                        0, 0, 0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x3au16,
                    subcode: 0x8u8,
                    size: 0i8,
                    name:
                    [114, 111, 117, 110, 100, 112, 115, 0, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0x2u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x3au16,
                    subcode: 0x9u8,
                    size: 0i8,
                    name:
                    [114, 111, 117, 110, 100, 112, 100, 0, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0x2u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x3au16,
                    subcode: 0xau8,
                    size: 0i8,
                    name:
                    [114, 111, 117, 110, 100, 115, 115, 0, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0x2u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x3au16,
                    subcode: 0xbu8,
                    size: 0i8,
                    name:
                    [114, 111, 117, 110, 100, 115, 100, 0, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0x2u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x3au16,
                    subcode: 0xcu8,
                    size: 0i8,
                    name:
                    [98, 108, 101, 110, 100, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0x2u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x3au16,
                    subcode: 0xdu8,
                    size: 0i8,
                    name:
                    [98, 108, 101, 110, 100, 112, 100, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0x2u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x3au16,
                    subcode: 0xeu8,
                    size: 0i8,
                    name:
                    [112, 98, 108, 101, 110, 100, 119, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0x2u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x3au16,
                    subcode: 0xfu8,
                    size: 0i8,
                    name:
                    [112, 97, 108, 105, 103, 110, 114, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0x2u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x3au16,
                    subcode: 0x14u8,
                    size: 0i8,
                    name:
                    [112, 101, 120, 116, 114, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: RM,
                    arg1: XMM,
                    flags: 0x2u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x3au16,
                    subcode: 0x15u8,
                    size: 0i8,
                    name:
                    [112, 101, 120, 116, 114, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: RM,
                    arg1: XMM,
                    flags: 0x2u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x3au16,
                    subcode: 0x16u8,
                    size: 0i8,
                    name:
                    [112, 101, 120, 116, 114, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: RM,
                    arg1: XMM,
                    flags: 0x2u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x3au16,
                    subcode: 0x17u8,
                    size: 0i8,
                    name:
                    [101, 120, 116, 114, 97, 99, 116, 112, 115, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: RM,
                    arg1: XMM,
                    flags: 0x2u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x3au16,
                    subcode: 0x20u8,
                    size: 0i8,
                    name:
                    [112, 105, 110, 115, 114, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: RM,
                    flags: 0x2u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x3au16,
                    subcode: 0x21u8,
                    size: 0i8,
                    name:
                    [105, 110, 115, 101, 114, 116, 112, 115, 0, 0, 0, 0, 0, 0,
                        0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0x2u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x3au16,
                    subcode: 0x22u8,
                    size: 0i8,
                    name:
                    [112, 105, 110, 115, 114, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: RM,
                    flags: 0x2u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x3au16,
                    subcode: 0x40u8,
                    size: 0i8,
                    name:
                    [100, 112, 112, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x3au16,
                    subcode: 0x41u8,
                    size: 0i8,
                    name:
                    [100, 112, 112, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x3au16,
                    subcode: 0x42u8,
                    size: 0i8,
                    name:
                    [109, 112, 115, 113, 100, 98, 119, 0, 0, 0, 0, 0, 0, 0, 0,
                        0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0x2u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x3au16,
                    subcode: 0x44u8,
                    size: 0i8,
                    name:
                    [112, 99, 108, 109, 117, 108, 113, 100, 113, 0, 0, 0, 0,
                        0, 0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0x2u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x3au16,
                    subcode: 0x60u8,
                    size: 0i8,
                    name:
                    [112, 99, 109, 112, 101, 115, 116, 114, 109, 0, 0, 0, 0,
                        0, 0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0x2u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x3au16,
                    subcode: 0x61u8,
                    size: 0i8,
                    name:
                    [112, 99, 109, 112, 101, 115, 116, 114, 105, 0, 0, 0, 0,
                        0, 0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0x2u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x3au16,
                    subcode: 0x62u8,
                    size: 0i8,
                    name:
                    [112, 99, 109, 112, 105, 115, 116, 114, 109, 0, 0, 0, 0,
                        0, 0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0x2u32,
                };
            init
        },
        {
            let mut init =
                Operation {
                    opcode: 0x3au16,
                    subcode: 0x63u8,
                    size: 0i8,
                    name:
                    [112, 99, 109, 112, 105, 115, 116, 114, 105, 0, 0, 0, 0,
                        0, 0, 0],
                    arg0: XMM,
                    arg1: XM,
                    flags: 0x2u32,
                };
            init
        }];
/* returns the flag if it's a prefix, 0 otherwise */
unsafe extern "C" fn get_prefix(mut opcode: word, mut bits: libc::c_int)
                                -> word {
    if bits == 64i32 {
        if opcode as libc::c_int & 0xfff0i32 ==
            0x40i32 {
            return (0x800i32 |
                (opcode as libc::c_int & 0xfi32) *
                    0x1000i32) as word;
        }
    }
    return match opcode as libc::c_int {
        38 => { 0x1u16 }
        46 => { 0x2u16 }
        54 => { 0x3u16 }
        62 => { 0x4u16 }
        100 => { 0x5u16 }
        101 => { 0x6u16 }
        102 => { 0x8u16 }
        103 => { 0x10u16 }
        155 => { 0x100u16 }
        240 => { 0x20u16 }
        242 => { 0x40u16 }
        243 => { 0x80u16 }
        _ => { 0u16 }
    };
}

unsafe extern "C" fn instr_matches(opcode: byte, subcode: byte,
                                   mut op: *const Operation) -> libc::c_int {
    return (opcode as libc::c_int == (*op).opcode as libc::c_int &&
        ((*op).subcode as libc::c_int == 8i32 ||
            subcode as libc::c_int == (*op).subcode as libc::c_int))
        as libc::c_int;
}
/* aka 3 byte opcode */
unsafe extern "C" fn get_sse_single(mut opcode: byte, mut subcode: byte,
                                    mut instr: *mut Instruction) -> libc::c_int {
    let mut i = 0;
    if (*instr).prefix as libc::c_int & 0x8i32 != 0 {
        i = 0i32;
        while (i as libc::c_ulong) <
            (::std::mem::size_of::<[Operation; 69]>() as
                libc::c_ulong).wrapping_div(::std::mem::size_of::<Operation>()
                as libc::c_ulong) {
            if instructions_sse_single_op32[i as usize].opcode as libc::c_int
                == opcode as libc::c_int &&
                instructions_sse_single_op32[i as usize].subcode as
                    libc::c_int == subcode as libc::c_int {
                *instr =
                    crate::src::x86_instr::Instruction {
                        op:
                        instructions_sse_single_op32[i as usize],
                        prefix:


                        ((*instr).prefix as libc::c_int & !(0x8i32))
                            as word,
                        ..*instr
                    };
                return 1i32;
            }
            i += 1
        }
    } else {
        i = 0i32;
        while (i as libc::c_ulong) <
            (::std::mem::size_of::<[Operation; 18]>() as
                libc::c_ulong).wrapping_div(::std::mem::size_of::<Operation>()
                as libc::c_ulong) {
            if instructions_sse_single[i as usize].opcode as libc::c_int ==
                opcode as libc::c_int &&
                instructions_sse_single[i as usize].subcode as libc::c_int
                    == subcode as libc::c_int {
                *instr =
                    crate::src::x86_instr::Instruction {
                        op:

                        instructions_sse_single[i as usize],
                        ..*instr
                    };
                return 1i32;
            }
            i += 1
        }
    }
    return 0i32;
}

unsafe extern "C" fn get_sse_instr(mut p: *const byte, mut instr: *mut Instruction)
                                   -> libc::c_int {
    let mut subcode =
        (*p.offset(1isize) as libc::c_int >>
            3i32 & 7i32) as byte;
    let mut i = 0;
    /* Clear the prefix if it matches. This makes the disassembler work right,
     * but it might break things later if we want to interpret these. The
     * solution in that case is probably to modify the size/name instead. */
    if (*instr).prefix as libc::c_int & 0x8i32 != 0 {
        i = 0u32;
        while (i as libc::c_ulong) <
            (::std::mem::size_of::<[Operation; 114]>() as
                libc::c_ulong).wrapping_div(::std::mem::size_of::<Operation>()
                as libc::c_ulong) {
            if instr_matches(*p.offset(0isize), subcode,
                             &*instructions_sse_op32.as_ptr().offset(i as
                                 isize))
                != 0 {
                *instr =
                    crate::src::x86_instr::Instruction {
                        op: instructions_sse_op32[i as usize],
                        prefix:


                        ((*instr).prefix as libc::c_int & !(0x8i32))
                            as word,
                        ..*instr
                    };
                return 0i32;
            }
            i = i.wrapping_add(1)
        }
    } else if (*instr).prefix as libc::c_int & 0x40i32 != 0 {
        i = 0u32;
        while (i as libc::c_ulong) <
            (::std::mem::size_of::<[Operation; 21]>() as
                libc::c_ulong).wrapping_div(::std::mem::size_of::<Operation>()
                as libc::c_ulong) {
            if instr_matches(*p.offset(0isize), subcode,
                             &*instructions_sse_repne.as_ptr().offset(i as
                                 isize))
                != 0 {
                *instr =
                    crate::src::x86_instr::Instruction {
                        op: instructions_sse_repne[i as usize],
                        prefix:


                        ((*instr).prefix as libc::c_int & !(0x40i32))
                            as word,
                        ..*instr
                    };
                return 0i32;
            }
            i = i.wrapping_add(1)
        }
    } else if (*instr).prefix as libc::c_int & 0x80i32 != 0 {
        i = 0u32;
        while (i as libc::c_ulong) <
            (::std::mem::size_of::<[Operation; 25]>() as
                libc::c_ulong).wrapping_div(::std::mem::size_of::<Operation>()
                as libc::c_ulong) {
            if instr_matches(*p.offset(0isize), subcode,
                             &*instructions_sse_repe.as_ptr().offset(i as
                                 isize))
                != 0 {
                *instr =
                    crate::src::x86_instr::Instruction {
                        op: instructions_sse_repe[i as usize],
                        prefix:


                        ((*instr).prefix as libc::c_int & !(0x80i32))
                            as word,
                        ..*instr
                    };
                return 0i32;
            }
            i = i.wrapping_add(1)
        }
    } else {
        i = 0u32;
        while (i as libc::c_ulong) <
            (::std::mem::size_of::<[Operation; 109]>() as
                libc::c_ulong).wrapping_div(::std::mem::size_of::<Operation>()
                as libc::c_ulong) {
            if instr_matches(*p.offset(0isize), subcode,
                             &*instructions_sse.as_ptr().offset(i as isize))
                != 0 {
                *instr =
                    crate::src::x86_instr::Instruction { op: instructions_sse[i as usize], ..*instr };
                return 0i32;
            }
            i = i.wrapping_add(1)
        }
    }
    return get_sse_single(*p.offset(0isize),
                          *p.offset(1isize), instr);
}

unsafe extern "C" fn get_0f_instr(mut p: *const byte, mut instr: *mut Instruction)
                                  -> libc::c_int {
    let mut subcode =
        (*p.offset(1isize) as libc::c_int >>
            3i32 & 7i32) as byte;
    let mut len = 0;
    /* a couple of special (read: annoying) cases first */
    if *p.offset(0isize) as libc::c_int ==
        0x1i32 &&
        *p.offset(1isize) as libc::c_int >>
            6i32 == 3i32 {
        (*instr).op =
            crate::src::x86_instr::Operation {
                opcode: 0xf01u16,
                subcode: *p.offset(1isize),
                ..
                (*instr).op
            };
        match *p.offset(1isize) as libc::c_int {
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
            _ => {}
        }
        return 1i32;
    } else {
        if *p.offset(0isize) as libc::c_int ==
            0xaei32 &&
            *p.offset(1isize) as libc::c_int >>
                6i32 == 3i32 {
            (*instr).op =
                crate::src::x86_instr::Operation {
                    opcode: 0xfaeu16,
                    subcode,
                    ..
                    (*instr).op
                };
            if subcode as libc::c_int == 0x5i32 {
                strcpy((*instr).op.name.as_mut_ptr(),
                       b"lfence\x00" as *const u8 as *const libc::c_char);
            }
            if subcode as libc::c_int == 0x6i32 {
                strcpy((*instr).op.name.as_mut_ptr(),
                       b"mfence\x00" as *const u8 as *const libc::c_char);
            }
            if subcode as libc::c_int == 0x7i32 {
                strcpy((*instr).op.name.as_mut_ptr(),
                       b"sfence\x00" as *const u8 as *const libc::c_char);
            }
            return 1i32;
        }
    }
    let mut i = 0u32;
    while (i as libc::c_ulong) <
        (::std::mem::size_of::<[Operation; 133]>() as
            libc::c_ulong).wrapping_div(::std::mem::size_of::<Operation>() as
            libc::c_ulong) {
        if instr_matches(*p.offset(0isize), subcode,
                         &*instructions_0F.as_ptr().offset(i as isize)) != 0 {
            *instr =
                crate::src::x86_instr::Instruction { op: instructions_0F[i as usize], ..*instr };
            len = 0i32;
            break;
        } else { i = i.wrapping_add(1) }
    }
    if (*instr).op.name[0usize] == 0 {
        len = get_sse_instr(p, instr)
    }


    (*instr).op =
        crate::src::x86_instr::Operation {
            opcode:


            (0xf00i32 |
                *p.offset(0isize) as libc::c_int) as word,
            ..
            (*instr).op
        };
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
                             mut arg: *mut Argument, mut instr: *mut Instruction,
                             mut bits: libc::c_int) -> libc::c_int {
    *arg = crate::src::x86_instr::Argument { value: 0u64, ..*arg }; /* signed */
    return match (*arg).type_0 {
        27 => {
            *arg = crate::src::x86_instr::Argument { ip, value: *p as qword, ..*arg };
            1i32
        }
        28 => {
            *arg =
                crate::src::x86_instr::Argument {
                    ip,
                    value: *(p as *mut word) as qword,
                    ..*arg
                };
            2i32
        }
        29 => {
            *arg = crate::src::x86_instr::Argument { ip, ..*arg };
            if (*instr).op.size as libc::c_int == 8i32 {
                *arg = crate::src::x86_instr::Argument { value: *p as qword, ..*arg };
                1i32
            } else if (*instr).op.size as libc::c_int == 16i32 {
                *arg = crate::src::x86_instr::Argument { value: *(p as *mut word) as qword, ..*arg };
                2i32
            } else if (*instr).op.size as libc::c_int == 64i32 &&
                (*instr).op.flags &
                    0x800u32 != 0 {
                *arg = crate::src::x86_instr::Argument { value: *(p as *mut qword), ..*arg };
                8i32
            } else {
                *arg = crate::src::x86_instr::Argument { value: *(p as *mut dword) as qword, ..*arg };
                4i32
            }
        }
        30 => {
            *arg =
                crate::src::x86_instr::Argument {
                    ip,
                    value:


                    ip.wrapping_add(1u32).wrapping_add(*(p as
                        *mut int8_t)
                        as
                        libc::c_uint)
                        as qword,
                    ..*arg
                };
            1i32
        }
        31 => {
            *arg = crate::src::x86_instr::Argument { ip, ..*arg };
            /* Equivalently signed or unsigned (i.e. clipped) */
            if bits == 16i32 {
                *arg =
                    crate::src::x86_instr::Argument {
                        value:


                        (ip.wrapping_add(2u32).wrapping_add(*(p as
                            *mut word)
                            as
                            libc::c_uint)
                            & 0xffffu32) as
                            qword,
                        ..*arg
                    }; /* I think this should be enough */
                2i32
            } else {
                *arg =
                    crate::src::x86_instr::Argument {
                        value:


                        (ip.wrapping_add(4u32).wrapping_add(*(p as
                            *mut dword))
                            & 0xffffffffu32) as qword,
                        ..*arg
                    };
                4i32
            }
        }
        32 => {
            *arg =
                crate::src::x86_instr::Argument {
                    ip,
                    value: *(p as *mut word) as qword,
                    ..*arg
                };
            4i32
        }
        33 => {
            *arg = crate::src::x86_instr::Argument { ip, ..*arg };
            if bits == 64i32 {
                *arg = crate::src::x86_instr::Argument { value: *(p as *mut qword), ..*arg };
                8i32
            } else if bits == 32i32 {
                *arg = crate::src::x86_instr::Argument { value: *(p as *mut dword) as qword, ..*arg };
                4i32
            } else {
                *arg = crate::src::x86_instr::Argument { value: *(p as *mut word) as qword, ..*arg };
                2i32
            }
        }
        37 | 40 | 38 | 39 => {
            let mut mod_0 =
                (*p as libc::c_int >> 6i32) as byte;
            let mut rm = (*p as libc::c_int & 7i32) as byte;
            let mut ret = 1i32;
            if mod_0 as libc::c_int == 3i32 {
                *instr =
                    crate::src::x86_instr::Instruction {
                        modrm_disp: DISP_REG,
                        modrm_reg: rm as libc::c_char,
                        ..*instr
                    };
                if (*instr).prefix as libc::c_int & 0x1000i32 != 0
                {
                    *instr =
                        crate::src::x86_instr::Instruction {
                            modrm_reg:

                            ((*instr).modrm_reg as libc::c_int + 8i32)
                                as libc::c_char,
                            ..*instr
                        }
                }
                return 1i32;
            }
            *arg = crate::src::x86_instr::Argument { ip, ..*arg };
            if (*instr).addrsize as libc::c_int != 16i32 &&
                rm as libc::c_int == 4i32 {
                /* SIB byte */
                p = p.offset(1);

                *instr =
                    crate::src::x86_instr::Instruction {
                        sib_scale:


                        ((1i32) <<
                            (*p as libc::c_int >> 6i32)) as byte,
                        sib_index:


                        (*p as libc::c_int >> 3i32 & 7i32)
                            as libc::c_char,
                        ..*instr
                    };
                if (*instr).prefix as libc::c_int & 0x2000i32 != 0
                {
                    *instr =
                        crate::src::x86_instr::Instruction {
                            sib_index:

                            ((*instr).sib_index as libc::c_int + 8i32)
                                as libc::c_char,
                            ..*instr
                        }
                }
                if (*instr).sib_index as libc::c_int == 4i32 {
                    *instr = crate::src::x86_instr::Instruction { sib_index: -1i8, ..*instr }
                }
                rm = (*p as libc::c_int & 7i32) as byte;
                ret += 1
            }
            if mod_0 as libc::c_int == 0i32 &&
                bits == 64i32 &&
                rm as libc::c_int == 5i32 &&
                (*instr).sib_scale == 0 {
                /* IP-relative addressing... */


                *arg =
                    crate::src::x86_instr::Argument {
                        value:


                        *(p.offset(1isize) as *mut dword) as
                            qword,
                        ..*arg
                    };
                *instr =
                    crate::src::x86_instr::Instruction {
                        modrm_disp: DISP_16,
                        modrm_reg: 16i8,
                        ..*instr
                    };
                ret += 4i32
            } else if mod_0 as libc::c_int == 0i32 &&
                ((*instr).addrsize as libc::c_int ==
                    16i32 &&
                    rm as libc::c_int == 6i32 ||
                    (*instr).addrsize as libc::c_int !=
                        16i32 &&
                        rm as libc::c_int == 5i32) {
                if (*instr).addrsize as libc::c_int == 16i32 {
                    *arg =
                        crate::src::x86_instr::Argument {
                            value:


                            *(p.offset(1isize) as *mut word) as
                                qword,
                            ..*arg
                        };
                    ret += 2i32
                } else {
                    *arg =
                        crate::src::x86_instr::Argument {
                            value:


                            *(p.offset(1isize) as *mut dword)
                                as qword,
                            ..*arg
                        };
                    ret += 4i32
                }
                *instr = crate::src::x86_instr::Instruction { modrm_disp: DISP_16, ..*instr };
                *instr = crate::src::x86_instr::Instruction { modrm_reg: -1i8, ..*instr }
            } else if mod_0 as libc::c_int == 0i32 {
                *instr =
                    crate::src::x86_instr::Instruction {
                        modrm_disp: DISP_NONE,
                        modrm_reg: rm as libc::c_char,
                        ..*instr
                    };
                if (*instr).prefix as libc::c_int & 0x1000i32 != 0
                {
                    *instr =
                        crate::src::x86_instr::Instruction {
                            modrm_reg:

                            ((*instr).modrm_reg as libc::c_int + 8i32)
                                as libc::c_char,
                            ..*instr
                        }
                }
            } else if mod_0 as libc::c_int == 1i32 {
                *arg = crate::src::x86_instr::Argument { value: *p.offset(1isize) as qword, ..*arg };
                *instr =
                    crate::src::x86_instr::Instruction {
                        modrm_disp: DISP_8,
                        modrm_reg: rm as libc::c_char,
                        ..*instr
                    };
                if (*instr).prefix as libc::c_int & 0x1000i32 != 0
                {
                    *instr =
                        crate::src::x86_instr::Instruction {
                            modrm_reg:

                            ((*instr).modrm_reg as libc::c_int + 8i32)
                                as libc::c_char,
                            ..*instr
                        }
                }
                ret += 1i32
            } else if mod_0 as libc::c_int == 2i32 {
                if (*instr).addrsize as libc::c_int == 16i32 {
                    *arg =
                        crate::src::x86_instr::Argument {
                            value:


                            *(p.offset(1isize) as *mut word) as
                                qword,
                            ..*arg
                        };
                    ret += 2i32
                } else {
                    *arg =
                        crate::src::x86_instr::Argument {
                            value:


                            *(p.offset(1isize) as *mut dword)
                                as qword,
                            ..*arg
                        };
                    ret += 4i32
                }

                *instr =
                    crate::src::x86_instr::Instruction {
                        modrm_disp: DISP_16,
                        modrm_reg: rm as libc::c_char,
                        ..*instr
                    };
                if (*instr).prefix as libc::c_int & 0x1000i32 != 0
                {
                    *instr =
                        crate::src::x86_instr::Instruction {
                            modrm_reg:

                            ((*instr).modrm_reg as libc::c_int + 8i32)
                                as libc::c_char,
                            ..*instr
                        }
                }
            }
            ret
        }
        44 | 46 | 49 | 50 | 51 => {
            /* doesn't exist in 64-bit mode */
            *arg =
                crate::src::x86_instr::Argument {
                    value:


                    (*p as libc::c_int >> 3i32 & 7i32) as
                        qword,
                    ..*arg
                };
            if (*instr).prefix as libc::c_int & 0x4000i32 != 0 {
                *arg =

                    crate::src::x86_instr::Argument {
                        value:

                        ((*arg).value).wrapping_add(8u64),
                        ..*arg
                    }
            }
            0i32
        }
        45 | 47 => {
            *arg =
                crate::src::x86_instr::Argument {
                    value:


                    (*p as libc::c_int >> 3i32 & 7i32) as
                        qword,
                    ..*arg
                };
            0i32
        }
        48 | 53 | 41 | 42 | 43 => {
            *arg =
                crate::src::x86_instr::Argument {
                    value:
                    (*p as libc::c_int & 7i32) as qword,
                    ..*arg
                };
            if (*instr).prefix as libc::c_int & 0x1000i32 != 0 {
                *arg =

                    crate::src::x86_instr::Argument {
                        value:

                        ((*arg).value).wrapping_add(8u64),
                        ..*arg
                    }
            }
            1i32
        }
        _ => {
            /* all others should be implicit */
            0i32
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
    if asm_syntax == GAS {
        strcat(out, b"%\x00" as *const u8 as *const libc::c_char);
    }
    strcat(out, seg16[reg as usize].as_ptr());
}

unsafe extern "C" fn get_reg8(mut out: *mut libc::c_char, mut reg: byte,
                              mut rex: libc::c_int) {
    if asm_syntax == GAS {
        strcat(out, b"%\x00" as *const u8 as *const libc::c_char);
    }
    strcat(out,
           if rex != 0 {
               reg8_rex[reg as usize].as_ptr()
           } else { reg8[reg as usize].as_ptr() });
}

unsafe extern "C" fn get_reg16(mut out: *mut libc::c_char, mut reg: byte,
                               mut size: libc::c_int) {
    if reg as libc::c_int != -(1i32) {
        if asm_syntax == GAS {
            strcat(out, b"%\x00" as *const u8 as *const libc::c_char);
        }
        if size == 16i32 {
            strcat(out, reg16[reg as usize].as_ptr());
        }
        if size == 32i32 {
            strcat(out, reg32[reg as usize].as_ptr());
        } else if size == 64i32 {
            strcat(out, reg64[reg as usize].as_ptr());
        }
    };
}

unsafe extern "C" fn get_xmm(mut out: *mut libc::c_char, mut reg: byte) {
    if asm_syntax == GAS {
        strcat(out, b"%\x00" as *const u8 as *const libc::c_char);
    }
    strcat(out, b"xmm0\x00" as *const u8 as *const libc::c_char);
    *out.offset(strlen(out).wrapping_sub(1u64) as
        isize) =
        ('0' as i32 + reg as libc::c_int) as libc::c_char;
}

unsafe extern "C" fn get_mmx(mut out: *mut libc::c_char, mut reg: byte) {
    if asm_syntax == GAS {
        strcat(out, b"%\x00" as *const u8 as *const libc::c_char);
    }
    strcat(out, b"mm0\x00" as *const u8 as *const libc::c_char);
    *out.offset(strlen(out).wrapping_sub(1u64) as
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
    return (arg >= AL &&

        arg <= GS ||

        arg >= REG &&

            arg <=

                TR32) as libc::c_int;
}
/* With MASM/NASM, use capital letters to help disambiguate them from the following 'h'. */
unsafe extern "C" fn print_arg(mut ip: *mut libc::c_char,
                               mut instr: *mut Instruction, mut i: libc::c_int,
                               mut bits: libc::c_int) {
    /* someone wants to print something special */

    let mut arg =
        &mut *(*instr).args.as_mut_ptr().offset(i as isize) as
            *mut Argument;
    let mut out = (*arg).string.as_mut_ptr();
    let mut value = (*arg).value;
    if (*arg).string[0usize] != 0 { return; }
    if (*arg).type_0 >= AL &&

        (*arg).type_0 <= BH
    {
        get_reg8(out,
                 ((*arg).type_0).wrapping_sub(AL) as byte,
                 0i32);
    } else if (*arg).type_0 >=

        AX &&

        (*arg).type_0 <=

            DI {
        get_reg16(out,
                  ((*arg).type_0).wrapping_sub(AX).wrapping_add((if (*instr).prefix
                      as
                      libc::c_int
                      &
                      0x1000i32
                      !=
                      0
                  {
                      8i32
                  } else {
                      0i32
                  })
                      as
                      libc::c_uint)
                      as byte, (*instr).op.size as libc::c_int);
    } else if (*arg).type_0 >=

        ES &&

        (*arg).type_0 <=

            GS {
        get_seg16(out,
                  ((*arg).type_0).wrapping_sub(ES) as byte);
    }
    match (*arg).type_0 {
        1 => {
            strcat(out,
                   if asm_syntax ==

                       GAS {
                       b"$0x1\x00" as *const u8 as *const libc::c_char
                   } else { b"1h\x00" as *const u8 as *const libc::c_char });
        }
        27 => {
            if (*instr).op.flags & 0x100u32 != 0 {
                /* 6a */
                if (*instr).op.size as libc::c_int == 64i32 {
                    sprintf(out,
                            if asm_syntax ==

                                GAS {
                                b"$0x%016lx\x00" as *const u8 as
                                    *const libc::c_char
                            } else {
                                b"qword %016lxh\x00" as *const u8 as
                                    *const libc::c_char
                            }, value as int8_t as qword);
                } else if (*instr).op.size as libc::c_int == 32i32
                {
                    sprintf(out,
                            if asm_syntax ==

                                GAS {
                                b"$0x%08x\x00" as *const u8 as
                                    *const libc::c_char
                            } else {
                                b"dword %08Xh\x00" as *const u8 as
                                    *const libc::c_char
                            }, value as int8_t as dword);
                } else {
                    sprintf(out,
                            if asm_syntax ==

                                GAS {
                                b"$0x%04x\x00" as *const u8 as
                                    *const libc::c_char
                            } else {
                                b"word %04Xh\x00" as *const u8 as
                                    *const libc::c_char
                            }, value as int8_t as word as libc::c_int);
                }
            } else {
                sprintf(out,
                        if asm_syntax ==

                            GAS {
                            b"$0x%02lx\x00" as *const u8 as
                                *const libc::c_char
                        } else {
                            b"%02lXh\x00" as *const u8 as *const libc::c_char
                        }, value);
            }
        }
        28 => {
            sprintf(out,
                    if asm_syntax ==

                        GAS {
                        b"$0x%04lx\x00" as *const u8 as *const libc::c_char
                    } else {
                        b"%04lXh\x00" as *const u8 as *const libc::c_char
                    }, value);
        }
        29 => {
            if (*instr).op.flags & 0x100u32 != 0 {
                if (*instr).op.size as libc::c_int == 64i32 {
                    sprintf(out,
                            if asm_syntax ==

                                GAS {
                                b"$0x%016lx\x00" as *const u8 as
                                    *const libc::c_char
                            } else {
                                b"qword %016lXh\x00" as *const u8 as
                                    *const libc::c_char
                            }, value);
                } else if (*instr).op.size as libc::c_int == 32i32
                {
                    sprintf(out,
                            if asm_syntax ==

                                GAS {
                                b"$0x%08lx\x00" as *const u8 as
                                    *const libc::c_char
                            } else {
                                b"dword %08lXh\x00" as *const u8 as
                                    *const libc::c_char
                            }, value);
                } else {
                    sprintf(out,
                            if asm_syntax ==

                                GAS {
                                b"$0x%04lx\x00" as *const u8 as
                                    *const libc::c_char
                            } else {
                                b"word %04lXh\x00" as *const u8 as
                                    *const libc::c_char
                            }, value);
                }
            } else if (*instr).op.size as libc::c_int == 8i32 {
                sprintf(out,
                        if asm_syntax ==

                            GAS {
                            b"$0x%02lx\x00" as *const u8 as
                                *const libc::c_char
                        } else {
                            b"%02lXh\x00" as *const u8 as *const libc::c_char
                        }, value);
            } else if (*instr).op.size as libc::c_int == 16i32 {
                sprintf(out,
                        if asm_syntax ==

                            GAS {
                            b"$0x%04lx\x00" as *const u8 as
                                *const libc::c_char
                        } else {
                            b"%04lXh\x00" as *const u8 as *const libc::c_char
                        }, value);
            } else if (*instr).op.size as libc::c_int == 64i32 &&
                (*instr).op.flags &
                    0x800u32 != 0 {
                sprintf(out,
                        if asm_syntax ==

                            GAS {
                            b"$0x%016lx\x00" as *const u8 as
                                *const libc::c_char
                        } else {
                            b"%016lXh\x00" as *const u8 as *const libc::c_char
                        }, value);
            } else {
                sprintf(out,
                        if asm_syntax ==

                            GAS {
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
            if asm_syntax ==

                GAS {
                if (*instr).prefix as libc::c_int & 0x7i32 != 0 {
                    get_seg16(out,
                              (((*instr).prefix as libc::c_int &
                                  0x7i32) - 1i32) as
                                  byte);
                    strcat(out, b":\x00" as *const u8 as *const libc::c_char);
                }
                sprintf(out.offset(strlen(out) as isize),
                        b"0x%04lx\x00" as *const u8 as *const libc::c_char,
                        value);
            } else {
                *out.offset(0isize) =

                    '[' as libc::c_char;
                if (*instr).prefix as libc::c_int & 0x7i32 != 0 {
                    get_seg16(out,
                              (((*instr).prefix as libc::c_int &
                                  0x7i32) - 1i32) as
                                  byte);
                    strcat(out, b":\x00" as *const u8 as *const libc::c_char);
                }
                sprintf(out.offset(strlen(out) as isize),
                        b"%04lXh]\x00" as *const u8 as *const libc::c_char,
                        value);
            }
            (*instr).set_usedmem(1i32)
        }
        34 | 35 => {
            if asm_syntax !=

                NASM {
                if (*instr).prefix as libc::c_int & 0x7i32 != 0 {
                    get_seg16(out,
                              (((*instr).prefix as libc::c_int &
                                  0x7i32) - 1i32) as
                                  byte);
                    strcat(out, b":\x00" as *const u8 as *const libc::c_char);
                }
                strcat(out,
                       if asm_syntax ==

                           GAS {
                           b"(\x00" as *const u8 as *const libc::c_char
                       } else {
                           b"[\x00" as *const u8 as *const libc::c_char
                       });
                get_reg16(out,
                          if (*arg).type_0 ==

                              DSBX {
                              3i32
                          } else { 6i32 } as byte,
                          (*instr).addrsize as libc::c_int);
                strcat(out,
                       if asm_syntax ==

                           GAS {
                           b")\x00" as *const u8 as *const libc::c_char
                       } else {
                           b"]\x00" as *const u8 as *const libc::c_char
                       });
            }
            (*instr).set_usedmem(1i32)
        }
        36 => {
            if asm_syntax !=

                NASM {
                strcat(out,
                       if asm_syntax ==

                           GAS {
                           b"%es:(\x00" as *const u8 as *const libc::c_char
                       } else {
                           b"es:[\x00" as *const u8 as *const libc::c_char
                       });
                get_reg16(out, 7u8,
                          (*instr).addrsize as libc::c_int);
                strcat(out,
                       if asm_syntax ==

                           GAS {
                           b")\x00" as *const u8 as *const libc::c_char
                       } else {
                           b"]\x00" as *const u8 as *const libc::c_char
                       });
            }
            (*instr).set_usedmem(1i32)
        }
        24 => {
            if asm_syntax ==

                GAS {
                strcpy(out, b"%al\x00" as *const u8 as *const libc::c_char);
            }
        }
        25 => {
            if asm_syntax ==

                GAS {
                strcpy(out, b"%ax\x00" as *const u8 as *const libc::c_char);
            }
        }
        26 => {
            if asm_syntax ==

                GAS {
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
            if (*instr).modrm_disp ==

                DISP_REG {
                if (*arg).type_0 ==

                    XM {
                    get_xmm(out, (*instr).modrm_reg as byte);
                    if (*instr).vex_256() != 0 {
                        *out.offset(if asm_syntax ==

                            GAS
                        {
                            1i32
                        } else { 0i32 } as isize) =

                            'y' as libc::c_char
                    }
                } else if (*arg).type_0 ==

                    MM {
                    get_mmx(out, (*instr).modrm_reg as byte);
                } else {
                    if (*arg).type_0 ==

                        MEM {
                        fprintf(stderr,
                                b"Warning: %s: \x00" as *const u8 as
                                    *const libc::c_char, ip);
                        fprintf(stderr,
                                b"ModRM byte has mod 3, but opcode only allows accessing memory.\n\x00"
                                    as *const u8 as *const libc::c_char);
                    }
                    if (*instr).op.size as libc::c_int == 8i32 ||
                        (*instr).op.opcode as libc::c_int ==
                            0xfb6i32 ||
                        (*instr).op.opcode as libc::c_int ==
                            0xfbei32 {
                        /* mov*b* */
                        get_reg8(out, (*instr).modrm_reg as byte,
                                 (*instr).prefix as libc::c_int &
                                     0x800i32);
                    } else if (*instr).op.opcode as libc::c_int ==
                        0xfb7i32 ||
                        (*instr).op.opcode as libc::c_int ==
                            0xfbfi32 {
                        /* mov*w* */
                        get_reg16(out, (*instr).modrm_reg as byte,
                                  16i32); /* fixme: 64-bit? */
                    } else {
                        get_reg16(out, (*instr).modrm_reg as byte,
                                  (*instr).op.size as libc::c_int);
                    }
                }
            } else {
                (*instr).set_usedmem(1i32);
                /* NASM: <size>    [<seg>: <reg>+<reg>+/-<offset>h] */
                /* MASM: <size> ptr <seg>:[<reg>+<reg>+/-<offset>h] */
                /* GAS:           *%<seg>:<->0x<offset>(%<reg>,%<reg>) */
                if asm_syntax ==

                    GAS {
                    if (*instr).op.opcode as libc::c_int ==
                        0xffi32 &&
                        (*instr).op.subcode as libc::c_int >=
                            2i32 &&
                        (*instr).op.subcode as libc::c_int <=
                            5i32 {
                        strcat(out,
                               b"*\x00" as *const u8 as *const libc::c_char);
                    }
                    if (*instr).prefix as libc::c_int & 0x7i32 !=
                        0 {
                        get_seg16(out,
                                  (((*instr).prefix as libc::c_int &
                                      0x7i32) -
                                      1i32) as byte);
                        strcat(out,
                               b":\x00" as *const u8 as *const libc::c_char);
                    }
                    /* offset */
                    if (*instr).modrm_disp ==

                        DISP_8 {
                        let mut svalue =
                            value as int8_t; /* absolute memory is unsigned */
                        if (svalue as libc::c_int) < 0i32 {
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
                    } else if (*instr).modrm_disp ==

                        DISP_16 &&
                        (*instr).addrsize as libc::c_int ==
                            16i32 {
                        let mut svalue_0 = value as int16_t;
                        if (*instr).modrm_reg as libc::c_int ==
                            -(1i32) {
                            sprintf(out.offset(strlen(out) as isize),
                                    b"0x%04lx\x00" as *const u8 as
                                        *const libc::c_char, value);
                            return;
                        }
                        if (svalue_0 as libc::c_int) < 0i32 {
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
                    } else if (*instr).modrm_disp ==

                        DISP_16 {
                        let mut svalue_1 = value as int32_t;
                        if (*instr).modrm_reg as libc::c_int ==
                            -(1i32) {
                            sprintf(out.offset(strlen(out) as isize),
                                    b"0x%08lx\x00" as *const u8 as
                                        *const libc::c_char, value);
                            return;
                        }
                        if svalue_1 < 0i32 {
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
                    if (*instr).addrsize as libc::c_int == 16i32 {
                        strcat(out,
                               modrm16_gas[(*instr).modrm_reg as
                                   usize].as_ptr());
                    } else {
                        get_reg16(out, (*instr).modrm_reg as byte,
                                  (*instr).addrsize as libc::c_int);
                        if (*instr).sib_scale as libc::c_int != 0 &&
                            (*instr).sib_index as libc::c_int !=
                                -(1i32) {
                            strcat(out,
                                   b",\x00" as *const u8 as
                                       *const libc::c_char);
                            get_reg16(out, (*instr).sib_index as byte,
                                      (*instr).addrsize as libc::c_int);
                            strcat(out,
                                   b",0\x00" as *const u8 as
                                       *const libc::c_char);
                            *out.offset(strlen(out).wrapping_sub(1u64)
                                as isize) =
                                ('0' as i32 +
                                    (*instr).sib_scale as libc::c_int) as
                                    libc::c_char
                        }
                    }
                    strcat(out, b")\x00" as *const u8 as *const libc::c_char);
                } else {
                    let mut has_sib =

                        ((*instr).sib_scale as libc::c_int != 0i32
                            &&
                            (*instr).sib_index as libc::c_int !=
                                -(1i32)) as libc::c_int;
                    if (*instr).op.flags &
                        0x400u32 != 0 {
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
                            _ => {}
                        }
                        if asm_syntax ==

                            MASM {
                            /* && instr->op.size == 0? */
                            strcat(out,
                                   b"ptr \x00" as *const u8 as
                                       *const libc::c_char);
                        }
                    } else if (*instr).op.opcode as libc::c_int ==
                        0xfb6i32 ||
                        (*instr).op.opcode as libc::c_int ==
                            0xfbei32 {
                        /* mov*b* */
                        strcat(out,
                               b"byte \x00" as *const u8 as
                                   *const libc::c_char);
                        if asm_syntax ==

                            MASM {
                            strcat(out,
                                   b"ptr \x00" as *const u8 as
                                       *const libc::c_char);
                        }
                    } else if (*instr).op.opcode as libc::c_int ==
                        0xfb7i32 ||
                        (*instr).op.opcode as libc::c_int ==
                            0xfbfi32 {
                        /* mov*w* */
                        strcat(out,
                               b"word \x00" as *const u8 as
                                   *const libc::c_char); /* absolute memory is unsigned */
                        if asm_syntax ==

                            MASM {
                            strcat(out,
                                   b"ptr \x00" as *const u8 as
                                       *const libc::c_char); /* absolute memory is unsigned */
                        }
                    }
                    if asm_syntax ==

                        NASM {
                        strcat(out,
                               b"[\x00" as *const u8 as *const libc::c_char);
                    }
                    if (*instr).prefix as libc::c_int & 0x7i32 !=
                        0 {
                        get_seg16(out,
                                  (((*instr).prefix as libc::c_int &
                                      0x7i32) -
                                      1i32) as byte);
                        strcat(out,
                               b":\x00" as *const u8 as *const libc::c_char);
                    }
                    if asm_syntax ==

                        MASM {
                        strcat(out,
                               b"[\x00" as *const u8 as *const libc::c_char);
                    }
                    if (*instr).modrm_reg as libc::c_int !=
                        -(1i32) {
                        if (*instr).addrsize as libc::c_int ==
                            16i32 {
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
                        *out.offset(strlen(out).wrapping_sub(1u64)
                            as isize) =
                            ('0' as i32 + (*instr).sib_scale as libc::c_int)
                                as libc::c_char
                    }
                    if (*instr).modrm_disp ==

                        DISP_8 {
                        let mut svalue_2 = value as int8_t;
                        if (svalue_2 as libc::c_int) < 0i32 {
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
                    } else if (*instr).modrm_disp ==

                        DISP_16 &&
                        (*instr).addrsize as libc::c_int ==
                            16i32 {
                        let mut svalue_3 = value as int16_t;
                        if (*instr).modrm_reg as libc::c_int ==
                            -(1i32) && has_sib == 0 {
                            sprintf(out.offset(strlen(out) as isize),
                                    b"%04lXh\x00" as *const u8 as
                                        *const libc::c_char, value);
                        } else if (svalue_3 as libc::c_int) < 0i32
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
                    } else if (*instr).modrm_disp ==

                        DISP_16 {
                        let mut svalue_4 = value as int32_t;
                        if (*instr).modrm_reg as libc::c_int ==
                            -(1i32) && has_sib == 0 {
                            sprintf(out.offset(strlen(out) as isize),
                                    b"%08lXh\x00" as *const u8 as
                                        *const libc::c_char, value);
                        } else if svalue_4 < 0i32 {
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
            if (*instr).op.size as libc::c_int == 8i32 {
                get_reg8(out, value as byte,
                         (*instr).prefix as libc::c_int &
                             0x800i32);
            } else if bits == 64i32 &&
                (*instr).op.opcode as libc::c_int ==
                    0x63i32 {
                get_reg16(out, value as byte, 64i32);
            } else {
                get_reg16(out, value as byte,
                          (*instr).op.size as libc::c_int);
            }
        }
        48 => { get_reg16(out, value as byte, bits); }
        47 => {
            if value > 5u64 {
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
                0 | 2 | 3 | 4 | 8 => {}
                _ => {
                    fprintf(stderr,
                            b"Warning: %s: \x00" as *const u8 as
                                *const libc::c_char, ip);
                    fprintf(stderr,
                            b"Invalid control register %ld\n\x00" as *const u8
                                as *const libc::c_char, value);
                }
            }
            if asm_syntax ==

                GAS {
                strcat(out, b"%\x00" as *const u8 as *const libc::c_char);
            }
            strcat(out, b"cr0\x00" as *const u8 as *const libc::c_char);
            *out.offset(strlen(out).wrapping_sub(1u64) as isize)
                =
                ('0' as i32 as libc::c_ulong).wrapping_add(value) as
                    libc::c_char
        }
        50 => {
            if asm_syntax ==

                GAS {
                strcat(out, b"%\x00" as *const u8 as *const libc::c_char);
            }
            strcat(out, b"dr0\x00" as *const u8 as *const libc::c_char);
            *out.offset(strlen(out).wrapping_sub(1u64) as isize)
                =
                ('0' as i32 as libc::c_ulong).wrapping_add(value) as
                    libc::c_char
        }
        51 => {
            if value < 3u64 {
                fprintf(stderr,
                        b"Warning: %s: \x00" as *const u8 as
                            *const libc::c_char, ip);
                fprintf(stderr,
                        b"Invalid test register %ld\n\x00" as *const u8 as
                            *const libc::c_char, value);
            }
            if asm_syntax ==

                GAS {
                strcat(out, b"%\x00" as *const u8 as *const libc::c_char);
            }
            strcat(out, b"tr0\x00" as *const u8 as *const libc::c_char);
            *out.offset(strlen(out).wrapping_sub(1u64) as isize)
                =
                ('0' as i32 as libc::c_ulong).wrapping_add(value) as
                    libc::c_char
        }
        52 => {
            if asm_syntax ==

                GAS {
                strcat(out, b"%\x00" as *const u8 as *const libc::c_char);
            }
            strcat(out, b"st\x00" as *const u8 as *const libc::c_char);
            if asm_syntax ==

                NASM {
                strcat(out, b"0\x00" as *const u8 as *const libc::c_char);
            }
        }
        53 => {
            if asm_syntax ==

                GAS {
                strcat(out, b"%\x00" as *const u8 as *const libc::c_char);
            }
            strcat(out, b"st\x00" as *const u8 as *const libc::c_char);
            if asm_syntax !=

                NASM {
                strcat(out, b"(\x00" as *const u8 as *const libc::c_char);
            }
            strcat(out, b"0\x00" as *const u8 as *const libc::c_char);
            *out.offset(strlen(out).wrapping_sub(1u64) as isize)
                =
                ('0' as i32 as libc::c_ulong).wrapping_add(value) as
                    libc::c_char;
            if asm_syntax !=

                NASM {
                strcat(out, b")\x00" as *const u8 as *const libc::c_char);
            }
        }
        45 | 42 => { get_mmx(out, value as byte); }
        46 | 43 => {
            get_xmm(out, value as byte);
            if (*instr).vex_256() != 0 {
                *out.offset(if asm_syntax ==

                    GAS {
                    1i32
                } else { 0i32 } as isize) =

                    'y' as libc::c_char
            }
        }
        32 | _ => {}
    };
}
/* helper to tack a length suffix onto a name */
unsafe extern "C" fn suffix_name(mut instr: *mut Instruction) {
    if (*instr).op.flags & 0x3000u32 ==
        0x3000u32 {
        strcat((*instr).op.name.as_mut_ptr(),
               b"ll\x00" as *const u8 as *const libc::c_char);
    } else if (*instr).op.flags & 0x1000u32 != 0 {
        strcat((*instr).op.name.as_mut_ptr(),
               b"s\x00" as *const u8 as *const libc::c_char);
    } else if (*instr).op.flags & 0x2000u32 != 0 {
        strcat((*instr).op.name.as_mut_ptr(),
               b"l\x00" as *const u8 as *const libc::c_char);
    } else if (*instr).op.size as libc::c_int == 80i32 {
        strcat((*instr).op.name.as_mut_ptr(),
               b"t\x00" as *const u8 as *const libc::c_char);
    } else if (*instr).op.size as libc::c_int == 8i32 {
        strcat((*instr).op.name.as_mut_ptr(),
               b"b\x00" as *const u8 as *const libc::c_char);
    } else if (*instr).op.size as libc::c_int == 16i32 {
        strcat((*instr).op.name.as_mut_ptr(),
               b"w\x00" as *const u8 as *const libc::c_char);
    } else if (*instr).op.size as libc::c_int == 32i32 {
        strcat((*instr).op.name.as_mut_ptr(),
               if asm_syntax ==

                   GAS {
                   b"l\x00" as *const u8 as *const libc::c_char
               } else { b"d\x00" as *const u8 as *const libc::c_char });
    } else if (*instr).op.size as libc::c_int == 64i32 {
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
pub unsafe extern "C" fn get_instr(ip: dword,
                                   p: *const byte,
                                   instr: *mut Instruction,
                                   bits: libc::c_int) -> libc::c_int {
    let mut len = 0i32;
    let mut prefix = 0;
    memset(instr as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<Instruction>() as libc::c_ulong);
    loop {
        prefix = get_prefix(*p.offset(len as isize) as word, bits);
        if !(prefix != 0) { break; }
        if (*instr).prefix as libc::c_int & 0x7i32 != 0 &&
            prefix as libc::c_int & 0x7i32 != 0 {
            *instr =
                crate::src::x86_instr::Instruction {
                    op:

                    instructions[*p.offset(len as isize) as usize],
                    ..*instr
                };
            *instr =
                crate::src::x86_instr::Instruction {
                    prefix:

                    ((*instr).prefix as libc::c_int & !(0x7i32)) as
                        word,
                    ..*instr
                }
        } else if !((*instr).prefix as libc::c_int & prefix as libc::c_int &
            0x8i32 != 0) {
            if (*instr).prefix as libc::c_int & prefix as libc::c_int != 0 {
                *instr =
                    crate::src::x86_instr::Instruction {
                        op:

                        instructions[*p.offset(len as isize) as usize],
                        prefix:


                        ((*instr).prefix as libc::c_int &
                            !(prefix as libc::c_int)) as word,
                        ..*instr
                    };
                return len;
            }
        }
        *instr =
            crate::src::x86_instr::Instruction {
                prefix:


                ((*instr).prefix as libc::c_int | prefix as libc::c_int) as word,
                ..*instr
            };
        len += 1
    }
    let mut opcode = *p.offset(len as isize);
    /* copy the op_info */
    if opcode as libc::c_int == 0xc4i32 &&
        *p.offset((len + 1i32) as isize) as libc::c_int >>
            6i32 == 3i32 &&
        bits != 16i32 {
        let mut subcode = 0xccu8;
        len += 1;
        (*instr).set_vex(1i32);
        if *p.offset(len as isize) as libc::c_int & 0x1fi32 ==
            2i32 {
            subcode = 0x38u8
        } else if *p.offset(len as isize) as libc::c_int & 0x1fi32
            == 3i32 {
            subcode = 0x3au8
        } else {
            fprintf(stderr,
                    b"Warning: Unhandled subcode %x at %x\n\x00" as *const u8
                        as *const libc::c_char,
                    *p.offset(len as isize) as libc::c_int, ip);
        }
        len += 1;
        (*instr).set_vex_reg(!(*p.offset(len as isize) as libc::c_int >>
            3i32 & 7i32) as
            libc::c_uint);
        (*instr).set_vex_256(if *p.offset(len as isize) as libc::c_int &
            4i32 != 0 {
            1i32
        } else { 0i32 });
        if *p.offset(len as isize) as libc::c_int & 3i32 ==
            3i32 {
            *instr =
                crate::src::x86_instr::Instruction {
                    prefix:

                    ((*instr).prefix as libc::c_int | 0x40i32) as word,
                    ..*instr
                }
        } else if *p.offset(len as isize) as libc::c_int & 3i32 ==
            2i32 {
            *instr =
                crate::src::x86_instr::Instruction {
                    prefix:

                    ((*instr).prefix as libc::c_int | 0x80i32) as word,
                    ..*instr
                }
        } else if *p.offset(len as isize) as libc::c_int & 3i32 ==
            1i32 {
            *instr =
                crate::src::x86_instr::Instruction {
                    prefix:

                    ((*instr).prefix as libc::c_int | 0x8i32) as word,
                    ..*instr
                }
        }
        len +=
            get_sse_single(subcode,
                           *p.offset((len + 1i32) as isize),
                           instr)
    } else if opcode as libc::c_int == 0xc5i32 &&
        *p.offset((len + 1i32) as isize) as libc::c_int
            >> 6i32 == 3i32 &&
        bits != 16i32 {
        len += 1;
        (*instr).set_vex(1i32);
        (*instr).set_vex_reg(!(*p.offset(len as isize) as libc::c_int >>
            3i32 & 7i32) as
            libc::c_uint);
        (*instr).set_vex_256(if *p.offset(len as isize) as libc::c_int &
            4i32 != 0 {
            1i32
        } else { 0i32 });
        if *p.offset(len as isize) as libc::c_int & 3i32 ==
            3i32 {
            *instr =
                crate::src::x86_instr::Instruction {
                    prefix:

                    ((*instr).prefix as libc::c_int | 0x40i32) as word,
                    ..*instr
                }
        } else if *p.offset(len as isize) as libc::c_int & 3i32 ==
            2i32 {
            *instr =
                crate::src::x86_instr::Instruction {
                    prefix:

                    ((*instr).prefix as libc::c_int | 0x80i32) as word,
                    ..*instr
                }
        } else if *p.offset(len as isize) as libc::c_int & 3i32 ==
            1i32 {
            *instr =
                crate::src::x86_instr::Instruction {
                    prefix:

                    ((*instr).prefix as libc::c_int | 0x8i32) as word,
                    ..*instr
                }
        }
        len += 1;
        len += get_0f_instr(p.offset(len as isize), instr)
    } else if bits == 64i32 &&
        instructions64[opcode as
            usize].name[0usize] as
            libc::c_int != 0 {
        *instr = crate::src::x86_instr::Instruction { op: instructions64[opcode as usize], ..*instr }
    } else if bits != 64i32 &&
        instructions[opcode as
            usize].name[0usize] as
            libc::c_int != 0 {
        *instr = crate::src::x86_instr::Instruction { op: instructions[opcode as usize], ..*instr }
    } else {
        let mut subcode_0 =

            (*p.offset((len + 1i32) as isize) as libc::c_int >>
                3i32 & 7i32) as byte;
        /* do we have a member of an instruction group? */
        if opcode as libc::c_int == 0xfi32 {
            len += 1;
            len += get_0f_instr(p.offset(len as isize), instr)
        } else if opcode as libc::c_int >= 0xd8i32 &&
            opcode as libc::c_int <= 0xdfi32 {
            len += get_fpu_instr(p.offset(len as isize), &mut (*instr).op)
        } else {
            let mut i = 0u32;
            while (i as libc::c_ulong) <
                (::std::mem::size_of::<[Operation; 108]>() as
                    libc::c_ulong).wrapping_div(::std::mem::size_of::<Operation>()
                    as libc::c_ulong) {
                if opcode as libc::c_int ==
                    instructions_group[i as usize].opcode as libc::c_int &&
                    subcode_0 as libc::c_int ==
                        instructions_group[i as usize].subcode as
                            libc::c_int {
                    *instr =
                        crate::src::x86_instr::Instruction {
                            op:
                            instructions_group[i as usize],
                            ..*instr
                        };
                    break;
                } else { i = i.wrapping_add(1) }
            }
        }
        /* if we get here and we haven't found a suitable instruction,
         * we ran into something unused (or inadequately documented) */
        if (*instr).op.name[0usize] == 0 {
            /* supply some default values so we can keep parsing */
            strcpy((*instr).op.name.as_mut_ptr(),
                   b"?\x00" as *const u8 as
                       *const libc::c_char); /* less arrogant than objdump's (bad) */


            (*instr).op =
                crate::src::x86_instr::Operation {
                    subcode: subcode_0,
                    size: 0i8,
                    arg0: NONE,
                    arg1: NONE,
                    ..
                    (*instr).op
                };

            (*instr).op = crate::src::x86_instr::Operation {
                flags: 0u32,
                ..
                (*instr).op
            }
        }
    }
    len += 1;
    /* resolve the size */
    if (*instr).op.size as libc::c_int == -(1i32) {
        if (*instr).prefix as libc::c_int & 0x8i32 != 0 {
            (*instr).op =
                crate::src::x86_instr::Operation {
                    size:

                    if bits == 16i32 {
                        32i32
                    } else { 16i32 } as libc::c_char,
                    ..
                    (*instr).op
                }
        } else if (*instr).prefix as libc::c_int & 0x8000i32 != 0
        {
            (*instr).op = crate::src::x86_instr::Operation {
                size: 64i8,
                ..
                (*instr).op
            }
        } else if (*instr).op.flags &
            (0x100i32 | 0x8i32) as
                libc::c_uint != 0 {
            (*instr).op = crate::src::x86_instr::Operation {
                size: bits as libc::c_char,
                ..
                (*instr).op
            }
        } else {
            (*instr).op =
                crate::src::x86_instr::Operation {
                    size:

                    if bits == 16i32 {
                        16i32
                    } else { 32i32 } as libc::c_char,
                    ..
                    (*instr).op
                }
        }
    }
    if (*instr).prefix as libc::c_int & 0x10i32 != 0 {
        *instr =
            crate::src::x86_instr::Instruction {
                addrsize:

                if bits == 32i32 {
                    16i32
                } else { 32i32 } as byte,
                ..*instr
            }
    } else { *instr = crate::src::x86_instr::Instruction { addrsize: bits as byte, ..*instr } }
    /* figure out what arguments we have */
    if (*instr).op.arg0 as u64 != 0 {
        let mut base = len;


        /* The convention is that an arg whose value is one or more bytes has
         * IP pointing to that value, but otherwise it points to the beginning
         * of the instruction. This way, we'll never think that e.g. a register
         * value is supposed to be relocated. */


        (*instr).args[0usize] =
            crate::src::x86_instr::Argument {
                type_0:
                (*instr).op.arg0,
                ..
                (*instr).args[0usize]
            };
        (*instr).args[1usize] =
            crate::src::x86_instr::Argument {
                type_0:
                (*instr).op.arg1,
                ..
                (*instr).args[1usize]
            };
        (*instr).args[2usize] =
            crate::src::x86_instr::Argument {
                ip,
                ..
                (*instr).args[2usize]
            };
        (*instr).args[1usize] =
            crate::src::x86_instr::Argument {
                ip:


                (*instr).args[2usize].ip,
                ..
                (*instr).args[1usize]
            };
        (*instr).args[0usize] =
            crate::src::x86_instr::Argument {
                ip:


                (*instr).args[1usize].ip,
                ..
                (*instr).args[0usize]
            };
        len +=
            get_arg(ip.wrapping_add(len as libc::c_uint),
                    &*p.offset(len as isize),
                    &mut *(*instr).args.as_mut_ptr().offset(0isize),
                    instr, bits);
        /* registers that read from the modrm byte, which we might have just processed */
        if (*instr).op.arg1 >=

            REG &&

            (*instr).op.arg1 <=

                TR32 {
            len +=
                get_arg(ip.wrapping_add(len as libc::c_uint),
                        &*p.offset(base as isize),
                        &mut *(*instr).args.as_mut_ptr().offset(1isize),
                        instr, bits)
        } else {
            len +=
                get_arg(ip.wrapping_add(len as libc::c_uint),
                        &*p.offset(len as isize),
                        &mut *(*instr).args.as_mut_ptr().offset(1isize),
                        instr, bits)
        }
        /* arg2 */
        if (*instr).op.flags & 0x1u32 != 0 {
            (*instr).args[2usize] = crate::src::x86_instr::Argument {
                type_0: IMM,
                ..
                (*instr).args[2usize]
            }
        } else if (*instr).op.flags & 0x2u32 != 0
        {
            (*instr).args[2usize] = crate::src::x86_instr::Argument {
                type_0: IMM8,
                ..
                (*instr).args[2usize]
            }
        } else if (*instr).op.flags & 0x4u32 != 0
        {
            (*instr).args[2usize] = crate::src::x86_instr::Argument {
                type_0: CL,
                ..
                (*instr).args[2usize]
            }
        }
        len +=
            get_arg(ip.wrapping_add(len as libc::c_uint),
                    &*p.offset(len as isize),
                    &mut *(*instr).args.as_mut_ptr().offset(2isize),
                    instr, bits)
    }
    /* modify the instruction name if appropriate */
    if asm_syntax == GAS {
        if (*instr).op.opcode as libc::c_int == 0xfb6i32 {
            strcpy((*instr).op.name.as_mut_ptr(),
                   b"movzb\x00" as *const u8 as *const libc::c_char);
            suffix_name(instr);
        } else if (*instr).op.opcode as libc::c_int == 0xfb7i32 {
            strcpy((*instr).op.name.as_mut_ptr(),
                   b"movzw\x00" as *const u8 as *const libc::c_char);
            suffix_name(instr);
        } else if (*instr).op.opcode as libc::c_int == 0xfbei32 {
            strcpy((*instr).op.name.as_mut_ptr(),
                   b"movsb\x00" as *const u8 as *const libc::c_char);
            suffix_name(instr);
        } else if (*instr).op.opcode as libc::c_int == 0xfbfi32 {
            strcpy((*instr).op.name.as_mut_ptr(),
                   b"movsw\x00" as *const u8 as *const libc::c_char);
            suffix_name(instr);
        } else if (*instr).op.opcode as libc::c_int == 0x63i32 &&
            bits == 64i32 {
            strcpy((*instr).op.name.as_mut_ptr(),
                   b"movslq\x00" as *const u8 as *const libc::c_char);
        }
    }
    if (*instr).op.flags & 0x100u32 != 0 &&
        (*instr).prefix as libc::c_int & 0x8i32 != 0 {
        suffix_name(instr);
    } else if (*instr).op.flags & 0x200u32 != 0 &&

        asm_syntax !=

            GAS {
        suffix_name(instr);
    } else if (*instr).op.opcode as libc::c_int == 0x98i32 {
        strcpy((*instr).op.name.as_mut_ptr(),
               if (*instr).op.size as libc::c_int == 16i32 {
                   b"cbw\x00" as *const u8 as *const libc::c_char
               } else if (*instr).op.size as libc::c_int == 32i32
               {
                   b"cwde\x00" as *const u8 as *const libc::c_char
               } else { b"cdqe\x00" as *const u8 as *const libc::c_char });
    } else if (*instr).op.opcode as libc::c_int == 0x99i32 {
        strcpy((*instr).op.name.as_mut_ptr(),
               if (*instr).op.size as libc::c_int == 16i32 {
                   b"cwd\x00" as *const u8 as *const libc::c_char
               } else if (*instr).op.size as libc::c_int == 32i32
               {
                   b"cdq\x00" as *const u8 as *const libc::c_char
               } else { b"cqo\x00" as *const u8 as *const libc::c_char });
    } else if (*instr).op.opcode as libc::c_int == 0xe3i32 &&
        (*instr).prefix as libc::c_int & 0x10i32 != 0 {
        strcpy((*instr).op.name.as_mut_ptr(),
               b"jecxz\x00" as *const u8 as *const libc::c_char);
    } else if (*instr).op.opcode as libc::c_int == 0xd4i32 &&
        (*instr).args[0usize].value ==
            10u64 {
        strcpy((*instr).op.name.as_mut_ptr(),
               b"aam\x00" as *const u8 as *const libc::c_char);

        (*instr).op = crate::src::x86_instr::Operation {
            arg0: NONE,
            ..
            (*instr).op
        }
    } else if (*instr).op.opcode as libc::c_int == 0xd5i32 &&
        (*instr).args[0usize].value ==
            10u64 {
        strcpy((*instr).op.name.as_mut_ptr(),
               b"aad\x00" as *const u8 as *const libc::c_char);

        (*instr).op = crate::src::x86_instr::Operation {
            arg0: NONE,
            ..
            (*instr).op
        }
    } else if (*instr).op.opcode as libc::c_int == 0xfc7i32 &&
        (*instr).op.subcode as libc::c_int == 1i32 &&
        (*instr).prefix as libc::c_int & 0x8000i32 != 0
    {
        strcpy((*instr).op.name.as_mut_ptr(),
               b"cmpxchg16b\x00" as *const u8 as *const libc::c_char);
    } else if asm_syntax == GAS
    {
        if (*instr).op.flags & 0x400u32 != 0 {
            memmove((*instr).op.name.as_mut_ptr().offset(1isize) as
                        *mut libc::c_void,
                    (*instr).op.name.as_mut_ptr() as *const libc::c_void,
                    strlen((*instr).op.name.as_mut_ptr()));
            (*instr).op.name[0usize] =

                'l' as libc::c_char
        } else if is_reg((*instr).op.arg0) == 0 &&
            is_reg((*instr).op.arg1) == 0 &&

            (*instr).modrm_disp !=

                DISP_REG {
            suffix_name(instr);
        }
    } else if asm_syntax != GAS
        &&
        ((*instr).op.opcode as libc::c_int == 0xcai32 ||
            (*instr).op.opcode as libc::c_int ==
                0xcbi32) {
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
                                     mut flags: byte, mut instr: *mut Instruction,
                                     mut comment: *const libc::c_char,
                                     mut bits: libc::c_int) {
    let mut i = 0;
    /* FIXME: now that we've had to add bits to this function, get rid of ip_string */
    /* get the arguments */
    print_arg(ip, instr, 0i32, bits);
    print_arg(ip, instr, 1i32, bits);
    print_arg(ip, instr, 2i32, bits);
    /* did we find too many prefixes? */
    if get_prefix((*instr).op.opcode, bits) != 0 {
        if get_prefix((*instr).op.opcode, bits) as libc::c_int &
            0x7i32 != 0 {
            fprintf(stderr,
                    b"Warning: %s: \x00" as *const u8 as *const libc::c_char,
                    ip);
            fprintf(stderr,
                    b"Multiple segment prefixes found: %s, %s. Skipping to next instruction.\n\x00"
                        as *const u8 as *const libc::c_char,
                    seg16[(((*instr).prefix as libc::c_int &
                        0x7i32) - 1i32) as
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
        (*instr).op.name[0usize] =
            0i8
    }
    /* check that the instruction exists */
    if (*instr).op.name[0usize] as libc::c_int ==
        '?' as i32 {
        fprintf(stderr,
                b"Warning: %s: \x00" as *const u8 as *const libc::c_char, ip);
        fprintf(stderr,
                b"Unknown opcode 0x%02x (extension %d)\n\x00" as *const u8 as
                    *const libc::c_char, (*instr).op.opcode as libc::c_int,
                (*instr).op.subcode as libc::c_int);
    }
    /* okay, now we begin dumping */
    if flags as libc::c_int & 0x4i32 != 0 &&
        opts as libc::c_int & 0x10i32 != 0 {
        /* output a label, which is like an address but without the segment prefix */
        /* FIXME: check masm */
        if asm_syntax == NASM {
            printf(b".\x00" as *const u8 as *const libc::c_char);
        }
        printf(b"%s:\x00" as *const u8 as *const libc::c_char, ip);
    }
    if opts as libc::c_int & 0x8i32 == 0 {
        printf(b"%s:\x00" as *const u8 as *const libc::c_char, ip);
    }
    printf(b"\t\x00" as *const u8 as *const libc::c_char);
    if opts as libc::c_int & 0x4i32 == 0 {
        i = 0i32;
        while i < len && i < 7i32 {
            printf(b"%02x \x00" as *const u8 as *const libc::c_char,
                   *p.offset(i as isize) as libc::c_int);
            i += 1
        }
        while i < 8i32 {
            printf(b"   \x00" as *const u8 as *const libc::c_char);
            i += 1
        }
    }
    /* mark instructions that are jumped to */
    if flags as libc::c_int & 0x4i32 != 0 &&
        opts as libc::c_int & 0x10i32 == 0 {
        printf(if flags as libc::c_int & 0x10i32 != 0 {
            b">>\x00" as *const u8 as *const libc::c_char
        } else { b" >\x00" as *const u8 as *const libc::c_char });
    } else { printf(b"  \x00" as *const u8 as *const libc::c_char); }
    /* print prefixes, including (fake) prefixes if ours are invalid */
    if (*instr).prefix as libc::c_int & 0x7i32 != 0 {
        /* note: is it valid to use overrides with lods and outs? */
        if (*instr).usedmem() == 0 ||
            ((*instr).op.arg0 ==

                ESDI ||

                (*instr).op.arg1 ==

                    ESDI &&

                    (*instr).op.arg0 !=

                        DSSI) {
            /* can't be overridden */
            fprintf(stderr,
                    b"Warning: %s: \x00" as *const u8 as *const libc::c_char,
                    ip);
            fprintf(stderr,
                    b"Segment prefix %s used with opcode 0x%02x %s\n\x00" as
                        *const u8 as *const libc::c_char,
                    seg16[(((*instr).prefix as libc::c_int &
                        0x7i32) - 1i32) as
                        usize].as_ptr(),
                    (*instr).op.opcode as libc::c_int,
                    (*instr).op.name.as_mut_ptr());
            printf(b"%s \x00" as *const u8 as *const libc::c_char,
                   seg16[(((*instr).prefix as libc::c_int &
                       0x7i32) - 1i32) as
                       usize].as_ptr());
        }
    }
    if (*instr).prefix as libc::c_int & 0x8i32 != 0 &&
        (*instr).op.size as libc::c_int != 16i32 &&
        (*instr).op.size as libc::c_int != 32i32 {
        fprintf(stderr,
                b"Warning: %s: \x00" as *const u8 as *const libc::c_char, ip);
        fprintf(stderr,
                b"Operand-size override used with opcode 0x%02x %s\n\x00" as
                    *const u8 as *const libc::c_char,
                (*instr).op.opcode as libc::c_int,
                (*instr).op.name.as_mut_ptr());
        printf(if asm_syntax ==

            GAS {
            b"data32 \x00" as *const u8 as *const libc::c_char
        } else { b"o32 \x00" as *const u8 as *const libc::c_char });
        /* fixme: how should MASM print it? */
    }
    if (*instr).prefix as libc::c_int & 0x10i32 != 0 &&

        asm_syntax == NASM
        && (*instr).op.flags & 0x200u32 != 0 {
        printf(b"a32 \x00" as *const u8 as *const libc::c_char);
    } else if (*instr).prefix as libc::c_int & 0x10i32 != 0 &&
        (*instr).usedmem() == 0 &&
        (*instr).op.opcode as libc::c_int != 0xe3i32 {
        /* jecxz */
        fprintf(stderr,
                b"Warning: %s: \x00" as *const u8 as *const libc::c_char, ip);
        fprintf(stderr,
                b"Address-size prefix used with opcode 0x%02x %s\n\x00" as
                    *const u8 as *const libc::c_char,
                (*instr).op.opcode as libc::c_int,
                (*instr).op.name.as_mut_ptr());
        printf(if asm_syntax ==

            GAS {
            b"addr32 \x00" as *const u8 as *const libc::c_char
        } else { b"a32 \x00" as *const u8 as *const libc::c_char });
        /* fixme: how should MASM print it? */
    }
    if (*instr).prefix as libc::c_int & 0x20i32 != 0 {
        if (*instr).op.flags & 0x80u32 == 0 {
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
    if (*instr).prefix as libc::c_int & 0x40i32 != 0 {
        if (*instr).op.flags & 0x10u32 == 0 {
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
    if (*instr).prefix as libc::c_int & 0x80i32 != 0 {
        if (*instr).op.flags & 0x20u32 == 0 {
            fprintf(stderr,
                    b"Warning: %s: \x00" as *const u8 as *const libc::c_char,
                    ip);
            fprintf(stderr,
                    b"repe prefix used with opcode 0x%02x %s\n\x00" as
                        *const u8 as *const libc::c_char,
                    (*instr).op.opcode as libc::c_int,
                    (*instr).op.name.as_mut_ptr());
        }
        printf(if (*instr).op.flags & 0x10u32 != 0
        {
            b"repe \x00" as *const u8 as *const libc::c_char
        } else { b"rep \x00" as *const u8 as *const libc::c_char });
    }
    if (*instr).prefix as libc::c_int & 0x100i32 != 0 {
        printf(b"wait \x00" as *const u8 as *const libc::c_char);
    }
    if (*instr).vex() != 0 {
        printf(b"v\x00" as *const u8 as *const libc::c_char);
    }
    printf(b"%s\x00" as *const u8 as *const libc::c_char,
           (*instr).op.name.as_mut_ptr());
    if (*instr).args[0usize].string[0usize] as
        libc::c_int != 0 ||
        (*instr).args[1usize].string[0usize] as
            libc::c_int != 0 {
        printf(b"\t\x00" as *const u8 as *const libc::c_char);
    }
    if asm_syntax == GAS {
        /* fixme: are all of these orderings correct? */
        if (*instr).args[1usize].string[0usize] != 0 {
            printf(b"%s,\x00" as *const u8 as *const libc::c_char,
                   (*instr).args[1usize].string.as_mut_ptr());
        }
        if (*instr).vex_reg() != 0 {
            printf(b"%%ymm%d, \x00" as *const u8 as *const libc::c_char,
                   (*instr).vex_reg() as libc::c_int);
        }
        if (*instr).args[0usize].string[0usize] != 0 {
            printf(b"%s\x00" as *const u8 as *const libc::c_char,
                   (*instr).args[0usize].string.as_mut_ptr());
        }
        if (*instr).args[2usize].string[0usize] != 0 {
            printf(b",%s\x00" as *const u8 as *const libc::c_char,
                   (*instr).args[2usize].string.as_mut_ptr());
        }
    } else {
        if (*instr).args[0usize].string[0usize] != 0 {
            printf(b"%s\x00" as *const u8 as *const libc::c_char,
                   (*instr).args[0usize].string.as_mut_ptr());
        }
        if (*instr).args[1usize].string[0usize] != 0 {
            printf(b", \x00" as *const u8 as *const libc::c_char);
        }
        if (*instr).vex_reg() != 0 {
            printf(b"ymm%d, \x00" as *const u8 as *const libc::c_char,
                   (*instr).vex_reg() as libc::c_int);
        }
        if (*instr).args[1usize].string[0usize] != 0 {
            printf(b"%s\x00" as *const u8 as *const libc::c_char,
                   (*instr).args[1usize].string.as_mut_ptr());
        }
        if (*instr).args[2usize].string[0usize] != 0 {
            printf(b", %s\x00" as *const u8 as *const libc::c_char,
                   (*instr).args[2usize].string.as_mut_ptr());
        }
    }
    if !comment.is_null() {
        printf(if asm_syntax ==

            GAS {
            b"\t// \x00" as *const u8 as *const libc::c_char
        } else { b"\t;\x00" as *const u8 as *const libc::c_char });
        printf(b" <%s>\x00" as *const u8 as *const libc::c_char, comment);
    }
    /* if we have more than 7 bytes on this line, wrap around */
    if len > 7i32 && opts as libc::c_int & 0x4i32 == 0
    {
        printf(b"\n\t\t\x00" as *const u8 as *const libc::c_char);

        for i in 7i32..len {
            printf(b"%02x\x00" as *const u8 as *const libc::c_char,
                   *p.offset(i as isize) as libc::c_int);

            if i < len {
                printf(b" \x00" as *const u8 as *const libc::c_char);
            }
        }
    }
    printf(b"\n\x00" as *const u8 as *const libc::c_char);
}
