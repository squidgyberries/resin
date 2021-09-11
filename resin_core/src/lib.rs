mod resin;
mod util;

use std::{fs::{File, create_dir_all}, io::{BufReader, BufWriter, Write}};

use crate::resin::resin::Resin;

pub fn thing() {
    let file = File::open("thing.resin").unwrap();
    let mut reader = BufReader::new(file);
    let thing = Resin::new(&mut reader);
    println!("{}", thing.header.num_empty_dirs);
    println!("{}", thing.header.num_files);
    if thing.header.num_empty_dirs > 0 {
        for i in thing.empty_dirs {
            println!("{}", i);
            std::fs::create_dir_all(i).expect("Unable to create directory.");
        }
    }
    if thing.header.num_files > 0 {
        for i in thing.files {
            println!("{}", i.path);
            println!("{}", i.data_length);
            for j in &i.data {
                print!("{:X} ", j);
            }
            println!("");
            let path_split: Vec<&str> = i.path.split("/").collect();
            let mut folders = String::new();
            for i in 0..path_split.len() - 1 {
                folders.push_str(&format!("{}/", path_split[i]));
            }
            create_dir_all(folders).expect("Unable to create directory");
            let file = File::create(i.path).expect("Unable to create file.");
            let mut writer = BufWriter::new(file);
            writer.write_all(&i.data).expect("Unable to write to file.");
        }
    }
}
