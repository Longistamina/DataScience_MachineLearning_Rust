/*
 * Closures can capture values from their environment in three ways (like functions):
 * + borrowing immutably
 * + borrowing mutably
 * + taking ownership
 */

 use std::thread;

 fn main() {
     println!();

     println!("================================================================================");
     // --------------------------------- Borrowing immutably --------------------------------- //

     let list = vec![1, 2, 3];
     println!("Before defining closure: {list:?}"); // 1st immutable reference, starts and ends here

     let only_borrows = || println!("From closure: {list:?}"); // 2nd immutable reference, starts here (not end here)

     println!("Before calling closure: {list:?}"); // 3rd immutable reference, starts while 2nd ref is still valid
     only_borrows(); // 2nd immutable reference ends here
     println!("After calling closure: {list:?}");

     /*
      * In this example, the only thing we do is printing out the ``list``.
      * To print something out, we just need its immutable reference.
      *
      * Since Rust allows multiple immutable references to the same object at a time,
      * we can call ``println!("Before calling closure: {list:?}")`` to create the 3rd immutable reference
      * while the 2nd immutable reference from ``only_borrows`` closure still persists.
      */

      println!("================================================================================");
      // ---------------------------------- Borrowing mutably ---------------------------------- //

      let mut list = vec![1, 2, 3];
      println!("Before defining closure: {list:?}");

      let mut borrows_mutably = || list.push(7); // This results in a mutable reference (because ``.push()`` takes mutable reference)

      // println!("Before calling closure: {list:?}"); // CANNOT DO IT HERE, because the mutable refence is still there, it's still valid
      borrows_mutably(); // mutable reference lifetime ends here
      println!("After calling closure: {list:?}"); // can invoke another reference (immutable) now

      /*
       * In this example, since ``.push()`` takes mutable reference from ``self`` (list),
       * it means this closure captures a mutable reference,
       * it also means running the closure actually modifies the closure's own internal state (it updates its captured borrow).
       * Therefore, the closure variable itself must be marked as mutable (so that Rust invoke the closure to implement ``FnMut``)
       * (using ``let mut ...``)
       *
       * Another important thing is that when we write ``let mut borrows_mutably = || list.push(7)``,
       * Rust will creates a mutable reference to ``list`` (because ``.push()`` takes mutable reference from ``self`` list).
       * If we then run ``println!("Before calling closure: {list:?}")``, this will invoke another immutable reference.
       * This is not allowed by Rust, because it prohibits creating multiple references to an object
       * if there is one mutable reference still valid there.
       *
       * The mutable reference must end its lifetime first (after calling ``borrows_mutable()``)
       * so that another reference can be invoked in ``println!("After calling closure: {list:?}")``
       */

       println!("=========================================================================================");
       // ---------------------------------- Takes ownership (``move``) ---------------------------------- //

       /*
        * If you want to force the closure to take ownership of the values it uses in the environment
        * even though the body of the closure doesn’t strictly need ownership,
        * you can use the ``move`` keyword before the parameter list.
        *
        * This technique is mostly useful when passing a closure to a new thread to move the data
        * so that it’s owned by the new thread.
        */

       let list = vec![1, 2, 3];
       println!("Before defining closure: {list:?}");

       thread::spawn(move || println!("From thread: {list:?}"))
           .join() // force the main thread to delay its executions to wait for this newly spawn thread finish
           .unwrap();

       /*
        * In this example, the closure ``move || println!("From thread: {list:?}")``
        * takes the ownership of ``list`` and pass it to the new thread created by ``thread::spawn()``
        *
        * Why should we use ``move`` here?
        *
        * When you start a new thread, it runs completely independently of your main() function.
        * Imagine this scenario without the move keyword:
        * 1) ``main()`` creates ``list = vec![1, 2, 3]``
        * 2) ``main()`` starts a new thread and gives it an immutable reference (a pointer) to ``list``.
        * 3) ``main()`` finishes its work quickly, reaches the end of the function, and DELETES (drops) ``list``.
        * 4) The background thread finally wakes up after main() thread finishes its jobs,
        *    and tries to read ``list`` through the reference.
        * Since ``list`` was already deleted by ``main()``, the background thread would be reading garbage memory.
        * This is called a dangling pointer, and Rust strictly forbids it.
        *
        * To prevent this memory disaster, Rust forces a rule on ``thread::spawn``:
        * The thread must take 100% full ownership of any variables it uses.
        *
        * => By adding move before the closure, you are telling ``main()`` to give up ownership of ``list``.
        *    ``list`` is physically moved out of ``main()`` and into the new thread.
        *    Now, even if ``main()`` finishes early, ``list`` remains safely alive inside the new thread.
        */
 }
