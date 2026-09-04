#![allow(non_snake_case, unused_variables)]

// use std::ops::Drop; // Don't actually need to import ``Drop`` trait
// use std::mem::drop

// ===============================================================================
// 1. Drop Trait (std::ops::Drop): drop types automatically
// ===============================================================================
/*
The second trait important to the smart pointer pattern is ``Drop``.
``Drop`` trait lets you customize what happens when a value is about to go out of scope.

You can provide an implementation for the Drop trait on any type,
and that code can be used to release resources like files or network connections.

When a smart pointer like ``Box<T>`` is dropped,
it will deallocate the space on the heap that the box points to.
=> That's why implementing ``Drop`` trait for smart pointer is a MUST.

In some languages, for some types, the programmer must call code to free memory or resources
every time they finish using an instance of those types.
If the programmer forgets, the system might become overloaded and crash due to memory leak.

In Rust, you can specify that a particular bit of code be run whenever a value goes out of scope,
and the compiler will insert this code automatically.
As a result, you don’t need to be careful about placing cleanup code everywhere in a program
that an instance of a particular type is finished with—you still won’t leak resources!

-----------------------------------------------------------------------------------------------

When you implement ``Drop`` trait, it will require you to implement one method named ``drop()``
that takes a mutable reference to self (``&mut self``). Let's see the demo.

-----------------------------------------------------------------------------------------------

Unfortunately, it’s not straightforward to disable the automatic ``drop`` functionality,
and actually that is not usually necessary. Because the whole point of the Drop trait is that it’s taken care of automatically.
*/

struct CustomSmartPointer {
    data: String
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) { // ``drop()`` method that takes a mutable reference to self (``&mut self``)
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
        /*
        before self goes out of scope and is dropped,
        Rust will execute this ``drop()`` method and print out the message in ``println!()``
        */
    }
}

fn demo_Drop_trait() {
    let csp1 = CustomSmartPointer {data: String::from("Custom SP 1")};
    let csp2 = CustomSmartPointer {data: String::from("Custom SP 2")};
    println!("CustomSmartPointer instances created!")
}
/*
```
CustomSmartPointer instances created!
Dropping CustomSmartPointer with data `Custom SP 2`!
Dropping CustomSmartPointer with data `Custom SP 1`!
```

When we call ``demo_Drop_trait()`` in main(),
it will creates the two ``csp1`` and ``csp2`` as instances of ``CustomSmartPointer`` struct.
Then it will execute ``println!("CustomSmartPointer instances created!")``.

After that, it will go out of scope of ``demo_Drop_trait()`` function,
hence ``csp1`` and ``csp2`` are dropped.

But since we implemented the ``Drop`` trait for ``CustomSmartPointer``,
before dropping these 2 instances, it will execute ``drop()`` method for each one,
that's why we see Rust prints out:
"Dropping CustomSmartPointer with data `Custom SP 2`!
Dropping CustomSmartPointer with data `Custom SP 1`!"

(``csp2`` is dropped first, then ``csp1`` according to last-in-first-out (LIFO) rule)

-----------------------------------------------------------------------------------------------

Unfortunately, it’s not straightforward to disable the automatic ``drop`` functionality,
and actually that is not usually necessary. Because the whole point of the Drop trait is that it’s taken care of automatically.
*/

// =====================================================================
// 2. Drop types before they go out of scope (std::mem::drop)
// =====================================================================
/*
What if we want to drop a variable/type even before it goes out of scope?
Rust does not allow us to call ``type.drop()``, it will panick.

In that case, you have to use a function which is also named ``drop()``,
but from ``std::mem::drop``.
For example: ``drop(type)``
*/

fn demo_drop_function() {
    let csp3 = CustomSmartPointer {data: String::from("Custom SP 3")};
    let csp4 = CustomSmartPointer {data: String::from("Custom SP 4")};

    drop(csp3);    // drop ``csp3`` before it goes out of scope of this function, using ``std::mem::drop``
    // csp3.drop() // This is not allowed!!!
    println!("CustomSmartPointer instances created!")
}
/*
```
Dropping CustomSmartPointer with data `Custom SP 3`!
CustomSmartPointer instances created!
Dropping CustomSmartPointer with data `Custom SP 4`!
```

Here, when we call ``drop(csp3)`` (or ``std::mem::drop(csp3)``),
it will implement ``drop()`` method that we implemented for ``CustomSmartPointer``,
hence we see it prints out "Dropping CustomSmartPointer with data `Custom SP 3`!" first,
then it drops ``csp3`` even before it goes out of scope.

Then, it executes ``println!("CustomSmartPointer instances created!")``,
then goes out of scope and drops ``csp4``,
and implements ``drop()`` method of ``csp4`` before dopping it.
*/

// ============ //
//    main()    //
// ============ //

fn main() {
    println!();

    demo_Drop_trait();
    /*
    ```
    CustomSmartPointer instances created!
    Dropping CustomSmartPointer with data `Custom SP 2`!
    Dropping CustomSmartPointer with data `Custom SP 1`!
    ```
    */

    println!("\n==================================================================\n");

    demo_drop_function();
    /*
    ```
    Dropping CustomSmartPointer with data `Custom SP 3`!
    CustomSmartPointer instances created!
    Dropping CustomSmartPointer with data `Custom SP 4`!
    ```
    */
}
