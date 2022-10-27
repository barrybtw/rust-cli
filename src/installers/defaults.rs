// Function to add default addons to the installer
// Path to addons: ../../template/addons/index.ts
extern crate fs_extra;

use crate::helpers::copy::copy_files;

pub fn install_defaults(_app_name: &str) {
    copy_files(&_app_name)
}
