use std::io::stdin;

macro_rules! parser {
    ($iter:expr) => {
        $iter.next().trim().parse().expect("")
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("");

    let mut iter = input.split_whitespace();
    let avg: u32 = parser!(iter);
    let num: u32 = parser!(iter);
    println!("{}", avg * num);
}