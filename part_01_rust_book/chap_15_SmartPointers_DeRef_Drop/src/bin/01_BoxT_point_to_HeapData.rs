/*
The most straightforward smart pointer is a box, whose type is written ``Box<T>``

Boxes allow you to store data on the heap rather than the stack.
What remains on the stack is the pointer to the heap data.

Because the only thing of ``Box<T>`` on the stack is pointer,
while its data is on the heap,
so it does not introduce any performance overhead by sorting Box's data on the stack,
just need to sort on the heap.
(On the stack, the only thing it need to sort is the pointer of the ``Box<T>``, which is small and has fixed size)

When to use ``Box<T>``? Here are 3 main cases:
+ When you have a type whose size can’t be known at compile time
+ When you have a large amount of data, and you want to transfer ownership without copying it
+ When you care about a trait rather than the exact concrete type
*/

#![allow(non_snake_case, dead_code)]

// ==================================================================
// 1. Box<T>: store data on the heap
// ==================================================================

fn demo_BoxT() {
    let b = Box::new(5); // define variable ``b`` as a ``Box`` that points to value 5 (on the heap)
    println!("b = {b}"); // ``b = 5`` (we can access value 5 like if it was on the stack)
                        // when ``b`` goes out of scope (main), the its memory will be deallocated and return
    /*
    Putting a single value like ``i32`` on the heap isn't very helpful
    (Box<T> is not truly designed for this).

    Let’s look at a case where boxes allow us to define types
    that we wouldn’t be allowed to define if we didn’t have boxes.
    */
}

// ==================================================================
// 2. Box<T>: allow Recursive Types
// ==================================================================
/*
A value of a ``recursive type`` can have another value of the same type as part of itself.
These guys pose an issue because Rust needs to know at compile time how much space a type takes up.

However, the nesting of values of recursive types could theoretically continue infinitely,
so Rust can’t know how much space the value needs.
=> Use ``Box<T>``

------------------------------------------

Let's use ``cons list`` to demonstrate.

``cons list`` is a data structure that comes from the Lisp programming language,
and is made up of nested pairs (the nested list version of Lisp).

For example: ``(1, (2, (3, Nil)))``

Each item in a cons list contains two elements: the value of the current item and of the next item.
So, for the given example cons list:
+ 1st pair contains: 1 and another list (2, (3, Nil))
+ 2nd pair contains: 2 and another list (3, Nil)
+ 3rd (last) pair has: 3 and Nil

The last item is ``Nil``, and it has now subsequent item after it.

Actually, ``cons list`` is not very common in Rust.
Most of the time, using ``Vec<T>`` is better in Rust when working with list.

Anyway, ``cons list`` is a recursive type that Rust cannot know its size at compile time,
so let's use Box<T> to enable the creation of a ``cons list``.
 */

#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>), // by using ``Box<List<T>>``, only the fixed-size pointers are on the stack -> still can compile
    Nil
}

// enum List<T> {
//     Cons(T, List<T>),
//     Nil
// }
// => This will panic because Rust cannot define the size for this infinite loop at compile time

use crate::List::{Cons, Nil};

fn demo_RecursiveType_ConsList() {
    // let cons_list = Cons(1, Cons(2, Cons(3, Nil))); This could not work either

    let cons_list = Cons(2.5, Box::new(Cons(3.2, Box::new(Cons(-9.8, Box::new(Nil))))));
    println!("cons_list = {:?}", cons_list) // cons_list = Cons(2.5, Cons(3.2, Cons(-9.8, Nil)))
}

// ============ //
//    main()    //
// ============ //

fn main() {
    println!();

    demo_BoxT();

    println!("\n==================================================================\n");

    demo_RecursiveType_ConsList();
}
