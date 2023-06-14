use std::fmt::{Display, Formatter, Result};
use std::io::{self};

struct Structure(i32);

impl Display for Structure {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.0)
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let num: i32 = input.trim().parse().unwrap();
    let s = Structure(num);
    println!("{}", s);
}