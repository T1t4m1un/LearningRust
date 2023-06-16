use std::io::stdin;

fn main() {
    let mut input = String::new();

    stdin().read_line(&mut input).expect("");

    let res: String = input.trim().chars().rev().collect();

    println!("{}", res);

}