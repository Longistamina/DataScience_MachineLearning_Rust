// Demonstrating Rust's `format!()` macro.
// `format!()` creates and RETURNS a String. It does not print by itself.

fn main() {
    println!();

    let salary: i32 = 5_000;
    let salary_coefficient: f64 = 1.475;
    let full_name = "Tran Ngoc Dung";

    println!("{}", "-".repeat(50));
    println!();

    // =========================================================================================
    // 1. Basic formatting
    // =========================================================================================

    // `{}` uses the Display trait: the usual human-readable form.
    let info_1 = format!("Full name: {}\nSalary: {}", full_name, salary);
    println!("{info_1}");

    // Positional arguments may be referred to by their index.
    let info_2 = format!(
        "Full name: {0}\nSalary: {1}\nSalary coefficient: {2}",
        full_name, salary, salary_coefficient
    );
    println!("{info_2}");

    // Named placeholders can use local variables directly (Rust 1.58+).
    let info_3 = format!(
        "Full name: {full_name}\nSalary: {salary}\nSalary coefficient: {salary_coefficient}"
    );
    println!("{info_3}");

    println!("\n===============================================================================\n");

    // =========================================================================================
    // 2. Precision and scientific notation
    // =========================================================================================

    // `.2` means two digits after the decimal point for a floating-point value.
    let rounded = format!("Salary coefficient: {:.2}", salary_coefficient);
    println!("{rounded}"); // Salary coefficient: 1.48

    // `e` formats a number in scientific notation.
    let scientific = format!(
        "Salary: {:.2e} VND\nSalary coefficient: {:.2e}",
        salary as f64, salary_coefficient
    );
    println!("{scientific}");

    println!("\n===============================================================================\n");

    // =========================================================================================
    // 3. Width, alignment, and padding
    // =========================================================================================

    // `>`, `<`, and `^` mean right, left, and center alignment.
    let table_row = format!("| {:>12} | {:<12} | {:^8} |", "Name", full_name, salary);
    println!("{table_row}\n");

    // `0>8` pads a value on the left with zeroes until it is eight characters wide.
    let employee_id = 42;
    let padded_id = format!("Employee ID: {employee_id:0>8}");
    println!("{padded_id}\n"); // Employee ID: 00000042

    // Width and precision may come from variables too.
    let width = 10;
    let precision = 3;
    let dynamic = format!("|{salary_coefficient:>width$.precision$}|");
    println!("{dynamic}"); // |     1.475|

    println!("\n===============================================================================\n");

    // =========================================================================================
    // 4. Other useful format traits
    // =========================================================================================

    let number = 255;
    println!("Decimal: {number}, binary: {number:b}, hex: {number:x}, HEX: {number:X}");

    // `:?` uses Debug formatting, useful while developing.
    let scores = vec![95, 87, 100];
    let debug_text = format!("Scores: {scores:?}");
    println!("{debug_text}");

    // Rust's standard `format!()` has no built-in `:,` thousands separator.
    // For that, use a crate such as `num-format` when your project needs it.
}
