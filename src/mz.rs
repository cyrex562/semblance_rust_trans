use ::libc;
use ::c2rust_bitfields;
extern "C" {
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    static mut stdout: *mut _IO_FILE;
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    fn _IO_getc(__fp: *mut _IO_FILE) -> libc::c_int;
    #[no_mangle]
    fn _IO_putc(__c: libc::c_int, __fp: *mut _IO_FILE) -> libc::c_int;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn fread(__ptr: *mut libc::c_void, __size: size_t, __n: size_t,
             __stream: *mut FILE) -> size_t;
    #[no_mangle]
    fn fseek(__stream: *mut FILE, __off: libc::c_long, __whence: libc::c_int)
     -> libc::c_int;
    
    
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;

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
pub type byte = uint8_t;
pub type word = uint16_t;
pub type dword = uint32_t;
pub type qword = uint64_t;
pub type C2RustUnnamed = libc::c_uint;
pub const MASM: C2RustUnnamed = 2;
pub const NASM: C2RustUnnamed = 1;
pub const GAS: C2RustUnnamed = 0;

#[repr(C)]#[derive(Copy, Clone)]
pub struct mz {
    pub header: header_mz,
    pub reltab: *mut reloc,
    pub entry_point: dword,
    pub flags: *mut byte,
    pub start: dword,
    pub length: dword,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct reloc {
    pub offset: word,
    pub segment: word,
}

#[repr(C, packed)]#[derive(Copy, Clone)]
pub struct header_mz {
    pub e_magic: word,
    pub e_cblp: word,
    pub e_cp: word,
    pub e_crlc: word,
    pub e_cparhdr: word,
    pub e_minalloc: word,
    pub e_maxalloc: word,
    pub e_ss: word,
    pub e_sp: word,
    pub e_csum: word,
    pub e_ip: word,
    pub e_cs: word,
    pub e_lfarlc: word,
    pub e_ovno: word,
}

#[repr(C)]#[derive(Copy, Clone, BitfieldStruct)]
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
pub type disptype = libc::c_uint;
/* register, i.e. mod == 3 */
/* two bytes */
pub const DISP_REG: disptype = 3;
/* one byte */
pub const DISP_16: disptype = 2;
/* no disp, i.e. mod == 0 && m != 6 */
pub const DISP_8: disptype = 1;
pub const DISP_NONE: disptype = 0;

#[repr(C)]#[derive(Copy, Clone)]
pub struct arg {
    pub string: [libc::c_char; 32],
    pub ip: dword,
    pub value: qword,
    pub type_0: argtype,
}
pub type argtype = libc::c_uint;
/* element of stack given by lowest three bytes of "modrm" */
/* top of stack aka st(0) */
pub const STX: argtype = 53;
/* test register */
/* floating point regs */
pub const ST: argtype = 52;
/* debug register */
pub const TR32: argtype = 51;
/* control register */
pub const DR32: argtype = 50;
/* 32-bit only register, used for cr/dr/tr */
pub const CR32: argtype = 49;
/* segment register */
pub const REG32: argtype = 48;
/* SSE register */
pub const SEG16: argtype = 47;
/* MMX register */
pub const XMM: argtype = 46;
/* register */
pub const MMX: argtype = 45;
/* SSE register only (not using 0x11xxxxxx is invalid) */
pub const REG: argtype = 44;
/* MMX register only (not using 0x11xxxxxx is invalid) */
pub const XMMONLY: argtype = 43;
/* register only (not using 0x11xxxxxx is invalid) */
pub const MMXONLY: argtype = 42;
/* memory only (using 0x11xxxxxx is invalid) */
pub const REGONLY: argtype = 41;
/* SSE register/memory */
pub const MEM: argtype = 40;
/* MMX register/memory */
pub const XM: argtype = 39;
/* register/memory */
pub const MM: argtype = 38;
/* to be read from ModRM, appropriately */
pub const RM: argtype = 37;
pub const ESDI: argtype = 36;
pub const DSSI: argtype = 35;
/* absolute location in memory, for A0-A3 MOV */
/* specific memory addresses for string operations */
pub const DSBX: argtype = 34;
/* absolute instruction, used for far calls/jumps */
pub const MOFFS16: argtype = 33;
/* relative to current instruction */
pub const PTR32: argtype = 32;
pub const REL16: argtype = 31;
/* immediate number */
pub const REL8: argtype = 30;
pub const IMM: argtype = 29;
pub const IMM16: argtype = 28;
/* the same as DX except GAS puts it in parentheses */
/* absolute or relative numbers, given as 1/2/4 bytes */
pub const IMM8: argtype = 27;
/* the same as AL/AX except MASM doesn't print them */
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
/* specific registers */
pub const AL: argtype = 2;
/* the literal value 1, used for bit shift ops */
pub const ONE: argtype = 1;
pub const NONE: argtype = 0;
/* branch to target (jmp, jXX) */

#[repr(C)]#[derive(Copy, Clone)]
pub struct op {
    pub opcode: word,
    pub subcode: byte,
    pub size: libc::c_char,
    pub name: [libc::c_char; 16],
    pub arg0: argtype,
    pub arg1: argtype,
    pub flags: dword,
}
#[no_mangle]
pub static mut f: *mut FILE =  0 as *mut FILE;
#[inline]
unsafe extern "C" fn read_byte() -> byte { return _IO_getc(f) as byte; }
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
#[inline]
unsafe extern "C" fn putchar(mut __c: libc::c_int) -> libc::c_int {
    return _IO_putc(__c, stdout);
}
/* MZ (aka real-mode) addresses are "segmented", but not really. Just
 * use the actual value. */
#[inline]
unsafe extern "C" fn realaddr(mut segment: word, mut offset: word) -> dword {
    if (segment as libc::c_uint) < 0xfff0u32 {
        return (segment as libc::c_int * 0x10i32 +
                    offset as libc::c_int) as dword
    } else {
        /* relative segments >= 0xfff0 really point into PSP */
        return (segment as libc::c_int * 0x10i32 +
                    offset as libc::c_int - 0x100000i32) as dword
    };
}
/*
 * MZ (DOS) files
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
unsafe extern "C" fn print_header(mut header: *mut header_mz) {
    putchar('\n' as i32); /* 0a */
    printf(b"Minimum extra allocation: %d bytes\n\x00" as *const u8 as
               *const libc::c_char,
           (*header).e_minalloc as libc::c_int * 16i32); /* 0c */
    printf(b"Maximum extra allocation: %d bytes\n\x00" as *const u8 as
               *const libc::c_char,
           (*header).e_maxalloc as libc::c_int * 16i32); /* 0e */
    printf(b"Initial stack location: %#x\n\x00" as *const u8 as
               *const libc::c_char,
           realaddr((*header).e_ss, (*header).e_sp)); /* 14 */
    printf(b"Program entry point: %#x\n\x00" as *const u8 as
               *const libc::c_char, realaddr((*header).e_cs, (*header).e_ip));
    printf(b"Overlay number: %d\n\x00" as *const u8 as *const libc::c_char,
           (*header).e_ovno as libc::c_int);
    /* 1a */
}
unsafe extern "C" fn print_mz_instr(mut ip: dword, mut p: *mut byte,
                                    mut flags: *const byte) -> libc::c_int {
    
    
    
      let mut instr =
    
        {
            let mut init =
                instr{usedmem_vex_vex_reg_vex_256: [0; 1],
                      c2rust_padding: [0; 4],
                      prefix: 0u16,
                      op:
                          op{opcode: 0,
                             subcode: 0,
                             size: 0,
                             name: [0; 16],
                             arg0: NONE,
                             arg1: NONE,
                             flags: 0,},
                      args:
                          [arg{string: [0; 32],
                               ip: 0,
                               value: 0,
                               type_0: NONE,}; 3],
                      addrsize: 0,
                      modrm_disp: DISP_NONE,
                      modrm_reg: 0,
                      sib_scale: 0,
                      sib_index: 0,};
            init.set_usedmem(0);
            init.set_vex(0);
            init.set_vex_reg(0);
            init.set_vex_256(0);
            init
        }; let mut ip_string =  [0; 7]; let mut len =
    
     crate::src::x86_instr::get_instr(ip, p, &mut instr, 16i32) as libc::c_uint;
    sprintf(ip_string.as_mut_ptr(),
            b"%05x\x00" as *const u8 as *const libc::c_char, ip);
    crate::src::x86_instr::print_instr(ip_string.as_mut_ptr(), p, len as libc::c_int,
                *flags.offset(ip as isize), &mut instr,
                0 as *const libc::c_char, 16i32);
    return len as libc::c_int;
}
unsafe extern "C" fn print_code(mut mz: *mut mz) {
    
     let mut ip =  0u32; let mut buffer =  [0; 16];
    putchar('\n' as i32);
    printf(b"Code (start = 0x%x, length = 0x%x):\n\x00" as *const u8 as
               *const libc::c_char, (*mz).start, (*mz).length);
    while ip < (*mz).length {
        fseek(f, (*mz).start.wrapping_add(ip) as libc::c_long,
              0i32);
        /* find a valid instruction */
        if *(*mz).flags.offset(ip as isize) as libc::c_int &
               0x2i32 == 0 {
            if opts as libc::c_int & 0x1i32 != 0 {
                /* still skip zeroes */
                if read_byte() as libc::c_int == 0i32 {
                    printf(b"      ...\n\x00" as *const u8 as
                               *const libc::c_char);
                    ip = ip.wrapping_add(1);
                    while read_byte() as libc::c_int == 0i32 {
                        ip = ip.wrapping_add(1)
                    }
                }
            } else {
                printf(b"     ...\n\x00" as *const u8 as *const libc::c_char);
                while ip < (*mz).length &&
                          *(*mz).flags.offset(ip as isize) as libc::c_int &
                              0x2i32 == 0 {
                    ip = ip.wrapping_add(1)
                }
            }
        }
        fseek(f, (*mz).start.wrapping_add(ip) as libc::c_long,
              0i32);
        if ip >= (*mz).length { return }
        /* fixme: disassemble everything for now; we'll try to fix it later.
         * this is going to be a little more difficult since dos executables
         * unabashedly mix code and data, so we need to figure out a solution
         * for that. but we needed to do that anyway. */
        fread(buffer.as_mut_ptr() as *mut libc::c_void,
              1u64,
              if (::std::mem::size_of::<[byte; 16]>() as libc::c_ulong) <
                     (*mz).length.wrapping_sub(ip) as libc::c_ulong {
                  ::std::mem::size_of::<[byte; 16]>() as libc::c_ulong
              } else { (*mz).length.wrapping_sub(ip) as libc::c_ulong }, f);
        if *(*mz).flags.offset(ip as isize) as libc::c_int &
               0x8i32 != 0 {
            printf(b"\n\x00" as *const u8 as *const libc::c_char);
            printf(b"%05x <no name>:\n\x00" as *const u8 as
                       *const libc::c_char, ip);
        }
        ip =
            
            (ip).wrapping_add(print_mz_instr(ip,
                                                           buffer.as_mut_ptr(),
                                                           (*mz).flags) as
                                                libc::c_uint)
    };
}
unsafe extern "C" fn scan_segment(mut ip: dword, mut mz: *mut mz) {
    
    
    
     let mut buffer =  [0; 16]; let mut instr =
    
        instr{prefix: 0,
              op:
                  op{opcode: 0,
                     subcode: 0,
                     size: 0,
                     name: [0; 16],
                     arg0: NONE,
                     arg1: NONE,
                     flags: 0,},
              args: [arg{string: [0; 32], ip: 0, value: 0, type_0: NONE,}; 3],
              addrsize: 0,
              modrm_disp: DISP_NONE,
              modrm_reg: 0,
              sib_scale: 0,
              sib_index: 0,
              usedmem_vex_vex_reg_vex_256: [0; 1],
              c2rust_padding: [0; 4],}; let mut instr_length =  0; let mut i =  0;
    if ip > (*mz).length {
        fprintf(stderr,
                b"Warning: %05x: \x00" as *const u8 as *const libc::c_char,
                ip);
        fprintf(stderr,
                b"Attempt to scan past end of segment.\n\x00" as *const u8 as
                    *const libc::c_char);
        return
    }
    if *(*mz).flags.offset(ip as isize) as libc::c_int &
           (0x2i32 | 0x1i32) == 0x1i32 {
        fprintf(stderr,
                b"Warning: %05x: \x00" as *const u8 as *const libc::c_char,
                ip);
        fprintf(stderr,
                b"Attempt to scan byte that does not begin instruction.\n\x00"
                    as *const u8 as *const libc::c_char);
    }
    while ip < (*mz).length {
        /* check if we already read from here */
        if *(*mz).flags.offset(ip as isize) as libc::c_int &
               0x1i32 != 0 {
            return
        }
        /* read the instruction */
        fseek(f, (*mz).start.wrapping_add(ip) as libc::c_long,
              0i32); // fixme
        memset(buffer.as_mut_ptr() as *mut libc::c_void, 0i32,
               ::std::mem::size_of::<[byte; 16]>() as libc::c_ulong);
        fread(buffer.as_mut_ptr() as *mut libc::c_void,
              1u64,
              if (::std::mem::size_of::<[byte; 16]>() as libc::c_ulong) <
                     (*mz).length.wrapping_sub(ip) as libc::c_ulong {
                  ::std::mem::size_of::<[byte; 16]>() as libc::c_ulong
              } else { (*mz).length.wrapping_sub(ip) as libc::c_ulong }, f);
        instr_length =
            crate::src::x86_instr::get_instr(ip, buffer.as_mut_ptr(), &mut instr, 16i32);
        /* mark the bytes */
        let ref mut fresh0 = *(*mz).flags.offset(ip as isize);
        *fresh0 = (*fresh0 as libc::c_int | 0x2i32) as byte;
        i = ip as libc::c_int;
        while (i as libc::c_uint) <
                  ip.wrapping_add(instr_length as libc::c_uint) &&
                  (i as libc::c_uint) < (*mz).length {
            let ref mut fresh1 = *(*mz).flags.offset(i as isize);
            *fresh1 = (*fresh1 as libc::c_int | 0x1i32) as byte;
            i += 1
        }
        /* instruction which hangs over the minimum allocation */
        if (i as libc::c_uint) < ip.wrapping_add(instr_length as libc::c_uint)
               && i as libc::c_uint == (*mz).length {
            break ;
        }
        /* handle conditional and unconditional jumps */
        if instr.op.flags & 0x8000u32 != 0 {
            /* near relative jump, loop, or call */
            if strcmp(instr.op.name.as_mut_ptr(),
                      b"call\x00" as *const u8 as *const libc::c_char) == 0 {
                let ref mut fresh2 =
                    *(*mz).flags.offset(instr.args[0usize].value as isize);
                *fresh2 =
                    (*fresh2 as libc::c_int | 0x8i32) as byte
            } else {
                let ref mut fresh3 =
                    *(*mz).flags.offset(instr.args[0usize].value as isize);
                *fresh3 =
                    (*fresh3 as libc::c_int | 0x4i32) as byte
            }
            /* scan it */
            scan_segment(instr.args[0usize].value as dword,
                         mz);
        }
        if instr.op.flags & 0x4000u32 != 0 {
            return
        }
        ip =
            
            (ip).wrapping_add(instr_length as libc::c_uint)
    }
    fprintf(stderr,
            b"Warning: %05x: \x00" as *const u8 as *const libc::c_char, ip);
    fprintf(stderr,
            b"Scan reached the end of segment.\n\x00" as *const u8 as
                *const libc::c_char);
}
unsafe extern "C" fn read_code(mut mz: *mut mz) {
    
     *mz =
    crate::src::mz::mz{entry_point:
                           
                        realaddr((*mz).header.e_cs, (*mz).header.e_ip),
                       length:
                           
                       
        (((*mz).header.e_cp as libc::c_int - 1i32) *
             512i32 + (*mz).header.e_cblp as libc::c_int) as
            dword, ..*mz};
    if (*mz).header.e_cblp as libc::c_int == 0i32 {
        *mz =
            
            crate::src::mz::mz{length: 
            
            ((*mz).length).wrapping_add(512u32), ..*mz}
    }
     *mz =
    crate::src::mz::mz{flags:
                           
                       
        calloc((*mz).length as libc::c_ulong,
               ::std::mem::size_of::<byte>() as libc::c_ulong) as *mut byte, ..*mz};
    if (*mz).entry_point > (*mz).length {
        fprintf(stderr,
                b"Warning: Entry point %05x exceeds segment length (%05x)\n\x00"
                    as *const u8 as *const libc::c_char, (*mz).entry_point,
                (*mz).length);
    }
    let ref mut fresh4 = *(*mz).flags.offset((*mz).entry_point as isize);
    *fresh4 = (*fresh4 as libc::c_int | 0x8i32) as byte;
    scan_segment((*mz).entry_point, mz);
}
#[no_mangle]
pub unsafe extern "C" fn readmz(mut mz: *mut mz) {
    fseek(f, 0i64, 0i32);
    fread(&mut (*mz).header as *mut header_mz as *mut libc::c_void,
          ::std::mem::size_of::<header_mz>() as libc::c_ulong,
          1u64, f);
     *mz =
    crate::src::mz::mz{reltab:
                           
                       
        malloc(((*mz).header.e_crlc as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<reloc>()
                                                    as libc::c_ulong)) as
            *mut reloc, ..*mz};
    fseek(f, (*mz).header.e_lfarlc as libc::c_long, 0i32);
    fread((*mz).reltab as *mut libc::c_void,
          ::std::mem::size_of::<reloc>() as libc::c_ulong,
          (*mz).header.e_crlc as size_t, f);
     *mz =
    crate::src::mz::mz{start:
                           
                       
        ((*mz).header.e_cparhdr as libc::c_int * 16i32) as dword, ..*mz};
    read_code(mz);
}
#[no_mangle]
pub unsafe extern "C" fn freemz(mut mz: *mut mz) {
    free((*mz).reltab as *mut libc::c_void);
    free((*mz).flags as *mut libc::c_void);
}
/* Common functions */
/* Common globals */
/* what to dump */
/* additional options */
/* Whether to print addresses relative to the image base for PE files. */
/* Entry points */
#[no_mangle]
pub unsafe extern "C" fn dumpmz() {
     let mut mz =
    
        mz{header:
               header_mz{e_magic: 0,
                         e_cblp: 0,
                         e_cp: 0,
                         e_crlc: 0,
                         e_cparhdr: 0,
                         e_minalloc: 0,
                         e_maxalloc: 0,
                         e_ss: 0,
                         e_sp: 0,
                         e_csum: 0,
                         e_ip: 0,
                         e_cs: 0,
                         e_lfarlc: 0,
                         e_ovno: 0,},
           reltab: 0 as *mut reloc,
           entry_point: 0,
           flags: 0 as *mut byte,
           start: 0,
           length: 0,};
    readmz(&mut mz);
    printf(b"Module type: MZ (DOS executable)\n\x00" as *const u8 as
               *const libc::c_char);
    if mode as libc::c_int & 0x1i32 != 0 {
        print_header(&mut mz.header);
    }
    if mode as libc::c_int & 0x10i32 != 0 { print_code(&mut mz); }
    freemz(&mut mz);
}
