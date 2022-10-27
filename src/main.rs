mod helpers;
mod installers;
mod prompts;

use crate::{
    helpers::welcome::welcome_message,
    installers::{
        defaults::install_defaults, env_vars::install_env, nextauth::install_nextauth,
        prisma::install_prisma, tailwind::install_tailwind, trpc::install_trpc,
    },
    prompts::{
        git_init_prompt::ask_do_git_init,
        install_prompt::ask_do_install,
        language_prompt::{ask_language, Languages},
        name_prompt::name_prompt,
        package_manager_prompt::{ask_package_manager, PackageManagers},
        packages_prompt::{ask_packages, Packages},
    },
};

fn main() {
    welcome_message();

    let app_name: String = name_prompt().unwrap();
    println!("App name: {}", app_name);

    let language: Languages = ask_language();
    println!("You selected: {:?} as your language.", language);

    let package_manager: PackageManagers = ask_package_manager();
    println!(
        "You selected: {:?} as your package manager.",
        package_manager
    );

    let packages: Vec<Packages> = ask_packages().expect("Package prompt failed.");
    println!("Packages:");
    let clone: &Vec<Packages> = &packages.clone();
    for (index, package) in clone.iter().enumerate() {
        if index == clone.len() - 2 {
            print!("{} and ", package);
        } else if index == clone.len() - 1 {
            print!("{}.\n", package);
        } else {
            print!("{}, ", package);
        }
    }

    let do_git_init: bool = ask_do_git_init().unwrap();
    match do_git_init {
        true => println!("You selected to initialize a git repository."),
        false => println!("You selected not to initialize a git repository."),
    }

    let do_install: bool = ask_do_install(package_manager.to_string()).unwrap();
    match do_install {
        true => println!("You selected to install the packages."),
        false => println!("You selected not to install the packages."),
    }

    for (_, package) in clone.iter().enumerate() {
        install_defaults(&app_name);
        match package {
            Packages::Trpc => {
                install_trpc(&app_name);
            }
            Packages::Prisma => {
                install_prisma(&app_name);
            }
            Packages::Tailwind => {
                install_tailwind(&app_name);
            }
            Packages::NextAuth => {
                install_nextauth(&app_name);
            }
        }
    }
    let mut package_list: Vec<Packages> = Vec::new();
    package_list.push(Packages::NextAuth.clone());
    // install_env("hello", &package_list);

    println!("Done! TODO: Install packages.");
    install_defaults(&app_name);
    install_env(&app_name, &packages)

    // TODO: Finish installers
}
