// This script demos the methods `.is_ok()`, `.is_ok_and()`, and `.ok()` of `Result<T, E>`

fn main() {
    // ------------------------------------------------------------
    // 1. Single Result<T, E> values
    // ------------------------------------------------------------

    let success: Result<i32, &str> = Ok(42);
    let failure: Result<i32, &str> = Err("something went wrong");

    // is_ok() checks whether the Result is Ok.
    // It returns a bool and does not extract the value.
    println!("success.is_ok() = {}", success.is_ok()); // true
    println!("failure.is_ok() = {}", failure.is_ok()); // false

    /////////////////////////////////

    // is_ok_and() checks two things:
    //
    // 1. Is the Result Ok?
    // 2. If it is Ok, does the contained value satisfy a condition?
    //
    // Ok(value)  -> run the predicate on value
    // Err(error) -> false
    let success: Result<i32, &str> = Ok(42);
    let failure: Result<i32, &str> = Err("something went wrong");

    println!(
        "success.is_ok_and(|value| value > 40) = {}",
        success.is_ok_and(|value| value > 40)
    ); // true

    println!(
        "failure.is_ok_and(|value| value > 40) = {}",
        failure.is_ok_and(|value| value > 40)
    ); // false

    let small_value: Result<i32, &str> = Ok(10);

    println!(
        "small_value.is_ok_and(|value| value > 40) = {}",
        small_value.is_ok_and(|value| value > 40)
    ); // false

    /////////////////////////////////

    // ok() converts:
    //
    // Result<T, E> -> Option<T>
    //
    // Ok(value) -> Some(value)
    // Err(error) -> None
    let success: Result<i32, &str> = Ok(42);
    let failure: Result<i32, &str> = Err("something went wrong");

    let success_value = success.ok();
    let failure_value = failure.ok();

    println!("success.ok() = {:?}", success_value); // Some(42)
    println!("failure.ok() = {:?}", failure_value); // None

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

    // is_ok() can be used with filter() to keep only successful Results.
    let successful_results: Vec<&Result<i32, &str>> =
        results.iter().filter(|result| result.is_ok()).collect();

    println!("successful Results = {:?}", successful_results);
    // [Ok(10), Ok(20), Ok(30)]

    /////////////////////////////////

    // is_ok_and() can filter successful Results based on the contained value.
    //
    // Here we keep only Ok values greater than or equal to 20.
    //
    // Because iter() gives us &Result<i32, &str>, we use as_ref()
    // to turn it into Result<&i32, &&str> without moving anything.
    let large_successful_results: Vec<&Result<i32, &str>> = results
        .iter()
        .filter(|result| result.as_ref().is_ok_and(|value| *value >= 20))
        .collect();

    println!(
        "successful Results >= 20 = {:?}",
        large_successful_results
    );
    // [Ok(20), Ok(30)]

    /////////////////////////////////

    // ok() is especially useful with filter_map().
    // Each Result becomes Option<T>:
    //
    // Ok(value) -> Some(value), so the value is kept (strips away the Some)
    // Err(_)     -> None,       so it is discarded
    let successful_values: Vec<i32> = results
        .into_iter()
        .filter_map(|result| result.ok())
        .collect();

    println!("successful values = {:?}", successful_values);
    // [10, 20, 30]
}
