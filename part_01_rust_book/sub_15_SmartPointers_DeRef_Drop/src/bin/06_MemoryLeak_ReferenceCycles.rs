/*
A memory leak is memory that is no longer useful to the program
but is never released.

If enough memory is leaked over time,
the process or system may eventually run out of memory.

Generally, Rust has rules to prevent memory leak,
this is sometimes called "safe Rust".

Safe Rust prevents:
- dangling references
- use-after-free
- double free
- data races

Safe Rust does NOT guarantee:
- every allocation will eventually be freed

Therefore ``Rc<RefCell<T>>`` reference cycles can leak memory
without using unsafe Rust.

One example of memory leak are Reference Cycles.
Let's demo it here
*/

use std::rc::Rc;
use std::cell::RefCell;
use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List<T> {
    Cons(T, RefCell<Rc<List<T>>>),
    Nil
}

impl<T> List<T> {
    fn tail(&self) -> Option<&RefCell<Rc<List<T>>>> {
        match self {
            Cons(_, item) => Some(item), // (a, (b, Nil)) => returns the tail (b, Nil)
            Nil => None
        }
    }
}

// ============ //
//    main()    //
// ============ //

fn main() {
    println!();

    let a = Rc::new(Cons(32.5, RefCell::new(Rc::new(Nil)))); // a = Cons(32.5, Nil)
    println!("a initial rc count = {}", Rc::strong_count(&a)); // 1
    println!("a next item = {:?}", a.tail()); // Some(RefCell { value: Nil })

    println!();

    let b = Rc::new(Cons(64.8, RefCell::new(Rc::clone(&a)))); // b = Cons(64.8, a) = Cons(64.8, Cons(32.5, Nil))
    println!("a rc count after b creation = {}", Rc::strong_count(&a)); // 2
    println!("b initial rc count = {}", Rc::strong_count(&b)); // 1
    println!("b next item = {:?}", b.tail()); // Some(RefCell { value: Cons(32.5, RefCell { value: Nil }) })

    println!();

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
        // ``link`` is a reference to ``RefCell<Rc<List<T>>>``.
        // ``link.borrow_mut()`` gives temporary mutable access to the Rc stored inside that RefCell -> ``RefMut<Rc<List<T>>>``
        // ``*link.borrow_mut()`` dereference to strip away the ``RefMut`` and get the assignable ``Rc<List<T>>`` -> ``Rc<List<T>>``
        // ``*link.borrow_mut() = Rc::clone(&b)`` -> mutates ``Rc<List<T>>`` into this ``Rc<&b>``
        //
        // This increases the strong_count of ``b`` by 1 => becomes 2
        // In short, we changed ``a`` from Cons(32.5, Nil) to Cons(32.5, b) -> results in a vicious cycle
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b)); // 2
    println!("a rc count after changing a = {}", Rc::strong_count(&a)); // 2

    /*
    Originally, we have ``b -> a`` like this ``b = Cons(64.8, a) = Cons(64.8, Cons(32.5, Nil))``.
    + a strong_count = 2
    + b strong_count = 1

    After running this ``*link.borrow_mut() = Rc::clone(&b)``,
    we modifies the value ``Nil`` inside ``a`` to become a ``Rc<T>`` pointer pointing to ``b``
    -> results in ``Cons(32.5, b)``.
    + a strong_count = 2
    + b strong_count = 2

    What is the point of this?

    When ``main()`` ends, local variables ``a`` and ``b`` are dropped,
    so their strong_count drops by 1:
    + a strong_count: 2 -> 1
    + b strong_count: 2 -> 1

    So, their strong_count never reaches 1. Why?

    Because inside b, we also has ``b = Cons(64.8, a)``
    => this form an infinite reference cyles ``a -> b -> a -> b -> a -> ....``
    => they will live forever

    b --Rc--> a
    ^         |
    |         |
    +---Rc----+

    => this reference cyle makes both allocations leak
    */

    //----------------------//
    // Make the LEAK happen //
    //----------------------//

    // Uncomment the next line to see that we have a cycle; it will overflow the stack ().
    // println!("a next item = {:?}", a.tail());
}
