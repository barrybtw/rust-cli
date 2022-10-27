pub fn get_version() -> String {
    let version = env!("CARGO_PKG_VERSION");
    let version = version.to_string();
    version
}
