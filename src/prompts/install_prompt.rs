use dialoguer::{theme::ColorfulTheme, Confirm};
use std::io::Error;

pub fn ask_do_install(package_manager: String) -> Result<bool, Error> {
    let message = format!(
        "Do you wish to install the packages with {} install?",
        package_manager.to_lowercase()
    );
    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(message)
        .interact()?
    {
        Ok(true)
    } else {
        println!("As you wish.");
        Ok(false)
    }
}
