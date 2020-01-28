#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(main)]
#![feature(ptr_wrapping_offset_from)]
#![feature(register_tool)]
#![register_tool(c2rust)]


#[macro_use]
extern crate c2rust_bitfields;
extern crate libc;



pub mod src {
pub mod dump;
pub mod mz;
pub mod ne_header;
pub mod ne_resource;
pub mod ne_segment;
pub mod pe_header;
pub mod pe_section;
pub mod x86_instr;
} // mod src

