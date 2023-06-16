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
    let h1: i32 = parser!(iter);
    let m1: i32 = parser!(iter);
    let h2: i32 = parser!(iter);
    let m2: i32 = parser!(iter);

    let totmin = (h2 - h1) * 60 + (m2 - m1);
    println!("{} {}", totmin / 60, totmin % 60);
}