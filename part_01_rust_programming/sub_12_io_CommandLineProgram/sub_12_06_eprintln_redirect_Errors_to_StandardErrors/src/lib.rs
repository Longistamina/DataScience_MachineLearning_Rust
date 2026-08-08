/*
 * In this ``lib.rs``, we define codes that are responsible for searching test
 *
 * Now, we add another test function for case_insensitive option.
 */

#![allow(unused_variables)]
#![allow(non_snake_case)]

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut outputs = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            outputs.push(line);
        }
    }

    outputs
}

///////////////////////////

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut outputs = Vec::new();

    let query = query.to_lowercase(); // lower case the query first, shadowing the original query

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) { // then lower case each line, and check contain
            outputs.push(line);
        }

        // here, we use ``.contains(&query)`` because after using ``query.to_lowercase()``,
        // the new ``query`` is a String, not a str anymore, while ``.contains()`` require a string slice,
        // so we use ``&query`` to create that string slice
    }

    outputs
}

// ---------------------------------------------------------- //

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

///////////////////////////

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."], // insensitive case: "rUsT" should match all these two
            search_case_insensitive(query, contents)
        )
    }
}
