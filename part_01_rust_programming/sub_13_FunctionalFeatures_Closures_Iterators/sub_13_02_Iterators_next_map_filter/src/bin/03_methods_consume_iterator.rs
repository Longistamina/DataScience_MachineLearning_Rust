/*
 * Some iterators have methods that call ``.next()`` inside them,
 * they are called ``consuming adapters`` because calling them uses up the iterator.
 *
 * For example, the ``.sum()`` method takes ownership of the iterator
 * and iterates through the items by repeatedly calling ``.next()``,
 * thus consuming the iterator.
 *
 * While iterating through the iterator,
 * it adds each item to a running total and returns the total when iteration is complete.
 */

use std::iter;

fn main() {
    println!();

    let v: Vec<f32> = iter::repeat_with(|| rand::random_range(0.0..=10.0))
        .take(5)
        .collect();

    println!("v = {:?}", v);

    let v_iter = v.iter();
    let v_sum: f32 = v_iter.sum(); // This will consume the ``v_iter``
    // Why do we have to annotate ``f32`` here?
    // Because many types implement ``sum()`` (or can say it is highly generic),
    // compiler cannot always guess the type.
    // => must specify the type to help the compiler figure out

    println!("v_sum = {}", v_sum);

    // for element in v_iter {
    //     println!("{}", element)
    // }
    /*
     * Cannot run this code anymore because ``v_iter`` has been consumed,
     * it is no longer valid to use.
     */

     ////////////////////////

     let v_sum2 = v.iter().sum::<f32>(); // calling ``v.iter()`` results in a completely new iterator
     // Another way to specify the type

     println!("v_sum2 = {}", v_sum2);
}
