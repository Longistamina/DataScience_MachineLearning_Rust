#![allow(dead_code, non_snake_case)]

/*
In the previous file ``06_MemoryLeak_ReferenceCycles.rs``,
we saw that reference cycles can lead to memory leak
and make the stack overflow.

To avoid this, one method is organizing your data structure more carefully.

Another solution is ``Weak<T>``

---------------------------------------------------------------

So far, we’ve demonstrated that calling ``Rc::clone``
increases the ``strong_count`` of an ``Rc<T>`` instance,
and an ``Rc<T>`` instance is only cleaned up if its ``strong_count`` is 0.

How about a weak reference? To create a weak reference, we use ``Rc::downgrade()``,
then pass a reference to ``Rc<T>`` to it.
=> we will get a smart pointer of type ``Weak<T>``

STRONG references are how you can share ownership of an ``Rc<T>`` instance.

WEAK references don’t express an ownership relationship,
and their count doesn’t affect when an ``Rc<T>`` instance is cleaned up.

So when we use ``Rc::downgrade()``,
it will increase the ``weak_count`` of ``Rc<T>`` instance by 1
(not the ``strong_count``)

DIFFERENCE: ``weak_count`` does not need to be 0 for the ``Rc<T>`` to be cleaned up.
*/

// =====================================================================================
// 1. Demo ``Weak<T>``: create a tree data structure
// =====================================================================================
/*
To best understand ``Weak<T>``, we will build a tree with nodes
that know about their child nodes.

Call that struct ``Node``.

The flow should be:
+ parent --Rc--> child
+ child --Weak--> parent

The parent owns the child, but the child does not own the parent.
That means the child can still know who its parent is without keeping that parent alive forever
(``child --Rc--> parent`` will keep this parent alive forever, making the stack overflow)
*/

use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>, // ``Rc<Node>`` to keep track how many pointers pointing to that child node
                                      // ``Vec<Rc<Node>>`` to share the ownership with variables so that we can access each Node in the tree directly.
                                      // ``RefCell<Vec<Rc<Node>>>`` to allow modifying which nodes are children of another node

    parent: RefCell<Weak<Node>> // makes the child aware of its parent without keeping the parent alive forever
                                // this increases ``weak_count``, leaves the ``strong_count`` the same
                                // so, the ``strong_count`` of parent can reach 0 -> no more cycle
}

// ---------------------- //

fn demo_WeakT() {
    let leaf = Rc::new(Node { // create a ``leaf`` with value=3, empty children, and empty WEAK parent
        value: 3,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new()),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); // None
    /*
    ``upgrade()`` explain:

    Because the value that ``Weak<T>`` references might have been dropped,
    to do anything with the value that a Weak<T> is pointing to you must make sure the value still exists.

    To do so, we can call ``upgrade()`` method on a ``Weak<T>`` instance.
    If the value is dropped or empty => ``None``
    If the value exists              => ``Some(...)``
    */

    let branch = Rc::new( Node { // create a ``branch`` with value=5, children is ``leaf``, and empty WEAK parent
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]), // ``branch`` wil own ``leaf``, increasing strong_count of ``leaf`` by 1
        parent: RefCell::new(Weak::new())
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    // ``leaf.parent`` is ``RefCell<Weak<T>>``
    // ``leaf.parent.borrow_mut()`` is ``RefMut<Weak<T>>``
    // ``*leaf.parent.borrow_mut()`` strips away the ``RefMut`` and returns the assignable ``Weak<T>``
    // ``Rc::downgrade(&branch)`` create a ``Weak<Node>`` reference to ``branch`` (increase weak_count by 1, not strong_count)
    // ``*leaf.parent.borrow_mut() = Rc::downgrade(&branch)`` mutates the ``Weak<T>`` into the ``Weak<&branch>``

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); // Some(...)
}

// =====================================================================================
// 2. Visualizing Changes to ``strong_count`` and ``weak_count``
// =====================================================================================
/*
Let’s look at how the ``strong_count`` and ``weak_count`` values of the ``Rc<Node>`` instances change
by creating a new inner scope and moving the creation of branch into that scope.

By doing so, we can see what happens when branch is created and then dropped when it goes out of scope.
*/

fn visualize_strong_weak_count() {
    let leaf = Rc::new(Node { // Create a ``leaf`` here
        value: 7,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new())
    });

    println!( // Print the strong_count and weak_count before going to scope
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf), // 1
        Rc::weak_count(&leaf) // 0
    );

    { // Go to inner scope
        let branch = Rc::new(Node { // Create ``branch``
            value: 12,
            children: RefCell::new(vec![Rc::clone(&leaf)]), // ``branch`` owns the ``leaf``, increase strong_count of ``leaf`` by 1
            parent: RefCell::new(Weak::new())
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch); // Create link ``branch`` to ``Weak<T>`` pointer of ``leaf``

        println!( // print the counts of ``branch``
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch), // 1
            Rc::weak_count(&branch), // 1
        );

        println!( // print the counts of ``leaf``
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf), // 2
            Rc::weak_count(&leaf), // 0
        );
    } // Go out of inner scope, the ``branch`` is now dropped

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); // should be ``None``
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf), // 1
        Rc::weak_count(&leaf), // 0
    );
}

// ============ //
//    main()    //
// ============ //

fn main() {
    println!();

    demo_WeakT();

    println!("\n====================================================================================\n");

    visualize_strong_weak_count();
}
