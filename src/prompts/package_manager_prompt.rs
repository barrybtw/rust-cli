use dialoguer::{theme::ColorfulTheme, Select};
use strum::{Display, EnumIter, IntoEnumIterator};

#[derive(Debug, EnumIter, Display, Clone, Copy)]
pub enum PackageManagers {
    Yarn,
    Npm,
    Pnpm,
}

pub fn ask_package_manager() -> PackageManagers {
    let package_manager_list: Vec<_> = PackageManagers::iter().collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a package manager")
        .items(&package_manager_list)
        .default(0)
        .interact()
        .unwrap();

    package_manager_list[selection]
}
