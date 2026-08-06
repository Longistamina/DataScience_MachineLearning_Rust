/*
 * A closure in Rust can be a ``FnOnce`` or ``FnMut`` function
 */

#![allow(non_snake_case, clippy::unnecessary_literal_unwrap)]

fn demo_FnOnce() {
    /*
     * An ``FnOnce`` closure can only be called exactly once.
     * This happens because the closure takes ownership of its captured variables
     * and moves them out of its environment when executed.
     * Once those variables are moved, the closure is consumed and cannot be run again.
     *
     * For example, why ``unwrap_or_else()`` use FnOnce closures?
     * The ``Option::unwrap_or_else()`` method extracts a value from an ``Option``.
     * If the option is ``None`` (or ``Err(error)``), it runs a fallback closure to do something else.
     *
     * Because ``unwrap_or_else`` will execute its closure at most once (only when the value is ``None`` or ``Err(error)``),
     * it accepts any closure that implements ``FnOnce``.
     * => This allows you to safely pass expensive operations or move ownership of values out of the environment.
     */

     let secret_code = String::from("SECRET_123"); // Owns a String
     let fallback_option: Option<String> = None;

     // The closure captures `secret_code` by VALUE (moving it)
     let value = fallback_option.unwrap_or_else(|| {
         println!("Value missing! Generating default payload...");
         secret_code // secret_code is MOVED out of the closure here
     });

     println!("Result: {}", value); // ``value`` now takes the ownership of ``secret code``

     // println!("{secret_code}")
     /*
      * ERROR: You cannot use `secret_code` here because it was moved into the closure,
      * and the closure consumed it when it executed.
      */
}

fn demo_FnMut() {
    /*
     * An ``FnMut`` closure can be called multiple times
     * and is allowed to mutate its captured environment.
     * It borrows variables from its environment using mutable references (&mut).
     *
     * Why ``sort_by_key()`` uses ``FnMut``?
     * The ``slice::sort_by_key`` method reorders elements in a collection
     * based on a key produced by your closure.
     *
     * Sorting requires evaluating the key for multiple elements in the collection,
     * meaning the closure must be executed many times.
     * => Because it runs iteratively, it cannot be an ``FnOnce``
     *
     * However, Rust allows it to be an ``FnMut``
     * so that you can modify an external state (like a counter or a cache) every time a key is evaluated.
     */

     let mut fruits = vec!["banana", "apple", "cherry"];
     let mut evaluation_count = 0; // External state to mutate

     // sort_by_key runs this closure multiple times
     fruits.sort_by_key(|fruit| {
         evaluation_count += 1; // Mutating the captured variable
         fruit.len()            // Sorting by string length
     });

     println!("Sorted fruits: {:?}", fruits);
     println!("The closure was executed {} times.", evaluation_count);
}

fn main() {
    println!();

    demo_FnOnce();

    println!("=========================================================================================");

    demo_FnMut();
}
