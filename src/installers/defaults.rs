// Function to add default addons to the installer
// Path to addons: ../../template/addons/index.ts
extern crate fs_extra;
use fs_extra::dir::{copy, CopyOptions};
use std::fs;

pub fn install_defaults(_app_name: &str) {
    let options = CopyOptions::new(); //Initialize default values for CopyOptions

    // check if directory app_name exists
    // if it does, ask if they want to overwrite it
    // if it doesn't, create it
    if fs::metadata(_app_name).is_ok() {
        println!("Directory {} already exists.", _app_name);
    } else {
        println!("Creating directory {}.", _app_name);
        fs::create_dir(_app_name).expect("Failed to create directory.");

        let source = "./src/template/base";
        let destination = &_app_name;
        copy(&source, &destination, &options).expect("Failed to copy directory.");
    }
}
