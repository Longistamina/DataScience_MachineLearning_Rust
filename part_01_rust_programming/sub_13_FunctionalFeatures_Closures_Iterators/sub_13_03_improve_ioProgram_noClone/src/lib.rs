/*
 * In this ``lib.rs``, we define codes that are responsible for searching test
 *
 * Now, we can also take advantage of the logic of iterator and ``filter()``
 * for the search logic.
 *
 * /////////////////////
 *
 * Before this change, the program won’t print any results until it has collected all of the results.
 *
 * but after the change, the results will be printed as each matching line is found
 * because the ``for`` loop in the ``run()`` function (main.rs) is able to take advantage of the laziness of the iterator.
 */

#![allow(unused_variables)]
#![allow(non_snake_case)]

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut outputs = Vec::new();

    // for line in contents.lines() {
    //     if line.contains(query) {
    //         outputs.push(line);
    //     }
    // }

    // outputs

    contents
        .lines() // return an iterator that iterates through each line of the contents
        .filter(|line| line.contains(query)) // use ``filter()`` to get the lines containing query only
        .collect() // calls ``collect()`` to collect the elements into a vector (Vec<&'a str>), then return it
}

///////////////////////////

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
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
