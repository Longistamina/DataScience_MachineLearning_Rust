#![allow(non_snake_case)]

use std::env;
use std::fs;
use std::process;
use std::error::Error;

use sub_12_04_AddFunctionality_TestDrivenDevelopment::search;

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments!")
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config {query, file_path})
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for output in search(&config.query, &contents) {
        println!("{}", output)
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
        println!("Error parsing arguments: {e}");
        process::exit(1);
    });

    println!("Searching for '{}'", config.query);
    println!("In file: {}", config.file_path);
    println!();

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

// cd .../sub_12_04_AddFunctionality_TestDrivenDevelopment
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
