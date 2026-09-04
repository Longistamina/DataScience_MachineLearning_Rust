#![allow(dead_code)]
/*
Message Passing is not the only way to handle concurency,
another method is letting multiple threads access the same shared data,
which is called "Shared-State concurency"

In Message Passing, the channel is a single ownership,
meaning once you send the data down the channel,
you should never use that value because it's ownership is also transfered.

On the contrary, Shared-State concurency works like multiple ownership,
meaning that multiple threads can access the same memory location at the same time
(like what Smart Pointers do).

However, multiple ownership can add complexity because these different owners need managing.
Fortunately, Rust’s type system and ownership rules greatly assist in getting this management correct.
=> ``Mutex`` and ``Arc``
*/

use std::sync::{Mutex, Arc};


// =============================================================================
// 1. ``Mutex<T>`` for controlling access
// =============================================================================
/*
"Mutex" stands for "mutual exclusion",
it allows only one thread to access some data at any given time.

For a thread to access this data, it must first "tell" the mutex
that it wants to access the data, by asking for acquiring the mutex's "lock".

"lock" is a data structure as a part of the mutext,
it keeps track of who currently has exclusive access to the data.
=> Mutex is thus described as "guarding" the data via locking system.

There are two rules you have to remember when using mutex
(which makes it hard to use):
+ To access to the data, you must first acquire the lock.
+ After using the data, you must release the lock so that other threads can use it.

Imagine a panel discussion with only one microphone,
Before a panelist can speak, they have to ask or signal that they want to use the microphone.
When they get the microphone, they can talk for as long as they want to
and then hand the microphone to the next panelist who requests to speak.
If he/she forgets to hand the microphone off, no one else is able to speak.
-> If management of the shared microphone goes wrong, the panel won’t work as planned!

Fortunately, Rust type systems and ownership rules will ensure that
you will never get locking and unlocking wrong.
*/

fn demo_mutex() {
    let m = Mutex::new(3.14); // create a Mutex named ``m`` with value ``3.14`` inside it

    {
        let mut num = m.lock().unwrap();
        // Calls ``m.lock()`` to get the lock and access the data ``3.14`` inside the mutex ``m``
        // By doing so, it blocks the current thread so that it can’t do any work until it’s our turn to have the lock.
        // ``.lock()`` returns a ``LockResult<MutexGuard<T>, E>``.
        // ``LockResult<MutexGuard<T>, E>`` actually is ``MutexGuard<T>`` wrapped in a ``LockResult<T, E>``,
        // That's why we have to call ``unwrap()`` or similar functions to get the ``MutexGuard<T>``.
        //
        // Here, we use ``unwrap()``.
        // If another thread holding the lock panicks, ``.lock()`` here would fail and returns ``Err(error)``.
        // In that case, ``.unwrap()`` will panick.
        // Otherwise, it returns ``Ok(MutexGuard<T>)`` and then ``.unwrap()`` will returns that ``MutexGuard<T>``

        *num = 4.18
        // ``MutexGuard<T>`` implements ``Deref`` trait,
        // so we need ``*`` operator to access the true data ``3.14``,
        // then mutate it into ``4.18``

    } // Goes out of scope, ``MutexGuard<T>`` also implements ``Drop`` and thus will be dropped here.

    println!("m = {m:?}"); // m = Mutex { data: 4.18, poisoned: false, .. }
}
/*
We created mutex ``m`` as immutable, but later on we can still modify it
=> that is because ``Mutex<T>`` also implements interior mutability.
*/


// =============================================================================
// 2. Errors while sharing access to ``Mutex<T>``
// =============================================================================
/*
fn main() {
    let counter = Mutex::new(0);
    let mut handles = vec![];

    for _ in 0..10 {
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

---------------------------

This code will panic because the mutex ``counter``
has been moved into spawn threads ``handle``,
making it no longer valid in the main thread.
=> The final ```println!("Result: {}", *counter.lock().unwrap());``` is impossible
*/

//////////////////////////////////////////////////

