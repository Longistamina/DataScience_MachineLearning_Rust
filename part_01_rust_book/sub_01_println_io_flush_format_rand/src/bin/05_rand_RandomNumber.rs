#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! rand = "0.8.5"
//! ```

use rand::prelude::*; // must use rand = "0.10.1" in Cargo.toml for this to work
                      // "prelude" is a common module of many Rust library, storing frequently used modules
                      // ::* means import everything from prelude

use rand::rngs::StdRng; // a seedable RNG engine
use std::iter; // for ``iter::repeat_with()``

fn main() {
    println!();

    println!("===============================================================================");
             // ------------------------ generate a random number ------------------------- //

    let mut rng = rand::rng(); // Create a random generator (must be "mut")

    let random_int = rng.random_range(1..=100);
    println!("The generated random INTEGER number is: {random_int}");
    // 1. rand::rng() provides a high-quality random number generator
    //    that is local to the current thread and automatically seeded by the OS.
    // 2. random_range() is a method from the RngExt trait
    //    used to generate a value within a specific bound.
    // 3. 1..=100 is an inclusive range syntax, meaning it covers all numbers
    //    from 1 up to and including 100.

    let random_float = rng.random_range(1.0..=100.0); // generate floating-point numbers
    println!("The generated random FLOATING number is: {random_float}");

    println!("===============================================================================");
             // --------------------------- generate with seed ---------------------------- //

    let seed: u64 = 42;
    let mut rng_seeded = StdRng::seed_from_u64(seed);

    let seeded_int = rng_seeded.random_range(1..=100);
    println!("Seeded random int = {}", seeded_int); // always the same

    println!("===============================================================================");
             // ------------- generate with std::iter::repeat_with(repeater) -------------- //

    let mut rng = rand::rng();

    // Generate 5 elements ranging from 2.0 (inclusive) to 5.0 (exclusive)
    let v1: Vec<f32> = std::iter::repeat_with(|| rng.random_range(2.0..5.0))
        .take(5)    // this results in an iterator, still lazy
        .collect(); // this realize the lazy iterator, to truly create the ``v1`` Vec<f32>

    println!("v1 = {:?}", v1);

    //////////////////////

    // Elegant shorthand utilizing top-level global random_range function
    let v2: Vec<f32> = iter::repeat_with(|| rand::random_range(3.0..=7.0)) // include 7.0
        .take(5)
        .collect();

    println!("v2 = {:?}", v2);
}
