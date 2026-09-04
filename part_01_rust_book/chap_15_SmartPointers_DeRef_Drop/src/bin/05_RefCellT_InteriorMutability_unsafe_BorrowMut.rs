#![allow(non_snake_case)]

// ==============================================================
// 1. Interior Mutability. What is it?
// ==============================================================
/*
``Interior mutability`` is a design pattern in Rust that allows you to mutate data
even when there are immutable references to that data.
(Normally, this action is disallowed by the borrowing rules.)

In these cases, to mutate that kind of data, the pattern uses ``unsafe`` code
inside a data structure to bend Rust’s usual rules of ownership and borrowing.

(When we use ``unsafe code``, we are telling the compiler that
we are checking the rules manually in stead of relying on it to do that for us.)

So, many types in Rust have ``interior mutability`` pattern
that allows us to run unsafe code. But we should only use those types
when can ensure that the borrowing rules will be followed at runtime.
(Even though the compiler can’t guarantee that)

But how to bypas the Rust compiler and make the code run?
We will need to wrap the ``unsafe code`` in a safe API,
that outer API is immutable -> pass the rules.
*/

use std::{cell::RefCell, panic};

fn demo_break_ownership_rules() {

    let result = panic::catch_unwind(||{

        let mut x = 5; // try ``let x = 5;`` to see compiler panicking
        let y = &mut x;

        *y
    });

    match result {
        Ok(value) => println!("value = {}", value),
        Err(error) => println!("\nCaught a panic, error = {:?}", error)
    }
}
/*
If you try changing ``let mut x = 5;`` => ``let x = 5;``,
the code will not compile. Because in that case, you make ``x`` immutable.
Then, you try to define ``y`` as a mutable reference to ``x``.
=> Not allowed by the rules.

But sometimes, we truly need to do that. Meaning there are situations in which
it would be useful for a value to mutate itself in its methods but appear immutable to other code.
(Code outside methods are not able to mutate the value).

In those cases, ``RefCell<T>`` is the answer.
*/

// ==============================================================================================
// 2. Demo ``RefCell<T>``, explain `Mock Objects`` and ``test double`` concepts
// ==============================================================================================
/*
While testing a code, programmers sometimes do not use the true type,
but they use another type in place of the true type,
in order to observe whether the code is implemented correctly or not.

The type that is used in place of the true type is called ``test double``.
(Think it like stunt doubles who do difficult actions for main actors in filmmaking)

Another concept is ``Mock Objects`` which are specific types of test doubles
that record what happens (to the observed value) during a test so that you can know if the action is right or not.

Rust does not have any ``Mock Objects`` built into standard libray like other programming languages,
but we can create a struct that will serve the same purposes as a mock object.

------------------------------------------------

In this example, we will create codes (library) that tracks a value against a maximum value
and sends messages based on how close to the maximum value the current value is.

This library could be used to keep track of a user’s quota
for the number of API calls they’re allowed to make, for example.

NOTE: this code will only provide the functionality of tracking how close to the maximum a value is
      and what the messages should be at what times.
      Other apps that use this code will be expected to provide the mechanism for sending the messages
      (print directly, send via email, sms, ...)
*/

pub trait Messenger {
    fn send(&self, msg: &str); // This ``send`` method is the mock object we talked about, it records what happens (to an observed value)
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize, // immutable
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T>{ // Create a new LimitTracker with given ``messenger`` and ``max`` from user
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!") // mock object: let us know what happens to the observed value
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!"); // mock object: let us know what happens to the observed value
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota!"); // mock object: let us know what happens to the observed value
        } else {
            let message = format!("Current usage: {}", percentage_of_max);
            self.messenger.send(&message);
        }
    }
    /*
    Why we need ``mock object`` as ``self.messenger.send()`` here?
    Because ``set_value()`` does not return anything,
    so we need that mock object to help us keep track of what happens to the observed value.
    */
}

// test module //
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger { // create a demo messenger struct (this is ``test double`` we talked above)
        // messages_to_send: Vec<String> // this will panick
        messages_to_send: RefCell<Vec<String>>
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            // MockMessenger { messages_to_send: vec![] } // this will panick
            MockMessenger { messages_to_send: RefCell::new(vec![]) } // initilize a new instance of MockMessenger with an empty RefCell list ``messages_to_send``
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            // self.messages_to_send.push(String::from(msg)); // this will panick
            self.messages_to_send.borrow_mut().push(String::from(msg)); // Allow unsafe modification of immutable here
                                                                        // ``borrow_mut()`` to borrow a mutable reference from ``RefCell<Vec<String>>`` to be able to modify it
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.messages_to_send.borrow().len(), 1); // ``borrow()`` to make an immutable reference to ``RefCell<Vec<String>>``
    }
}
/*
If we uncomment those ``this will panick`` codes,
Rust will not compile and panick.

That is because those codes create an immutable MockMessgener instance,
but then in the method ``send()``, we are trying to make modification with ``push()``
which in turn tries to make a mutable borrow ``&mut self``
=> Not allowed by ownership rules

--------------------------------------

That's why here we have to use ``RefCell<T>`` from ``std::cell::RefCell``
to wrap our MockMessger into a RefCell (as ``RefCell<Vec<String>>``) to enable
interior mutability for it.

Here, we call 2 methods of a ``RefCell<T>`` instance:
+ ``borrow_mut()`` to borrow a mutable reference from ``RefCell<Vec<String>>`` to be able to modify it
+ ``borrow()`` to make an immutable reference to ``RefCell<Vec<String>>``

By doing so, we do not need to change the trait of Messenger but can still test it out in test module.
*/

