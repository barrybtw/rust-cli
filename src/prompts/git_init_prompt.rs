use dialoguer::{theme::ColorfulTheme, Confirm};
use inquire::validator::ErrorMessage;

pub fn ask_do_git_init() -> Result<bool, ErrorMessage> {
    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you wish to initialize an empty git repository?")
        .interact()?
    {
        Ok(true)
    } else {
        println!("As you wish.");
        Ok(false)
    }
}
