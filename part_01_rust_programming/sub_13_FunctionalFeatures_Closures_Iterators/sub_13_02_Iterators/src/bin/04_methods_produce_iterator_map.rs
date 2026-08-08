/*
 * We also have another group of methods,
 * called ``iterator adapters``.
 *
 * One example is ``.map(mapper)`` (or ``iterable.iter().map(mapper)``).
 *
 * Since ``.map(mapper)`` produces another iterator from the original one,
 * we must use a variable to receive the returned.
 *
 * ``let iter_map = iterable.iter().map(mapper).collect()``
 *
 * Since ``.map()`` also returns a lazy interator only,
 * we need to use ``.collect()`` to realize the collection/iterable.
 */

use std::iter;

fn main() {
    println!();

    let v: Vec<i32> = iter::repeat_with(|| rand::random_range(0..=20))
        .take(5)
        .collect();

    println!("v = {:?}", v);

    // v.iter().map(|x| x*2);
    /*
     * Cannot run this code,
     * this will encounter ``#[warn(unused_must_use)]``
     */

     let v_map: Vec<_> = v.iter().map(|x| x*2).collect();
     // Here, why do we have to annotate ``Vec<_>``?
     // Same idea as ``.sum()``, the ``.collect()`` is highly generic,
     // it can return a Vec, a HashMap, a LinkedList, a String, or even a Result.
     // The compiler cannot always guess it
     // => we must specify the type to help the compiler figure out.

     let v_map2 = v.iter().map(|x| x*2).collect::<Vec<_>>();
     // Another way to specify the type

     println!("v_map = {:?}", v_map);
     println!("v_map2 = {:?}", v_map2);
}
