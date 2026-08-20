#![allow(non_snake_case)]

// =====================================================
// 0. Dereference operator
// =====================================================
/*
In Rust, the dereference operator is the asterisk symbol (*).
It is a unary prefix operator used to follow a reference or pointer
to access, read, or modify the actual data stored at that memory location.
*/

fn demo_dereference_operator() {

    let x = 5;
    let y = &x; // y is a reference/pointer to x (&i32)
    let z = &y; // z is a 2-layer reference/pointer to x (&&i32)

    // assert_eq!(5, y);
    // This fails because ``5`` is ``i32``, while ``y`` is ``&i32``

    // Using *y to get the underlying value
    // The * demands Rust to follow the reference/pointer to the value it is pointing (hence it is called "dereferebce")
    // So, ``*y`` will have type i32, same type as ``x``
    assert_eq!(5, *y); // compare i32 with i32
    assert_eq!(&5, y); // compare &i32 with &i32

    // Using ``**z`` to derefrence 2 times.
    // First, it traces back to y, then traces back one more time to reach x
    assert_eq!(5, **z); // compare i32 with i32
    assert_eq!(&5, *z); // compare &i32 with &i32
    assert_eq!(&&5, z); // compare &&i32 with &&i32

    println!("All assertations passed!");

    //////////////////////

    let mut score = 10;
    println!("score (before modified): {score}"); // 10

    let mut score_ref = &mut score;
    *score_ref = 20; // Modifies 'score' directly to 20
    println!("score (after modified 1st time): {}", score_ref); // 20

    let score_ref_2 = &mut score_ref;
    **score_ref_2 = 30; // Modifies 'score' directly to 30
    println!("score (after modified 2nd time): {}", score_ref); // 30
}

// =====================================================
// 1. Using Box<T> like a reference
// =====================================================

fn demo_BoxT_as_reference() {
    let x = 5;
    let y = Box::new(x);
    // Here, we set ``y`` to be an instance of a box pointing to ``a copied value of x``
    // rather than a reference (&x) pointing to the value of x

    assert_eq!(5, x);
    assert_eq!(5, *y); // We can also use "*" to dereference box ``y`` (follow the pointer to the true value)
                       // We can do so because ``Box<T>`` implements Deref Trait

    println!("All assertations passed!");
}

// ==================================================================================
// 2. Create custom wraper like Box<T> and implement Deref trait for it
// ==================================================================================
/*
The Box<T> type is ultimately defined as a tuple struct with one element.
So, we can use Struct to create ``MyBox<T>`` wrapper that is similar like Box as below.

We also implement ``new()`` method for it to mimic ``Box::new(T)``
*/

struct MyBox<T>(T); // tuple struct NewBox with generic type T

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

/*
As discussed earlier, to make a type enabled to use dereference operator ``*``,
we have to implement trait Deref for it.
*/

use std::ops::Deref; // import trait Deref to use

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// ============ //
//    main()    //
// ============ //

fn main() {
    println!();

    demo_dereference_operator();

    println!("\n==================================================================\n");

    demo_BoxT_as_reference();
}
