/*
// ==============================================================================================
// 1. ``std::marker::Send`` helps transfer ownership between threads
// ==============================================================================================

Any type T that implements ``Send`` trait is safe to transfer ownerships of its values between threads.

Almost every Rust type implements ``Send``, but there are some exceptions like ``Rc<T>``.
Because if you cloned an ``Rc<T>`` value and tried to transfer ownership of the clone to another thread,
both threads might update the reference count at the same time.
=> ``Rc<T>`` is implemented for use in single-threaded situations
    where you don’t want to pay the thread-safe performance penalty.

// ================================================================================================
// 2. ``std::marker::Sync`` allows data from one thread to be referenced from multiple threads
// ================================================================================================

Any type T that implements ``Sync`` trait is safe to be referenced from multiple threads.
(it answers this question: "If several threads hold an immutable reference &T to the same value, is that safe?")

So if type T implements ``Sync``, you can send its reference ``&T`` to another thread.
``Mutext<T>`` is an example of this kind.

``Rc<T>`` does not implement ``Sync`` for the same reason with ``Send``.

``RefCell<T>`` and the family of ``Cell<T>`` do not implement ``Sync`` either,
because ``RefCell<T>`` allows mutation through an immutable reference.
Its runtime borrow-checking state is just ordinary, non-atomic bookkeeping (not safe).
So if two threads calling ``borrow_mut()`` could both observe “not borrowed yet” and both obtain mutable access.
=> data race

Remind: ``Mutex<T>`` prevents data race thanks to ``lock`` system.
*/

fn main() {

}
