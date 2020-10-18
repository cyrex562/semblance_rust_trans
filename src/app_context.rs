use std::fs::File;

#[derive(Default, Clone)]
pub struct AppContext {
    pub input_file: String,
    pub file_handle: File,
    pub file_buf: Vec<u8>,
    pub buf_size: usize,
}