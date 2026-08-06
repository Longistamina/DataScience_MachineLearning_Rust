/*
 * Closures don’t usually require you to annotate the types of the parameters or the return value like ``fn`` functions do.
 * Because closures are typically short and relevant only within a narrow context rather than in any arbitrary scenario.
 * Within these limited contexts, the compiler can infer the types of the parameters and the return type.
 * (Similar to how it's able to infer the types of most variables)
 *
 * We can also add type annotations for closures if we want to increase explicitness.
 */

#![allow(unused_variables, unused)]

use std::any::type_name_of_val;
use std::thread;
use std::time::Duration;
use std::error::Error;
use std::panic;

fn main() {
    println!();

    // A closure with fully annotated types
    let demo_closure = |num: u32| -> u32 {
         println!("calculating slowly...");
         thread::sleep(Duration::from_secs(1));
         num
     };

     // Call out the closure with its input
     demo_closure(39u32);

     println!("===============================================================================");

     fn  add_one_v1   (x: u32) -> u32 { x + 1 }
     let add_one_v2 = |x: u32| -> u32 { x + 1 };
     let add_one_v3 = |x|             { x + 1 };
     let add_one_v4 = |x|               x + 1  ;

     /*
      * The add_one_v3 and add_one_v4 lines require the closures to be evaluated to be able to compile
      * because the types will be inferred from their usage.
      *
      * This is similar to let v = Vec::new(); needing either type annotations or values of some type
      * to be inserted into the Vec for Rust to be able to infer the type.
      */

      add_one_v3(4u8); // ``x`` of ``add_one_v3`` now will have type u8
      add_one_v4(-2); // ``x`` of ``add_one_v4`` now will have type i32
      println!("Tested done!");

      println!("===============================================================================");

      let example_closure = |x| x;

      let s = example_closure(String::from("hello"));
      println!("s = {}", s);

      // let n = example_closure(5);
      /*
       * This code will fail
       * because the ``x`` in ``example_closure`` is compiled with String type
       * after the first call ``example_closure(String::from("hello"))``.
       *
       * So it could not accept ``5`` as input anymore
       */

       let n = example_closure(5.to_string()); // Must convert ``5`` from i32 to String first.
       println!("n = {}", n)
}
