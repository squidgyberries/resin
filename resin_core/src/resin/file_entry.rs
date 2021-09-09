use std::io::{BufReader, Read};

use crate::util::reader::ReadUtil;

pub struct FileEntry {
    pub path: String,
    pub data_length: u64,
    pub data: Vec<u8>,
}

impl FileEntry {
    pub fn new(reader: &mut BufReader<impl Read>) -> Self {
        let path = reader.read_utf8_nl_string();
        let data_length = reader.read_u64();
        println!("{} {}", data_length, data_length as usize);
        let mut data = vec![0u8; data_length as usize];
        reader
            .read_exact(&mut data)
            .expect("Unable to read file data.");
        Self {
            path: path,
            data_length: data_length,
            data: data,
        }
    }
}
