fn main() {
    let p;

    {
        let x = 5;
        p = &x;
    }

    println!("{}", p);
}