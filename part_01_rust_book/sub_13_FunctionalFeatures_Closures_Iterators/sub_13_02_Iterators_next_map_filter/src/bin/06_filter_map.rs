/*
 * Creates an iterator that both filters and maps.
 * The returned iterator yields only the values for which the supplied closure returns Some(value).
 *
 * ``filter_map`` can be used to make chains of ``filter`` and ``map`` more concise.
 *
 * The examples below shows how a map().filter().map() can be shortened to a single call to filter_map.
 */

fn main() {
    println!();

    let a = ["1", "two", "NaN", "four", "5"];

    let a_num = a.iter().filter_map(|s| s.parse::<f32>().ok()).collect::<Vec<_>>();

    println!("a_num = {:?}", a_num); // [1.0, NaN, 5.0]

    /*
     * How did it work?
     *
     * For "5", ``s.parse::<f32>()`` results in ``Ok(5.0)``,
     * then ``.ok()`` converts it into ``Some(5.0)``,
     * then ``filter_map()`` sees ``Some(5.0)``,
     * it strips away the ``Some``
     * => returns ``5.0``
     * => same for "1" and "NaN"
     *
     * For "two", ``s.parse::<f32>()`` results in ``Err(error)``,
     * then ``.ok()`` converts it into ``None``,
     * then ``filter_map()`` sees ``None``,
     * => it discards this "two" element.
     * => same for "four"
     */

    /////////////////////////////////////////////

    let b = ["1", "two", "NaN", "four", "5"];

    let b_num = b.iter() // implementation without ``filter_map()``
        .map(|s| s.parse::<f32>())
        .filter(|s| s.is_ok())
        .map(|s| s.unwrap())
        .collect::<Vec<_>>();

    println!("b_num = {:?}", b_num); // [1.0, NaN, 5.0]
}
