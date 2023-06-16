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
    let t: f32 = parser!(iter);
    let n: f32 = parser!(iter);

    println!("{:.3}", t / n);
    println!("{}", 2 * n as u32);
}