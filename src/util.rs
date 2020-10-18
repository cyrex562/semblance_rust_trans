use fern::InitError;
use crate::app_context::AppContext;
use crate::multi_error::MultiError;
use std::fs::File;
use std::io::Read;

///
/// Set up the logger
/// 
pub fn setup_logger() -> Result<(), InitError> {
    fern::Dispatch::new()
        .format(|out, message, record | {
            out.finish(format_args!(
                "{}: {}: {}: {}",
                chrono::Local::now().format("%Y%m%d.%H%M%S"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}

pub fn read_file(app_ctx: &mut AppContext) -> Result<(), MultiError> {
    let mut file = File::open(&app_ctx.input_file)?;

    app_ctx.buf_size = file.read_to_end(&mut app_ctx.file_buf)?;

    app_ctx.file_handle = file;

    Ok(())
}

pub fn read_byte(app_ctx: &AppContext, offset: usize) -> u8 {
    app_ctx.file_buf[offset]
}

pub fn read_word(app_ctx: &AppContext, offset: usize, little_endian: bool) -> u16 {

    let word_bytes: [u8;2] = [app_ctx.file_buf[offset], app_ctx.file_buf[offset+1]];

    if little_endian {
        u16::from_le_bytes(word_bytes)
    } else {
        u16::from_be_bytes(word_bytes)
    }
}

pub fn read_dword(app_ctx: &AppContext, offset: usize, little_endian: bool) -> u32 {

    let word_bytes: [u8;4] = [app_ctx.file_buf[offset], app_ctx.file_buf[offset+1], app_ctx.file_buf[offset+2], app_ctx.file_buf[offset+3]];

    if little_endian {
        u32::from_le_bytes(word_bytes)
    } else {
        u32::from_be_bytes(word_bytes)
    }
}

pub fn read_qword(app_ctx: &AppContext, offset: usize, little_endian: bool) -> u64 {

    let word_bytes: [u8;8] = [app_ctx.file_buf[offset], app_ctx.file_buf[offset+1], app_ctx.file_buf[offset+2], app_ctx.file_buf[offset+3], app_ctx.file_buf[offset+4], app_ctx.file_buf[offset+5], app_ctx.file_buf[offset+6], app_ctx.file_buf[offset+7]];

    if little_endian {
        u64::from_le_bytes(word_bytes)
    } else {
        u64::from_be_bytes(word_bytes)
    }
}