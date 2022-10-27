use std::io::Error;

use console::Term;
use dialoguer::{theme::ColorfulTheme, MultiSelect};
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};

#[derive(Debug, EnumIter, Display, Clone, Copy, EnumString, PartialEq)]
pub enum Packages {
    Trpc,
    Prisma,
    Tailwind,
    NextAuth,
}

pub fn ask_packages() -> Result<Vec<Packages>, Error> {
    let package_list: Vec<_> = Packages::iter().collect();

    let selection = MultiSelect::with_theme(&ColorfulTheme::default())
        .items(&package_list)
        .interact_on_opt(&Term::stdout())?;

    match selection {
        Some(positions) => {
            let mut selected_packages: Vec<Packages> = Vec::new();

            for position in positions {
                selected_packages.push(package_list[position]);
            }

            Ok(selected_packages)
        }
        None => Ok(package_list),
    }
}

// pub fn ask_language() -> Languages {
//     let language_list: Vec<_> = Languages::iter().collect();

//     let selection = Select::with_theme(&ColorfulTheme::default())
//         .with_prompt("Select a language")
//         .items(&language_list)
//         .default(0)
//         .interact()
//         .unwrap();

//     match selection {
//         0 => language_list[0],
//         1 => {
//             println!("Wrong answer... Using TypeScript");
//             language_list[0]
//         }
//         _ => panic!("Invalid selection"),
//     }
// }
