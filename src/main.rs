mod installers;
mod prompts;

use strum::{Display, EnumIter};

use crate::prompts::{
    language_prompt::{ask_language, Languages},
    package_manager_prompt::{ask_package_manager, PackageManagers},
};

#[derive(Debug, EnumIter, Display, Clone, Copy)]
enum Packages {
    Trpc,
    Prisma,
    Tailwind,
    NextAuth,
}

fn main() {
    let language: Languages = ask_language();
    let _package_manager: PackageManagers = ask_package_manager();

    println!("You selected: {:?} as your language.", language);
    println!(
        "You selected: {:?} as your package manager.",
        _package_manager
    );

    println!("Done!");
}