// ==============================================================================================
// 3. ``borrow()`` and ``borrow_mut()``
// ==============================================================================================
/*
Generally in Rust, we use ``&`` to create immutable reference,
and use ``&mut`` to create mutable reference.

In the above codes, we use ``borrow()`` and ``borrow_mut()``
from a ``RefCell<T>`` instance.

+ ``RefCell<T>.borrow()``: returns an immutable smart pointer of type ``Ref<T>``
+ ``RefCell<T>.borrow_mut()``: returns a mutable smart pointer of type ``RefMut<T>``

``RefCell<T>`` also keeps track of how many ``Ref<T>`` and ``RefMut<T>`` are currently active.
Everytime we call ``borrow()`` or ``borrow_mut()``, the ``RefCell<T>`` will increase its count
of how many immutable borrows are active. When these references out of scope, the count will decrease (by 1).

 Just like the compile-time borrowing rules, ``RefCell<T>`` lets us have many immutable borrows
 or one mutable borrow at any point in time. If we try to violate these rules,
 rather than getting a compiler error as we would with references,
 the implementation of ``RefCell<T>`` will PANIC AT RUNTIME.
*/

#[test]
fn refcell_violate_borrowing_rules() {
    let original = RefCell::new(String::from("I am original"));

    // Create two mutable references with ``borrow_mut()`` -> violate borrowing rules!!!!
    let mut brw_mut1 = original.borrow_mut();
    let mut brw_mut2 = original.borrow_mut();
    // let brw = original.borrow(); // even with 1 borrow() and 1 borrow_mut() is not possible

    brw_mut1.replace_range(.., "I am borrow mut 1");
    brw_mut2.replace_range(.., "I am borrow mut 2")
}
/*
RefCell already borrowed
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test refcell_violate_borrowing_rules ... FAILED

----------------------------------------------------

It even does not allow only one ``borrow_mut()`` and one ``borrow()`` at a time.
*/

// ==========================================================================================================
// 4. Combine ``RefCell<T>`` with ``Rc<T>``: allow multiple owners and interior mutability
//              Rc::new(RefCell::new(some_type))
// ==========================================================================================================
/*
To enable both multiple owners/references and interior mutability,
we can combine ``RefCell<T>`` with ``Rc<T>``.

Here, in this demo, we create 2 ``Rc<T>`` smart pointers to the same ``original``,
then use ``borrow()`` to get immutable borrows from these 2 smart pointers
=> increase count from 1 to 3

Before creating a mutable borrow ``borrow_mut`` from the ``original``,
we have to drop the 2 active immutable borrows first (not the pointers).
Then use the mutable borrow to modify the whole content.

After that, we drop the mutable borrow to release the lock of borrowing rules,
then create two immutable borrows again from the two pointers.
*/

use std::rc::Rc;

fn demo_Rc_RefCell_combine() {
    let original = Rc::new(RefCell::new("I am original".to_string()));

    println!("\n---------------- create 2 ptrs and 2 immut brws ----------------\n");

    let rc_ptr_1 = Rc::clone(&original); // create an ``Rc<T>`` smart pointer to `original` (increments reference count)
    let rc_ptr_2 = Rc::clone(&original); // create another one (increments reference count)

    let brw1 = rc_ptr_1.borrow(); // create immutable borrow from rc_ptr_1
    let brw2 = rc_ptr_2.borrow(); // create immutable borrow from rc_ptr_2

    println!("``original`` number of references: {}\n", Rc::strong_count(&original)); // 3
    println!("brw1 = {}", brw1);
    println!("brw2 = {}", brw2);
    println!("original = {:?}", original);

    println!("\n----------------- drop the 2 immutable borrows ---------------\n");

    drop(brw1); // release reference before create mutable borrow
    drop(brw2); // release reference before create mutable borrow

    println!("``original`` number of references: {}", Rc::strong_count(&original)); // 3 (still 3 because the smart pointers are still there)

    println!("\n---------------- create brw_mut and modify data -----------------\n");

    // Create a new scope for the mutable borrow
    {
        let mut brw_mut = original.borrow_mut(); // Obtain exclusive mutable access
        brw_mut.replace_range(.., "I have been modified");

        println!("``original`` number of references: {}\n", Rc::strong_count(&original)); // 2

        // We can print the mutable reference while we hold it
        println!("brw_mut = {}", brw_mut);

        println!("\nDroping brw_mut... (goes out of scope)")
    } // ``brw_mu``t goes out of scope here, no need ``drop()``, releasing the mutable lock!

    println!("\n--------------- create 2 immutable borrows again ------------------\n");

    let brw1 = rc_ptr_1.borrow(); // create immutable borrow again from rc_ptr_1
    let brw2 = rc_ptr_2.borrow(); // create immutable borrow again from rc_ptr_2

    println!("brw1 = {}", brw1); // now it is modified as "I have been modified"
    println!("brw2 = {}", brw2); // now it is modified as "I have been modified"
    println!("original = {:?}", original); // now it is modified as "I have been modified"

    println!("\n``original`` number of references: {}\n", Rc::strong_count(&original)); // 3
}

// ============ //
//    main()    //
// ============ //

fn main() {
    println!();

    demo_break_ownership_rules();

    println!("\n==================================================================\n");

    demo_Rc_RefCell_combine();
}
