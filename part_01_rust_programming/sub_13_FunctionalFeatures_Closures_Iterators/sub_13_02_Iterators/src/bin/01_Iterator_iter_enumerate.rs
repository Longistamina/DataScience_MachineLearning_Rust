/*
 * An iterator is responsible for the logic of iterating over each item
 * and determining when the sequence has finished.
 * When you use iterators, you don’t have to reimplement that logic yourself.
 *
 * In Rust, iterators are ``lazy``,
 * meaning they have no effect until you call methods that consume the iterator to use it up.
 *
 * Use ``.iter()`` method to create an interator.
 * Use ``.iter().enumerate()`` to get both the index and value while looping.
 */

#![allow(clippy::useless_vec)]

fn main() {
    println!();

    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();
    // Calls ``.iter()`` method to create an iterator ``v1_iter`` from the original ``v1``
    // This ``v1_iter`` is just a ``lazy`` object, they do nothing.

    println!("Iterationg through v1...");

    for val in v1_iter {
        println!("{}", val)
    }

    println!("===============================================");

    let v2 = vec![4, 5, 6];

    println!("Iterationg through v2...");

    for val in v2.iter() { // A shorter version
        println!("{}", val)
    }

    println!("===============================================");

    let v3 = vec![7, 8, 9];

    println!("Iterationg through v3 with indices...");
    println!("idx: val");
    for (idx, val) in v3.iter().enumerate() {
        println!(" {} :  {}", idx, val)
    }
    // idx: val
    //  0 :  7
    //  1 :  8
    //  2 :  9
}
