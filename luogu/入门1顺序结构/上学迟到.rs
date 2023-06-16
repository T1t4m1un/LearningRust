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
    let s: f32 = parser!(iter);
    let v: f32 = parser!(iter);

    let cost = (s / v).ceil() as u32 + 10;
    let mm = 60 - (cost % 60);
    let hh: u32;
    if cost / 60 <= 7 {
        hh = 7 - (cost / 60);
    } else {
        hh = 31 - (cost / 60);
    }
    println!("{:02}:{:02}", hh, mm);
}