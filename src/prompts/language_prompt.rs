use dialoguer::{theme::ColorfulTheme, Select};
use strum::{Display, EnumIter, IntoEnumIterator};

#[derive(Debug, EnumIter, Display, Clone, Copy)]
pub enum Languages {
    TypeScript,
    JavaScript,
}

pub fn ask_language() -> Languages {
    let language_list: Vec<_> = Languages::iter().collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a language")
        .items(&language_list)
        .default(0)
        .interact()
        .unwrap();

    match selection {
        0 => language_list[0],
        1 => {
            println!("Wrong answer... Using TypeScript");
            language_list[0]
        }
        _ => panic!("Invalid selection"),
    }
}
