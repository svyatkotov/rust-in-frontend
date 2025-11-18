fn main() {
    assert_eq!(format_message("Слава", 15, 50), "Привет, Слава! Ваш счёт: 15, уровень: 50.");

    assert_eq!(build_greeting("Rust", "is cool"), "Rust is cool");
}

fn format_message(name: &str, score: u32, level: u32) -> String {
    format!("Привет, {name}! Ваш счёт: {score}, уровень: {level}.")
}

fn build_greeting(name: &str, suffix: &str) -> String {
    let mut res = name.to_string();

    res.push(' ');
    res.push_str(suffix);
    res
}
