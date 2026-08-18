/*
The most straightforward smart pointer is a box, whose type is written ``Box<T>``

Boxes allow you to store data on the heap rather than the stack.
What remains on the stack is the pointer to the heap data.

Because the only thing of ``Box<T>`` on the stack is pointer,
while its data is on the heap,
so it does not introduce any performance overhead by sorting Box's data on the stack,
just need to sort on the heap.
(On the stack, the only thing it need to sort is the pointer of the ``Box<T>``, which is small and has fixed size)

When to use ``Box<T>``? Here are 3 main cases:
+ When you have a type whose size can’t be known at compile time
+ When you have a large amount of data, and you want to transfer ownership without copying it
+ When you care about a trait rather than the exact concrete type
*/
