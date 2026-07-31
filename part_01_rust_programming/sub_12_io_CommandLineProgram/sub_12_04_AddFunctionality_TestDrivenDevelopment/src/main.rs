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

    for line in search(&config.query, &contents) {
        println!("{}", line)
    }

    Ok(())
}

// ################ //
//       main       //
// ################ //

fn main() {
    println!("Hello, world!");
}
