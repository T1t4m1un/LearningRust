use std::io::{stdin};

fn main() {
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("");

    let parser = |s: &str| -> i32 {
        s.trim().parse().expect("")
    };

    let mut iter = input.split_whitespace();
    let a: i32 = parser(iter.next().expect(""));
    let b: i32 = parser(iter.next().expect(""));

    println!("{}", a + b);
}