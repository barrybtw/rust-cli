use super::get_version::get_version;
use super::render_title::render_title;

pub fn welcome_message() {
    let message = format!(
        "Welcome to the Rusty T3 CLI, you are running version: {}",
        get_version()
    );
    render_title(&message);
}
