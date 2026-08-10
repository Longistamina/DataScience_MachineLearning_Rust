/*
 * Once a closure has captured a reference or captured ownership of a value from the environment,
 * the code in the body of the closure defines what happens to the references or values when the closure is evaluated later
 * => thus it affects what, if anything, is moved out of the closure
 *
 * A closure body can do things like:
 * + move a captured value out of the closure
 * + mutate the captured value
 * + neither move nor mutate the value
 * + capture nothing from the environment to begin with
 */

 #[derive(Debug)]
 #[allow(dead_code)]
 struct Rectangle {
     width: u32,
     height: u32,
 }

 fn main() {
     let mut list = [
         Rectangle { width: 10, height: 1 },
         Rectangle { width: 3, height: 5 },
         Rectangle { width: 7, height: 12 },
     ];

     let mut num_sort_operations = 0;

     // let mut sort_operations = vec![];
     // let value = String::from("closure called");

     list.sort_by_key(|r| {
         // sort_operations.push(value);
         num_sort_operations += 1;
         r.width
     });
     println!("{list:#?}, sorted in {num_sort_operations} operations");
 }

 /*
  * In this example ``.sort_by_key(|r| {num_sort_operations += 1; r.width})``,
  * the closure ``|r| {num_sort_operations += 1; r.width}`` is an ``FnMut`` closure.
  * (While the closure in ``unwrap_or_else()`` is an ``FnOnce`` closure)
  *
  * Why, because to sort a list, it must calls the closure ``|r| {num_sort_operations += 1; r.width}`` multiple times,
  * each time it accesses ``r.width`` then returns it for ``sort_by_key`` to use it as sorting key.
  *
  * The closure ``|r| {num_sort_operations += 1; r.width}`` doesn’t capture, mutate, or move anything out from its environment,
  * so it meets the trait bound requirements.
  *
  * If we remove the comment operators ``//`` of the ``sort_operations.push(value);``,
  * the program will panic. Why?
  * Because, at the first run, the ``value`` is moved from ``main()`` into the closure, then dropped after this run ended.
  * Therefore, in the second run, there is no more valid ``value`` to be ``.push()`` into the sort_operations
  * => fails
  */
