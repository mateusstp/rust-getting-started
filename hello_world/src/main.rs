use regex::Regex;

fn main() {
    println!("Hello, world!");
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("Did our date match? {}", re.is_match("2025-02-17"));
}
