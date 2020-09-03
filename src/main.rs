use skewes::slice::*;

fn main() {
    let a = vec!(20, 3);
    let b = vec!(7);
    let (d, r) = short_div(&a, &b);
    println!("Answer: {}", d);
}