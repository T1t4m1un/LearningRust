use std::io::stdin;

macro_rules! parser {
    ($iter:expr) => {
        $iter.next().expect("").parse().expect("")
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("");

    let mut iter = input.split_whitespace();
    let a: i32 = parser!(iter);
    let b: i32 = parser!(iter);

    let tot = a * 10 + b;
    println!("{}", tot / 19);
}