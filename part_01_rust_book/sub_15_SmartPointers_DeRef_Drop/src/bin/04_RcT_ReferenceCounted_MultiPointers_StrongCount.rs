#![allow(non_snake_case, unused_variables)]

// ==============================================================
// 1. ``Rc<T>``, the Reference-Counted Smart Pointer
// ==============================================================
/*
In most cases, ownership is clear: You know exactly which variable owns a given value.

However, there are cases when a single value might have multiple owners.
For example, in graph data structures, multiple edges might point to the same node,
and that node is conceptually owned by all of the edges that point to it.

So, if a node still has edges pointing to it, we should not clean that node.
Only when there are no more edges pointing to it, that node is now fine to clean up.

---------------------------------------------------------

The solution is ``Rc<T>`` smart pointer type in Rust.
(Rc = reference counting)

The ``Rc<T>`` type keeps track of the number of references to a value
to determine whether or not the value is still in use.

If there are zero references to a value,
the value can be cleaned up without any references becoming invalid.

We use the ``Rc<T>`` type when we want to allocate some data on the heap for multiple parts of our program to read
and we can’t determine at compile time which part will finish using the data last.

(If we knew which part would finish last, we could just make that part the data’s owner,
and the normal ownership rules enforced at compile time would take effect.)

NOTE: ``Rc<T>`` is only for use in single-threaded scenarios.
(for concurency, discuss later)
*/

// ==============================================================
// 2. Demo ``Rc<T>`` with data sharing
// ==============================================================
/*
This code will not compile, and panicks.
```
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    let c = Cons(4, Box::new(a));
}
```

Here are the errors:
```
9  |     let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
   |         - move occurs because `a` has type `List`, which does not implement the `Copy` trait
10 |     let b = Cons(3, Box::new(a));
   |                              - value moved here
11 |     let c = Cons(4, Box::new(a));
   |                              ^ value used here after move
```

That means when we calls ``let b = Cons(3, Box::new(a))``,
``a`` was moved into ``Box::new(a)`` and is no longer valid in main().
So we could not call ``let c = Cons(4, Box::new(a))`` anymore.

We could modify ``Cons`` to hold references ``Box<&List>`` instead,
but then we have to specify lifetime parameters. And by doing so,
we would be specifying that every element in the list will live at least as long as the entire list.
This sometime works, but not always.

We can also implement ``Clone`` trait for List,
then do something like ``Box::new(a.clone())``,
but it could be expensive because ``Clone`` usually creates a deep copy.

------------------------------------------------------------------------------------

Now, let's use ``Rc<T>`` to handle this problem in the most possible way.
*/

use std::rc::Rc; // for ``Rc<T>``, must import
use crate::List::{Cons, Nil};

enum List<T> {
    Cons(T, Rc<List<T>>), // Use ``Rc<T>`` instead of ``Box<T>``
    Nil
}

fn demo_Rc_sharing_data() {
    let a = Rc::new(Cons(2.5, Rc::new(Cons(10.3, Rc::new(Nil)))));

    let b = Cons(8.6, Rc::clone(&a));
    let c = Cons(11.9, Rc::clone(&a));
}
/*
Here, we write ``let a = Rc::new(Cons(2.5, Rc::new(Cons(10.3, Rc::new(Nil)))))``
(NOT ``let a = Cons(....)``) to wrap the cons list ``a`` in a ``Rc<T>``.
This allows Rust to to keep track of the number of references pointing to that specific list.

This later enables you to use ``Rc::clone(&a)`` so both ``b`` and ``c`` can safely share ownership
of the exact same data without moving the original value ``a``.

We can write ``let b = Cons(8.6, a.clone())``,
but Rust convention is ``let b = Cons(8.6, Rc::clone(&a))`` (using ``Rc::clone()``).
Why? Because the implementation of ``Rc::clone()`` doesn’t make a deep copy
of all the data like most types’ implementations of ``clone()`` do.

The call to ``Rc::clone()`` only increments the reference count, which doesn’t take much time like deep copy.
(Which also means increasing the number of pointers pointing to the original value ``a``. Yeah, it's shallow copy)
*/

// ===================================================================================
// 3. Counts the number of references/pointers pointing to an original value
// ===================================================================================
/*
``Rc::strong_count()`` helps us count the number of references pointing to an original ``Rc<T>``

(another one is ``Rc::weak_count()``, we will discuss it later)
*/

fn demo_count_number_of_references() {

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a)); // 1

    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a)); // 2

    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a)); // 3
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a)); // 2
}
/*
```
count after creating a = 1
count after creating b = 2
count after creating c = 3
count after c goes out of scope = 2
```
*/

// ============ //
//    main()    //
// ============ //

fn main() {
    println!();

    demo_Rc_sharing_data();
    demo_count_number_of_references();
}
