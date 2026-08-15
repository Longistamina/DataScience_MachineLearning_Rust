#![allow(unused_imports)]
use rand;

fn main() {
    println!();

    let x = 32;
    let y = 43;

    println!("{x} + {y} = {}", adder::add(x, y));
    println!("{x} - {y} = {}", subtracter::subtract(x, y));
}
