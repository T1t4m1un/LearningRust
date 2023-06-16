use std::io::stdin;

const PI: f32 = 3.14;

macro_rules! parse {
    ($iter:expr) => {
        $iter.next().expect("").trim().parse().expect("")
    };
}

fn main() {
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("");

    let mut iter = input.split_whitespace();
    let h: f32 = parse!(iter);
    let r: f32 = parse!(iter);
    let res: u32 = (20000.0 / (r * r * h * PI)).ceil() as u32;
    println!("{}", res);
}