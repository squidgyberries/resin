mod resin;
mod util;

use std::{fs::{create_dir_all, File}, io::{BufReader, BufWriter, Write}, path::Path};

use crate::resin::resin::Resin;

pub fn extract(in_file: &str, target: &str) {
    create_dir_all(&target).expect("Unable to create target folder.");

    let file = File::open(in_file).expect("Unable to open resin file.");
    let mut reader = BufReader::new(file);
    let resin_file = Resin::new(&mut reader);
    println!("{} empty directories", resin_file.header.num_empty_dirs);
    println!("{} files", resin_file.header.num_files);

    // Create empty directories
    if resin_file.header.num_empty_dirs > 0 {
        for dir in resin_file.empty_dirs {
            let full_dir_path = format!("{}{}", target, dir);
            println!("Creating directory {}", dir);
            create_dir_all(full_dir_path).expect("Unable to create directory.");
        }
    }

    // Extract files
    if resin_file.header.num_files > 0 {
        for i in resin_file.files {
            let path_split: Vec<&str> = i.path.split("/").collect();

            // Create directory
            let mut dirs = String::new();
            for i in 0..path_split.len() - 1 {
                dirs.push_str(&format!("{}/", path_split[i]));
            }
            let full_dir_path = format!("{}{}", target, dirs);
            if !Path::new(&full_dir_path).exists() {
                println!("Creating directory {}", full_dir_path);
                create_dir_all(full_dir_path).expect("Unable to create directory");
            }

            // Create file
            let full_file_path = format!("{}{}", target, i.path);
            println!("Creating file {}", full_file_path);
            let file = File::create(full_file_path).expect("Unable to create file.");
            let mut writer = BufWriter::new(file);
            writer.write_all(&i.data).expect("Unable to write to file.");
        }
    }
}
