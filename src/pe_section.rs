use ::libc;
use ::c2rust_bitfields;
extern "C" {
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn _IO_getc(__fp: *mut _IO_FILE) -> libc::c_int;
    #[no_mangle]
    fn _IO_putc(__c: libc::c_int, __fp: *mut _IO_FILE) -> libc::c_int;
    #[no_mangle]
    static mut stdout: *mut _IO_FILE;
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
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
    fn fseek(__stream: *mut FILE, __off: libc::c_long, __whence: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn fread(__ptr: *mut libc::c_void, __size: size_t, __n: size_t,
             __stream: *mut FILE) -> size_t;
    
    
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
/* 16 */

#[repr(C, packed)]#[derive(Copy, Clone)]
pub struct directory {
    pub address: dword,
    pub size: dword,
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

#[repr(C, packed)]#[derive(Copy, Clone, BitfieldStruct)]
pub struct reloc_pe {
    #[bitfield(name = "offset", ty = "libc::c_uint", bits = "0..=11")]
    #[bitfield(name = "type_0", ty = "libc::c_uint", bits = "12..=15")]
    pub offset_type_0: [u8; 2],
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct export {
    pub address: dword,
    pub ordinal: word,
    pub name: *mut libc::c_char,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct import_module {
    pub module: *mut libc::c_char,
    pub nametab_addr: dword,
    pub nametab: *mut *mut libc::c_char,
    pub count: libc::c_uint,
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

#[repr(C)]#[derive(Copy, Clone)]
pub union C2RustUnnamed_0 {
    pub opt32: optional_header,
    pub opt64: optional_header_pep,
}
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
#[no_mangle]
pub static mut resource_filters_count: libc::c_uint = 0;
#[no_mangle]
pub static mut resource_filters: *mut *mut libc::c_char =
    
    0 as *mut *mut libc::c_char;
#[no_mangle]
pub static mut asm_syntax: C2RustUnnamed = GAS;
#[no_mangle]
pub static mut opts: word = 0;
#[no_mangle]
pub static mut mode: word = 0;
#[inline]
unsafe extern "C" fn read_dword() -> dword {
     let mut d =  0;
    fread(&mut d as *mut dword as *mut libc::c_void,
          4u64, 1u64, f);
    return d;
}
#[inline]
unsafe extern "C" fn putchar(mut __c: libc::c_int) -> libc::c_int {
    return _IO_putc(__c, stdout);
}
#[inline]
unsafe extern "C" fn read_byte() -> byte { return _IO_getc(f) as byte; }
#[no_mangle]
pub static mut f: *mut FILE =  0 as *mut FILE;
/*
 * Functions for dumping PE code and data sections
 *
 * Copyright 2018 Zebediah Figura
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
#[no_mangle]
pub static mut pe_rel_addr: libc::c_int = -(1i32);
/* in pe_section.c */
#[no_mangle]
pub unsafe extern "C" fn addr2section(mut addr: dword, mut pe: *const pe)
 -> *mut section {
    /* Even worse than the below, some data is sensitive to which section it's in! */
    
      let mut i =   0i32;
    while i < (*pe).header.NumberOfSections as libc::c_int {
        if addr >= (*(*pe).sections.offset(i as isize)).address &&
               addr <
                   (*(*pe).sections.offset(i as
                                               isize)).address.wrapping_add((*(*pe).sections.offset(i
                                                                                                        as
                                                                                                        isize)).min_alloc)
           {
            return &mut *(*pe).sections.offset(i as isize) as *mut section
        }
        i += 1
    }
    return 0 as *mut section;
}
#[no_mangle]
pub unsafe extern "C" fn addr2offset(mut addr: dword, mut pe: *const pe)
 -> libc::c_long {
    /* Everything inside a PE file is built so that the file is read while it's
     * already loaded. Offsets aren't file offsets, they're *memory* offsets.
     * We don't want to load the whole file, so we have to search through each
     * section to figure out where in the *file* a virtual address points. */
     let mut section =  addr2section(addr, pe);
    if section.is_null() { return 0i64 }
    return addr.wrapping_sub((*section).address).wrapping_add((*section).offset)
               as libc::c_long;
}
/* index function */
unsafe extern "C" fn get_export_name(mut ip: dword, mut pe: *const pe)
 -> *mut libc::c_char {
    
      let mut i =   0i32;
    while (i as libc::c_uint) < (*pe).export_count {
        if (*(*pe).exports.offset(i as isize)).address == ip {
            return (*(*pe).exports.offset(i as isize)).name
        }
        i += 1
    }
    return 0 as *mut libc::c_char;
}
unsafe extern "C" fn get_imported_name(mut offset: dword, mut pe: *const pe)
 -> *mut libc::c_char {
    
    offset =
        
        (offset as libc::c_ulong).wrapping_sub((*pe).imagebase) as
            dword;
      let mut i =   0u32;
    while i < (*pe).import_count {
         let mut index =
    
            (offset.wrapping_sub((*(*pe).imports.offset(i as
                                                            isize)).nametab_addr)
                 as
                 libc::c_ulong).wrapping_div((if (*pe).magic as libc::c_int ==
                                                     0x10bi32 {
                                                  ::std::mem::size_of::<dword>()
                                                      as libc::c_ulong
                                              } else {
                                                  ::std::mem::size_of::<qword>()
                                                      as libc::c_ulong
                                              })) as libc::c_uint;
        if index < (*(*pe).imports.offset(i as isize)).count {
            return *(*(*pe).imports.offset(i as
                                               isize)).nametab.offset(index as
                                                                          isize)
        }
        i = i.wrapping_add(1)
    }
    return 0 as *mut libc::c_char;
}
/* index function */
unsafe extern "C" fn get_reloc(mut ip: dword, mut pe: *const pe)
 -> *const reloc_pe {
     /* not even a real relocation, just padding */
      let mut i =   0u32;
    while i < (*pe).reloc_count {
        if (*pe).reloc_base.wrapping_add((*(*pe).relocs.offset(i as
                                                                   isize)).offset())
               == ip {
            return &mut *(*pe).relocs.offset(i as isize) as *mut reloc_pe
        }
        i = i.wrapping_add(1)
    }
    fprintf(stderr, b"Warning: %x: \x00" as *const u8 as *const libc::c_char,
            ip);
    fprintf(stderr,
            b"Byte tagged INSTR_RELOC has no reloc; this is a bug.\n\x00" as
                *const u8 as *const libc::c_char);
    return 0 as *const reloc_pe;
}
unsafe extern "C" fn relocate_arg(mut instr: *mut instr, mut arg: *mut arg,
                                  mut pe: *const pe) -> *mut libc::c_char {
     let mut r =  get_reloc((*arg).ip, pe);
    static mut comment: [libc::c_char; 10] = [0; 10];
    if (*r).type_0() as libc::c_int == 0i32 {
        return 0 as *mut libc::c_char
    } else {
        if (*r).type_0() as libc::c_int == 3i32 {
            if  (*arg).type_0 ==
                   
                   IMM ||
                   
                   (*arg).type_0 ==
                       
                       RM &&
                       (*instr).modrm_reg as libc::c_int ==
                           -(1i32) ||
                   
                   (*arg).type_0 ==
                       
                       MOFFS16 {
                snprintf(comment.as_mut_ptr(),
                         10u64,
                         b"%lx\x00" as *const u8 as *const libc::c_char,
                         (*arg).value.wrapping_sub((*pe).c2rust_unnamed.opt32.ImageBase
                                                       as libc::c_ulong));
                return comment.as_mut_ptr()
            }
        }
    }
    return 0 as *mut libc::c_char;
}
unsafe extern "C" fn print_pe_instr(mut sec: *const section, mut ip: dword,
                                    mut p: *mut byte, mut pe: *const pe)
 -> libc::c_int {
    
    
    
    
    
    
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
        }; let mut comment =  0 as *mut libc::c_char; let mut ip_string =  [0; 17]; let mut absip =  ip as qword; let mut bits =
    
        if (*pe).magic as libc::c_int == 0x10bi32 {
            32i32
        } else { 64i32 }; let mut comment_str =  [0; 10];
    if pe_rel_addr == 0 {
        absip =
            
            (absip).wrapping_add((*pe).imagebase)
    }
      let mut len =
    
     crate::src::x86_instr::get_instr(ip, p, &mut instr, bits) as libc::c_uint;
    sprintf(ip_string.as_mut_ptr(),
            b"%8lx\x00" as *const u8 as *const libc::c_char, absip);
    /* Check for relocations and imported names. PE separates the two concepts:
     * imported names are done by jumping into a block in .idata which is
     * relocated, and relocations proper are scattered throughout code sections
     * and relocated according to the contents of .reloc. */
    if instr.op.opcode as libc::c_int == 0xffi32 &&
           (instr.op.subcode as libc::c_int == 2i32 ||
                instr.op.subcode as libc::c_int == 4i32) &&
           
           instr.modrm_disp ==
               
               DISP_16 &&
           instr.modrm_reg as libc::c_int == -(1i32) {
        /* call/jmp to an absolute memory address */
        comment =
            get_imported_name(instr.args[0usize].value as
                                  dword, pe)
    }
    /* check if we are referencing a named export */
    if  instr.op.arg0 ==  REL16
           && comment.is_null() {
        comment =
            get_export_name(instr.args[0usize].value as
                                dword, pe);
        if comment.is_null() {
            /* Sometimes we have TWO levels of indirection—call to jmp to
             * relocated address. mingw-w64 does this. */
            fseek(f,
                  addr2offset(instr.args[0usize].value as
                                  dword, pe), 0i32);
            if read_byte() as libc::c_int == 0xffi32 &&
                   read_byte() as libc::c_int == 0x25i32 {
                /* absolute jmp */
                comment = get_imported_name(read_dword(), pe)
            }
        }
    }
    /* Not an import or an export. Is it a regular relocation? If so, adjust the
     * comment, since we prefer to print addresses relative to the image base. */
    if comment.is_null() {
        if  instr.args[0usize].type_0 != 0
               &&
               *(*sec).instr_flags.offset(instr.args[0usize].ip.wrapping_sub((*sec).address)
                                              as isize) as libc::c_int &
                   0x20i32 != 0 {
            comment =
                relocate_arg(&mut instr,
                             &mut *instr.args.as_mut_ptr().offset(0isize),
                             pe)
        }
        if  instr.args[1usize].type_0 != 0
               &&
               *(*sec).instr_flags.offset(instr.args[1usize].ip.wrapping_sub((*sec).address)
                                              as isize) as libc::c_int &
                   0x20i32 != 0 {
            comment =
                relocate_arg(&mut instr,
                             &mut *instr.args.as_mut_ptr().offset(1isize),
                             pe)
        }
    }
    /* 64-bit does it with IP-relative addressing. */
    if comment.is_null() &&
           instr.modrm_reg as libc::c_int == 16i32 {
         let mut tip =  0;
        
        if  instr.args[0usize].type_0 >=
               
               RM &&
               
               instr.args[0usize].type_0 <=
                   
                   MEM {
            tip =
                (ip.wrapping_add(len) as
                     libc::c_ulong).wrapping_add(instr.args[0usize].value)
                    as dword
        } else {
            tip =
                (ip.wrapping_add(len) as
                     libc::c_ulong).wrapping_add(instr.args[1usize].value)
                    as dword
        }
          let mut abstip =   tip as qword;
        if pe_rel_addr == 0 {
            abstip =
                
                (abstip).wrapping_add((*pe).imagebase)
        }
        comment =
            get_imported_name((tip as
                                   libc::c_ulong).wrapping_add((*pe).imagebase)
                                  as dword, pe);
        if comment.is_null() { comment = get_export_name(tip, pe) }
        if comment.is_null() {
            snprintf(comment_str.as_mut_ptr(),
                     10u64,
                     b"%lx\x00" as *const u8 as *const libc::c_char, abstip);
            comment = comment_str.as_mut_ptr()
        }
    }
    /* We deal in relative addresses internally everywhere. That means we have
     * to fix up the values for relative jumps if we're not displaying relative
     * addresses. */
    if (instr.op.arg0 ==  REL8
            ||
            
            instr.op.arg0 ==
                
                REL16) && pe_rel_addr == 0 {
        
        instr.args[0usize] =
            
            crate::src::pe_section::arg{value:
                                
            
            (instr.args[0usize].value).wrapping_add((*pe).imagebase),
                                                                                          ..
        instr.args[0usize]}
    }
    crate::src::x86_instr::print_instr(ip_string.as_mut_ptr(), p, len as libc::c_int,
                *(*sec).instr_flags.offset(ip.wrapping_sub((*sec).address) as
                                               isize), &mut instr, comment,
                bits);
    return len as libc::c_int;
}
unsafe extern "C" fn print_disassembly(mut sec: *const section,
                                       mut pe: *const pe) {
    
    
    
     let mut relip =  0u32; let mut ip =  0; let mut absip =  0; let mut buffer =  [0; 16];
    while relip < (*sec).length && relip < (*sec).min_alloc {
        fseek(f, (*sec).offset.wrapping_add(relip) as libc::c_long,
              0i32);
        /* find a valid instruction */
        if *(*sec).instr_flags.offset(relip as isize) as libc::c_int &
               0x2i32 == 0 {
            if opts as libc::c_int & 0x1i32 != 0 {
                /* still skip zeroes */
                if read_byte() as libc::c_int == 0i32 {
                    printf(b"     ...\n\x00" as *const u8 as
                               *const libc::c_char);
                    relip = relip.wrapping_add(1);
                    while read_byte() as libc::c_int == 0i32 {
                        relip = relip.wrapping_add(1)
                    }
                }
            } else {
                printf(b"     ...\n\x00" as *const u8 as *const libc::c_char);
                while relip < (*sec).length && relip < (*sec).min_alloc &&
                          *(*sec).instr_flags.offset(relip as isize) as
                              libc::c_int & 0x2i32 == 0 {
                    relip = relip.wrapping_add(1)
                }
            }
        }
        ip = relip.wrapping_add((*sec).address);
        fseek(f, (*sec).offset.wrapping_add(relip) as libc::c_long,
              0i32);
        if relip >= (*sec).length || relip >= (*sec).min_alloc { return }
        /* Instructions can "hang over" the end of a segment.
         * Zero should be supplied. */
        memset(buffer.as_mut_ptr() as *mut libc::c_void, 0i32,
               ::std::mem::size_of::<[byte; 16]>() as libc::c_ulong);
        fread(buffer.as_mut_ptr() as *mut libc::c_void,
              1u64,
              if (::std::mem::size_of::<[byte; 16]>() as libc::c_ulong) <
                     (*sec).length.wrapping_sub(relip) as libc::c_ulong {
                  ::std::mem::size_of::<[byte; 16]>() as libc::c_ulong
              } else { (*sec).length.wrapping_sub(relip) as libc::c_ulong },
              f);
        absip = ip as qword;
        if pe_rel_addr == 0 {
            absip =
                
                (absip).wrapping_add((*pe).imagebase)
        }
        if *(*sec).instr_flags.offset(relip as isize) as libc::c_int &
               0x8i32 != 0 {
             let mut name =  get_export_name(ip, pe);
            printf(b"\n\x00" as *const u8 as *const libc::c_char);
            printf(b"%lx <%s>:\n\x00" as *const u8 as *const libc::c_char,
                   absip,
                   if !name.is_null() {
                       name
                   } else {
                       b"no name\x00" as *const u8 as *const libc::c_char
                   });
        }
        relip =
            
            (relip).wrapping_add(print_pe_instr(sec, ip,
                                                           buffer.as_mut_ptr(),
                                                           pe) as
                                                libc::c_uint)
    }
    putchar('\n' as i32);
}
unsafe extern "C" fn print_data(mut sec: *const section, mut pe: *mut pe) {
    
    
    /* Page alignment means that (contrary to NE) sections are going to end with
     * a bunch of annoying zeroes. So don't read past the minimum allocation. */
    
      let mut absip =  0; let mut length =
    
        if (*sec).length < (*sec).min_alloc {
            (*sec).length
        } else { (*sec).min_alloc }; let mut relip =   0u32;
    while relip < length {
        
         let mut row =  [0; 16]; let mut len =
    
            if length.wrapping_sub(relip) < 16u32
               {
                length.wrapping_sub(relip)
            } else { 16u32 } as libc::c_int;
        
        fseek(f, (*sec).offset.wrapping_add(relip) as libc::c_long,
              0i32);
        fread(row.as_mut_ptr() as *mut libc::c_void,
              ::std::mem::size_of::<byte>() as libc::c_ulong, len as size_t,
              f);
        absip = relip.wrapping_add((*sec).address) as qword;
        if pe_rel_addr == 0 {
            absip =
                
                (absip).wrapping_add((*pe).imagebase)
        }
        printf(b"%8lx\x00" as *const u8 as *const libc::c_char, absip);
          let mut i =   0i32;
        while i < 16i32 {
            if i < len {
                printf(b" %02x\x00" as *const u8 as *const libc::c_char,
                       row[i as usize] as libc::c_int);
            } else { printf(b"   \x00" as *const u8 as *const libc::c_char); }
            i += 1
        }
        printf(b"  \x00" as *const u8 as *const libc::c_char);
        
         for i in  0i32.. len {
    
            if row[i as usize] as libc::c_int >= ' ' as i32 &&
                   row[i as usize] as libc::c_int <= '~' as i32 {
                putchar(row[i as usize] as libc::c_int);
            } else { putchar('.' as i32); }
}
        putchar('\n' as i32);
        relip =
            
            (relip).wrapping_add(16u32)
    };
}
unsafe extern "C" fn scan_segment(mut ip: dword, mut pe: *mut pe) {
    
    
    
    
    
     let mut sec =  addr2section(ip, pe); let mut buffer =  [0; 16]; let mut instr =
    
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
    //    fprintf(stderr, "scanning at %x, in section %s\n", ip, sec ? sec->name : "<none>");
    if sec.is_null() {
        fprintf(stderr,
                b"Warning: %x: \x00" as *const u8 as *const libc::c_char, ip);
        fprintf(stderr,
                b"Attempt to scan byte not in image.\n\x00" as *const u8 as
                    *const libc::c_char);
        return
    }
      let mut relip =   ip.wrapping_sub((*sec).address);
    if *(*sec).instr_flags.offset(relip as isize) as libc::c_int &
           (0x2i32 | 0x1i32) == 0x1i32 {
        fprintf(stderr,
                b"Warning: %x: \x00" as *const u8 as *const libc::c_char, ip);
        fprintf(stderr,
                b"Attempt to scan byte that does not begin instruction.\n\x00"
                    as *const u8 as *const libc::c_char);
    }
    /* This code assumes that one stretch of code won't span multiple sections.
     * Is this a valid assumption? */
    while relip < (*sec).length {
        /* check if we've already read from here */
        if *(*sec).instr_flags.offset(relip as isize) as libc::c_int &
               0x1i32 != 0 {
            return
        }
        /* read the instruction */
        fseek(f, (*sec).offset.wrapping_add(relip) as libc::c_long,
              0i32);
        memset(buffer.as_mut_ptr() as *mut libc::c_void, 0i32,
               ::std::mem::size_of::<[byte; 16]>() as libc::c_ulong);
        fread(buffer.as_mut_ptr() as *mut libc::c_void,
              1u64,
              if (::std::mem::size_of::<[byte; 16]>() as libc::c_ulong) <
                     (*sec).length.wrapping_sub(relip) as libc::c_ulong {
                  ::std::mem::size_of::<[byte; 16]>() as libc::c_ulong
              } else { (*sec).length.wrapping_sub(relip) as libc::c_ulong },
              f);
        instr_length =
            crate::src::x86_instr::get_instr(ip, buffer.as_mut_ptr(), &mut instr,
                      if (*pe).magic as libc::c_int == 0x10bi32 {
                          32i32
                      } else { 64i32 });
        /* mark the bytes */
        let ref mut fresh0 = *(*sec).instr_flags.offset(relip as isize);
        *fresh0 = (*fresh0 as libc::c_int | 0x2i32) as byte;
        i = relip as libc::c_int;
        while (i as libc::c_uint) <
                  relip.wrapping_add(instr_length as libc::c_uint) &&
                  (i as libc::c_uint) < (*sec).min_alloc {
            let ref mut fresh1 = *(*sec).instr_flags.offset(i as isize);
            *fresh1 = (*fresh1 as libc::c_int | 0x1i32) as byte;
            i += 1
        }
        /* instruction which hangs over the minimum allocation */
        if (i as libc::c_uint) <
               relip.wrapping_add(instr_length as libc::c_uint) &&
               i as libc::c_uint == (*sec).min_alloc {
            break ;
        }
        /* handle conditional and unconditional jumps */
        if instr.op.flags & 0x8000u32 != 0 {
            /* relative jump, loop, or call */
             let mut tsec = 
                addr2section(instr.args[0usize].value as
                                 dword, pe);
            if !tsec.is_null() {
                 let mut trelip =
    
                    instr.args[0usize].value.wrapping_sub((*tsec).address
                                                                 as
                                                                 libc::c_ulong)
                        as dword;
                if strcmp(instr.op.name.as_mut_ptr(),
                          b"call\x00" as *const u8 as *const libc::c_char) ==
                       0 {
                    let ref mut fresh2 =
                        *(*tsec).instr_flags.offset(trelip as isize);
                    *fresh2 =
                        (*fresh2 as libc::c_int | 0x8i32) as byte
                } else {
                    let ref mut fresh3 =
                        *(*tsec).instr_flags.offset(trelip as isize);
                    *fresh3 =
                        (*fresh3 as libc::c_int | 0x4i32) as byte
                }
                /* scan it */
                scan_segment(instr.args[0usize].value as
                                 dword, pe);
            } else {
                fprintf(stderr,
                        b"Warning: %x: \x00" as *const u8 as
                            *const libc::c_char, ip);
                fprintf(stderr,
                        b"Branch \'%s\' to byte %lx not in image.\n\x00" as
                            *const u8 as *const libc::c_char,
                        instr.op.name.as_mut_ptr(),
                        instr.args[0usize].value);
            }
        }
        i = relip as libc::c_int;
        while (i as libc::c_uint) <
                  relip.wrapping_add(instr_length as libc::c_uint) {
            if *(*sec).instr_flags.offset(i as isize) as libc::c_int &
                   0x20i32 != 0 {
                
                
                 let mut r = 
                    get_reloc((i as
                                   libc::c_uint).wrapping_add((*sec).address),
                              pe); let mut tsec_0 =  0 as *mut section; let mut taddr =  0;
                fseek(f,
                      (*sec).offset.wrapping_add(i as libc::c_uint) as
                          libc::c_long, 0i32);
                match (*r).type_0() as libc::c_int {
                    3 => {
                        /* HIGHLOW */
                        if (*pe).magic as libc::c_int != 0x10bi32
                           {
                            fprintf(stderr,
                                    b"Warning: %x: \x00" as *const u8 as
                                        *const libc::c_char, ip);
                            fprintf(stderr,
                                    b"HIGHLOW relocation in 64-bit image?\n\x00"
                                        as *const u8 as *const libc::c_char);
                        }
                        taddr =
                            read_dword().wrapping_sub((*pe).c2rust_unnamed.opt32.ImageBase);
                        tsec_0 = addr2section(taddr, pe);
                        /* Only try to scan it if it's an immediate address. If someone is
                     * dereferencing an address inside a code section, it's data. */
                        if (*tsec_0).flags &
                               0x20u32 != 0 &&
                               (instr.op.arg0 ==
                                    
                                    IMM ||
                                    
                                    instr.op.arg1 ==
                                        
                                        IMM) {
                            let ref mut fresh4 =
                                *(*tsec_0).instr_flags.offset(taddr.wrapping_sub((*tsec_0).address)
                                                                  as isize);
                            *fresh4 =
                                (*fresh4 as libc::c_int | 0x8i32)
                                    as byte;
                            scan_segment(taddr, pe);
                        }
                    }
                    _ => {
                        fprintf(stderr,
                                b"Warning: %x: \x00" as *const u8 as
                                    *const libc::c_char, ip);
                        fprintf(stderr,
                                b"Don\'t know how to handle relocation type %d\n\x00"
                                    as *const u8 as *const libc::c_char,
                                (*r).type_0() as libc::c_int);
                    }
                }
                break ;
            } else { i += 1 }
        }
        if instr.op.flags & 0x4000u32 != 0 {
            return
        }
        ip =
            
            (ip).wrapping_add(instr_length as libc::c_uint);
        relip = ip.wrapping_sub((*sec).address)
    }
    fprintf(stderr, b"Warning: %x: \x00" as *const u8 as *const libc::c_char,
            ip);
    fprintf(stderr,
            b"Scan reached the end of section.\n\x00" as *const u8 as
                *const libc::c_char);
}
unsafe extern "C" fn print_section_flags(mut flags: dword) {
    
     let mut buffer =
    
        *::std::mem::transmute::<&[u8; 1024],
                                 &mut [libc::c_char; 1024]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"); let mut alignment =
    
        (flags &
             0xf00000u32).wrapping_div(0x100000u32) as libc::c_int;
    /* Most of these shouldn't occur in an image file, either because they're
     * COFF flags that PE doesn't want or because they're object-only. Print
     * the COFF names. */
    if flags & 0x1u32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", STYP_DSECT\x00" as *const u8 as
                   *const libc::c_char); /* or 16BIT */
    }
    if flags & 0x2u32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", STYP_NOLOAD\x00" as *const u8 as *const libc::c_char);
    }
    if flags & 0x4u32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", STYP_GROUP\x00" as *const u8 as *const libc::c_char);
    }
    if flags & 0x8u32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", STYP_PAD\x00" as *const u8 as *const libc::c_char);
    }
    if flags & 0x10u32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", STYP_COPY\x00" as *const u8 as *const libc::c_char);
    }
    if flags & 0x20u32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", code\x00" as *const u8 as *const libc::c_char);
    }
    if flags & 0x40u32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", data\x00" as *const u8 as *const libc::c_char);
    }
    if flags & 0x80u32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", bss\x00" as *const u8 as *const libc::c_char);
    }
    if flags & 0x100u32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", S_NEWCFN\x00" as *const u8 as *const libc::c_char);
    }
    if flags & 0x200u32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", STYP_INFO\x00" as *const u8 as *const libc::c_char);
    }
    if flags & 0x400u32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", STYP_OVER\x00" as *const u8 as *const libc::c_char);
    }
    if flags & 0x800u32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", STYP_LIB\x00" as *const u8 as *const libc::c_char);
    }
    if flags & 0x1000u32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", COMDAT\x00" as *const u8 as *const libc::c_char);
    }
    if flags & 0x2000u32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", STYP_MERGE\x00" as *const u8 as *const libc::c_char);
    }
    if flags & 0x4000u32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", STYP_REVERSE_PAD\x00" as *const u8 as *const libc::c_char);
    }
    if flags & 0x8000u32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", FARDATA\x00" as *const u8 as *const libc::c_char);
    }
    if flags & 0x10000u32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", (unknown flags 0x10000)\x00" as *const u8 as
                   *const libc::c_char);
    }
    if flags & 0x20000u32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", purgeable\x00" as *const u8 as *const libc::c_char);
    }
    if flags & 0x40000u32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", locked\x00" as *const u8 as *const libc::c_char);
    }
    if flags & 0x80000u32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", preload\x00" as *const u8 as *const libc::c_char);
    }
    if flags & 0x1000000u32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", extended relocations\x00" as *const u8 as
                   *const libc::c_char);
    }
    if flags & 0x2000000u32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", discardable\x00" as *const u8 as *const libc::c_char);
    }
    if flags & 0x4000000u32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", not cached\x00" as *const u8 as *const libc::c_char);
    }
    if flags & 0x8000000u32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", not paged\x00" as *const u8 as *const libc::c_char);
    }
    if flags & 0x10000000u32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", shared\x00" as *const u8 as *const libc::c_char);
    }
    if flags & 0x20000000u32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", executable\x00" as *const u8 as *const libc::c_char);
    }
    if flags & 0x40000000u32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", readable\x00" as *const u8 as *const libc::c_char);
    }
    if flags & 0x80000000u32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", writable\x00" as *const u8 as *const libc::c_char);
    }
    printf(b"    Flags: 0x%08x (%s)\n\x00" as *const u8 as
               *const libc::c_char, flags,
           buffer.as_mut_ptr().offset(2isize));
    printf(b"    Alignment: %d (2**%d)\n\x00" as *const u8 as
               *const libc::c_char, (1i32) << alignment,
           alignment);
}
/* We don't actually know what sections contain code. In theory it could be any
 * of them. Fortunately we actually have everything we need already. */
