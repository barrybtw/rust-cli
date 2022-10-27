// Function to add default addons to the installer
// Path to addons: ../../template/addons/index.ts
use fs_extra::dir::{copy, CopyOptions};
use std::{fs, process};

pub fn copy_files(_app_name: &str) {
    let options = CopyOptions::new();

    if fs::metadata(_app_name).is_ok() {
        println!("Directory {} already exists, shutting down.", _app_name);
        process::exit(0);
    } else {
        fs::create_dir(_app_name).expect("Failed to create directory.");
        let source = "./src/template/base";
        let destination = &_app_name;
        copy(&source, &destination, &options).expect("Failed to copy directory.");
    }
}
