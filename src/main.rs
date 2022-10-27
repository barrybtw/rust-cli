mod helpers;
mod installers;
mod prompts;

use crate::{
    helpers::{get_version::get_version, render_title::render_title},
    prompts::{
        git_init_prompt::ask_do_git_init,
        install_prompt::ask_do_install,
        language_prompt::{ask_language, Languages},
        name_prompt::name_prompt,
        package_manager_prompt::{ask_package_manager, PackageManagers},
        packages_prompt::ask_packages,
    },
};

fn main() {
    render_title("Welcome to the Rusty CLI");
    println!("Version: {}", get_version());

    let app_name = name_prompt().unwrap();
    println!("App name: {}", app_name);

    let language: Languages = ask_language();
    println!("You selected: {:?} as your language.", language);

    let package_manager: PackageManagers = ask_package_manager();
    println!(
        "You selected: {:?} as your package manager.",
        package_manager
    );

    let packages = ask_packages().expect("Package prompt failed.");
    println!("Packages:");
    let clone = &packages.clone();
    for (index, package) in clone.iter().enumerate() {
        if index == clone.len() - 2 {
            print!("{} and ", package);
        } else if index == clone.len() - 1 {
            print!("{}.\n", package);
        } else {
            print!("{}, ", package);
        }
    }

    let do_git_init = ask_do_git_init().unwrap();
    match do_git_init {
        true => println!("You selected to initialize a git repository."),
        false => println!("You selected not to initialize a git repository."),
    }

    let do_install = ask_do_install(package_manager.to_string()).unwrap();
    match do_install {
        true => println!("You selected to install the packages."),
        false => println!("You selected not to install the packages."),
    }

    println!("Done!");

    // TODO: Finish installers
}
