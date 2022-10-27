use regex::Regex;

pub fn validate_app_name(name: String) {
    let re = Regex::new(r"/^(?:@[a-z0-9-*~][a-z0-9-*._~]*\/)?[a-z0-9-~][a-z0-9-._~]*$/").unwrap();
    if !re.is_match(&name) {
        println!("App name must be lowercase, alphanumeric, and only use -, _, and @");
        std::process::exit(1);
    }
}
