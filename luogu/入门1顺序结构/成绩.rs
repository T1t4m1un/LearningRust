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
    let a: f32 = parser!(iter);
    let b: f32 = parser!(iter);
    let c: f32 = parser!(iter);

    println!("{}", (0.2 * a + 0.3 * b + 0.5 * c) as i32);
}