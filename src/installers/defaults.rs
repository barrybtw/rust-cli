// Function to add default addons to the installer
// Path to addons: ../../template/addons/index.ts
extern crate fs_extra;

use crate::helpers::copy::copy_files;

pub fn install_defaults(_app_name: &str) {
    println!("Installing default addons...");
    println!("{}", _app_name);
    copy_files("./src/template/base", &_app_name)
}
