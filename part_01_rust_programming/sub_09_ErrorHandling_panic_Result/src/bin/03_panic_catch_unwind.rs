/*
 * ``panic::catch_unwind()`` is a built-in Rust function
 * that allows you to catch unhandled runtime crashes (panics)
 * before they terminate your entire program.
 *
 * When a panic happens in Rust, the program begins a process called "unwinding."
 * This means it walks backward through the execution history (call stack),
 * cleans up data (runs destructors/drops memory), and then aborts the program.
 *
 * ``catch_unwind()`` acts like a safety net that stops this backward walk,
 * intercepts the panic object, and lets the program keep running.
 *
 * ``catch_unwind()`` returns a Result<T, E>
 * => use ``match``, ``unwrap_or_else`` to handle.
 *
 * ------------------------------------------------------------------------------------
 *
 * To compare with ``try-except`` in Python:
 * + ``try-except`` can catch all operational errors (file missing, wrong types, ...),
 *    while ``catch_unwind()`` only catches casttrophic bugs (index out of bound, divsion by zero, ``panic()!``)
 * + ``try-except`` can catch runtime type mismatches easily,
 *   while ``catch_unwind()`` catch type mismatches at runtime, runtime types must always match to the compiled types
 */

use std::panic;

fn main() {
    println!();

    let divisor = 0;

    // 1. Wrap the risky runtime code inside the ``catch_unwind()`` closure
    let result = panic::catch_unwind(|| {
        println!("Attempting division by zero ...");
        let answer = 100 / divisor; // This will crash/panic at runtime if divisor is 0!!!
        answer // Return value if successful
    });

    // 2. Handle the outcome using standard pattern matching
    match result {
        Ok(value) => println!("Codes run successfully, value = {}", value),
        Err(error) => { // This acts exactly like your Python 'except:' block in Python
            println!("\nCaught a panic, error = {:?}", error);
            println!("But the program still runs.")
        }
    }

}

// Attempting division by zero ...

// thread 'main' (543321) panicked at part_01_rust_programming/sub_09_ErrorHandling_panic_Result/src/bin/03_panic_catch_unwind.rs:35:22:
// attempt to divide by zero
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

// Caught a panic, error = Any { .. }
// But the program still runs.
