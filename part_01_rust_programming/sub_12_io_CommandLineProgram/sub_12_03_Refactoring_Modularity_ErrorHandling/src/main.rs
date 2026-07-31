/*
 * As the program grows, the number of separate tasks the main function handles will increase.
 * In this case, it is best to split up the separate concerns of a binary program
 * when the main function starts getting large.
 *
 * The process contains the following steps:
 * + Split your program into a ``main.rs`` file and a ``lib.rs`` file and move your program’s logic to ``lib.rs``
 * + As long as your command line parsing logic is small, it can remain in the ``main`` function.
 * + When the command line parsing logic starts getting complicated,
 *   extract it from the ``main`` function into ``other functions`` or ``types``
 *
 * After this process, the ``main`` function should be limited to the following:
 * + Calling the command line parsing logic with the argument values
 * + Setting up any other configuration
 * + Calling a ``run`` function in lib.rs
 * + Handling the error if run returns an error
 *
 * This pattern is about separating concerns:
 * + ``main.rs`` handles running the program
 * + ``lib.rs`` handles all the logic of the task at hand
 *
 * Because you can’t test the ``main`` function directly,
 * this structure lets you test all of your program’s logic by moving it out of the ``main`` function.
 *
 * The code that remains in the ``main`` function will be small enough to verify its correctness by reading it.
 * Let’s rework our program by following this process
 */

#![allow(non_snake_case)]

use std::env;
use std::fs;
use std::process; // ``process::exit()`` to exit programs
use std::error::Error; // for Error trait

// import ``search`` functionality from package name (sub_12_...) to search text
use sub_12_03_Refactoring_Modularity_ErrorHandling::search;

// ``query`` and ``file_path`` are configuration variables => group them into a new type named Configs
struct Config {
    query: String,
    file_path: String
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> { // The ``build`` method is a constructor, takes arguments from args to construct a Config
                                                                // Our error values will always be string literals that have the 'static lifetime.
        if args.len() < 3 { // Prevent index out of bound error when users don't provide enough inputs
            return Err("not enough arguments"); // if fails, return ``Err(error)``
        }

        let query = args[1].clone(); // Now, use ``clone()`` to make it straightforward, but it has trade-offs
        let file_path = args[2].clone();

        Ok(Config {query, file_path}) // if it works, returns ``Ok(Config)``
    }
}

// Create the ``run`` function that handles the logic of the program
fn run(config: Config) -> Result<(), Box<dyn Error>> { // means the function will return a type that implements the ``Error`` trait, but does not know specifically yet
    let contents = fs::read_to_string(config.file_path)?; // if Ok(T) then assign T to ``contents``, else returns Err(error)

    for line in search(&config.query, &contents) { // put the ``query`` and ``contents`` into search to search the file
        println!("{line}"); // for now, just print the line
    }

    Ok(())
}

// ################ //
//       main       //
// ################ //

fn main() {
    println!();

    let args: Vec<String> = env::args().collect(); // get the arguments

    let config = Config::build(&args).unwrap_or_else(|error| { // Config::build() returns a Result type, use ``unwrap_or_else`` to handle
        println!("Problem parsing arguments: {error}");
        process::exit(1); // stop the program here right away if encounters ``Err(error)``, return number 1
    });

    println!("Searching for '{}'", config.query); // access config variables like attributes
    println!("In file: {}", config.file_path);
    println!();

    // Because ``run`` returns ``Result<(), Box<dyn Error>>``, it could be () or an ``Err(error)``
    // therefore, we must use ``if let Err(e) = ...`` here to instruct Rust how to handle the returned ``Err(error)``
    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

// cd .../sub_12_03_Refactoring_Modularity_ErrorHandling
// cargo run -- the ./src/poem.txt
/*
    Searching for 'the'
    In file: ./src/poem.txt
    With text:
    I'm nobody! Who are you?
    Are you nobody, too?
    Then there's a pair of us - don't tell!
    They'd banish us, you know.

    How dreary to be somebody!
    How public, like a frog
    To tell your name the livelong day
    To an admiring bog!
 */
