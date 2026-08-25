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
