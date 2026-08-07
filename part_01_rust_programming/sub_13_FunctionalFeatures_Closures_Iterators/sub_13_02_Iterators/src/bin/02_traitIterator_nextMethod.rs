/*
 * All iterators implement a trait named Iterator that is defined in the standard library.
 ```
 pub trait Iterator {
     type Item;

     fn next(&mut self) -> Option<Self::Item>;

     // methods with default implementations elided
 }
 ```
 * New syntax: ``type Item`` and ``Self::Item``
 * => They are used to define new type named ``Item`` that is only associated with this trait (learn about it later)
 *
 * For now, all you need to know is that
 * this code says implementing the ``Iterator`` trait requires that you also define an ``Item`` type,
 * and this ``Item`` type is used in the return type of the next method.
 *
 * The Iterator trait only requires implementors to define one method: ``next()``
 * ``next()`` method is used to returns one item of the iterator at a time, wrapped in ``Some``,
 *  and, when iteration is over, returns ``None``.
 *
 * We can call the ``.next()`` method on iterators directly.
 */

#![allow(clippy::useless_vec)]

fn main() {
     println!();
     let v1 = vec![1, 2, 3];

     let mut v1_iter = v1.iter();
     // Here, we have to make it mutable,
     // because calling the ``next()`` method on an iterator changes internal state
     // that the iterator uses to keep track of where it is in the sequence.

     assert_eq!(v1_iter.next(), Some(&1)); // Get the first element, returns as Some(&1)
     assert_eq!(v1_iter.next(), Some(&2)); // Get the first element, returns as Some(&2)
     assert_eq!(v1_iter.next(), Some(&3));
     assert_eq!(v1_iter.next(), None);

     println!("Demo iter-next passed!!!");

     println!("===============================================");

     let v2 = vec![4, 5, 6];

     let mut v2_iter_enumerate = v2.iter().enumerate();

     assert_eq!(v2_iter_enumerate.next(), Some((0, &4))); // Get the first pair index-element, returns as Some((0, &4))
     assert_eq!(v2_iter_enumerate.next(), Some((1, &5)));
     assert_eq!(v2_iter_enumerate.next(), Some((2, &6)));
     assert_eq!(v2_iter_enumerate.next(), None);

     println!("Demo iter-next-enumerate passed!!!");
}
