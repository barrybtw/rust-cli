// Function to add default addons to the installer
// Path to addons: ../../template/addons/index.ts
use fs_extra::dir::{copy, CopyOptions};
use std::{fs, process};

pub fn copy_files(from: &str, to: &str) {
    let options = CopyOptions::new();

    if fs::metadata(&to).is_ok() {
        println!("Directory {} already exists, shutting down.", &from);
        process::exit(0);
    } else {
        fs::create_dir(&to).expect("Failed to create directory.");
        let source = &from;
        let destination = &to;
        copy(&source, &destination, &options).expect("Failed to copy directory.");
    }
}