#[no_mangle]
pub unsafe extern "C" fn read_sections(mut pe: *mut pe) {
    
    
    /* We already read the section header (unlike NE, we had to in order to read
     * everything else), so our job now is just to scan the section contents. */
    /* Relocations first. */
      let mut entry_point =
    
        if (*pe).magic as libc::c_int == 0x10bi32 {
            (*pe).c2rust_unnamed.opt32.AddressOfEntryPoint
        } else { (*pe).c2rust_unnamed.opt64.AddressOfEntryPoint }; let mut i =   0i32;
    while (i as libc::c_uint) < (*pe).reloc_count {
        
         let mut address =
    
            (*pe).reloc_base.wrapping_add((*(*pe).relocs.offset(i as
                                                                    isize)).offset()); let mut sec =  addr2section(address, pe);
        if sec.is_null() {
            fprintf(stderr,
                    b"Warning: Relocation at %#x isn\'t in a section?\n\x00"
                        as *const u8 as *const libc::c_char, address);
        } else if (*sec).flags & 0x20u32 != 0 {
            match (*(*pe).relocs.offset(i as isize)).type_0() as libc::c_int {
                0 => { }
                3 => {
                    /* HIGHLOW */
                    /* scanning is done in scan_segment() */
                    let ref mut fresh5 =
                        *(*sec).instr_flags.offset(address.wrapping_sub((*sec).address)
                                                       as isize);
                    *fresh5 =
                        (*fresh5 as libc::c_int | 0x20i32) as byte
                }
                _ => {
                    fprintf(stderr,
                            b"Warning: %#x: Don\'t know how to handle relocation type %d\n\x00"
                                as *const u8 as *const libc::c_char,
                            (*(*pe).relocs.offset(i as isize)).offset() as
                                libc::c_int,
                            (*(*pe).relocs.offset(i as isize)).type_0() as
                                libc::c_int);
                }
            }
        }
        i += 1
    }
    i = 0i32;
    while (i as libc::c_uint) < (*pe).export_count {
        
         let mut address_0 = 
            (*(*pe).exports.offset(i as isize)).address; let mut sec_0 =  addr2section(address_0, pe);
        if sec_0.is_null() {
            fprintf(stderr,
                    b"Warning: Export %s at %#x isn\'t in a section?\n\x00" as
                        *const u8 as *const libc::c_char,
                    (*(*pe).exports.offset(i as isize)).name,
                    (*(*pe).exports.offset(i as isize)).address);
        } else if (*sec_0).flags & 0x20u32 != 0 &&
                      !(address_0 >=
                            (*(*pe).dirs.offset(0isize)).address &&
                            address_0 <
                                (*(*pe).dirs.offset(0isize)).address.wrapping_add((*(*pe).dirs.offset(0isize)).size))
         {
            let ref mut fresh6 =
                *(*sec_0).instr_flags.offset(address_0.wrapping_sub((*sec_0).address)
                                                 as isize);
            *fresh6 = (*fresh6 as libc::c_int | 0x8i32) as byte;
            scan_segment((*(*pe).exports.offset(i as isize)).address, pe);
        }
        i += 1
    }
    if entry_point != 0 {
         let mut sec_1 =  addr2section(entry_point, pe);
        if sec_1.is_null() {
            fprintf(stderr,
                    b"Warning: Entry point %#x isn\'t in a section?\n\x00" as
                        *const u8 as *const libc::c_char, entry_point);
        } else if (*sec_1).flags & 0x20u32 != 0 {
            let ref mut fresh7 =
                *(*sec_1).instr_flags.offset(entry_point.wrapping_sub((*sec_1).address)
                                                 as isize);
            *fresh7 = (*fresh7 as libc::c_int | 0x8i32) as byte;
            scan_segment(entry_point, pe);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn print_sections(mut pe: *mut pe) {
    
    
      let mut sec =  0 as *mut section; let mut i =   0i32;
    while i < (*pe).header.NumberOfSections as libc::c_int {
        sec = &mut *(*pe).sections.offset(i as isize) as *mut section;
        putchar('\n' as i32);
        printf(b"Section %s (start = 0x%x, length = 0x%x, minimum allocation = 0x%x):\n\x00"
                   as *const u8 as *const libc::c_char,
               (*sec).name.as_mut_ptr(), (*sec).offset, (*sec).length,
               (*sec).min_alloc);
        printf(b"    Address: %x\n\x00" as *const u8 as *const libc::c_char,
               (*sec).address);
        print_section_flags((*sec).flags);
        /* These fields should only be populated for object files (I think). */
        if (*sec).reloc_offset != 0 || (*sec).reloc_count as libc::c_int != 0
           {
            fprintf(stderr,
                    b"Warning: Section %s has relocation data: offset = %x, count = %d\n\x00"
                        as *const u8 as *const libc::c_char,
                    (*sec).name.as_mut_ptr(), (*sec).reloc_offset,
                    (*sec).reloc_count as libc::c_int);
        }
        /* Sometimes the .text section is marked as both code and data. I've
         * seen mingw-w64 do this. (Because there's data stored in it?) */
        if (*sec).flags & 0x20u32 != 0 {
            if opts as libc::c_int & 0x20i32 != 0 {
                print_data(sec, pe);
            }
            print_disassembly(sec, pe);
        } else if (*sec).flags & 0x40u32 != 0 {
            /* see the appropriate FIXMEs on the NE side */
            /* Don't print .rsrc by default. Some others should probably be
             * excluded, too, but .rsrc is a particularly bad offender since
             * large binaries might be put into it. */
            if strcmp((*sec).name.as_mut_ptr(),
                      b".rsrc\x00" as *const u8 as *const libc::c_char) != 0
                   &&
                   strcmp((*sec).name.as_mut_ptr(),
                          b".reloc\x00" as *const u8 as *const libc::c_char)
                       != 0 || opts as libc::c_int & 0x20i32 != 0
               {
                print_data(sec, pe);
            }
        }
        i += 1
    };
}
