
/// The main function is the entry point of the Rust program.
/// cargo run
fn main() {
    // Print a greeting message
    println!("Hello World! Audyari WIyono");

    // Print a test message
    println!("test Pencetaan");

    // Print a second test message
    println!("test Pencetaan ke dua");
}

/// This is a test function for `main`.
/// The function prints a test message to ensure the `main` function works as expected.
/// cargo test nama_test_function -- --exact
/// cargo test test_main -- --exact --nocapture

#[test]
fn test_main() {
    // Print a message indicating the test is running
    println!("Audyari Wiyono");
}

