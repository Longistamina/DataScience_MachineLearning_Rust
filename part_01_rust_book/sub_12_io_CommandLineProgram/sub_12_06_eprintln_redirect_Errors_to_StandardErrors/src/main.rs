#![allow(non_snake_case)]

use std::env;
use std::fs;
use std::process;
use std::error::Error;

use sub_12_06_redirect_Errors_to_StandardErrors::{
    search,
    search_case_insensitive
};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,

}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments!")
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
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

    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|e| {
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

// if run WITHOUT using eprintln!() => all the outputs including errors will be written INTO output.txt, not showed in terminal
// if run WITH eprintln!() => the errors will be showed in the TERMINAL ONLY, not in the output.txt

// -------------------------------------

// IGNORE_CASE=1 cargo run -- th ./src/poem.txt > ./src/output.txt
//              OR
// cargo run -- th ./src/poem.txt > ./src/output.txt

// even if using eprintln!(), since it runs successfully, there will be no error to display in the terminal
