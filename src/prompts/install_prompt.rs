use dialoguer::{theme::ColorfulTheme, Confirm};
use inquire::validator::ErrorMessage;

pub fn ask_do_install(package_manager: String) -> Result<bool, ErrorMessage> {
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
