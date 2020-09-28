use std::{fs::{self, File}, path::Path};

use crate::multi_error::MultiError;



pub fn dump_file(file: &str) -> Result<(), MultiError> {
    // open file
    let path = Path::new(file);
    let mut file = File::open(&path)?;

    // get file stats
    let stats = fs::metadata(&path)?;

    // memory map the file
    
}