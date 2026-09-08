#![allow(dead_code)]

/*
"Message Passing" is an increasingly popular approach to ensuring safe concurency,
where threads or actors communicate by sending each other messages containing data.

To achieve message-sending concurency, Rust std lib implements a feature named "channel".
"Channel" is a concept by which data is sent from one thread to another.

Can immagine a channel like a directional water stream, it flows from one end to the other.
Mean while, our message is like a boat on that water stream and is moved by the flow,
hence following the direction of the stream.

A channel has two halves:
+ a transmitter: the upstream part, where the information starts "flowing"
+ a receiver: the downstream part, the destination of the information

So, one part of the code will call methods on transmitter on the data you want to send,
the other part calls methods to check the receiver for arriving messages.

A channel is said to be "closed" if either the transmitter or receiver half is dropped.

-----------------------------------------------------------------------------------------

Now, we will write a program that has one transmitter thread
that generates values and sends them down the channel.

Another receiver thread will receive the value and print them out.
*/

use std::sync::mpsc; // mpsc = multiple-producer and single-consumer
use std::thread;
use std::time::Duration;

// ======================================================================================
// 1. Create and demo a channel with ``sync::mpsc::channel``
// ======================================================================================
/*
``mpsc`` stands for multiple-producer and single-consumer.

So, Rust implements this with the philosophy where
a channel can have multiple sending ends that produce values
but only one receiving end that consumes those values.

Imagine multiple streams flowing together into one big river:
Everything sent down any of the streams will end up in one river at the end.

Let's start with the case of single producer first.
*/

fn demo_channel() {
    let (tx, rx) = mpsc::channel();
    // ``mpsc::channel()`` returns a tuple,
    // the first element is the transmitter ``tx``,
    // the second one is the receiver ``rx``

    thread::spawn(move || { // move the transmitter ``tx`` into a spawn thread
        let val = String::from("hi"); // create a value to send
        tx.send(val).unwrap(); // use ``tx`` to send the value
        /*
        ``tx.send(val)`` returns a ``Result<T, E>``,
        so that if the receiver has already been dropped
        and there is nowhere to send the value,
        it will return ``Err(error)``
        */
    });

    let received = rx.recv().unwrap(); // call ``rx.recv()`` in the main thread to receive value from the spawn thread
    // let received = rx.try_recv().unwrap(); // use ``try_recv`` instead of ``recv``
    println!("Got: {received}");
    /*
    ``rx.recv()`` also returns a Result<T, E>.
    So when the transmitter closes, ``recv`` will return an error
    to signal that no more values will be coming.

    ``rx.recv()`` will block the main thread’s execution
    and wait until a value is sent down the channel.

    ``rx.try_recv()`` does not block the main thread's execution,
     but will instead return a Result<T, E> immediately:
     + an Ok value holding a message if one is available
     + and an Err value if there aren’t any messages this time.
     => Using ``try_recv`` is useful if this thread has other work to do while waiting for messages
    */
}

// ======================================================================================
// 2. Values' ownership is transfered
// ======================================================================================

fn demo_channel_ownership() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // println!("val is {val}");
        /*
        This line is impossible, because when we call ``tx.send(val)``,
        the ownership of ``val`` has also been sent down the channel to another thread,
        so ``val`` is no longer valid in this scope and thus we could not print it out here.
        */
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");
}

// ======================================================================================
// 3. Sending multiple values using ``vec![]`` and ``for`` loop
// ======================================================================================

fn demo_send_multiple_values() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![ // Create multiple values and store in a vector ``vals``
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap(); // send each value in ``vals``
            thread::sleep(Duration::from_secs_f32(0.5)); // sleep for 0.5s
        }
    });

    for received in rx { // we can treat ``rx`` as an iterator => no need to call ``recv()`` anymore
        println!("Got: {received}");
    }
}
/*
Got: hi
-- wait 0.5s --
Got: from
-- wait 0.5s --
Got: the
-- wait 0.5s --
Got: thread

------------------------

The producer sends "hi",
then the receiver prints "Got: hi",
then the producer pauses for 0.5s

After than, the producer send "from",
then the receiver prints "Got: from",
then the producer pauses for 0.5s again.

It keeps repeating like that,
until all values are consumed.
*/

// ======================================================================================
// 4. Create Multiple Producers
// ======================================================================================

fn demo_multiple_producers() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone(); // call clone on the transmitter ``tx`` to create a new transmitter

    // ----------------------------
    // Producer 1
    // ----------------------------

    thread::spawn(move || { // spawn the first thread
        let vals = vec![ // values of the first thread
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap(); // use ``tx1`` as transmitter for this thread
            thread::sleep(Duration::from_secs_f32(0.5));
        }
    });

    // ----------------------------
    // Producer 2
    // ----------------------------

    thread::spawn(move || { // spawn the second thread
        let vals = vec![ // values of the second thread
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap(); // use ``tx`` as transmitter for this thread
            thread::sleep(Duration::from_secs_f32(0.5));
        }
    });

    // ----------------------------
    // Receiver - Consumer
    // ----------------------------

    for received in rx { // ``rx`` will receive information from both threads
        println!("Got: {received}");
    }
}
/*
Got: hi
Got: more
-- wait 0.5s --
Got: from
Got: messages
-- wait 0.5s --
Got: the
Got: for
-- wait 0.5s --
Got: thread
Got: you

-------------------------------------

Since we have 2 producers, at each time,
the receiver will collect and use both inputs from producers,
and print them simultaneously.

After each sending, both producers pause for 0.5s,
then send and the receivers print them both again.

That's why we would see it prints out 2 output at a time before a pause.
=> CONCURENCY
*/

// ============ //
//    main()    //
// ============ //

fn main() {
    println!();

    // demo_channel();
    // demo_send_multiple_values();
    demo_multiple_producers();
}
