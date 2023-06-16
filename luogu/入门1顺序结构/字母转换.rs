use std::io::stdin;

fn main() {
    let mut input = String::new();

    stdin()
        .read_line(&mut input)
        .expect("");

    let lowercase = input.trim();
    let uppercase = match lowercase.chars().next() {
        Some(c) if c.is_ascii_lowercase() => c.to_ascii_uppercase(),
        _ => return,
    };
    println!("{}", uppercase);
}