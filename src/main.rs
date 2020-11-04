pub mod app_context;
pub mod common;
pub mod dump;
pub mod exe_format;
pub mod multi_error;
pub mod ne_header;
pub mod schema;
pub mod util;

use crate::app_context::AppContext;
use crate::util::setup_logger;
use crate::schema::{ImageDosHeader, ImageOs2Header, NeSegmentTable, NeSegmentTableEntry};
use clap::{App, Arg};
use schema::NeSegmentFlag;
use std::fs;
use std::error::Error;
use std::convert::TryInto;

fn main() -> Result<(), Box<dyn Error>> {
    let mut app_ctx: AppContext = parse_cmd_line()?;

    setup_logger()?;

    // read the file into memmap
    // let mut fh: File = File::open(&app_ctx.input_file_path)?;
    // parse the file
    app_ctx.file_buf = fs::read(&app_ctx.input_file_path)?;


    // app_ctx
    //     .file_buf
    //     .copy_from_slice(fs::read(&app_ctx.input_file_path)?.as_slice());
    app_ctx.buf_size = app_ctx.file_buf.len();
    let mut ptr: usize = 0;

    let dos_hdr = ImageDosHeader::parse_from_bytes(&app_ctx.file_buf, &mut ptr)?;
    log::debug!("MZ header={}", dos_hdr);

    // identify the entry point
    let ne_hdr_start: usize = dos_hdr.e_lfanew.try_into()?;
    ptr = ne_hdr_start;
    let ne_hdr = ImageOs2Header::parse_from_bytes(&app_ctx.file_buf, &mut ptr)?;
    log::debug!("NE header={}", ne_hdr);

    // parse the segment table
    let seg_tbl_offset: usize = ne_hdr.ne_segtab.try_into()?;
    ptr = ne_hdr_start + seg_tbl_offset;
    let mut seg_tbl = NeSegmentTable::parse_from_bytes(&app_ctx.file_buf, &mut ptr, ne_hdr.ne_cseg.try_into()?)?;
    log::debug!("Segment Table={}", seg_tbl);

    // iterate through the assembly
    // identify code segments
    for seg in seg_tbl.entries {
        for flag in seg.get_flags() {
            if flag == NeSegmentFlag::CODE {
                log::debug!("{{\n\toffset={} ({:04x})\n\tlength={} ({:04x})\n\tflags={}\n\talloc sz={} ({:04x})\n}}",
                    seg.logical_sector_offset,
                    seg.logical_sector_offset,
                    seg.segment_length,
                    seg.segment_length,
                    seg.get_flags_as_str(),
                    seg.min_seg_alloc_sz,
                    seg.min_seg_alloc_sz);
                break;
            }
        }
    }


    //
    Ok(())
}

///
/// parse command line arguments
///
fn parse_cmd_line() -> Result<AppContext, Box<dyn Error>> {
    let args = App::new("ne_runner")
        .version("0.1")
        .author("cyrex562")
        .about("")
        .arg(
            Arg::with_name("input_exe")
                .short("i")
                .long("input_file")
                .help("exe file to load")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let mut app_ctx: AppContext = Default::default();

    if args.is_present("input_exe") {
        let input_file_str = args.value_of("input_exe").unwrap();
        app_ctx.input_file_path = input_file_str.to_string();
    }

    Ok(app_ctx)
}
