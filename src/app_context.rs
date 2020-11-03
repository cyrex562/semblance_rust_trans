use crate::util::bytes_to_hex_str;
use std::{fs::File, fmt::Display, fmt};

#[derive(Default)]
pub struct AppContext {
    pub input_file_path: String,
    pub file_buf: Vec<u8>,
    pub buf_size: usize,
}

// impl Copy for AppContext {

// }

impl Clone for AppContext {
    fn clone(&self) -> Self {
        AppContext {
            input_file_path: self.input_file_path.clone(),
            file_buf: self.file_buf.clone(),
            buf_size: self.buf_size,
        }
    }
}

impl Display for AppContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "input_file_path={}, file_buf={}, buf_size={}",
            self.input_file_path,
            bytes_to_hex_str(&self.file_buf[..], 8, false),
            self.buf_size
        )
    }
}
