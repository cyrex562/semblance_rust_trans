use crate::app_context::AppContext;
use crate::common::Endianness;
use crate::multi_error::MultiError;
use fern::InitError;
use std::fs::File;
use std::io::Read;
use std::convert::TryInto;
use std::error::Error;
use libc::c_long;

/// Create a hex-string representation of a sequence of bytes, optionally separated by spaces
pub fn bytes_to_hex_str(bytes: &[u8], count: usize, spaces: bool) -> String {
    let mut out = String::new();
    for b in bytes {
        out.push(char::from(*b));
        if spaces {
            out.push_str(" ");
        }
    }

    out
}

/// Configure the logger
pub fn setup_logger() -> Result<(), InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
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
    let mut file = File::open(&app_ctx.input_file_path)?;

    app_ctx.buf_size = file.read_to_end(&mut app_ctx.file_buf)?;

    // app_ctx.file_handle = Some(file);

    Ok(())
}

// pub fn read_byte(app_ctx: &AppContext, offset: usize) -> u8 {
//     app_ctx.file_buf[offset]
// }

/// Read a u16 from a byte slice at a specified offset, accounting for endianness
pub fn read_word(buffer: &[u8], offset: &usize, endianness: Endianness) -> Result<u16, Box<dyn Error>> {
    let bytes: [u8;2] = [buffer[*offset], buffer[*offset+1]];
    if endianness == Endianness::LittleEndian {
        return Ok(u16::from_le_bytes(bytes));
    }
    Ok(u16::from_be_bytes(bytes))
}

/// Read a u32 from a byte slice at a specified offset, accounting for endianness
pub fn read_dword(buffer: &[u8], offset: &usize, endianness: Endianness) -> Result<u32, Box<dyn Error>> {
    let bytes: [u8;4] = [buffer[*offset], buffer[*offset+1], buffer[*offset+2], buffer[*offset+3]];
    if endianness == Endianness::LittleEndian {
        return Ok(u32::from_le_bytes(bytes));
    }
    Ok(u32::from_be_bytes(bytes))
}

/// Read an i32/i64 from a byte slice at a specified offset, accounting for endianness
pub fn read_long(buffer: &[u8], offset: &usize, endianness: Endianness) -> Result<c_long, Box<dyn Error>> {
    let word_bytes: [u8;4] = [buffer[*offset], buffer[*offset+1], buffer[*offset+2], buffer[*offset+3]];
    if endianness == Endianness::LittleEndian {
        return Ok(c_long::from_le_bytes(word_bytes));
    }
    Ok(c_long::from_be_bytes(word_bytes))
}

pub fn read_qword(app_ctx: &AppContext, offset: usize, endianness: Endianness) -> u64 {
    let word_bytes: [u8; 8] = [
        app_ctx.file_buf[offset],
        app_ctx.file_buf[offset + 1],
        app_ctx.file_buf[offset + 2],
        app_ctx.file_buf[offset + 3],
        app_ctx.file_buf[offset + 4],
        app_ctx.file_buf[offset + 5],
        app_ctx.file_buf[offset + 6],
        app_ctx.file_buf[offset + 7],
    ];

    if endianness == Endianness::LittleEndian {
        u64::from_le_bytes(word_bytes)
    } else {
        u64::from_be_bytes(word_bytes)
    }
}
