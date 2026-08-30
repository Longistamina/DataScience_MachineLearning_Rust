#![allow(dead_code)]

/*
When we use a program on our PC or Laptop, it is run in a "process",
and the operating system can manage multiple processes at once.

Within a program, we can also have independent parts run simultaneously.
The features that run these independent parts are called "threads".

Doing so improves performance significantly, but also adds complexity.
Since threads can run simultaneously, there’s no inherent guarantee
about the order in which parts of your code on different threads will run.
=> lead to some problems:
+ Race conditions, in which threads are accessing data or resources in an inconsistent order
+ Deadlocks, in which two threads are waiting for each other, preventing both threads from continuing
+ Bugs that only happen in certain situations and are hard to reproduce and fix reliably

Rust devs have attempted to reduce the problems of concurent programming.
But a program written in multithreaded context still needs more careful thoughts
and requires a code structure that is different from that in programs running in a single thread.
*/

// ======================================================================================================
// 1. ``thread::spawn``: create a new thread
// ======================================================================================================

use std::thread;
use std::time::Duration;

fn demo_thread_spawn() {
    thread::spawn(|| { // spawn a thread and pass it a closure to tell it what we want to do
        for i in 1..10 {
            println!("Number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1)) // force a thread to stop its execution for a short duration, allowing a different thread to run.
        }
    });

    for i in 1..5 { // main thread
        println!("Number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1))
    }
}
/*
Number 1 from the main thread!
Number 1 from the spawned thread!
Number 2 from the main thread!
Number 2 from the spawned thread!
Number 3 from the main thread!
Number 3 from the spawned thread!
Number 4 from the main thread!
Number 4 from the spawned thread!

------------------------------------------

Notice, even though we told the spawned thread to print until i is 9,
it only got to 4 before the main thread shut down.

Because here in the main thread, we only print to 4,
so the main thread will complete its task first and finish the program,
making the spawned thread also stopped before it could print i upto 9.
*/


// ======================================================================================================
// 2. ``.join().unwrap()``: wait for all threads to finish
// ======================================================================================================
/*
To prevent the above situation, we can assign the spawned thread to a variable.
When we spawn a thread with ``thread::spawn``, it will return a ``JoinHandle<T>>`` instance.
So let store it into a variable named ``handle``.

Later on, we call ``handle.join().unwrap()`` to force the main thread
to wait until the spawn thread finishes its task.

``handle.join()`` returns a ``Result<T, E>`` type,
so we use ``.unwrap()`` right after to get the returned value T (or the error E).

In this case, since our closure just print out the number
and we don't specify anything for it to return,
so it will return unit tuple ``()`` by default (like main() function)
*/

// ----------
// ``.join().unwrap()`` after main thread
// ----------

fn demo_join_after_main() {
    let handle = thread::spawn(|| { // assign this ``JoinHandle<T>`` to variable ``handle``
        for i in 1..10 {
            println!("Number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1))
        }
    });

    for i in 1..5 { // main thread
        println!("Number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1))
    }

    handle.join().unwrap() // join() after main thread, force main thread to wait until spawned thread finishes
}
/*
Number 1 from the main thread!
Number 1 from the spawned thread!
Number 2 from the main thread!
Number 2 from the spawned thread!
Number 3 from the main thread!
Number 3 from the spawned thread!
Number 4 from the main thread!
Number 4 from the spawned thread!
Number 5 from the spawned thread!
Number 6 from the spawned thread!
Number 7 from the spawned thread!
Number 8 from the spawned thread!
Number 9 from the spawned thread!

------------------------------------------

Now you see that the spawned thread could print upto 9
even after the main thread finished its task.
*/

// ----------
// ``.join().unwrap()`` before main thread
// ----------

fn demo_join_before_main() {
    let handle = thread::spawn(|| { // assign this ``JoinHandle<T>`` to variable ``handle``
        for i in 1..10 {
            println!("Number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1))
        }
    });

    handle.join().unwrap(); // join() before main thread, main thread will wait until spawned fisnishes, then execute later

    for i in 1..5 { // main thread
        println!("Number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1))
    }
}
/*
Number 1 from the spawned thread!
Number 2 from the spawned thread!
Number 3 from the spawned thread!
Number 4 from the spawned thread!
Number 5 from the spawned thread!
Number 6 from the spawned thread!
Number 7 from the spawned thread!
Number 8 from the spawned thread!
Number 9 from the spawned thread!
Number 1 from the main thread!
Number 2 from the main thread!
Number 3 from the main thread!
Number 4 from the main thread!

-----------------------------------------

Since we call ``handle.join().unwrap()`` before main thread,
it will have to wait for the spawned thread to finish first,
then it runs its turn later.
*/

// ======================================================================================================
// 3. Use ``move`` to transfer ownership of a value from main thread to spawned thread
// ======================================================================================================
/*
To use data from the main thread in the spawned thread,
the spawned thread’s closure must capture the values it needs.
(meaning transfering the ownership of the data from main to spawned thread)

Why do we have to do so?

The Rust runtime cannot know when a spawned thread will finish.
Even if your main thread ends, the spawned thread could theoretically keep running detached in the background.

Imagine a value is defined in main thread,
then we borrow it in the spawned thread to use.
Then what will happen if the main thread finishes and drop that value?
Yes, it will result in a dangling reference (a use-after-free bug)!!!
(or can say that the reference value in spawned thread outlive the original value in main thread)
=> compiler will not compile!!!

In those cases, we have to transfer the data ownership from main thread to spawned thread
=> use ``move`` keyword

Talk a bit about backend:
```
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
where
    F: FnOnce() -> T + Send + 'static, // Requires 'static
```
=> As you can see, ``thread::spawn()`` applies
static lifetime constraint on its closure,
therefore spawned thread must own the data for the lifetime to be static
*/

fn demo_move() {
    let v = vec![1, 2, 3]; // define vector ``v`` in main thread

    let handle = thread::spawn(move || { // use ``move`` keyword here to transfer the ownership of ``v`` from main to spawned thread
        println!("Vector ``v`` (moved to spawned thread): {v:?}");
    });

    handle.join().unwrap();
}

// ============ //
//    main()    //
// ============ //

fn main() {
    println!();

    // demo_thread_spawn();
    // demo_join_after_main();
    // demo_join_before_main();
    demo_move();
}