/*
use std::rc::Rc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Rc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

----------------------------------------------------------

Even after wrapping ``Mutex<T>`` inside the smart pointer ``Rc<T>``
to leverage multiple pointers pointing to ``Mutex<T>``,
the code will still panick.

That is because ``Rc<Mutext<T>>`` cannot be sent between threads safely,
the reason is that ``Rc<T>`` does not implement ``Send``.

When Rc<T> manages the reference count,
it adds to the count for each call to ``clone``
and subtracts from the count when each ``clone`` is dropped.

But it doesn’t use any concurrency primitives
to make sure that changes to the count can’t be interrupted by another thread.
=> can lead to wrong count!!!
=> can then lead to memory leaks or value being dropped before we’re done with it.

So, we need something that is like ``Rc<T>``, but also makes changes to the reference count in a thread-safe way.
=> ``Arc<T>``
*/


// =============================================================================
// 3. ``Arc<T>``: allows accessing from multiple threads
// =============================================================================
/*
``Arc<T>`` stands for Atomic Reference Count,
it works like ``Rc<T>`` but is safe for use in concurent situations.
(because it has concurency primitive ``std::sync::atomic``)

At this point, you just need to know that atomic types work like primitive types
but are safe to share across threads.

Primitive types are common types that we already knew like: i32, f32, f64, ...
Atomic types are like primitive types but have more complexity to ensure thread safety.

Why Rust don't make all primitive types atomic by default?
Because thread safety comes with a performance penalty that you only want to pay when you really need to.
If you’re just performing operations on values within a single thread,
your code can run faster if it doesn’t have to enforce the guarantees atomics provide.

----------------------------------------------

``Arc<T>`` and ``Rc<T>`` have the same APIs.
*/

use std::thread;

fn demo_arc() {
    let counter = Arc::new(Mutex::new(0)); // use ``Arc<T>`` to allow multiple ownership of mutex and ensure thread safety
    let mut handles = vec![]; // an empty list to store all the spawn thread (handle)

    for _ in 0..10 {
        let counter = Arc::clone(&counter); // create another smart pointer pointing to the original, but has thread safety

        let handle = thread::spawn(move || { // spawn a thread, move the created SP ``counter`` to thread
            let mut num = counter.lock().unwrap(); // acquire lock and get the value
            *num += 1; // update the value
        });

        handles.push(handle); // add the spawn thread to ``handles`` list
    }

    for handle in handles {
        handle.join().unwrap(); // join all spawn threads
    }

    println!("Result: {}", *counter.lock().unwrap()); // 10
}
/*
What we did was wrapping ``Mutex<T>`` into an ``Arc<T>``
to allow multiple threads to access to the same ``Mutex<T>``
with thread safety ensured.

We spawned different threads, each thread tries to acquire lock
to access the mutex ``counter`` and mutate it (add 1 count).

We can use this structure for more complicated logics
(rather than just add 1)
*/


// ===========================================================================
// 4. Comparing ``RefCell<T>``-``Rc<T>`` and ``Mutex<T>``-``Arc<T>``
// ===========================================================================
/*
As we can see, ``Mutex<T>`` is like ``RefCell<T>``.
Both implement interior mutability.

``Arc<T>`` in turns is like ``Rc<T>``.
Both allow us to create multiple pointers pointing to the same value.

The only difference is that ``Mutex<T>``-``Arc<T>``
are designed for concurent situations.

Again, like ``RefCell<T>``-``Rc<T>``,
Rust cannot protect you from all kinds of logic errors
when you use ``Mutex<T>``-``Arc<T>``.
(In other words, Rust cannot protect you from all runtime errors
when the codes use these unsafe logics.)

Like ``RefCell<T>``-``Rc<T>`` can cause reference cycles,
``Mutex<T>``-``Arc<T>`` can cause "deadlocks".

An example of "deadlocks" is where:
+ Thread A holds Lock 1 and wants Lock 2
+ Thread B holds Lock 2 and wants Lock 1.
=> making them wait for each other forever
*/

// ============ //
//    main()    //
// ============ //

fn main() {
    println!();

    // demo_mutex();
    demo_arc();
}
