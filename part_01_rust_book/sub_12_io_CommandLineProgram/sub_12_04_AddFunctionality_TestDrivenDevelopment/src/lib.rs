/*
 * In this ``lib.rs``, we define codes that are responsible for searching test
 *
 * In this section, we’ll add the searching logic to the minigrep program
 * using the test-driven development (TDD) process with the following steps:
 * + Write a test that fails and run it to make sure it fails for the reason you expect.
 * + Write or modify just enough code to make the new test pass.
 * + Refactor the code you just added or changed and make sure the tests continue to pass.
 * + Repeat from step 1!
 *
 * Though it’s just one of many ways to write software, TDD can help drive code design.
 * Writing the test before you write the code that makes the test pass
 * helps maintain high test coverage throughout the process.
 */

#![allow(unused_variables)]
#![allow(non_snake_case)]

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut outputs = Vec::new(); // create an empty vector to store the matched lines

    for line in contents.lines() {
        if line.contains(query) {
            outputs.push(line);
        }
    }

    outputs
}
/*
 * The lifetime signature 'a is used to tell Rust that the returned vector
 * has the same lifetime as ``contents`` (lives as long as ``contents``).
 * This also implies that the returned vector is a referenced slice of ``contents``, not ``query``
 *
 * Why need 'a here? Because both parameters are borrowed, and Rust does not know which one we need for the output,
 * so must use lifetime signature 'a to help Rust specify that.
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }
}
