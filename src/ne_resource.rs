use ::libc;
extern "C" {
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
    fn fscanf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn _IO_getc(__fp: *mut _IO_FILE) -> libc::c_int;
    #[no_mangle]
    fn _IO_putc(__c: libc::c_int, __fp: *mut _IO_FILE) -> libc::c_int;
    #[no_mangle]
    fn fread(__ptr: *mut libc::c_void, __size: size_t, __n: size_t,
             __stream: *mut FILE) -> size_t;
    #[no_mangle]
    fn fseek(__stream: *mut FILE, __off: libc::c_long, __whence: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
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
    #[no_mangle]
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn strncasecmp(_: *const libc::c_char, _: *const libc::c_char,
                   _: libc::c_ulong) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;

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
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type byte = uint8_t;
/* Common functions */
pub type word = uint16_t;
pub type dword = uint32_t;
/* additional options */
pub type C2RustUnnamed = libc::c_uint;
pub const MASM: C2RustUnnamed = 2;
pub const NASM: C2RustUnnamed = 1;
pub const GAS: C2RustUnnamed = 0;

#[repr(C, packed)]#[derive(Copy, Clone)]
pub struct resource {
    pub offset: word,
    pub length: word,
    pub flags: word,
    pub id: word,
    pub handle: word,
    pub usage: word,
}
/* This is actually two headers, with the first (VS_VERSIONINFO)
 * describing the second. However it seems the second is always
 * a VS_FIXEDFILEINFO header, so we ignore most of those details. */

#[repr(C, packed)]#[derive(Copy, Clone)]
pub struct version_header {
    pub length: word,
    pub value_length: word,
    pub string: [byte; 16],
    pub magic: dword,
    pub struct_2: word,
    pub struct_1: word,
    pub file_2: word,
    pub file_1: word,
    pub file_4: word,
    pub file_3: word,
    pub prod_2: word,
    pub prod_1: word,
    pub prod_4: word,
    pub prod_3: word,
    pub flags_file_mask: dword,
    pub flags_file: dword,
    pub flags_os: dword,
    pub flags_type: dword,
    pub flags_subtype: dword,
    pub date_1: dword,
    pub date_2: dword,
}

#[repr(C, packed)]#[derive(Copy, Clone)]
pub struct dialog_control {
    pub x: word,
    pub y: word,
    pub width: word,
    pub height: word,
    pub id: word,
    pub style: dword,
    pub class: byte,
}
/*
 * Function(s) for dumping resources from NE files
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

#[repr(C, packed)]#[derive(Copy, Clone)]
pub struct header_bitmap_info {
    pub biSize: dword,
    pub biWidth: dword,
    pub biHeight: dword,
    pub biPlanes: word,
    pub biBitCount: word,
    pub biCompression: dword,
    pub biSizeImage: dword,
    pub biXPelsPerMeter: dword,
    pub biYPelsPerMeter: dword,
    pub biClrUsed: dword,
    pub biClrImportant: dword,
}
#[inline]
unsafe extern "C" fn putchar(mut __c: libc::c_int) -> libc::c_int {
    return _IO_putc(__c, stdout);
}
#[no_mangle]
pub static mut f: *mut FILE =  0 as *mut FILE;
#[inline]
unsafe extern "C" fn read_byte() -> byte { return _IO_getc(f) as byte; }
//#[inline]
//unsafe extern "C" fn read_word() -> word {
//     let mut w =  0;
//    fread(&mut w as *mut word as *mut libc::c_void,
//          2u64, 1u64, f);
//    return w;
//}
#[inline]
unsafe extern "C" fn read_dword() -> dword {
     let mut d =  0;
    fread(&mut d as *mut dword as *mut libc::c_void,
          4u64, 1u64, f);
    return d;
}
#[inline]
unsafe extern "C" fn skip_padding(mut bytes: libc::c_char) {
    fseek(f,
          (((bytes as libc::c_int - 1i32) as libc::c_long &
                bytes as libc::c_long - ftell(f) % bytes as libc::c_long) as
               libc::c_ulong).wrapping_mul(::std::mem::size_of::<byte>() as
                                               libc::c_ulong) as libc::c_long,
          1i32);
}
/* Common globals */
#[no_mangle]
pub static mut mode: word = 0;
/* what to dump */
#[no_mangle]
pub static mut opts: word = 0;
#[no_mangle]
pub static mut asm_syntax: C2RustUnnamed = GAS;
#[no_mangle]
pub static mut resource_filters: *mut *mut libc::c_char =
    
    0 as *mut *mut libc::c_char;
#[no_mangle]
pub static mut resource_filters_count: libc::c_uint = 0;
unsafe extern "C" fn dup_string_resource(mut ptr: libc::c_long)
 -> *mut libc::c_char {
    
    
     let mut cursor =  ftell(f);
    fseek(f, ptr, 0i32);
    
       let mut length =   read_byte() as libc::c_char; let mut ret =
    
    
        malloc((length as libc::c_int + 1i32) as libc::c_ulong) as
            *mut libc::c_char;
    fread(ret as *mut libc::c_void,
          ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
          length as size_t, f);
    *ret.offset(length as isize) = 0i8;
    fseek(f, cursor, 0i32);
    return ret;
}
/* length-indexed */
unsafe extern "C" fn print_escaped_string(mut length: libc::c_long) {
    putchar('\"' as i32);
    loop  {
        let fresh0 = length;
        length = length - 1;
        if !(fresh0 != 0) { break ; }
         let mut c =  read_byte() as libc::c_char;
        if c as libc::c_int == '\t' as i32 {
            printf(b"\\t\x00" as *const u8 as *const libc::c_char);
        } else if c as libc::c_int == '\n' as i32 {
            printf(b"\\n\x00" as *const u8 as *const libc::c_char);
        } else if c as libc::c_int == '\r' as i32 {
            printf(b"\\r\x00" as *const u8 as *const libc::c_char);
        } else if c as libc::c_int == '\"' as i32 {
            printf(b"\\\"\x00" as *const u8 as *const libc::c_char);
        } else if c as libc::c_int == '\\' as i32 {
            printf(b"\\\\\x00" as *const u8 as *const libc::c_char);
        } else if c as libc::c_int >= ' ' as i32 &&
                      c as libc::c_int <= '~' as i32 {
            putchar(c as libc::c_int);
        } else {
            printf(b"\\x%02hhx\x00" as *const u8 as *const libc::c_char,
                   c as libc::c_int);
        }
    }
    putchar('\"' as i32);
}
/* null-terminated */
unsafe extern "C" fn print_escaped_string0() {
     let mut c =  0;
    putchar('\"' as i32);
    loop  {
        c = read_byte() as libc::c_char;
        if !(c != 0) { break ; }
        if c as libc::c_int == '\t' as i32 {
            printf(b"\\t\x00" as *const u8 as *const libc::c_char);
        } else if c as libc::c_int == '\n' as i32 {
            printf(b"\\n\x00" as *const u8 as *const libc::c_char);
        } else if c as libc::c_int == '\r' as i32 {
            printf(b"\\r\x00" as *const u8 as *const libc::c_char);
        } else if c as libc::c_int == '\"' as i32 {
            printf(b"\\\"\x00" as *const u8 as *const libc::c_char);
        } else if c as libc::c_int == '\\' as i32 {
            printf(b"\\\\\x00" as *const u8 as *const libc::c_char);
        } else if c as libc::c_int >= ' ' as i32 &&
                      c as libc::c_int <= '~' as i32 {
            putchar(c as libc::c_int);
        } else {
            printf(b"\\x%02hhx\x00" as *const u8 as *const libc::c_char,
                   c as libc::c_int);
        }
    }
    putchar('\"' as i32);
}
unsafe extern "C" fn print_timestamp(mut high: dword, mut low: dword) {
    /* TODO */
}
#[no_mangle]
pub static mut rsrc_types: [*const libc::c_char; 19] =
    [0 as *const libc::c_char,
     b"Cursor\x00" as *const u8 as *const libc::c_char,
     b"Bitmap\x00" as *const u8 as *const libc::c_char,
     b"Icon\x00" as *const u8 as *const libc::c_char,
     b"Menu\x00" as *const u8 as *const libc::c_char,
     b"Dialog box\x00" as *const u8 as *const libc::c_char,
     b"String\x00" as *const u8 as *const libc::c_char,
     b"Font directory\x00" as *const u8 as *const libc::c_char,
     b"Font component\x00" as *const u8 as *const libc::c_char,
     b"Accelerator table\x00" as *const u8 as *const libc::c_char,
     b"Resource data\x00" as *const u8 as *const libc::c_char,
     b"Message table\x00" as *const u8 as *const libc::c_char,
     b"Cursor directory\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char,
     b"Icon directory\x00" as *const u8 as *const libc::c_char,
     b"Name table\x00" as *const u8 as *const libc::c_char,
     b"Version\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char];
// Initialized in run_static_initializers
#[no_mangle]
pub static mut rsrc_types_count: size_t = 0;
static mut rsrc_bmp_compression: [*const libc::c_char; 15] =
    [b"none\x00" as *const u8 as *const libc::c_char,
     b"RLE (8 bpp)\x00" as *const u8 as *const libc::c_char,
     b"RLE (4 bpp)\x00" as *const u8 as *const libc::c_char,
     b"RGB bit field masks\x00" as *const u8 as *const libc::c_char,
     b"JPEG\x00" as *const u8 as *const libc::c_char,
     b"PNG\x00" as *const u8 as *const libc::c_char,
     b"RGBA bit field masks\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     b"none (CMYK)\x00" as *const u8 as *const libc::c_char,
     b"RLE (8 bpp, CMYK)\x00" as *const u8 as *const libc::c_char,
     b"RLE (4 bpp, CMYK)\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char];
unsafe extern "C" fn print_rsrc_flags(mut flags: word) {
    if flags as libc::c_int & 0x10i32 != 0 {
        printf(b", moveable\x00" as *const u8 as *const libc::c_char);
    }
    if flags as libc::c_int & 0x20i32 != 0 {
        printf(b", shareable\x00" as *const u8 as *const libc::c_char);
    }
    if flags as libc::c_int & 0x40i32 != 0 {
        printf(b", preloaded\x00" as *const u8 as *const libc::c_char);
    }
    if flags as libc::c_int & 0xff8fi32 != 0 {
        printf(b", (unknown flags 0x%04x)\x00" as *const u8 as
                   *const libc::c_char,
               flags as libc::c_int & 0xff8fi32);
    };
}
/* There are a lot of styles here and most of them would require longer
 * descriptions, so we're just going to use the C names.
 * Not all of these are dialog box-related, but I'm not going to try to
 * sort through them. */
static mut rsrc_dialog_style: [*const libc::c_char; 33] =
    [b"DS_ABSALIGN\x00" as *const u8 as *const libc::c_char,
     b"DS_SYSMODAL\x00" as *const u8 as *const libc::c_char,
     b"DS_3DLOOK\x00" as *const u8 as *const libc::c_char,
     b"DS_FIXEDSYS\x00" as *const u8 as *const libc::c_char,
     b"DS_NOFAILCREATE\x00" as *const u8 as *const libc::c_char,
     b"DS_LOCALEDIT\x00" as *const u8 as *const libc::c_char,
     b"DS_SETFONT\x00" as *const u8 as *const libc::c_char,
     b"DS_MODALFRAME\x00" as *const u8 as *const libc::c_char,
     b"DS_NOIDLEMSG\x00" as *const u8 as *const libc::c_char,
     b"DS_SETFOREGROUND\x00" as *const u8 as *const libc::c_char,
     b"DS_CONTROL\x00" as *const u8 as *const libc::c_char,
     b"DS_CENTER\x00" as *const u8 as *const libc::c_char,
     b"DS_CENTERMOUSE\x00" as *const u8 as *const libc::c_char,
     b"DS_CONTEXTHELP\x00" as *const u8 as *const libc::c_char,
     b"(unrecognized flag 0x00004000)\x00" as *const u8 as
         *const libc::c_char,
     b"DS_USEPIXELS\x00" as *const u8 as *const libc::c_char,
     b"WS_TABSTOP\x00" as *const u8 as *const libc::c_char,
     b"WS_GROUP\x00" as *const u8 as *const libc::c_char,
     b"WS_THICKFRAME\x00" as *const u8 as *const libc::c_char,
     b"WS_SYSMENU\x00" as *const u8 as *const libc::c_char,
     b"WS_HSCROLL\x00" as *const u8 as *const libc::c_char,
     b"WS_VSCROLL\x00" as *const u8 as *const libc::c_char,
     b"WS_DLGFRAME\x00" as *const u8 as *const libc::c_char,
     b"WS_BORDER\x00" as *const u8 as *const libc::c_char,
     b"WS_MAXIMIZE\x00" as *const u8 as *const libc::c_char,
     b"WS_CLIPCHILDREN\x00" as *const u8 as *const libc::c_char,
     b"WS_CLIPSIBLINGS\x00" as *const u8 as *const libc::c_char,
     b"WS_DISABLED\x00" as *const u8 as *const libc::c_char,
     b"WS_VISIBLE\x00" as *const u8 as *const libc::c_char,
     b"WS_MINIMIZE\x00" as *const u8 as *const libc::c_char,
     b"WS_CHILD\x00" as *const u8 as *const libc::c_char,
     b"WS_POPUP\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char];
unsafe extern "C" fn print_rsrc_dialog_style(mut flags: dword) {
    
     let mut buffer =  [0; 1024];
    buffer[0usize] = 0i8;
      let mut i =   0i32;
    while i < 32i32 {
        if flags & ((1i32) << i) as libc::c_uint != 0 {
            strcat(buffer.as_mut_ptr(),
                   b", \x00" as *const u8 as *const libc::c_char);
            strcat(buffer.as_mut_ptr(), rsrc_dialog_style[i as usize]);
        }
        i += 1
    }
    printf(b"    Style: %s\n\x00" as *const u8 as *const libc::c_char,
           buffer.as_mut_ptr().offset(2isize));
}
static mut rsrc_button_type: [*const libc::c_char; 17] =
    [b"BS_PUSHBUTTON\x00" as *const u8 as *const libc::c_char,
     b"BS_DEFPUSHBUTTON\x00" as *const u8 as *const libc::c_char,
     b"BS_CHECKBOX\x00" as *const u8 as *const libc::c_char,
     b"BS_AUTOCHECKBOX\x00" as *const u8 as *const libc::c_char,
     b"BS_RADIOBUTTON\x00" as *const u8 as *const libc::c_char,
     b"BS_3STATE\x00" as *const u8 as *const libc::c_char,
     b"BS_AUTO3STATE\x00" as *const u8 as *const libc::c_char,
     b"BS_GROUPBOX\x00" as *const u8 as *const libc::c_char,
     b"BS_USERBUTTON\x00" as *const u8 as *const libc::c_char,
     b"BS_AUTORADIOBUTTON\x00" as *const u8 as *const libc::c_char,
     b"BS_PUSHBOX\x00" as *const u8 as *const libc::c_char,
     b"BS_OWNERDRAW\x00" as *const u8 as *const libc::c_char,
     b"(unknown type 12)\x00" as *const u8 as *const libc::c_char,
     b"(unknown type 13)\x00" as *const u8 as *const libc::c_char,
     b"(unknown type 14)\x00" as *const u8 as *const libc::c_char,
     b"(unknown type 15)\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char];
static mut rsrc_edit_style: [*const libc::c_char; 17] =
    [0 as *const libc::c_char, 0 as *const libc::c_char,
     b"ES_MULTILINE\x00" as *const u8 as *const libc::c_char,
     b"ES_UPPERCASE\x00" as *const u8 as *const libc::c_char,
     b"ES_LOWERCASE\x00" as *const u8 as *const libc::c_char,
     b"ES_PASSWORD\x00" as *const u8 as *const libc::c_char,
     b"ES_AUTOVSCROLL\x00" as *const u8 as *const libc::c_char,
     b"ES_AUTOHSCROLL\x00" as *const u8 as *const libc::c_char,
     b"ES_NOHIDESEL\x00" as *const u8 as *const libc::c_char,
     b"ES_COMBO\x00" as *const u8 as *const libc::c_char,
     b"ES_OEMCONVERT\x00" as *const u8 as *const libc::c_char,
     b"ES_READONLY\x00" as *const u8 as *const libc::c_char,
     b"ES_WANTRETURN\x00" as *const u8 as *const libc::c_char,
     b"ES_NUMBER\x00" as *const u8 as *const libc::c_char,
     b"(unknown flag 0x4000)\x00" as *const u8 as *const libc::c_char,
     b"(unknown flag 0x8000)\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char];
static mut rsrc_static_type: [*const libc::c_char; 20] =
    [b"SS_LEFT\x00" as *const u8 as *const libc::c_char,
     b"SS_CENTER\x00" as *const u8 as *const libc::c_char,
     b"SS_RIGHT\x00" as *const u8 as *const libc::c_char,
     b"SS_ICON\x00" as *const u8 as *const libc::c_char,
     b"SS_BLACKRECT\x00" as *const u8 as *const libc::c_char,
     b"SS_GRAYRECT\x00" as *const u8 as *const libc::c_char,
     b"SS_WHITERECT\x00" as *const u8 as *const libc::c_char,
     b"SS_BLACKFRAME\x00" as *const u8 as *const libc::c_char,
     b"SS_GRAYFRAME\x00" as *const u8 as *const libc::c_char,
     b"SS_WHITEFRAME\x00" as *const u8 as *const libc::c_char,
     b"SS_USERITEM\x00" as *const u8 as *const libc::c_char,
     b"SS_SIMPLE\x00" as *const u8 as *const libc::c_char,
     b"SS_LEFTNOWORDWRAP\x00" as *const u8 as *const libc::c_char,
     b"SS_OWNERDRAW\x00" as *const u8 as *const libc::c_char,
     b"SS_BITMAP\x00" as *const u8 as *const libc::c_char,
     b"SS_ENHMETAFILE\x00" as *const u8 as *const libc::c_char,
     b"SS_ETCHEDHORZ\x00" as *const u8 as *const libc::c_char,
     b"SS_ETCHEDVERT\x00" as *const u8 as *const libc::c_char,
     b"SS_ETCHEDFRAME\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char];
static mut rsrc_static_style: [*const libc::c_char; 15] =
    [0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char,
     b"(unknown flag 0x0020)\x00" as *const u8 as *const libc::c_char,
     b"SS_REALSIZECONTROL\x00" as *const u8 as *const libc::c_char,
     b"SS_NOPREFIX\x00" as *const u8 as *const libc::c_char,
     b"SS_NOTIFY\x00" as *const u8 as *const libc::c_char,
     b"SS_CENTERIMAGE\x00" as *const u8 as *const libc::c_char,
     b"SS_RIGHTJUST\x00" as *const u8 as *const libc::c_char,
     b"SS_REALSIZEIMAGE\x00" as *const u8 as *const libc::c_char,
     b"SS_SUNKEN\x00" as *const u8 as *const libc::c_char,
     b"SS_EDITCONTROL\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char];
static mut rsrc_listbox_style: [*const libc::c_char; 17] =
    [b"LBS_NOTIFY\x00" as *const u8 as *const libc::c_char,
     b"LBS_SORT\x00" as *const u8 as *const libc::c_char,
     b"LBS_NOREDRAW\x00" as *const u8 as *const libc::c_char,
     b"LBS_MULTIPLESEL\x00" as *const u8 as *const libc::c_char,
     b"LBS_OWNERDRAWFIXED\x00" as *const u8 as *const libc::c_char,
     b"LBS_OWNERDRAWVARIABLE\x00" as *const u8 as *const libc::c_char,
     b"LBS_HASSTRINGS\x00" as *const u8 as *const libc::c_char,
     b"LBS_USETABSTOPS\x00" as *const u8 as *const libc::c_char,
     b"LBS_NOINTEGRALHEIGHT\x00" as *const u8 as *const libc::c_char,
     b"LBS_MULTICOLUMN\x00" as *const u8 as *const libc::c_char,
     b"LBS_WANTKEYBOARDINPUT\x00" as *const u8 as *const libc::c_char,
     b"LBS_EXTENDEDSEL\x00" as *const u8 as *const libc::c_char,
     b"LBS_DISABLENOSCROLL\x00" as *const u8 as *const libc::c_char,
     b"LBS_NODATA\x00" as *const u8 as *const libc::c_char,
     b"LBS_NOSEL\x00" as *const u8 as *const libc::c_char,
     b"LBS_COMBOBOX\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char];
static mut rsrc_combobox_style: [*const libc::c_char; 16] =
    [0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     b"CBS_OWNERDRAWFIXED\x00" as *const u8 as *const libc::c_char,
     b"CBS_OWNERDRAWVARIABLE\x00" as *const u8 as *const libc::c_char,
     b"CBS_AUTOHSCROLL\x00" as *const u8 as *const libc::c_char,
     b"CBS_OEMCONVERT\x00" as *const u8 as *const libc::c_char,
     b"CBS_SORT\x00" as *const u8 as *const libc::c_char,
     b"CBS_HASSTRINGS\x00" as *const u8 as *const libc::c_char,
     b"CBS_NOINTEGRALHEIGHT\x00" as *const u8 as *const libc::c_char,
     b"CBS_DISABLENOSCROLL\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char,
     b"CBS_UPPERCASE\x00" as *const u8 as *const libc::c_char,
     b"CBS_LOWERCASE\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char];
unsafe extern "C" fn print_rsrc_control_style(mut class: byte,
                                              mut flags: dword) {
    
     let mut i =  0; let mut buffer =  [0; 1024];
    buffer[0usize] = 0i8;
    printf(b"        Style: \x00" as *const u8 as *const libc::c_char);
    match class as libc::c_int {
        128 => {
            /* Button */
            strcpy(buffer.as_mut_ptr(),
                   rsrc_button_type[(flags &
                                         0xfu32)
                                        as usize]);
            if flags & 0x10u32 != 0 {
                strcat(buffer.as_mut_ptr(),
                       b", (unknown flag 0x0010)\x00" as *const u8 as
                           *const libc::c_char);
            }
            if flags & 0x20u32 != 0 {
                strcat(buffer.as_mut_ptr(),
                       b", BS_LEFTTEXT\x00" as *const u8 as
                           *const libc::c_char);
            }
            if flags & 0x40u32 ==
                   0u32 {
                strcat(buffer.as_mut_ptr(),
                       b", BS_TEXT\x00" as *const u8 as *const libc::c_char);
            } else {
                if flags & 0x40u32 != 0 {
                    strcat(buffer.as_mut_ptr(),
                           b", BS_ICON\x00" as *const u8 as
                               *const libc::c_char);
                }
                if flags & 0x80u32 != 0 {
                    strcat(buffer.as_mut_ptr(),
                           b", BS_BITMAP\x00" as *const u8 as
                               *const libc::c_char);
                }
            }
            if flags & 0x300u32 ==
                   0x100u32 {
                strcat(buffer.as_mut_ptr(),
                       b", BS_LEFT\x00" as *const u8 as *const libc::c_char);
            } else if flags & 0x300u32 ==
                          0x200u32 {
                strcat(buffer.as_mut_ptr(),
                       b", BS_RIGHT\x00" as *const u8 as *const libc::c_char);
            } else if flags & 0x300u32 ==
                          0x300u32 {
                strcat(buffer.as_mut_ptr(),
                       b", BS_CENTER\x00" as *const u8 as
                           *const libc::c_char);
            }
            if flags & 0xc00u32 ==
                   0x400u32 {
                strcat(buffer.as_mut_ptr(),
                       b", BS_TOP\x00" as *const u8 as *const libc::c_char);
            } else if flags & 0xc00u32 ==
                          0x800u32 {
                strcat(buffer.as_mut_ptr(),
                       b", BS_BOTTOM\x00" as *const u8 as
                           *const libc::c_char);
            } else if flags & 0xc00u32 ==
                          0xc00u32 {
                strcat(buffer.as_mut_ptr(),
                       b", BS_VCENTER\x00" as *const u8 as
                           *const libc::c_char);
            }
            if flags & 0x1000u32 != 0 {
                strcat(buffer.as_mut_ptr(),
                       b", BS_PUSHLIKE\x00" as *const u8 as
                           *const libc::c_char);
            }
            if flags & 0x2000u32 != 0 {
                strcat(buffer.as_mut_ptr(),
                       b", BS_MULTILINE\x00" as *const u8 as
                           *const libc::c_char);
            }
            if flags & 0x4000u32 != 0 {
                strcat(buffer.as_mut_ptr(),
                       b", BS_NOTIFY\x00" as *const u8 as
                           *const libc::c_char);
            }
            if flags & 0x8000u32 != 0 {
                strcat(buffer.as_mut_ptr(),
                       b", BS_FLAT\x00" as *const u8 as *const libc::c_char);
            }
        }
        129 => {
            /* Edit */
            if flags & 3u32 ==
                   0u32 {
                strcpy(buffer.as_mut_ptr(),
                       b"ES_LEFT\x00" as *const u8 as *const libc::c_char);
            } else if flags & 3u32 ==
                          1u32 {
                strcpy(buffer.as_mut_ptr(),
                       b"ES_CENTER\x00" as *const u8 as *const libc::c_char);
            } else if flags & 3u32 ==
                          2u32 {
                strcpy(buffer.as_mut_ptr(),
                       b"ES_RIGHT\x00" as *const u8 as *const libc::c_char);
            } else if flags & 3u32 ==
                          3u32 {
                strcpy(buffer.as_mut_ptr(),
                       b"(unknown type 3)\x00" as *const u8 as
                           *const libc::c_char);
            }
            
             for i in  2i32.. 16i32 {
    
                if flags & ((1i32) << i) as libc::c_uint != 0 {
                    strcat(buffer.as_mut_ptr(),
                           b", \x00" as *const u8 as *const libc::c_char);
                    strcat(buffer.as_mut_ptr(), rsrc_edit_style[i as usize]);
                }
}
        }
        130 => {
            /* Static */
            if flags & 0x1fu32 <=
                   0x12u32 {
                strcpy(buffer.as_mut_ptr(),
                       rsrc_static_type[(flags &
                                             0x1fu32) as usize]);
            } else {
                sprintf(buffer.as_mut_ptr(),
                        b"(unknown type %d)\x00" as *const u8 as
                            *const libc::c_char,
                        flags & 0x1fu32);
            }
            
             for i in  5i32.. 14i32 {
    
                if flags & ((1i32) << i) as libc::c_uint != 0 {
                    strcat(buffer.as_mut_ptr(),
                           b", \x00" as *const u8 as *const libc::c_char);
                    strcat(buffer.as_mut_ptr(),
                           rsrc_static_style[i as usize]);
                }
}
        }
        131 => {
            
             for i in  0i32.. 16i32 {
    
                if flags & ((1i32) << i) as libc::c_uint != 0 {
                    strcat(buffer.as_mut_ptr(),
                           b", \x00" as *const u8 as *const libc::c_char);
                    strcat(buffer.as_mut_ptr(),
                           rsrc_listbox_style[i as usize]);
                }
}
        }
        132 => {
            /* ScrollBar */
            if flags & 0x18u32 != 0 {
                if flags & 0x8u32 != 0 {
                    strcpy(buffer.as_mut_ptr(),
                           b"SBS_SIZEBOX\x00" as *const u8 as
                               *const libc::c_char);
                } else if flags & 0x10u32 != 0 {
                    strcpy(buffer.as_mut_ptr(),
                           b"SBS_SIZEGRIP\x00" as *const u8 as
                               *const libc::c_char);
                }
                if flags & 0x2u32 != 0 {
                    strcat(buffer.as_mut_ptr(),
                           b", SBS_SIZEBOXTOPLEFTALIGN\x00" as *const u8 as
                               *const libc::c_char);
                }
                if flags & 0x4u32 != 0 {
                    strcat(buffer.as_mut_ptr(),
                           b", SBS_SIZEBOXBOTTOMRIGHTALIGN\x00" as *const u8
                               as *const libc::c_char);
                }
            } else if flags & 0x1u32 != 0 {
                strcpy(buffer.as_mut_ptr(),
                       b"SBS_VERT\x00" as *const u8 as *const libc::c_char);
                if flags & 0x2u32 != 0 {
                    strcat(buffer.as_mut_ptr(),
                           b", SBS_LEFTALIGN\x00" as *const u8 as
                               *const libc::c_char);
                }
                if flags & 0x4u32 != 0 {
                    strcat(buffer.as_mut_ptr(),
                           b", SBS_RIGHTALIGN\x00" as *const u8 as
                               *const libc::c_char);
                }
            } else {
                strcpy(buffer.as_mut_ptr(),
                       b"SBS_HORZ\x00" as *const u8 as *const libc::c_char);
                if flags & 0x2u32 != 0 {
                    strcat(buffer.as_mut_ptr(),
                           b", SBS_TOPALIGN\x00" as *const u8 as
                               *const libc::c_char);
                }
                if flags & 0x4u32 != 0 {
                    strcat(buffer.as_mut_ptr(),
                           b", SBS_BOTTOMALIGN\x00" as *const u8 as
                               *const libc::c_char);
                }
            }
            if flags & 0xffe0u32 != 0 {
                sprintf(buffer.as_mut_ptr().offset(strlen(buffer.as_mut_ptr())
                                                       as isize),
                        b", (unknown flags 0x%04x)\x00" as *const u8 as
                            *const libc::c_char,
                        flags & 0xffe0u32);
            }
        }
        133 => {
            /* ComboBox */
            if flags & 3u32 ==
                   1u32 {
                strcat(buffer.as_mut_ptr(),
                       b", CBS_SIMPLE\x00" as *const u8 as
                           *const libc::c_char);
            } else if flags & 3u32 ==
                          2u32 {
                strcat(buffer.as_mut_ptr(),
                       b", CBS_DROPDOWN\x00" as *const u8 as
                           *const libc::c_char);
            } else if flags & 3u32 ==
                          3u32 {
                strcat(buffer.as_mut_ptr(),
                       b", CBS_DROPDOWNLIST\x00" as *const u8 as
                           *const libc::c_char);
            }
            
             for i in  4i32.. 15i32 {
    
                if flags & ((1i32) << i) as libc::c_uint != 0 &&
                       !rsrc_combobox_style[i as usize].is_null() {
                    strcat(buffer.as_mut_ptr(),
                           b", \x00" as *const u8 as *const libc::c_char);
                    strcat(buffer.as_mut_ptr(),
                           rsrc_combobox_style[i as usize]);
                }
}
            if flags & 0x900cu32 != 0 {
                sprintf(buffer.as_mut_ptr().offset(strlen(buffer.as_mut_ptr())
                                                       as isize),
                        b", (unknown flags 0x%04x)\x00" as *const u8 as
                            *const libc::c_char,
                        flags & 0x900cu32);
            }
        }
        _ => {
            sprintf(buffer.as_mut_ptr(),
                    b"0x%04x\x00" as *const u8 as *const libc::c_char,
                    flags & 0xffffu32);
        }
    }
    
     for i in  16i32.. 32i32 {
    
        if flags & ((1i32) << i) as libc::c_uint != 0 {
            strcat(buffer.as_mut_ptr(),
                   b", \x00" as *const u8 as *const libc::c_char);
            strcat(buffer.as_mut_ptr(), rsrc_dialog_style[i as usize]);
        }
}
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           if buffer[0usize] as libc::c_int == ',' as i32 {
               buffer.as_mut_ptr().offset(2isize)
           } else { buffer.as_mut_ptr() });
}
static mut rsrc_dialog_class: [*const libc::c_char; 7] =
    [b"Button\x00" as *const u8 as *const libc::c_char,
     b"Edit\x00" as *const u8 as *const libc::c_char,
     b"Static\x00" as *const u8 as *const libc::c_char,
     b"ListBox\x00" as *const u8 as *const libc::c_char,
     b"ScrollBar\x00" as *const u8 as *const libc::c_char,
     b"ComboBox\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char];
unsafe extern "C" fn print_rsrc_menu_items(mut depth: libc::c_int) {
    
    
    
     let mut flags =  0; let mut id =  0; let mut buffer =  [0; 1024]; let mut i =  0;
    loop  {
        flags = read_word();
        printf(b"        \x00" as *const u8 as *const libc::c_char);
        
         for i in  0i32.. depth {
    
            printf(b"  \x00" as *const u8 as *const libc::c_char);
}
        if flags as libc::c_int & 0x10i32 == 0 {
            /* item ID */
            id = read_word();
            printf(b"%d: \x00" as *const u8 as *const libc::c_char,
                   id as libc::c_int);
        }
        print_escaped_string0();
        /* and print flags */
        buffer[0usize] =  '\u{0}' as libc::c_char;
        if flags as libc::c_int & 0x1i32 != 0 {
            strcat(buffer.as_mut_ptr(),
                   b", grayed\x00" as *const u8 as *const libc::c_char);
        }
        if flags as libc::c_int & 0x2i32 != 0 {
            strcat(buffer.as_mut_ptr(),
                   b", inactive\x00" as *const u8 as *const libc::c_char);
        }
        if flags as libc::c_int & 0x4i32 != 0 {
            strcat(buffer.as_mut_ptr(),
                   b", bitmap\x00" as *const u8 as *const libc::c_char);
        }
        if flags as libc::c_int & 0x8i32 != 0 {
            strcat(buffer.as_mut_ptr(),
                   b", checked\x00" as *const u8 as *const libc::c_char);
        }
        if flags as libc::c_int & 0x10i32 != 0 {
            strcat(buffer.as_mut_ptr(),
                   b", popup\x00" as *const u8 as *const libc::c_char);
        }
        if flags as libc::c_int & 0x20i32 != 0 {
            strcat(buffer.as_mut_ptr(),
                   b", menu bar break\x00" as *const u8 as
                       *const libc::c_char);
        }
        if flags as libc::c_int & 0x40i32 != 0 {
            strcat(buffer.as_mut_ptr(),
                   b", menu break\x00" as *const u8 as *const libc::c_char);
        }
        /* don't print ENDMENU */
        if flags as libc::c_int & 0xff00i32 != 0 {
            sprintf(buffer.as_mut_ptr().offset(strlen(buffer.as_mut_ptr()) as
                                                   isize),
                    b", unknown flags 0x%04x\x00" as *const u8 as
                        *const libc::c_char,
                    flags as libc::c_int & 0xff00i32);
        }
        if buffer[0usize] != 0 {
            printf(b" (%s)\x00" as *const u8 as *const libc::c_char,
                   buffer.as_mut_ptr().offset(2isize));
        }
        putchar('\n' as i32);
        /* if we have a popup, recurse */
        if flags as libc::c_int & 0x10i32 != 0 {
            print_rsrc_menu_items(depth + 1i32);
        }
        if flags as libc::c_int & 0x80i32 != 0 { break ; }
    };
}
static mut rsrc_version_file: [*const libc::c_char; 7] =
    [b"VS_FF_DEBUG\x00" as *const u8 as *const libc::c_char,
     b"VS_FF_PRERELEASE\x00" as *const u8 as *const libc::c_char,
     b"VS_FF_PATCHED\x00" as *const u8 as *const libc::c_char,
     b"VS_FF_PRIVATEBUILD\x00" as *const u8 as *const libc::c_char,
     b"VS_FF_INFOINFERRED\x00" as *const u8 as *const libc::c_char,
     b"VS_FF_SPECIALBUILD\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char];
static mut rsrc_version_type: [*const libc::c_char; 9] =
    [b"unknown\x00" as *const u8 as *const libc::c_char,
     b"application\x00" as *const u8 as *const libc::c_char,
     b"DLL\x00" as *const u8 as *const libc::c_char,
     b"device driver\x00" as *const u8 as *const libc::c_char,
     b"font\x00" as *const u8 as *const libc::c_char,
     b"virtual device\x00" as *const u8 as *const libc::c_char,
     b"(unknown type 6)\x00" as *const u8 as *const libc::c_char,
     b"static-link library\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char];
static mut rsrc_version_subtype_drv: [*const libc::c_char; 14] =
    [b"unknown\x00" as *const u8 as *const libc::c_char,
     b"printer\x00" as *const u8 as *const libc::c_char,
     b"keyboard\x00" as *const u8 as *const libc::c_char,
     b"language\x00" as *const u8 as *const libc::c_char,
     b"display\x00" as *const u8 as *const libc::c_char,
     b"mouse\x00" as *const u8 as *const libc::c_char,
     b"network\x00" as *const u8 as *const libc::c_char,
     b"system\x00" as *const u8 as *const libc::c_char,
     b"installable\x00" as *const u8 as *const libc::c_char,
     b"sound\x00" as *const u8 as *const libc::c_char,
     b"communications\x00" as *const u8 as *const libc::c_char,
     b"input method\x00" as *const u8 as *const libc::c_char,
     b"versioned printer\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char];
unsafe extern "C" fn print_rsrc_version_flags(mut header: version_header) {
     let mut buffer =  [0; 1024];
    
    buffer[0usize] =  '\u{0}' as libc::c_char;
      let mut i =   0i32;
    while i < 6i32 {
        if header.flags_file & ((1i32) << i) as libc::c_uint != 0
           {
            strcat(buffer.as_mut_ptr(),
                   b", \x00" as *const u8 as *const libc::c_char);
            strcat(buffer.as_mut_ptr(), rsrc_version_file[i as usize]);
        }
        i += 1
    }
    if header.flags_file & 0xffc0u32 != 0 {
        sprintf(buffer.as_mut_ptr().offset(strlen(buffer.as_mut_ptr()) as
                                               isize),
                b", (unknown flags 0x%04x)\x00" as *const u8 as
                    *const libc::c_char,
                header.flags_file & 0xffc0u32);
    }
    printf(b"    File flags: \x00" as *const u8 as *const libc::c_char);
    if header.flags_file != 0 {
        printf(b"%s\x00" as *const u8 as *const libc::c_char,
               buffer.as_mut_ptr().offset(2isize));
    }
    buffer[0usize] =  '\u{0}' as libc::c_char;
    if header.flags_os == 0u32 {
        strcpy(buffer.as_mut_ptr(),
               b", VOS_UNKNOWN\x00" as *const u8 as *const libc::c_char);
    } else {
        match header.flags_os & 0xffffu32 {
            1 => {
                strcpy(buffer.as_mut_ptr(),
                       b", VOS__WINDOWS16\x00" as *const u8 as
                           *const libc::c_char);
            }
            2 => {
                strcpy(buffer.as_mut_ptr(),
                       b", VOS__PM16\x00" as *const u8 as
                           *const libc::c_char);
            }
            3 => {
                strcpy(buffer.as_mut_ptr(),
                       b", VOS__PM32\x00" as *const u8 as
                           *const libc::c_char);
            }
            4 => {
                strcpy(buffer.as_mut_ptr(),
                       b", VOS__WINDOWS32\x00" as *const u8 as
                           *const libc::c_char);
            }
            _ => {
                sprintf(buffer.as_mut_ptr(),
                        b", (unknown OS 0x%04x)\x00" as *const u8 as
                            *const libc::c_char,
                        header.flags_os &
                            0xffffu32);
            }
        }
        match header.flags_os >> 16i32 {
            1 => {
                strcat(buffer.as_mut_ptr(),
                       b", VOS_DOS\x00" as *const u8 as *const libc::c_char);
            }
            2 => {
                strcat(buffer.as_mut_ptr(),
                       b", VOS_OS216\x00" as *const u8 as
                           *const libc::c_char);
            }
            3 => {
                strcat(buffer.as_mut_ptr(),
                       b", VOS_OS232\x00" as *const u8 as
                           *const libc::c_char);
            }
            4 => {
                strcat(buffer.as_mut_ptr(),
                       b", VOS_NT\x00" as *const u8 as *const libc::c_char);
            }
            5 => {
                strcat(buffer.as_mut_ptr(),
                       b", VOS_WINCE\x00" as *const u8 as
                           *const libc::c_char);
            }
            _ => {
                sprintf(buffer.as_mut_ptr().offset(strlen(buffer.as_mut_ptr())
                                                       as isize),
                        b", (unknown OS 0x%04x)\x00" as *const u8 as
                            *const libc::c_char,
                        header.flags_os >> 16i32);
            }
        }
    }
    printf(b"\n    OS flags: %s\n\x00" as *const u8 as *const libc::c_char,
           buffer.as_mut_ptr().offset(2isize));
    if header.flags_type <= 7u32 {
        printf(b"    Type: %s\n\x00" as *const u8 as *const libc::c_char,
               rsrc_version_type[header.flags_type as usize]);
    } else {
        printf(b"    Type: (unknown type %d)\n\x00" as *const u8 as
                   *const libc::c_char, header.flags_type);
    }
    if header.flags_type == 3u32 {
        /* driver */
        if header.flags_subtype <= 12u32 {
            printf(b"    Subtype: %s driver\n\x00" as *const u8 as
                       *const libc::c_char,
                   rsrc_version_subtype_drv[header.flags_subtype as usize]);
        } else {
            printf(b"    Subtype: (unknown subtype %d)\n\x00" as *const u8 as
                       *const libc::c_char, header.flags_subtype);
        }
    } else if header.flags_type == 4u32 {
        /* font */
        if header.flags_subtype == 0u32 {
            printf(b"    Subtype: unknown font\n\x00" as *const u8 as
                       *const libc::c_char);
        } else if header.flags_subtype == 1u32 {
            printf(b"    Subtype: raster font\n\x00" as *const u8 as
                       *const libc::c_char);
        } else if header.flags_subtype == 2u32 {
            printf(b"    Subtype: vector font\n\x00" as *const u8 as
                       *const libc::c_char);
        } else if header.flags_subtype == 3u32 {
            printf(b"    Subtype: TrueType font\n\x00" as *const u8 as
                       *const libc::c_char);
        } else {
            printf(b"    Subtype: (unknown subtype %d)\n\x00" as *const u8 as
                       *const libc::c_char, header.flags_subtype);
        }
    } else if header.flags_type == 5u32 {
        /* VXD */
        printf(b"    Virtual device ID: %d\n\x00" as *const u8 as
                   *const libc::c_char, header.flags_subtype);
    } else if header.flags_subtype != 0 {
        /* according to MSDN nothing else is valid */
        printf(b"    Subtype: (unknown subtype %d)\n\x00" as *const u8 as
                   *const libc::c_char, header.flags_subtype);
    };
}
unsafe extern "C" fn print_rsrc_strings(mut end: libc::c_long) {
     let mut length =  0;
    while ftell(f) < end {
        /* first length is redundant */
        fseek(f,
              
              ::std::mem::size_of::<word>() as libc::c_long,
              1i32);
        length = read_word();
        printf(b"        \x00" as *const u8 as *const libc::c_char);
        print_escaped_string0();
        skip_padding(4i8);
        printf(b": \x00" as *const u8 as *const libc::c_char);
        /* According to MSDN this is zero-terminated, and in most cases it is.
         * However, at least one application (msbsolar) has NEs with what
         * appears to be a non-zero-terminated string. In Windows this is cut
         * off at one minus the given length, just like other strings, so
         * we'll do that here. */
        print_escaped_string((length as libc::c_int - 1i32) as
                                 libc::c_long); /* and skip the zero */
        fseek(f, 1i64, 1i32);
        skip_padding(4i8);
        putchar('\n' as i32);
    };
}
unsafe extern "C" fn print_rsrc_stringfileinfo(mut end: libc::c_long) {
    
    
    
     let mut cursor =  0; let mut length =  0; let mut lang =  0u32; let mut codepage =  0u32;
    loop 
         /* we already processed the StringFileInfo header */
         {
        cursor = ftell(f);
        if !(cursor < end) { break ; }
        /* StringTable header */
        length = read_word(); /* ValueLength, always 0 */
        fseek(f,
              
              ::std::mem::size_of::<word>() as libc::c_long,
              1i32);
        /* codepage and language code */
        fscanf(f, b"%4x%4x\x00" as *const u8 as *const libc::c_char,
               &mut lang as *mut libc::c_uint,
               &mut codepage as *mut libc::c_uint); /* padding */
        printf(b"    String table (lang=%04x, codepage=%04x):\n\x00" as
                   *const u8 as *const libc::c_char, lang, codepage);
        fseek(f,
              (4u64).wrapping_mul(::std::mem::size_of::<byte>()
                                                   as libc::c_ulong) as
                  libc::c_long, 1i32);
        print_rsrc_strings(cursor + length as libc::c_long);
    };
}
unsafe extern "C" fn print_rsrc_varfileinfo(mut end: libc::c_long) {
     let mut length =  0;
    while ftell(f) < end {
        /* first length is redundant */
        length = read_word(); /* Translation\0 */
        fseek(f,
              (12u64).wrapping_mul(::std::mem::size_of::<byte>()
                                                   as libc::c_ulong) as
                  libc::c_long, 1i32);
        loop  {
            length = (length as libc::c_int - 4i32) as word;
            if !(length != 0) { break ; }
            printf(b"    Var (lang=%04x, codepage=%04x)\n\x00" as *const u8 as
                       *const libc::c_char, read_word() as libc::c_int,
                   read_word() as libc::c_int);
        }
    };
}
unsafe extern "C" fn print_rsrc_resource(mut type_0: word,
                                         mut offset: libc::c_long,
                                         mut length: libc::c_long,
                                         mut rn_id: word) {
    fseek(f, offset, 0i32);
    let mut current_block_166: u64;
    match type_0 as libc::c_int {
        32769 => {
            /* Cursor */
            printf(b"    Hotspot: (%d, %d)\n\x00" as *const u8 as
                       *const libc::c_char, read_word() as libc::c_int,
                   read_word() as libc::c_int);
            /* fall through */
            current_block_166 = 5358520172556667279;
        }
        32770 => { current_block_166 = 5358520172556667279; }
        32771 => { current_block_166 = 17778012151635330486; }
        32772 => {
            
             let mut extended =  read_word(); let mut offset_0 =  read_word();
            if extended as libc::c_int > 1i32 {
                fprintf(stderr,
                        b"Warning: Unknown menu version %d\n\x00" as *const u8
                            as *const libc::c_char, extended as libc::c_int);
            } else {
                printf(if extended as libc::c_int != 0 {
                           b"    Type: extended\n\x00" as *const u8 as
                               *const libc::c_char
                       } else {
                           b"    Type: standard\n\x00" as *const u8 as
                               *const libc::c_char
                       });
                if offset_0 as libc::c_int !=
                       extended as libc::c_int * 4i32 {
                    fprintf(stderr,
                            b"Warning: Unexpected offset value %d (expected %d)\n\x00"
                                as *const u8 as *const libc::c_char,
                            offset_0 as libc::c_int,
                            extended as libc::c_int * 4i32);
                }
                if extended != 0 {
                    printf(b"    Help ID: %d\n\x00" as *const u8 as
                               *const libc::c_char, read_dword());
                }
                printf(b"    Items:\n\x00" as *const u8 as
                           *const libc::c_char);
                print_rsrc_menu_items(0i32);
            }
            current_block_166 = 2885804587290726961;
        }
        32773 => {
            
            
             let mut font_size =  0; let mut style =  read_dword();
            print_rsrc_dialog_style(style);
              let mut count =   read_byte();
            printf(b"    Position: (%d, %d)\n\x00" as *const u8 as
                       *const libc::c_char, read_word() as libc::c_int,
                   read_word() as libc::c_int);
            printf(b"    Size: %dx%d\n\x00" as *const u8 as
                       *const libc::c_char, read_word() as libc::c_int,
                   read_word() as libc::c_int);
            if read_byte() as libc::c_int == 0xffi32 {
                printf(b"    Menu resource: #%d\x00" as *const u8 as
                           *const libc::c_char, read_word() as libc::c_int);
            } else {
                printf(b"    Menu name: \x00" as *const u8 as
                           *const libc::c_char);
                fseek(f,
                      (::std::mem::size_of::<byte>() as
                           libc::c_ulong).wrapping_neg() as libc::c_long,
                      1i32);
                print_escaped_string0();
            }
            printf(b"\n    Class name: \x00" as *const u8 as
                       *const libc::c_char);
            print_escaped_string0();
            printf(b"\n    Caption: \x00" as *const u8 as
                       *const libc::c_char);
            print_escaped_string0();
            if style & 0x40u32 != 0 {
                /* DS_SETFONT */
                font_size = read_word();
                printf(b"\n    Font: \x00" as *const u8 as
                           *const libc::c_char);
                print_escaped_string0();
                printf(b" (%d pt)\x00" as *const u8 as *const libc::c_char,
                       font_size as libc::c_int);
            }
            putchar('\n' as i32);
            loop  {
                let fresh1 = count;
                count = count.wrapping_sub(1);
                if !(fresh1 != 0) { break ; }
                 let mut control =
    
                    dialog_control{x: 0,
                                   y: 0,
                                   width: 0,
                                   height: 0,
                                   id: 0,
                                   style: 0,
                                   class: 0,};
                fread(&mut control as *mut dialog_control as
                          *mut libc::c_void,
                      ::std::mem::size_of::<dialog_control>() as
                          libc::c_ulong, 1u64, f);
                if control.class as libc::c_int & 0x80i32 != 0 {
                    if control.class as libc::c_int <= 0x85i32 {
                        printf(b"    %s\x00" as *const u8 as
                                   *const libc::c_char,
                               rsrc_dialog_class[(control.class as libc::c_int
                                                      &
                                                      !(0x80i32))
                                                     as usize]);
                    } else {
                        printf(b"    (unknown class %d)\x00" as *const u8 as
                                   *const libc::c_char,
                               control.class as libc::c_int);
                    }
                } else {
                    fseek(f,
                          (::std::mem::size_of::<byte>() as
                               libc::c_ulong).wrapping_neg() as libc::c_long,
                          1i32);
                    print_escaped_string0();
                }
                printf(b" %d:\n\x00" as *const u8 as *const libc::c_char,
                       control.id as libc::c_int);
                printf(b"        Position: (%d, %d)\n\x00" as *const u8 as
                           *const libc::c_char, control.x as libc::c_int,
                       control.y as libc::c_int);
                printf(b"        Size: %dx%d\n\x00" as *const u8 as
                           *const libc::c_char, control.width as libc::c_int,
                       control.height as libc::c_int);
                print_rsrc_control_style(control.class, control.style);
                if read_byte() as libc::c_int == 0xffi32 {
                    /* todo: we can check the style for SS_ICON/SS_BITMAP and *maybe* also
                 * refer back to a printed RT_GROUPICON/GROUPCUROR/BITMAP resource. */
                    printf(b"        Resource: #%d\x00" as *const u8 as
                               *const libc::c_char,
                           read_word() as libc::c_int);
                } else {
                    fseek(f,
                          (::std::mem::size_of::<byte>() as
                               libc::c_ulong).wrapping_neg() as libc::c_long,
                          1i32);
                    printf(b"        Text: \x00" as *const u8 as
                               *const libc::c_char);
                    print_escaped_string0();
                }
                /* todo: WINE parses this as "data", but all of my testcases return 0. */
                read_byte();
                putchar('\n' as i32);
            }
            current_block_166 = 2885804587290726961;
        }
        32774 => {
            
             let mut i =  0i32; let mut cursor =  0;
            loop  {
                cursor = ftell(f);
                if !(cursor < offset + length) { break ; }
                 let mut length_0 =  read_byte();
                if length_0 != 0 {
                    printf(b"    %3d (0x%06lx): \x00" as *const u8 as
                               *const libc::c_char,
                           i +
                               ((rn_id as libc::c_int &
                                     !(0x8000i32)) -
                                    1i32) * 16i32,
                           cursor);
                    print_escaped_string(length_0 as libc::c_long);
                    putchar('\n' as i32);
                }
                i += 1
            }
            current_block_166 = 2885804587290726961;
        }
        32780 | 32782 => {
            
            fseek(f,
                  (2u64).wrapping_mul(::std::mem::size_of::<word>()
                                                       as libc::c_ulong) as
                      libc::c_long, 1i32);
              let mut count_0 =   read_word();
            printf(b"    Resources: \x00" as *const u8 as
                       *const libc::c_char);
            let fresh2 = count_0;
            count_0 = count_0.wrapping_sub(1);
            if fresh2 != 0 {
                fseek(f,
                      (6u64).wrapping_mul(::std::mem::size_of::<word>()
                                                           as libc::c_ulong)
                          as libc::c_long, 1i32);
                printf(b"#%d\x00" as *const u8 as *const libc::c_char,
                       read_word() as libc::c_int);
            }
            loop  {
                let fresh3 = count_0;
                count_0 = count_0.wrapping_sub(1);
                if !(fresh3 != 0) { break ; }
                fseek(f,
                      (6u64).wrapping_mul(::std::mem::size_of::<word>()
                                                           as libc::c_ulong)
                          as libc::c_long, 1i32);
                printf(b", #%d\x00" as *const u8 as *const libc::c_char,
                       read_word() as libc::c_int);
            }
            printf(b"\n\x00" as *const u8 as *const libc::c_char);
            current_block_166 = 2885804587290726961;
        }
        32784 => {
             let mut header_0 =
    
                version_header{length: 0,
                               value_length: 0,
                               string: [0; 16],
                               magic: 0,
                               struct_2: 0,
                               struct_1: 0,
                               file_2: 0,
                               file_1: 0,
                               file_4: 0,
                               file_3: 0,
                               prod_2: 0,
                               prod_1: 0,
                               prod_4: 0,
                               prod_3: 0,
                               flags_file_mask: 0,
                               flags_file: 0,
                               flags_os: 0,
                               flags_type: 0,
                               flags_subtype: 0,
                               date_1: 0,
                               date_2: 0,}; /* for the String/VarFileInfo */
             /* fixme: what is this? */
            
            
            
            fread(&mut header_0 as *mut version_header as *mut libc::c_void,
                  ::std::mem::size_of::<version_header>() as libc::c_ulong,
                  1u64, f);
            if header_0.value_length as libc::c_int != 52i32 {
                fprintf(stderr,
                        b"Warning: Version header length is %d (expected 52).\n\x00"
                            as *const u8 as *const libc::c_char,
                        header_0.value_length as libc::c_int);
            }
            if strcmp(header_0.string.as_mut_ptr() as *mut libc::c_char,
                      b"VS_VERSION_INFO\x00" as *const u8 as
                          *const libc::c_char) != 0 {
                fprintf(stderr,
                        b"Warning: Version header is %.16s (expected VS_VERSION_INFO).\n\x00"
                            as *const u8 as *const libc::c_char,
                        header_0.string.as_mut_ptr());
            }
            if header_0.magic != 0xfeef04bdu32 {
                fprintf(stderr,
                        b"Warning: Version magic number is 0x%08x (expected 0xfeef04bd).\n\x00"
                            as *const u8 as *const libc::c_char,
                        header_0.magic);
            }
            if header_0.struct_1 as libc::c_int != 1i32 ||
                   header_0.struct_2 as libc::c_int != 0i32 {
                fprintf(stderr,
                        b"Warning: Version header version is %d.%d (expected 1.0).\n\x00"
                            as *const u8 as *const libc::c_char,
                        header_0.struct_1 as libc::c_int,
                        header_0.struct_2 as libc::c_int);
            }
            print_rsrc_version_flags(header_0);
            printf(b"    File version:    %d.%d.%d.%d\n\x00" as *const u8 as
                       *const libc::c_char, header_0.file_1 as libc::c_int,
                   header_0.file_2 as libc::c_int,
                   header_0.file_3 as libc::c_int,
                   header_0.file_4 as libc::c_int);
            printf(b"    Product version: %d.%d.%d.%d\n\x00" as *const u8 as
                       *const libc::c_char, header_0.prod_1 as libc::c_int,
                   header_0.prod_2 as libc::c_int,
                   header_0.prod_3 as libc::c_int,
                   header_0.prod_4 as libc::c_int);
            /* header's out of the way, now we have to possibly parse a StringFileInfo */
            if header_0.length as libc::c_ulong ==
                   ::std::mem::size_of::<version_header>() as libc::c_ulong {
                return
            } /* I don't have any testcases available so I think this is correct */
            
            
                let mut cursor_0 =   ftell(f); let mut info_length =   read_word(); let mut value_length =   read_word();
            if value_length as libc::c_int != 0i32 {
                fprintf(stderr,
                        b"Warning: Value length is nonzero: %04x\n\x00" as
                            *const u8 as *const libc::c_char,
                        value_length as libc::c_int);
            }
              let mut info_type =   read_byte() as libc::c_char;
            if info_type as libc::c_int == 'S' as i32 {
                /* we have a StringFileInfo */
                fseek(f,
                      (15u64).wrapping_mul(::std::mem::size_of::<byte>()
                                                           as libc::c_ulong)
                          as libc::c_long, 1i32);
                print_rsrc_stringfileinfo(cursor_0 +
                                              info_length as libc::c_long);
                if header_0.length as libc::c_ulong ==
                       (::std::mem::size_of::<version_header>() as
                            libc::c_ulong).wrapping_add(info_length as
                                                            libc::c_ulong) {
                    return
                }
                info_length = read_word();
                fseek(f,
                      
                      ::std::mem::size_of::<word>() as
                          libc::c_long, 1i32);
                info_type = read_byte() as libc::c_char
            }
            if info_type as libc::c_int == 'V' as i32 {
                /* we have a VarFileInfo */
                fseek(f,
                      (11u64).wrapping_mul(::std::mem::size_of::<byte>()
                                                           as libc::c_ulong)
                          as libc::c_long, 1i32);
                print_rsrc_varfileinfo(cursor_0 +
                                           info_length as libc::c_long);
            } else {
                 let mut c =  0;
                fprintf(stderr,
                        b"Warning: Unrecognized file info key: \x00" as
                            *const u8 as *const libc::c_char);
                fseek(f,
                      (::std::mem::size_of::<byte>() as
                           libc::c_ulong).wrapping_neg() as libc::c_long,
                      1i32);
                loop  {
                    c = read_byte() as libc::c_char;
                    if !(c != 0) { break ; }
                    fputc(c as libc::c_int, stderr);
                }
                printf(b"\n\x00" as *const u8 as *const libc::c_char);
            }
            current_block_166 = 2885804587290726961;
        }
        _ => {
            
            
            
             let mut cursor_1 =  0; let mut row =  [0; 16]; let mut len =  0; let mut i_0 =  0;
            loop 
                 /* hexl-style dump */
                 {
                cursor_1 = ftell(f);
                if !(cursor_1 < offset + length) { break ; }
                len =
                    if offset + length - cursor_1 >=
                           16i64 {
                        16i64
                    } else { (offset + length) - cursor_1 } as libc::c_char;
                fread(row.as_mut_ptr() as *mut libc::c_void,
                      ::std::mem::size_of::<byte>() as libc::c_ulong,
                      len as size_t, f);
                printf(b"    %lx:\x00" as *const u8 as *const libc::c_char,
                       cursor_1);
                
                 for i_0 in  0i32.. 16i32 {
    
                    if i_0 & 1i32 == 0 {
                        /* Since this is 16 bits, we put a space after (before) every other two bytes. */
                        putchar(' ' as i32);
                    }
    
                    if i_0 < len as libc::c_int {
                        printf(b"%02x\x00" as *const u8 as
                                   *const libc::c_char,
                               row[i_0 as usize] as libc::c_int);
                    } else {
                        printf(b"  \x00" as *const u8 as *const libc::c_char);
                    }
}
                printf(b"  \x00" as *const u8 as *const libc::c_char);
                
                 for i_0 in  0i32.. len as libc::c_int {
    
                    if row[i_0 as usize] as libc::c_int >= ' ' as i32 &&
                           row[i_0 as usize] as libc::c_int <= '~' as i32 {
                        putchar(row[i_0 as usize] as libc::c_int);
                    } else { putchar('.' as i32); }
}
                putchar('\n' as i32);
            }
            current_block_166 = 2885804587290726961;
        }
    }
    match current_block_166 {
        5358520172556667279 =>
        /* Bitmap */
        {
            current_block_166 = 17778012151635330486;
        }
        _ => { }
    }
    match current_block_166 {
        17778012151635330486 =>
        /* Icon */
        {
             let mut header =
    
                {
                    let mut init =
                        header_bitmap_info{biSize: 0u32,
                                           biWidth: 0,
                                           biHeight: 0,
                                           biPlanes: 0,
                                           biBitCount: 0,
                                           biCompression: 0,
                                           biSizeImage: 0,
                                           biXPelsPerMeter: 0,
                                           biYPelsPerMeter: 0,
                                           biClrUsed: 0,
                                           biClrImportant: 0,};
                    init
                };
             
            header =
    crate::src::ne_resource::header_bitmap_info{biSize:
                                                      read_dword(), ..
            header}; /* todo: implied */
            if header.biSize == 12u32 {
                
                
                
                
                
                
                
                 
                header =
    crate::src::ne_resource::header_bitmap_info{biWidth: 
                                                 read_word() as dword,
                                                biHeight:
                                                    
                                                 read_word() as dword,
                                                biPlanes:   read_word(),
                                                biBitCount:
                                                      read_word(), ..
                header};
                current_block_166 = 8236137900636309791;
            } else if header.biSize == 40u32 {
                fseek(f,
                      (::std::mem::size_of::<dword>() as
                           libc::c_ulong).wrapping_neg() as libc::c_long,
                      1i32);
                fread(&mut header as *mut header_bitmap_info as
                          *mut libc::c_void,
                      ::std::mem::size_of::<header_bitmap_info>() as
                          libc::c_ulong, 1u64, f);
                current_block_166 = 8236137900636309791;
            } else {
                fprintf(stderr,
                        b"Warning: Unknown bitmap header size %d.\n\x00" as
                            *const u8 as *const libc::c_char, header.biSize);
                current_block_166 = 2885804587290726961;
            }
            match current_block_166 {
                2885804587290726961 => { }
                _ => {
                    printf(b"    Size: %dx%dx%d\n\x00" as *const u8 as
                               *const libc::c_char, header.biWidth,
                           header.biHeight.wrapping_div(2u32),
                           header.biBitCount as libc::c_int);
                    if header.biPlanes as libc::c_int != 1i32 {
                        fprintf(stderr,
                                b"Warning: Bitmap specifies %d color planes (expected 1).\n\x00"
                                    as *const u8 as *const libc::c_char,
                                header.biPlanes as libc::c_int);
                    }
                    if header.biCompression <=
                           13u32 &&
                           !rsrc_bmp_compression[header.biCompression as
                                                     usize].is_null() {
                        printf(b"    Compression: %s\n\x00" as *const u8 as
                                   *const libc::c_char,
                               rsrc_bmp_compression[header.biCompression as
                                                        usize]);
                    } else {
                        printf(b"    Compression: (unknown value %d)\n\x00" as
                                   *const u8 as *const libc::c_char,
                               header.biCompression);
                    }
                    if header.biXPelsPerMeter != 0 ||
                           header.biYPelsPerMeter != 0 {
                        printf(b"    Resolution: %dx%d pixels/meter\n\x00" as
                                   *const u8 as *const libc::c_char,
                               header.biXPelsPerMeter,
                               header.biYPelsPerMeter);
                    }
                    printf(b"    Colors used: %d\x00" as *const u8 as
                               *const libc::c_char, header.biClrUsed);
                    if header.biClrImportant != 0 {
                        printf(b" (%d marked important)\x00" as *const u8 as
                                   *const libc::c_char,
                               header.biClrImportant);
                    }
                    putchar('\n' as i32);
                }
            }
        }
        _ => { }
    };
}
/* return true if this was one of the resources that was asked for */
unsafe extern "C" fn filter_resource(mut type_0: *const libc::c_char,
                                     mut id: *const libc::c_char)
 -> libc::c_int {
    
    if resource_filters_count == 0 { return 1i32 }
      let mut i =   0u32;
    while i < resource_filters_count {
        
        
         let mut filter_type = 
            *resource_filters.offset(i as isize); let mut p =  0 as *const libc::c_char; let mut len =  strlen(type_0);
        /* note that both resource types and IDs are case insensitive */
        /* if the filter is just a resource type or ID and we match that */
        if strcasecmp(type_0, filter_type) == 0 ||
               strcasecmp(id, filter_type) == 0 {
            return 1i32
        }
        /* if the filter is a resource type followed by an ID and we match both */
        if !(strncasecmp(type_0, filter_type, len) != 0 ||
                 *filter_type.offset(len as isize) as libc::c_int !=
                     ' ' as i32) {
            p = filter_type.offset(len as isize);
            while *p as libc::c_int == ' ' as i32 { p = p.offset(1) }
            if strcasecmp(id, p) == 0 { return 1i32 }
        }
        i = i.wrapping_add(1)
    }
    return 0i32;
}
/* in ne_resource.c */
#[no_mangle]
pub unsafe extern "C" fn print_rsrc(mut start: libc::c_long) {
    let mut current_block: u64; /* fixme: what is this? */
    
    
    
    
    
    
     let mut align =  read_word(); let mut type_id =  0; let mut count =  0; let mut resloader =  0; let mut rn =
    
        resource{offset: 0, length: 0, flags: 0, id: 0, handle: 0, usage: 0,}; let mut cursor =  0; let mut idstr =  0 as *mut libc::c_char;
    loop  {
        type_id = read_word();
        if !(type_id != 0) { break ; }
        count = read_word();
        resloader = read_dword();
        if resloader != 0 {
            fprintf(stderr,
                    b"Warning: resloader is nonzero: %08x\n\x00" as *const u8
                        as *const libc::c_char, resloader);
        }
        loop  {
            let fresh4 = count;
            count = count.wrapping_sub(1);
            if !(fresh4 != 0) { break ; }
            fread(&mut rn as *mut resource as *mut libc::c_void,
                  ::std::mem::size_of::<resource>() as libc::c_ulong,
                  1u64, f);
            cursor = ftell(f);
            if rn.id as libc::c_int & 0x8000i32 != 0 {
                idstr =
                    malloc(6u64) as
                        *mut libc::c_char;
                sprintf(idstr, b"%d\x00" as *const u8 as *const libc::c_char,
                        rn.id as libc::c_int & !(0x8000i32));
            } else {
                idstr = dup_string_resource(start + rn.id as libc::c_long)
            }
            if type_id as libc::c_int & 0x8000i32 != 0 {
                if ((type_id as libc::c_int & !(0x8000i32)) as
                        libc::c_ulong) < rsrc_types_count &&
                       !rsrc_types[(type_id as libc::c_int &
                                        !(0x8000i32)) as
                                       usize].is_null() {
                    if filter_resource(rsrc_types[(type_id as libc::c_int &
                                                       !(0x8000i32)) as
                                                      usize], idstr) == 0 {
                        current_block = 1916172849665352563;
                    } else {
                        printf(b"\n%s\x00" as *const u8 as
                                   *const libc::c_char,
                               rsrc_types[(type_id as libc::c_int &
                                               !(0x8000i32)) as
                                              usize]);
                        current_block = 17788412896529399552;
                    }
                } else {
                     let mut typestr =  [0; 7];
                    sprintf(typestr.as_mut_ptr(),
                            b"0x%04x\x00" as *const u8 as *const libc::c_char,
                            type_id as libc::c_int);
                    if filter_resource(typestr.as_mut_ptr(), idstr) == 0 {
                        current_block = 1916172849665352563;
                    } else {
                        printf(b"\n%s\x00" as *const u8 as
                                   *const libc::c_char, typestr.as_mut_ptr());
                        current_block = 17788412896529399552;
                    }
                }
            } else {
                 let mut typestr_0 = 
                    dup_string_resource(start + type_id as libc::c_long);
                if filter_resource(typestr_0, idstr) == 0 {
                    free(typestr_0 as *mut libc::c_void);
                    current_block = 1916172849665352563;
                } else {
                    printf(b"\n\"%s\"\x00" as *const u8 as
                               *const libc::c_char, typestr_0);
                    free(typestr_0 as *mut libc::c_void);
                    current_block = 17788412896529399552;
                }
            }
            match current_block {
                17788412896529399552 => {
                    printf(b" %s\x00" as *const u8 as *const libc::c_char,
                           idstr);
                    printf(b" (offset = 0x%x, length = %d [0x%x]\x00" as
                               *const u8 as *const libc::c_char,
                           (rn.offset as libc::c_int) << align as libc::c_int,
                           (rn.length as libc::c_int) << align as libc::c_int,
                           (rn.length as libc::c_int) <<
                               align as libc::c_int);
                    print_rsrc_flags(rn.flags);
                    printf(b"):\n\x00" as *const u8 as *const libc::c_char);
                    print_rsrc_resource(type_id,
                                        ((rn.offset as libc::c_int) <<
                                             align as libc::c_int) as
                                            libc::c_long,
                                        ((rn.length as libc::c_int) <<
                                             align as libc::c_int) as
                                            libc::c_long, rn.id);
                }
                _ => { }
            }
            free(idstr as *mut libc::c_void);
            fseek(f, cursor, 0i32);
        }
    };
}
unsafe extern "C" fn run_static_initializers() {
    rsrc_types_count =
        (::std::mem::size_of::<[*const libc::c_char; 19]>() as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<*const libc::c_char>()
                                             as libc::c_ulong)
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
