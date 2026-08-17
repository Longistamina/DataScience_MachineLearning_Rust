/*
 In Rust, beside pointers, we also have Smart Pointers.
 Before talking about Smart Pointers, let's review ordinary reference.

 ===========================================
 1. Normal reference: &T
 ===========================================

 Given codes like this
 ```
 let x = 5;
 let y = &x;

 println!("{}", y);
 ```

 Conceptually, y does not own x, it just ``borrows`` or ``refers to`` x
 ```
 x
 ┌─────┐
 │  5  │
 └─────┘
    ▲
    │
    y = &x
 ```

 Another example:
 ```
 let x = String::from("hello");
 let y = &x;
 ```

 Here:
 ```
 x owns String
      ▲
      │ borrow
      y
 ```

 ===========================================
 2. Then what is a smart pointer?
 ===========================================

 A smart pointer is basically "a value that behaves somewhat like a pointer/reference,
 but also provides extra behavior and metadata."

 Most of the time, we will work with these Smart Pointers provided by the Rust standard library (std)
 + Box<T>, for allocating values on the heap
 + Rc<T>, a reference counting type that enables multiple ownership
 + Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that enforces the borrowing rules at runtime instead of compile time

 One major distinction from ordinary references is "Smart pointers often own the thing they point to".
 For example:
 ```
 let b = Box::new(5);

 Stack                    Heap
 ┌────────────┐          ┌─────┐
 │ b: Box<i32>│ ───────> │  5  │
 └────────────┘          └─────┘
 ```
 So when ``b`` goes out of scope, Rust automatically destroys the heap value too.

 ===========================================
 3. Why not just use references?
 ===========================================

 Because references deliberately have very limited responsibilities.
 They just temporarily borrowing a specific value.

 Meanwhile, smart pointers can solve other ownership problems.
 For example:
 + Problem A: "I want this object stored on the heap." => use ``Box<T>``
 + Problem B: "I want several values to own the same data." => use ``Rc<T>``
 + Problem C: "I need to mutate something even though I only have an immutable outer reference." => use ``RefCell<T>``
 */

fn main() {
    println!("Overview of Smart Pointers")
}
