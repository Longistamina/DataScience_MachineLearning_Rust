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

    println!("x = {x}"); // 5
    println!("*y = {}", *y); // 5
    println!("y = {}", y) // 5
}

// ==================================================================================
// 2. Create custom wraper like Box<T> and implement Deref trait for it
// ==================================================================================
/*
The Box<T> type is ultimately defined as a tuple struct with one element.
So, we can use Struct to create ``MyBox<T>`` wrapper that is similar like Box as below.

We also implement ``new()`` method for it to mimic ``Box::new(T)``
*/

#[derive(Debug)]
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
    type Target = T; // defines an associated type for the Deref trait to use

    fn deref(&self) -> &Self::Target {
        &self.0 // ``&self`` here is ``&MyBox<T>(T)``. since it's a single-element tuple, ``&self.0`` returns the value T
    }
    /*
    If you notice, the ``deref()`` method returns a reference of value T (``&Self::Target`` or ``&T``), not the value T itself.
    This is very reasonable, because if it returns the value T,
    then that value will be moved out of ``Self`` => dangerous
    */
}

/*
Now, after implementing Deref trait for ``MyBox<T>``,
we can now use dereference operator ``*`` for any instance of type ``MyBox<T>``
*/

fn demo_deref_MyBoxT() {
    let x = 32.5;
    let y = MyBox::new(x); // y = MyBox(x) = MyBox(32.5)

    assert_eq!(x, *y); // now when we run ``*y``, it will run ``y.deref()`` and return ``&y.0`` = &32.5

    println!("x = {x}"); // 32.5
    println!("*y = {}", *y); // 32.5 (actually &32)
    println!("y = {:?}", y) // MyBox(32.5)
}

// ==================================================================================
// 3. Deref Coercion
// ==================================================================================
/*
Deref coercion converts a reference to a type
that implements the Deref trait
into a reference to another type.

Another way of understanding this:
“You gave me a reference to one type, but I know how to follow Deref
to turn it into the reference type this function actually wants.”

For example, deref coercion can convert ``&String`` to ``&str``
because ``String`` implements the Deref trait such that it returns &str.
```
String
  │
  │ Deref
  ▼
str
```

So if a function expects ``&str``, but you give it ``&String``,
Rust automatically performs the conversion for you.

Without deref coercion, you would have to write more explicit ``*`` and ``&`` operations yourself.

The important thing is that this happens mainly
when Rust is trying to make an argument match a function or method parameter.
*/

fn demo_deref_corecion() {

    fn hello(name: &str) { // This function expects a ``&str`` as input
        println!("Hello, {name}!");
    }

    let m = MyBox::new(String::from("Rust"));

    hello(&m)
    // Although ``&m`` is ``MyBox<String>`` and is not ``&string``, function ``hello()`` can still process it
    /*
    That is because ``MyBox<T>`` implements trait Deref,
    hence Rust can turn ``&MyBox<String>`` into ``&String`` by calling deref.

    ``String`` has a specific implementation of Deref
    that allows it to return a string slice,
    so here Rust calls Deref again and turns ``&String`` into a ``&str``

    --------------------------------------------------------------------

    If Rust didn’t implement deref coercion, we would have to write the code like this
    ```
    hello(&(*m)[..]);
    ```

    The ``(*m)`` dereferences the ``MyBox<String>`` into a ``String``.
    Then, the ``&`` and ``[..]`` take a string slice of the String
    that is equal to the whole string to match the signature of hello.
    */
}

// ==================================================================================
// 4. Deref Coercion for Mutable Reference
// ==================================================================================
/*
Similar to how you use the ``Deref`` trait to override the ``*`` operator on immutable references,
you can use the ``DerefMut`` trait to override the ``*`` operator on mutable references.

Rust does deref coercion when it finds types and trait implementations in three cases:
1) From &T to &U when ``T: Deref<Target=U>``
2) From &mut T to &mut U when ``T: DerefMut<Target=U>``
3) From &mut T to &U when ``T: Deref<Target=U>``

The first two cases are the same except that the second implements mutability.

The third case is trickier: Rust will also coerce a mutable reference to an immutable one.
But the reverse is NOTE POSSIBLE: Immutable references will never coerce to mutable references.

Because it will break the rule of borrowing and ownership:
"There must only be one mutable reference to a type at a time"

Converting an immutable reference to a mutable reference would require that
the initial immutable reference is the only immutable reference to that data,
but who know if that data only has one immutable reference or not...!
So the borrowing rules don’t guarantee that (perhaps that data has more than.
*/

// ============ //
//    main()    //
// ============ //

fn main() {
    println!();

    demo_dereference_operator();

    println!("\n==================================================================\n");

    demo_BoxT_as_reference();

    println!("\n==================================================================\n");

    demo_deref_MyBoxT();

    println!("\n==================================================================\n");

    demo_deref_corecion();

}
