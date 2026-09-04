/*
 * In this script, we will improve our old io commandline project
 * using the logic of iterator to remove the two expensive ``.clone()`` methods
 */

#![allow(non_snake_case)]

use std::env;
use std::fs;
use std::process;
use std::error::Error;

use sub_13_03_improve_ioProgram_noClone::{
    search,
    search_case_insensitive
};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,

}

impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // Our ``Config::build()`` now will take iterator as argument, not a vector anymore
        /*
         * We add ``impl Iterator<Item = String>`` to tell Rust that
         * the iterator ``args`` here can be any type that implements the Iterator trait
         * and returns String items.
         */

        args.next(); // first ``next()`` call to go through the program name
                     // (we don't need this argument so use no variable to capture)

        // second ``next()`` call, get the 1st true argument
        let query = match args.next() { // ``iterator.next()`` returns Option<T> => use match to get
            Some(args) => args,
            None => return Err("Didn't get a query string")
        };

        // third ``next()`` call, get the 2nd true argument
        let file_path = match args.next() {
            Some(args) => args,
            None => return Err("Didn't get a file path")
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        /*
         * if the user set env variable ``IGNORE_CASE=1``
         * then ``ignore_case = true``
         */

        Ok(Config {query, file_path, ignore_case})
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let outputs = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for matched_line in outputs {
        println!("{}", matched_line)
    }

    Ok(())
}

// ################ //
//       main       //
// ################ //

fn main() {
    println!();

    let args = env::args(); // Just get the iterator
                            // No call ``collect()``, no collect arguments into a vector anymore

    let config = Config::build(args).unwrap_or_else(|e| {
        // println!("Error parsing arguments: {e}");
        eprintln!("Error parsing arguments: {e}"); // redirects the error to be displayed in the standard error stream (terminal)
        process::exit(1);
    });

    println!("Searching for '{}'", config.query);
    println!("In file: {}", config.file_path);
    println!();

    if let Err(e) = run(config) {
        // println!("Application error: {e}");
        eprintln!("Application error: {e}"); // redirects the error to be displayed in the standard error stream (terminal)
        process::exit(1);
    }
}

// ">" operator is used to redirect standard error (terminal) to standard output (file like output.txt)

// cd .../sub_12_06_redirect_Errors_to_StandardErrors
// IGNORE_CASE=1 cargo run > ./src/output.txt
//              OR
// cargo run -- th input.txt > ./src/output.txt

// -------------------------------------

// IGNORE_CASE=1 cargo run -- th ./src/poem.txt > ./src/output.txt
//              OR
// cargo run -- th ./src/poem.txt > ./src/output.txt
