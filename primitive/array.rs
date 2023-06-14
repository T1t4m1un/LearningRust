fn cnt(arr: &[i32]) {
    println!("{}", arr.len());
}

fn main() {
    let my_array = [1, 1, 4, 5, 1, 4];
    for i in 1..my_array.len() + 1 {
        cnt(&my_array[0 .. i]);
    }
}