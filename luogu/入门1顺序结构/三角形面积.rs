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
    let a: f64 = parser!(iter);
    let b: f64 = parser!(iter);
    let c: f64 = parser!(iter);

    let p = (a + b + c) * 0.5;
    let square = (p * (p - a) * (p - b) * (p - c)).sqrt();
    println!("{:.1}", square);
}