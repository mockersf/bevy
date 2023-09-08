use std::{ffi::OsStr, fs, path::Path};

use regex::Regex;

fn main() {
    check_directory(Path::new("./crates"));
}

fn parse_wgsl(path: &Path) {
    let define_import_path_regex =
        Regex::new(r"^\s*#\s*define_import_path\s+([a-z0-9_]+::)*([^\s]+)").unwrap();

    let file_content = fs::read_to_string(path).unwrap();
    let mut lines = file_content.lines();
    let first_line = lines.next().unwrap();
    if let Some(cap) = define_import_path_regex.captures(first_line) {
        let name = cap.get(2).unwrap().as_str().to_string();
        println!("   {:?}", first_line);
        println!("   -> {:?}", name);
    } else if first_line.starts_with("#define_import_path") {
        println!("   {:?}", first_line);
    }
}

fn check_directory(path: &Path) {
    let paths = fs::read_dir(path).unwrap();

    for path in paths {
        if let Ok(dir_entry) = path {
            if Some(OsStr::new("wgsl")) == dir_entry.path().extension() {
                println!("=> {:?}", dir_entry.path());
                parse_wgsl(&dir_entry.path());
            }
            if dir_entry.file_type().unwrap().is_dir() {
                check_directory(&dir_entry.path());
            }
        }
    }
}
