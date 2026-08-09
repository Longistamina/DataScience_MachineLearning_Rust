/*
 * Rust’s closures are anonymous functions you can save in a variable or pass as arguments to other functions.
 * You can create the closure in one place and then call the closure elsewhere to evaluate it in a different context.
 *
 * Unlike functions, closures can capture values from the scope in which they’re defined.
 * This script demonstrate how closures can capture the environment value without specifying types
 *
 * --------------------------------------------
 *
 * Here, we create a program to give away shirts.
 * If the user preference is given -> give away the shirt corresponding to his reference.
 * If the user preference is None -> give away the shirt whose count is the highest in the inventory
 */

#[derive(Debug, PartialEq, Copy, Clone)] // tells the compiler to implement these traits on ShirtColor
enum ShirtColor {
    Red,
    Blue
}

struct Inventory {
    shirts: Vec<ShirtColor>
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
        /*
         * ``|| self.most_stocked()`` is closure passed into ``.unwrap_or_else()``
         * This closure has no argument (there is nothing between vertical pipes ||)
         * This closure calls ``self.most_stocked()`` on the current Inventory instance.
         * Here, the standard library didn’t need to know anything about the Inventory or ShirtColor types we defined,
         * or the logic we want to use in this scenario.
         *
         * The closer automatically captured an immutable reference to the current ``self Inventory``
         * (the ``store = Inventory {shirts: vec![....]}`` variable below),
         * and passed it to the ``self.most_stocked()``
         *
         * (Functions created with ``fn`` cannot do this)
         */
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut red_count = 0;
        let mut blue_count = 0;

        for shirt in &self.shirts {
            match shirt {
                ShirtColor::Red => red_count += 1,
                ShirtColor::Blue => blue_count += 1,
            }
        }

        if red_count > blue_count {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }

}

// ################# //
//       main()      //
// ################# //

fn main() {
    println!();

    let store = Inventory { // define ``store`` whose Inventory has 2 blue shirts and 1 red shirt
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    let calc = |m: f64, n: f64| {m.powf(n)};
    println!("{}", calc(2.5, 4.6));
}

// The user with preference Some(Red) gets Red
// The user with preference None gets Blue
