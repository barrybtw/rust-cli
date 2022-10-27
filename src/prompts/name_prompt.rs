use crate::helpers::validate_app_name::validate_app_name;
use dialoguer::Input;
use std::io::Error;

pub fn name_prompt() -> Result<String, Error> {
    let input: String = Input::new()
        .with_prompt("What will the name of your app be?")
        .default("t3-app".into())
        .interact_text()?;
    validate_app_name(input.clone());
    Ok(input)
}
