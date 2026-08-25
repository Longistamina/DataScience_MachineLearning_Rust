/*
"Memory leak" is the memory that is never cleaned up.
=> causes system to overflow and crash.

Generally, Rust has rules to prevent memory leak,
this is sometimes called "safe Rust".

However, Rust does allow "unsafe Rust" via using ``Rc<T>`` and ``RefCell<T>``.
In those cases, Rust cannot fully guarantee memory safe.

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

fn main() {
    println!();

    let a = Rc::new(Cons(32.5, RefCell::new(Rc::new(Nil)))); // a = Cons(32.5, Nil)
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    println!();

    let b = Rc::new(Cons(64.8, RefCell::new(Rc::clone(&a)))); // b = Cons(64.8, a) = Cons(64.8, Cons(32.5, Nil))
    println!("a rc count after b creation = {}", Rc::strong_count(&a)); // 2
    println!("b initial rc count = {}", Rc::strong_count(&b)); // 1
    println!("b next item = {:?}", b.tail());

    println!();

    if let Some(link) = a.tail() { // ``link`` will be ``RefCell { value: Nil }`` (not ``Nil``)
        *link.borrow_mut() = Rc::clone(&b);
        // ``*link`` is ``Nil``
        // so ``*link.borrow_mut()`` returns ``&mut Nil``
        // then mutates this ``&mut Nil`` to ``Rc::clone(&b)``
        // In short, we changed ``a`` from Cons(32.5, Nil) to Cons(32.5, b) -> results in a vicious cycle
    }
    /*
    Originally, we have ``b -> a`` like this ``b = Cons(64.8, a) = Cons(64.8, Cons(32.5, Nil))``.

    But after running this ``*link.borrow_mut() = Rc::clone(&b)``, we modifies the value ``Nil`` in a to b
    -> results in ``Cons(32.5, b)``.

    But inside b, we also has ``b = Cons(64.8, a)``
    => this form an infinite reference cyles ``a -> b -> a -> b -> a -> ....``

    b ---> a
    ^      |
    |      v
    <------

    => this reference cyle makes the stack overflow
    */

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle; it will overflow the stack ().
    // println!("a next item = {:?}", a.tail());
}
