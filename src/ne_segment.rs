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
    static mut stdout: *mut _IO_FILE;
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn _IO_getc(__fp: *mut _IO_FILE) -> libc::c_int;
    #[no_mangle]
    fn _IO_putc(__c: libc::c_int, __fp: *mut _IO_FILE) -> libc::c_int;
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
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    
    
}



#[repr(C, packed)]#[derive(Copy, Clone)]
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

#[repr(C)]#[derive(Copy, Clone)]
pub struct entry {
    pub flags: byte,
    pub segment: byte,
    pub offset: word,
    pub name: *mut libc::c_char,
}
/* may be NULL */

#[repr(C)]#[derive(Copy, Clone)]
pub struct export {
    pub ordinal: word,
    pub name: *mut libc::c_char,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct import_module {
    pub name: *mut libc::c_char,
    pub exports: *mut export,
    pub export_count: libc::c_uint,
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct reloc {
    pub size: byte,
    pub type_0: byte,
    pub offset_count: word,
    pub offsets: *mut word,
    pub tseg: word,
    pub toffset: word,
    pub text: *mut libc::c_char,
}

#[repr(C)]#[derive(Copy, Clone)]
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

#[repr(C)]#[derive(Copy, Clone)]
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
/* branch to target (jmp, jXX) */


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




/*
 * Functions for dumping NE code and data segments
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
/* index function */
unsafe extern "C" fn get_entry_name(mut cs: word, mut ip: word,
                                    mut ne: *const ne) -> *mut libc::c_char {
    
      let mut i =   0u32;
    while i < (*ne).entcount {
        if (*(*ne).enttab.offset(i as isize)).segment as libc::c_int ==
               cs as libc::c_int &&
               (*(*ne).enttab.offset(i as isize)).offset as libc::c_int ==
                   ip as libc::c_int {
            return (*(*ne).enttab.offset(i as isize)).name
        }
        i = i.wrapping_add(1)
    }
    return 0 as *mut libc::c_char;
}
/* index function */
unsafe extern "C" fn get_reloc(mut seg: *const segment, mut ip: word)
 -> *const reloc {
    
    
      let mut o =  0; let mut i =   0u32;
    while i < (*seg).reloc_count as libc::c_uint {
        o = 0u32;
        while o <
                  (*(*seg).reloc_table.offset(i as isize)).offset_count as
                      libc::c_uint {
            if *(*(*seg).reloc_table.offset(i as
                                                isize)).offsets.offset(o as
                                                                           isize)
                   as libc::c_int == ip as libc::c_int {
                return &mut *(*seg).reloc_table.offset(i as isize) as
                           *mut reloc
            }
            o = o.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    }
    return 0 as *const reloc;
}
/* load an imported name from a specfile */
unsafe extern "C" fn get_imported_name(mut module: word, mut ordinal: word,
                                       mut ne: *const ne)
 -> *mut libc::c_char {
    
      let mut i =   0u32;
    while i <
              (*(*ne).imptab.offset((module as libc::c_int - 1i32)
                                        as isize)).export_count {
        if (*(*(*ne).imptab.offset((module as libc::c_int - 1i32)
                                       as
                                       isize)).exports.offset(i as
                                                                  isize)).ordinal
               as libc::c_int == ordinal as libc::c_int {
            return (*(*(*ne).imptab.offset((module as libc::c_int -
                                                1i32) as
                                               isize)).exports.offset(i as
                                                                          isize)).name
        }
        i = i.wrapping_add(1)
    }
    return 0 as *mut libc::c_char;
}
/* Tweak the inline string and return the comment. */
unsafe extern "C" fn relocate_arg(mut seg: *const segment, mut arg: *mut Argument,
                                  mut ne: *const ne) -> *const libc::c_char {
    
     let mut r =  get_reloc(seg, (*arg).ip as word); let mut module =  0 as *mut libc::c_char;
    if r.is_null() &&
           
           (*arg).type_0 ==
               
               PTR32 {
        r =
            get_reloc(seg,
                      (*arg).ip.wrapping_add(2u32)
                          as word)
    }
    if r.is_null() {
        fprintf(stderr,
                b"Warning: %#x: Byte tagged INSTR_RELOC has no reloc attached; this is a bug.\n\x00"
                    as *const u8 as *const libc::c_char, (*arg).ip);
        return b"?\x00" as *const u8 as *const libc::c_char
    }
    if (*r).type_0 as libc::c_int == 1i32 ||
           (*r).type_0 as libc::c_int == 2i32 {
        module =
            (*(*ne).imptab.offset(((*r).tseg as libc::c_int -
                                       1i32) as isize)).name
    }
    if  (*arg).type_0 ==  PTR32
           && (*r).size as libc::c_int == 3i32 {
        /* 32-bit relocation on 32-bit pointer, so just copy the name */
        if (*r).type_0 as libc::c_int == 0i32 {
            snprintf((*arg).string.as_mut_ptr(),
                     ::std::mem::size_of::<[libc::c_char; 32]>() as
                         libc::c_ulong,
                     b"%d:%04x\x00" as *const u8 as *const libc::c_char,
                     (*r).tseg as libc::c_int, (*r).toffset as libc::c_int);
            return (*r).text
        } else {
            if (*r).type_0 as libc::c_int == 1i32 {
                snprintf((*arg).string.as_mut_ptr(),
                         ::std::mem::size_of::<[libc::c_char; 32]>() as
                             libc::c_ulong,
                         b"%s.%d\x00" as *const u8 as *const libc::c_char,
                         module, (*r).toffset as libc::c_int);
                return get_imported_name((*r).tseg, (*r).toffset, ne)
            } else {
                if (*r).type_0 as libc::c_int == 2i32 {
                    snprintf((*arg).string.as_mut_ptr(),
                             ::std::mem::size_of::<[libc::c_char; 32]>() as
                                 libc::c_ulong,
                             b"%s.%.*s\x00" as *const u8 as
                                 *const libc::c_char, module,
                             *(*ne).nametab.offset((*r).toffset as isize) as
                                 libc::c_int,
                             &mut *(*ne).nametab.offset(((*r).toffset as
                                                             libc::c_int +
                                                             1i32)
                                                            as isize) as
                                 *mut byte);
                    return 0 as *const libc::c_char
                }
            }
        }
    } else if  (*arg).type_0 ==
                  
                  PTR32 &&
                  (*r).size as libc::c_int == 2i32 &&
                  (*r).type_0 as libc::c_int == 0i32 {
        /* segment relocation on 32-bit pointer; copy the segment but keep the
         * offset */
        snprintf((*arg).string.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
                 b"%d:%04lx\x00" as *const u8 as *const libc::c_char,
                 (*r).tseg as libc::c_int, (*arg).value);
        return get_entry_name((*r).tseg, (*arg).value as word, ne)
    } else {
        if  (*arg).type_0 ==  IMM
               &&
               ((*r).size as libc::c_int == 2i32 ||
                    (*r).size as libc::c_int == 5i32) {
             let mut pfx =
    
                if (*r).size as libc::c_int == 2i32 {
                    b"seg \x00" as *const u8 as *const libc::c_char
                } else { b"\x00" as *const u8 as *const libc::c_char };
            if (*r).type_0 as libc::c_int == 0i32 {
                snprintf((*arg).string.as_mut_ptr(),
                         ::std::mem::size_of::<[libc::c_char; 32]>() as
                             libc::c_ulong,
                         b"%s%d\x00" as *const u8 as *const libc::c_char, pfx,
                         (*r).tseg as libc::c_int);
                return 0 as *const libc::c_char
            } else {
                if (*r).type_0 as libc::c_int == 1i32 {
                    snprintf((*arg).string.as_mut_ptr(),
                             ::std::mem::size_of::<[libc::c_char; 32]>() as
                                 libc::c_ulong,
                             b"%s%s.%d\x00" as *const u8 as
                                 *const libc::c_char, pfx, module,
                             (*r).toffset as libc::c_int);
                    return get_imported_name((*r).tseg, (*r).toffset, ne)
                } else {
                    if (*r).type_0 as libc::c_int == 2i32 {
                        snprintf((*arg).string.as_mut_ptr(),
                                 ::std::mem::size_of::<[libc::c_char; 32]>()
                                     as libc::c_ulong,
                                 b"%s%s.%.*s\x00" as *const u8 as
                                     *const libc::c_char, pfx, module,
                                 *(*ne).nametab.offset((*r).toffset as isize)
                                     as libc::c_int,
                                 &mut *(*ne).nametab.offset(((*r).toffset as
                                                                 libc::c_int +
                                                                 1i32)
                                                                as isize) as
                                     *mut byte);
                        return 0 as *const libc::c_char
                    }
                }
            }
        }
    }
    fprintf(stderr,
            b"Warning: %#x: unhandled relocation: size %d, type %d, argtype %x\n\x00"
                as *const u8 as *const libc::c_char, (*arg).ip,
            (*r).size as libc::c_int, (*r).type_0 as libc::c_int,
            
            (*arg).type_0);
    return 0 as *const libc::c_char;
}
/* Returns the number of bytes processed (same as get_instr). */
unsafe extern "C" fn print_ne_instr(mut seg: *const segment, mut ip: word,
                                    mut p: *mut byte, mut ne: *const ne)
 -> libc::c_int {
    
    
    
    
    
    
      let mut cs =  (*seg).cs; let mut instr =
    
        {
            let mut init =
                Instruction {usedmem_vex_vex_reg_vex_256: [0; 1],
                      c2rust_padding: [0; 4],
                      prefix: 0u16,
                      op:
                          Operation {opcode: 0,
                             subcode: 0,
                             size: 0,
                             name: [0; 16],
                             arg0: NONE,
                             arg1: NONE,
                             flags: 0,},
                      args:
                          [Argument {string: [0; 32],
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
        }; let mut bits =
    
        if (*seg).flags as libc::c_int & 0x2000i32 != 0 {
            32i32
        } else { 16i32 }; let mut comment =  0 as *const libc::c_char; let mut ip_string =  [0; 11]; let mut len =
    
     crate::src::x86_instr::get_instr(ip as dword, p, &mut instr, bits) as libc::c_uint;
    sprintf(ip_string.as_mut_ptr(),
            b"%3d:%04x\x00" as *const u8 as *const libc::c_char,
            (*seg).cs as libc::c_int, ip as libc::c_int);
    /* check for relocations */
    if *(*seg).instr_flags.offset(instr.args[0usize].ip as
                                      isize) as libc::c_int &
           0x20i32 != 0 {
        comment =
            relocate_arg(seg,
                         &mut *instr.args.as_mut_ptr().offset(0isize),
                         ne)
    }
    if *(*seg).instr_flags.offset(instr.args[1usize].ip as
                                      isize) as libc::c_int &
           0x20i32 != 0 {
        comment =
            relocate_arg(seg,
                         &mut *instr.args.as_mut_ptr().offset(1isize),
                         ne)
    }
    /* make sure to check for PTR32 segment-only relocations */
    if  instr.op.arg0 ==  PTR32
           &&
           *(*seg).instr_flags.offset(instr.args[0usize].ip.wrapping_add(2u32)
                                          as isize) as libc::c_int &
               0x20i32 != 0 {
        comment =
            relocate_arg(seg,
                         &mut *instr.args.as_mut_ptr().offset(0isize),
                         ne)
    }
    /* check if we are referencing a named export */
    if comment.is_null() &&
           
           instr.op.arg0 ==
               
               REL16 {
        comment =
            get_entry_name(cs,
                           instr.args[0usize].value as
                               word, ne)
    }
    crate::src::x86_instr::print_instr(ip_string.as_mut_ptr(), p, len as libc::c_int,
                *(*seg).instr_flags.offset(ip as isize), &mut instr, comment,
                bits);
    return len as libc::c_int;
}
unsafe extern "C" fn print_disassembly(mut seg: *const segment,
                                       mut ne: *const ne) {
    
    
     let cs =  (*seg).cs; let mut ip =  0u16; let mut buffer =  [0; 16];
    while (ip as libc::c_int) < (*seg).length as libc::c_int {
        fseek(f, (*seg).start + ip as libc::c_long, 0i32);
        /* find a valid instruction */
        if *(*seg).instr_flags.offset(ip as isize) as libc::c_int &
               0x2i32 == 0 {
            if opts as libc::c_int & 0x1i32 != 0 {
                /* still skip zeroes */
                if read_byte() as libc::c_int == 0i32 {
                    printf(b"     ...\n\x00" as *const u8 as
                               *const libc::c_char);
                    ip = ip.wrapping_add(1);
                    while read_byte() as libc::c_int == 0i32 {
                        ip = ip.wrapping_add(1)
                    }
                }
            } else {
                printf(b"     ...\n\x00" as *const u8 as *const libc::c_char);
                while (ip as libc::c_int) < (*seg).length as libc::c_int &&
                          *(*seg).instr_flags.offset(ip as isize) as
                              libc::c_int & 0x2i32 == 0 {
                    ip = ip.wrapping_add(1)
                }
            }
        }
        fseek(f, (*seg).start + ip as libc::c_long, 0i32);
        if ip as libc::c_int >= (*seg).length as libc::c_int { return }
        /* Instructions can "hang over" the end of a segment.
         * Zero should be supplied. */
        memset(buffer.as_mut_ptr() as *mut libc::c_void, 0i32,
               ::std::mem::size_of::<[byte; 16]>() as libc::c_ulong);
        fread(buffer.as_mut_ptr() as *mut libc::c_void,
              1u64,
              if (::std::mem::size_of::<[byte; 16]>() as libc::c_ulong) <
                     ((*seg).length as libc::c_int - ip as libc::c_int) as
                         libc::c_ulong {
                  ::std::mem::size_of::<[byte; 16]>() as libc::c_ulong
              } else {
                  ((*seg).length as libc::c_int - ip as libc::c_int) as
                      libc::c_ulong
              }, f);
        if *(*seg).instr_flags.offset(ip as isize) as libc::c_int &
               0x8i32 != 0 {
             let mut name =  get_entry_name(cs, ip, ne);
            printf(b"\n\x00" as *const u8 as *const libc::c_char);
            printf(b"%d:%04x <%s>:\n\x00" as *const u8 as *const libc::c_char,
                   cs as libc::c_int, ip as libc::c_int,
                   if !name.is_null() {
                       name
                   } else {
                       b"no name\x00" as *const u8 as *const libc::c_char
                   });
            /* don't mark far functions—we can't reliably detect them
             * because of "push cs", and they should be evident anyway. */
        } /* well, not really ip */
        ip =
            (ip as libc::c_int +
                 print_ne_instr(seg, ip, buffer.as_mut_ptr(), ne)) as word
    }
    putchar('\n' as i32);
}
unsafe extern "C" fn print_data(mut seg: *const segment) {
    
      let mut ip =   0u16;
    while (ip as libc::c_int) < (*seg).length as libc::c_int {
        
         let mut row =  [0; 16]; let mut len =
    
            if ((*seg).length as libc::c_int - ip as libc::c_int) <
                   16i32 {
                ((*seg).length as libc::c_int) - ip as libc::c_int
            } else { 16i32 };
        
        fseek(f, (*seg).start + ip as libc::c_long, 0i32);
        fread(row.as_mut_ptr() as *mut libc::c_void,
              ::std::mem::size_of::<byte>() as libc::c_ulong, len as size_t,
              f);
        printf(b"%3d:%04x\x00" as *const u8 as *const libc::c_char,
               (*seg).cs as libc::c_int, ip as libc::c_int);
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
        ip = (ip as libc::c_int + 16i32) as word
    };
}
unsafe extern "C" fn scan_segment(mut cs: word, mut ip: word,
                                  mut ne: *mut ne) {
    
    
    
    
     let mut seg =
    
        &mut *(*ne).segments.offset((cs as libc::c_int - 1i32) as
                                        isize) as *mut segment; let mut buffer =  [0; 16]; let mut instr =
    
        Instruction {prefix: 0,
              op:
                  Operation {opcode: 0,
                     subcode: 0,
                     size: 0,
                     name: [0; 16],
                     arg0: NONE,
                     arg1: NONE,
                     flags: 0,},
              args: [Argument {string: [0; 32], ip: 0, value: 0, type_0: NONE,}; 3],
              addrsize: 0,
              modrm_disp: DISP_NONE,
              modrm_reg: 0,
              sib_scale: 0,
              sib_index: 0,
              usedmem_vex_vex_reg_vex_256: [0; 1],
              c2rust_padding: [0; 4],}; let mut instr_length =  0; let mut i =  0;
    if ip as libc::c_int >= (*seg).length as libc::c_int {
        fprintf(stderr,
                b"Warning: %d:%04x: \x00" as *const u8 as *const libc::c_char,
                cs as libc::c_int, ip as libc::c_int);
        fprintf(stderr,
                b"Attempt to scan past end of segment.\n\x00" as *const u8 as
                    *const libc::c_char);
        return
    }
    if *(*seg).instr_flags.offset(ip as isize) as libc::c_int &
           (0x2i32 | 0x1i32) == 0x1i32 {
        fprintf(stderr,
                b"Warning: %d:%04x: \x00" as *const u8 as *const libc::c_char,
                cs as libc::c_int, ip as libc::c_int);
        fprintf(stderr,
                b"Attempt to scan byte that does not begin instruction.\n\x00"
                    as *const u8 as *const libc::c_char);
    }
    while (ip as libc::c_int) < (*seg).length as libc::c_int {
        /* check if we already read from here */
        if *(*seg).instr_flags.offset(ip as isize) as libc::c_int &
               0x1i32 != 0 {
            return
        }
        /* read the instruction */
        fseek(f, (*seg).start + ip as libc::c_long, 0i32);
        memset(buffer.as_mut_ptr() as *mut libc::c_void, 0i32,
               ::std::mem::size_of::<[byte; 16]>() as libc::c_ulong);
        fread(buffer.as_mut_ptr() as *mut libc::c_void,
              1u64,
              if (::std::mem::size_of::<[byte; 16]>() as libc::c_ulong) <
                     ((*seg).length as libc::c_int - ip as libc::c_int) as
                         libc::c_ulong {
                  ::std::mem::size_of::<[byte; 16]>() as libc::c_ulong
              } else {
                  ((*seg).length as libc::c_int - ip as libc::c_int) as
                      libc::c_ulong
              }, f);
        instr_length =
            crate::src::x86_instr::get_instr(ip as dword, buffer.as_mut_ptr(), &mut instr,
                      if (*seg).flags as libc::c_int & 0x2000i32
                             != 0 {
                          32i32
                      } else { 16i32 });
        /* mark the bytes */
        let ref mut fresh0 = *(*seg).instr_flags.offset(ip as isize);
        *fresh0 = (*fresh0 as libc::c_int | 0x2i32) as byte;
        i = ip as libc::c_int;
        while i < ip as libc::c_int + instr_length &&
                  i < (*seg).min_alloc as libc::c_int {
            let ref mut fresh1 = *(*seg).instr_flags.offset(i as isize);
            *fresh1 = (*fresh1 as libc::c_int | 0x1i32) as byte;
            i += 1
        }
        /* instruction which hangs over the minimum allocation */
        if i < ip as libc::c_int + instr_length &&
               i == (*seg).min_alloc as libc::c_int {
            break ;
        }
        /* handle conditional and unconditional jumps */
        if  instr.op.arg0 ==
               
               PTR32 {
            i = ip as libc::c_int;
            while i < ip as libc::c_int + instr_length {
                if *(*seg).instr_flags.offset(i as isize) as libc::c_int &
                       0x20i32 != 0 {
                     let mut r =  get_reloc(seg, i as word);
                    
                    if r.is_null() { break ; }
                      let mut tseg =
    
    
                        &mut *(*ne).segments.offset(((*r).tseg as libc::c_int
                                                         - 1i32)
                                                        as isize) as
                            *mut segment;
                    if (*r).type_0 as libc::c_int != 0i32 {
                        break ;
                    }
                    if (*r).size as libc::c_int == 3i32 {
                        /* 32-bit relocation on 32-bit pointer */
                        let ref mut fresh2 =
                            *(*tseg).instr_flags.offset((*r).toffset as
                                                            isize);
                        *fresh2 =
                            (*fresh2 as libc::c_int | 0x10i32) as
                                byte;
                        if strcmp(instr.op.name.as_mut_ptr(),
                                  b"call\x00" as *const u8 as
                                      *const libc::c_char) == 0 {
                            let ref mut fresh3 =
                                *(*tseg).instr_flags.offset((*r).toffset as
                                                                isize);
                            *fresh3 =
                                (*fresh3 as libc::c_int | 0x8i32)
                                    as byte
                        } else {
                            let ref mut fresh4 =
                                *(*tseg).instr_flags.offset((*r).toffset as
                                                                isize);
                            *fresh4 =
                                (*fresh4 as libc::c_int | 0x4i32)
                                    as byte
                        }
                        scan_segment((*r).tseg, (*r).toffset, ne);
                    } else if (*r).size as libc::c_int == 2i32 {
                        /* segment relocation on 32-bit pointer */
                        let ref mut fresh5 =
                            *(*tseg).instr_flags.offset(instr.args[0usize].value
                                                            as isize);
                        *fresh5 =
                            (*fresh5 as libc::c_int | 0x10i32) as
                                byte;
                        if strcmp(instr.op.name.as_mut_ptr(),
                                  b"call\x00" as *const u8 as
                                      *const libc::c_char) == 0 {
                            let ref mut fresh6 =
                                *(*tseg).instr_flags.offset(instr.args[0usize].value
                                                                as isize);
                            *fresh6 =
                                (*fresh6 as libc::c_int | 0x8i32)
                                    as byte
                        } else {
                            let ref mut fresh7 =
                                *(*tseg).instr_flags.offset(instr.args[0usize].value
                                                                as isize);
                            *fresh7 =
                                (*fresh7 as libc::c_int | 0x4i32)
                                    as byte
                        }
                        scan_segment((*r).tseg,
                                     instr.args[0usize].value as word, ne);
                    }
                    break ;
                } else { i += 1 }
            }
        } else if instr.op.flags & 0x8000u32 != 0
         {
            /* near relative jump, loop, or call */
            if strcmp(instr.op.name.as_mut_ptr(),
                      b"call\x00" as *const u8 as *const libc::c_char) == 0 {
                let ref mut fresh8 =
                    *(*seg).instr_flags.offset(instr.args[0usize].value as
                                                   isize);
                *fresh8 =
                    (*fresh8 as libc::c_int | 0x8i32) as byte
            } else {
                let ref mut fresh9 =
                    *(*seg).instr_flags.offset(instr.args[0usize].value as
                                                   isize);
                *fresh9 =
                    (*fresh9 as libc::c_int | 0x4i32) as byte
            }
            /* scan it */
            scan_segment(cs,
                         instr.args[0usize].value as word,
                         ne);
        }
        if instr.op.flags & 0x4000u32 != 0 {
            return
        }
        ip = (ip as libc::c_int + instr_length) as word
    }
    fprintf(stderr,
            b"Warning: %d:%04x: \x00" as *const u8 as *const libc::c_char,
            cs as libc::c_int, ip as libc::c_int);
    fprintf(stderr,
            b"Scan reached the end of segment.\n\x00" as *const u8 as
                *const libc::c_char);
}
unsafe extern "C" fn print_segment_flags(mut flags: word) {
     let mut buffer =  [0; 1024];
    if flags as libc::c_int & 0x1i32 != 0 {
        strcpy(buffer.as_mut_ptr(),
               b"data\x00" as *const u8 as *const libc::c_char);
    } else {
        strcpy(buffer.as_mut_ptr(),
               b"code\x00" as *const u8 as *const libc::c_char);
    }
    /* I think these three should never occur in a file */
    if flags as libc::c_int & 0x2i32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", allocated\x00" as *const u8 as *const libc::c_char);
    }
    if flags as libc::c_int & 0x4i32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", loaded\x00" as *const u8 as *const libc::c_char);
    }
    if flags as libc::c_int & 0x8i32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", iterated\x00" as *const u8 as *const libc::c_char);
    }
    if flags as libc::c_int & 0x10i32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", moveable\x00" as *const u8 as *const libc::c_char);
    }
    if flags as libc::c_int & 0x20i32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", shareable\x00" as *const u8 as *const libc::c_char);
    }
    if flags as libc::c_int & 0x40i32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", preload\x00" as *const u8 as *const libc::c_char);
    }
    if flags as libc::c_int & 0x80i32 != 0 {
        strcat(buffer.as_mut_ptr(),
               if flags as libc::c_int & 0x1i32 != 0 {
                   b", read-only\x00" as *const u8 as *const libc::c_char
               } else {
                   b", execute-only\x00" as *const u8 as *const libc::c_char
               });
    }
    if flags as libc::c_int & 0x100i32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", has relocation data\x00" as *const u8 as
                   *const libc::c_char);
    }
    /* there's still an unidentified flag 0x0400 which appears in all of my testcases.
     * but WINE doesn't know what it is, so... */
    if flags as libc::c_int & 0x800i32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", self-loading\x00" as *const u8 as
                   *const libc::c_char); /* or segment */
    } /* or offset */
    if flags as libc::c_int & 0x1000i32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", discardable\x00" as *const u8 as *const libc::c_char);
    }
    if flags as libc::c_int & 0x2000i32 != 0 {
        strcat(buffer.as_mut_ptr(),
               b", 32-bit\x00" as *const u8 as *const libc::c_char);
    }
    if flags as libc::c_int & 0xc608i32 != 0 {
        sprintf(buffer.as_mut_ptr().offset(strlen(buffer.as_mut_ptr()) as
                                               isize),
                b", (unknown flags 0x%04x)\x00" as *const u8 as
                    *const libc::c_char,
                flags as libc::c_int & 0xc608i32);
    }
    printf(b"    Flags: 0x%04x (%s)\n\x00" as *const u8 as
               *const libc::c_char, flags as libc::c_int,
           buffer.as_mut_ptr());
}
unsafe extern "C" fn read_reloc(mut seg: *const segment, mut r: *mut reloc,
                                mut ne: *mut ne) {
    
    
    
    
    
    
     let mut size =  read_byte(); let mut type_0 =  read_byte(); let mut offset =  read_word(); let mut module =  read_word(); let mut ordinal =  read_word(); let mut next =  0;
    memset(r as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<reloc>() as libc::c_ulong);
    
     *r =
    crate::src::ne_segment::reloc{size:   size,
                                  type_0:
                                      
                                   (type_0 as libc::c_int & 3i32) as byte, ..*r};
    if type_0 as libc::c_int & 3i32 == 0i32 {
        /* internal reference */
        
        if module as libc::c_int == 0xffi32 {
             *r =
    crate::src::ne_segment::reloc{tseg:
                                      
                                  
                (*(*ne).enttab.offset((ordinal as libc::c_int -
                                           1i32) as
                                          isize)).segment as word, ..*r};
            *r =
                crate::src::ne_segment::reloc{toffset:
                                  
                (*(*ne).enttab.offset((ordinal as libc::c_int -
                                           1i32) as isize)).offset,
                                                                              ..*r}
        } else {  *r = crate::src::ne_segment::reloc{tseg:   module, ..*r}; *r = crate::src::ne_segment::reloc{toffset:  ordinal, ..*r} }
        /* grab the name, if we can */
          let mut name =   get_entry_name((*r).tseg, (*r).toffset, ne);
        if !name.is_null() { *r = crate::src::ne_segment::reloc{text:  name, ..*r} }
    } else if type_0 as libc::c_int & 3i32 == 1i32 {
        /* imported ordinal */
         *r = crate::src::ne_segment::reloc{tseg:   module, ..*r};
        *r = crate::src::ne_segment::reloc{toffset:  ordinal, ..*r}
    } else if type_0 as libc::c_int & 3i32 == 2i32 {
        /* imported name */
         *r = crate::src::ne_segment::reloc{tseg:   module, ..*r};
        *r = crate::src::ne_segment::reloc{toffset:  ordinal, ..*r}
    } else if type_0 as libc::c_int & 3i32 == 3i32 {
        /* OSFIXUP */
        /* FIXME: the meaning of this is not understood! */
        return
    }
    /* get the offset list */
      let mut offset_cursor =   offset;
     *r = crate::src::ne_segment::reloc{offset_count:   0u16, ..*r};
    loop 
         /* One of my testcases has relocation offsets that exceed the length of
         * the segment. Until we figure out what that's about, ignore them. */
         {
        if offset_cursor as libc::c_int >= (*seg).length as libc::c_int {
            fprintf(stderr,
                    b"Warning: %d:%04x: Relocation offset exceeds segment length (%04x).\n\x00"
                        as *const u8 as *const libc::c_char,
                    (*seg).cs as libc::c_int, offset_cursor as libc::c_int,
                    (*seg).length as libc::c_int);
            break ;
        } else {
            if *(*seg).instr_flags.offset(offset_cursor as isize) as
                   libc::c_int & 0x20i32 != 0 {
                fprintf(stderr,
                        b"Warning: %d:%04x: Infinite loop reading relocation data.\n\x00"
                            as *const u8 as *const libc::c_char,
                        (*seg).cs as libc::c_int,
                        offset_cursor as libc::c_int);
                 *r = crate::src::ne_segment::reloc{offset_count:   0u16, ..*r};
                return
            }
             *r =
    crate::src::ne_segment::reloc{offset_count:
                                      
                                   (*r).offset_count.wrapping_add(1),
                                                                           ..*r};
            let ref mut fresh10 =
                *(*seg).instr_flags.offset(offset_cursor as isize);
            *fresh10 =
                (*fresh10 as libc::c_int | 0x20i32) as byte;
            fseek(f, (*seg).start + offset_cursor as libc::c_long,
                  0i32);
            next = read_word();
            if type_0 as libc::c_int & 4i32 != 0 {
                offset_cursor =
                    (offset_cursor as libc::c_int + next as libc::c_int) as
                        word
            } else { offset_cursor = next }
            if !((next as libc::c_int) < 0xfffbi32) { break ; }
        }
    }
     *r =
    crate::src::ne_segment::reloc{offsets:
                                      
                                  
        malloc(((*r).offset_count as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut word>()
                                                    as libc::c_ulong)) as
            *mut word, ..*r};
    offset_cursor = offset;
     *r = crate::src::ne_segment::reloc{offset_count:   0u16, ..*r};
    while !(offset_cursor as libc::c_int >= (*seg).length as libc::c_int) {
        *(*r).offsets.offset((*r).offset_count as isize) = offset_cursor;
         *r =
    crate::src::ne_segment::reloc{offset_count:
                                      
                                   (*r).offset_count.wrapping_add(1),
                                                                           ..*r};
        fseek(f, (*seg).start + offset_cursor as libc::c_long,
              0i32);
        next = read_word();
        if type_0 as libc::c_int & 4i32 != 0 {
            offset_cursor =
                (offset_cursor as libc::c_int + next as libc::c_int) as word
        } else { offset_cursor = next }
        if !((next as libc::c_int) < 0xfffbi32) { break ; }
    };
}
unsafe extern "C" fn free_reloc(mut reloc_data: *mut reloc,
                                mut reloc_count: word) {
    
      let mut i =   0i32;
    while i < reloc_count as libc::c_int {
        free((*reloc_data.offset(i as isize)).offsets as *mut libc::c_void);
        i += 1
    }
    free(reloc_data as *mut libc::c_void);
}
/* in ne_segment.c */
#[no_mangle]
pub unsafe extern "C" fn read_segments(mut start: libc::c_long,
                                       mut ne: *mut ne) {
    
    
    
    
    
     let mut entry_cs =  (*ne).header.ne_cs; let mut entry_ip =  (*ne).header.ne_ip; let mut count =  (*ne).header.ne_cseg; let mut i =  0; let mut seg =  0 as *mut segment;
    fseek(f, start, 0i32);
     *ne =
    crate::src::ne_segment::ne{segments:
                                   
                               
        malloc((count as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<segment>()
                                                    as libc::c_ulong)) as
            *mut segment, ..*ne};
      let mut cs =   1u32;
    while cs <= count as libc::c_uint {
        seg =
            &mut *(*ne).segments.offset(cs.wrapping_sub(1u32) as
                                            isize) as *mut segment;
        
        
        
        
        
        /* Use min_alloc rather than length because data can "hang over". */
         *seg =
    crate::src::ne_segment::segment{cs:   cs as word,
                                    start:
                                        
                                    
            ((read_word() as libc::c_int) <<
                 (*ne).header.ne_align as libc::c_int) as libc::c_long,
                                    length:   read_word(),
                                    flags:   read_word(),
                                    min_alloc:   read_word(),
                                    instr_flags:
                                        
                                    
            calloc((*seg).min_alloc as libc::c_ulong,
                   ::std::mem::size_of::<byte>() as libc::c_ulong) as
                *mut byte, ..*seg};
        cs = cs.wrapping_add(1)
    }
    /* First pass: just read the relocation data */
    cs = 1u32;
    while cs <= count as libc::c_uint {
        seg =
            &mut *(*ne).segments.offset(cs.wrapping_sub(1u32) as
                                            isize) as *mut segment;
        if (*seg).flags as libc::c_int & 0x100i32 != 0 {
            fseek(f, (*seg).start + (*seg).length as libc::c_long,
                  0i32);
            
             *seg =
    crate::src::ne_segment::segment{reloc_count:   read_word(),
                                    reloc_table:
                                        
                                    
                malloc(((*seg).reloc_count as
                            libc::c_ulong).wrapping_mul(::std::mem::size_of::<reloc>()
                                                            as libc::c_ulong))
                    as *mut reloc, ..*seg};
            i = 0u32;
            while i < (*seg).reloc_count as libc::c_uint {
                fseek(f,
                      (*seg).start + (*seg).length as libc::c_long +
                          2i64 +
                          i.wrapping_mul(8u32) as
                              libc::c_long, 0i32);
                read_reloc(seg, &mut *(*seg).reloc_table.offset(i as isize),
                           ne);
                i = i.wrapping_add(1)
            }
        } else {
             *seg = crate::src::ne_segment::segment{reloc_count:   0u16, ..*seg};
            *seg = crate::src::ne_segment::segment{reloc_table:  0 as *mut reloc, ..*seg}
        }
        cs = cs.wrapping_add(1)
    }
    /* Second pass: scan entry points (we have to do this after we read
     * relocation data for all segments.) */
    i = 0u32;
    while i < (*ne).entcount {
        /* don't scan exported values */
        if !((*(*ne).enttab.offset(i as isize)).segment as libc::c_int ==
                 0i32 ||
                 (*(*ne).enttab.offset(i as isize)).segment as libc::c_int ==
                     0xfei32) {
            /* or values that live in data segments */
            if !((*(*ne).segments.offset(((*(*ne).enttab.offset(i as
                                                                    isize)).segment
                                              as libc::c_int -
                                              1i32) as
                                             isize)).flags as libc::c_int &
                     0x1i32 != 0) {
                /* Annoyingly, data can be put in code segments, and without any
         * apparent indication that it is not code. As a dumb heuristic,
         * only scan exported entries—this won't work universally, and it
         * may potentially miss private entries, but it's better than nothing. */
                if !((*(*ne).enttab.offset(i as isize)).flags as libc::c_int &
                         1i32 == 0) {
                    scan_segment((*(*ne).enttab.offset(i as isize)).segment as
                                     word,
                                 (*(*ne).enttab.offset(i as isize)).offset,
                                 ne);
                    let ref mut fresh11 =
                        *(*(*ne).segments.offset(((*(*ne).enttab.offset(i as
                                                                            isize)).segment
                                                      as libc::c_int -
                                                      1i32) as
                                                     isize)).instr_flags.offset((*(*ne).enttab.offset(i
                                                                                                          as
                                                                                                          isize)).offset
                                                                                    as
                                                                                    isize);
                    *fresh11 =
                        (*fresh11 as libc::c_int | 0x8i32) as byte
                }
            }
        }
        i = i.wrapping_add(1)
    }
    /* and don't forget to scan the program entry point */
    if !(entry_cs as libc::c_int == 0i32 &&
             entry_ip as libc::c_int == 0i32) {
        if entry_ip as libc::c_int >=
               (*(*ne).segments.offset((entry_cs as libc::c_int -
                                            1i32) as
                                           isize)).length as libc::c_int {
            /* see note above under relocations */
            fprintf(stderr,
                    b"Warning: Entry point %d:%04x exceeds segment length (%04x)\n\x00"
                        as *const u8 as *const libc::c_char,
                    entry_cs as libc::c_int, entry_ip as libc::c_int,
                    (*(*ne).segments.offset((entry_cs as libc::c_int -
                                                 1i32) as
                                                isize)).length as
                        libc::c_int);
        } else {
            let ref mut fresh12 =
                *(*(*ne).segments.offset((entry_cs as libc::c_int -
                                              1i32) as
                                             isize)).instr_flags.offset(entry_ip
                                                                            as
                                                                            isize);
            *fresh12 = (*fresh12 as libc::c_int | 0x8i32) as byte;
            scan_segment(entry_cs, entry_ip, ne);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn free_segments(mut ne: *mut ne) {
    
    
      let mut seg =  0 as *mut segment; let mut cs =   1u32;
    while cs <= (*ne).header.ne_cseg as libc::c_uint {
        seg =
            &mut *(*ne).segments.offset(cs.wrapping_sub(1u32) as
                                            isize) as *mut segment;
        free_reloc((*seg).reloc_table, (*seg).reloc_count);
        free((*seg).instr_flags as *mut libc::c_void);
        cs = cs.wrapping_add(1)
    }
    free((*ne).segments as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn print_segments(mut ne: *mut ne) {
    
    
    /* Final pass: print data */
      let mut seg =  0 as *mut segment; let mut cs =   1u32;
    while cs <= (*ne).header.ne_cseg as libc::c_uint {
        seg =
            &mut *(*ne).segments.offset(cs.wrapping_sub(1u32) as
                                            isize) as *mut segment;
        putchar('\n' as i32);
        printf(b"Segment %d (start = 0x%lx, length = 0x%x, minimum allocation = 0x%x):\n\x00"
                   as *const u8 as *const libc::c_char, cs, (*seg).start,
               (*seg).length as libc::c_int,
               if (*seg).min_alloc as libc::c_int != 0 {
                   (*seg).min_alloc as libc::c_int
               } else { 65536i32 });
        print_segment_flags((*seg).flags);
        if (*seg).flags as libc::c_int & 0x1i32 != 0 {
            /* FIXME: We should at least make a special note of entry points. */
            /* FIXME #2: Data segments can still have relocations... */
            print_data(seg);
        } else {
            /* like objdump, print the whole code segment like a data segment */
            if opts as libc::c_int & 0x20i32 != 0 {
                print_data(seg);
            }
            print_disassembly(seg, ne);
        }
        cs = cs.wrapping_add(1)
    };
}
