/*
 * Many iterator adapters take closures as arguments,
 * and commonly those closures will capture their environment.
 *
 * For this example, we’ll use the ``filter()`` method that takes a closure.
 * The closure gets an item from the iterator and returns a bool.
 *
 * If the closure returns ``true``, the value will be included in the iteration produced by ``filter``.
 * If the closure returns ``false``, the value won’t be included.
 *
 * NOTE: here, we also use ``iterable.into_iter()``,
 *       it also creates an iterator from the iterable,
 *       but will take the ownership of the iterable!
 *
 * ``let iter = iterable.into_iter()``
 */

#[derive(Debug)]
#[allow(dead_code)]
struct Shoe<T> {
    size: T,
    style: String,
}

fn main() {
    println!();

    let shoes = vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker"),
                },
                Shoe {
                    size: 13,
                    style: String::from("sandal"),
                },
                Shoe {
                    size: 10,
                    style: String::from("boot"),
                },
            ];

    let target_size: u32 = 10;
    let shoes_filtered = shoes.into_iter() // This will take the onwership of ``shoes``, cannot use it anymore
        .filter(|s| s.size == target_size) // only get the shoes whose s.size == target_size
        .collect::<Vec<Shoe<_>>>();

    println!("shoes_filtered = {:?}", shoes_filtered)
    // shoes_filtered = [Shoe { size: 10, style: "sneaker" }, Shoe { size: 10, style: "boot" }]
}
