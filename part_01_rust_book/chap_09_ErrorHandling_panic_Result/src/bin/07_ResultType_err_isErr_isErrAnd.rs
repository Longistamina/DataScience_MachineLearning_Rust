// This script demos the methods `.is_err()`, `.is_err_and()`, and `.err()` of `Result<T, E>`

fn main() {
    // ------------------------------------------------------------
    // 1. Single Result<T, E> values
    // ------------------------------------------------------------

    let success: Result<i32, &str> = Ok(42);
    let failure: Result<i32, &str> = Err("something went wrong");

    // is_err() checks whether the Result is Err.
    // It returns a bool and does not extract the error.
    println!("success.is_err() = {}", success.is_err()); // false
    println!("failure.is_err() = {}", failure.is_err()); // true

    /////////////////////////////////

    // is_err_and() checks two things:
    //
    // 1. Is the Result Err?
    // 2. If it is Err, does the contained error satisfy a condition?
    //
    // Err(error) -> run the predicate on error
    // Ok(value)  -> false
    let success: Result<i32, &str> = Ok(42);
    let failure: Result<i32, &str> = Err("something went wrong");

    println!(
        "failure.is_err_and(|error| error.contains(\"wrong\")) = {}",
        failure.is_err_and(|error| error.contains("wrong"))
    ); // true

    println!(
        "success.is_err_and(|error| error.contains(\"wrong\")) = {}",
        success.is_err_and(|error| error.contains("wrong"))
    ); // false

    let other_failure: Result<i32, &str> = Err("invalid input");

    println!(
        "other_failure.is_err_and(|error| error.contains(\"wrong\")) = {}",
        other_failure.is_err_and(|error| error.contains("wrong"))
    ); // false

    /////////////////////////////////

    // err() converts:
    //
    // Result<T, E> -> Option<E>
    //
    // Err(error) -> Some(error)
    // Ok(value)  -> None
    let success: Result<i32, &str> = Ok(42);
    let failure: Result<i32, &str> = Err("something went wrong");

    let success_error = success.err();
    let failure_error = failure.err();

    println!("success.err() = {:?}", success_error); // None
    println!("failure.err() = {:?}", failure_error); // Some("something went wrong")

    // ------------------------------------------------------------
    // 2. Apply the same ideas to an iterable: Vec<Result<T, E>>
    // ------------------------------------------------------------

    let results: Vec<Result<i32, &str>> = vec![
        Ok(10),
        Err("bad value"),
        Ok(20),
        Err("another bad value"),
        Ok(30),
    ];

    // is_err() can be used with filter() to keep only failed Results.
    let failed_results: Vec<&Result<i32, &str>> =
        results.iter().filter(|result| result.is_err()).collect();

    println!("failed Results = {:?}", failed_results);
    // [Err("bad value"), Err("another bad value")]

    /////////////////////////////////

    // is_err_and() can filter failed Results based on the contained error.
    //
    // Here we keep only errors whose message contains "another".
    //
    // Because iter() gives us &Result<i32, &str>, we use as_ref()
    // so the original Results are not moved.
    let matching_failed_results: Vec<&Result<i32, &str>> = results
        .iter()
        .filter(|result| {
            result
                .as_ref()
                .is_err_and(|error| error.contains("another"))
        })
        .collect();

    println!(
        "matching failed Results = {:?}",
        matching_failed_results
    );
    // [Err("another bad value")]

    /////////////////////////////////

    // err() is especially useful with filter_map().
    // Each Result becomes Option<E>:
    //
    // Err(error) -> Some(error), so the error is kept (strips away the Some)
    // Ok(_)      -> None,        so it is discarded
    let errors: Vec<&str> = results
        .into_iter()
        .filter_map(|result| result.err())
        .collect();

    println!("errors = {:?}", errors);
    // ["bad value", "another bad value"]
}
