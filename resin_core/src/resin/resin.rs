use std::{
    fs::File,
    io::{BufReader, Seek, SeekFrom},
};

use crate::util::reader::ReadUtil;

use super::{file_entry::FileEntry, header::Header};

pub struct Resin {
    pub header: Header,
    pub empty_dirs: Vec<String>,
    pub files: Vec<FileEntry>,
}

impl Resin {
    pub fn new(reader: &mut BufReader<File>) -> Self {
        reader
            .seek(SeekFrom::Start(0))
            .expect("Unable to seek file stream.");
        let header = Header::new(reader);
        let mut empty_dirs = Vec::new();
        for _i in 0..header.num_empty_dirs {
            empty_dirs.push(reader.read_utf8_nl_string());
        }
        let mut files = Vec::new();
        for _i in 0..header.num_files {
            files.push(FileEntry::new(reader));
        }
        Self {
            header: header,
            empty_dirs: empty_dirs,
            files: files,
        }
    }
}
