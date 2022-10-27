pub fn render_title(title: &str) {
    println!("{}{}", "=".repeat(80), " ".repeat(80)); // 80 chars
    println!(
        "{}{}{}",
        title,
        " ".repeat(80 - title.len()),
        " ".repeat(80)
    ); // 80 chars
    println!("{}{}", "=".repeat(80), " ".repeat(80)); // 80 chars
}
