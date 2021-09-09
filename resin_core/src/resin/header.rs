use std::io::{BufReader, Read};

use crate::util::reader::ReadUtil;

pub struct Header {
    pub num_empty_dirs: u64,
    pub num_files: u64,
}

impl Header {
    pub fn new(reader: &mut BufReader<impl Read>) -> Self {
        Self {
            num_empty_dirs: reader.read_u64(),
            num_files: reader.read_u64(),
        }
    }
}
