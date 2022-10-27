mod installers;
mod prompts;

use inquire::validator::ErrorMessage;

use crate::prompts::{
    language_prompt::{ask_language, Languages},
    package_manager_prompt::{ask_package_manager, PackageManagers},
    packages_prompt::{ask_packages, Packages},
};

fn main() {
    let language: Languages = ask_language();
    let package_manager: PackageManagers = ask_package_manager();
    let packages: Result<Vec<Packages>, ErrorMessage> = ask_packages();

    println!("You selected: {:?} as your language.", language);
    println!(
        "You selected: {:?} as your package manager.",
        package_manager
    );
    println!("Packages:");
    let clone = &packages.clone().unwrap();
    for (index, package) in clone.iter().enumerate() {
        if index == clone.len() - 2 {
            print!("{} and ", package);
        } else if index == clone.len() - 1 {
            print!("{}.\n", package);
        } else {
            print!("{},", package);
        }
    }

    println!("Done!");
}
