

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

/*
This is a test function for `main`.
 The function prints a test message to ensure the `main` function works as expected.
 cargo test nama_test_function -- --exact
 cargo test test_main -- --exact --nocapture
 */
 
#[test]
fn test_main() {
    // Print a message indicating the test is running
    println!("Audyari Wiyono");
}


//variable

#[test]
fn variable_test() {

    // Declare a variable `name` and assign it a string
    let name: &str = "Audyari Wiyono";
    // Print a greeting message using the `name` variable
    println!("Hello, {}!", name);
  
    // Declare a mutable variable `x` and assign it the value 5
    let mut x = 5;
    // Print a message indicating the value of `x`
    println!("The value of x is: {}", x);
    // Change the value of `x` to 6
    x = 6;
    // Print another message indicating the new value of `x`
    println!("The value of x is: {}", x);

    // Declare a mutable variable `datanama` and assign it an initial value
    let mut datanama = "Budi Wiyono";
    // Print the initial value of `datanama`
    println!("data nama: {}", datanama);
    // Change the value of `datanama`
    datanama = "Audyari Wiyono";
    // Print the new value of `datanama`
    println!("data nama updated: {}", datanama);

}

#[test]
fn variable_test_eror() {

    let x = 5;
    // Print the value of `x`
    println!("The value of x is: {}", x);
    // Attempting to assign a string to `x` will cause a type error
    //x = "testing";
    // Print the value of `x` again to demonstrate that it did not change
    println!("The value of x is: {}", x);
}

#[test]
fn shadow_test() {

    // Declare a variable `nama` and assign it a string
    let nama = "Audyari Wiyono";
    // Print the initial value of `nama`
    println!("data nama: {}", nama);
    // Shadow the `nama` variable with a new value
    let nama = "Budi Wiyono";
    // Print the updated value of `nama`
    println!("data nama updated: {}", nama);
}


#[test]
fn tipe_data() {

    // Declare a variable `a` and assign it an integer
    let a = 10;
    // Print the value of `a`
    println!("{}", a);

    // Declare a variable `b` and assign it a floating-point number
    let b = 10.5;
    // Print the value of `b`
    println!("{}", b);

    //number conversion

    // Declare a variable `a` and assign it an integer
    let a = 10;
    
    let b = a as f64;

    let c = b as i32;

    // Print the values of `a`, `b`, and `c`
    println!("a: {}, b: {}, c: {}", a, b, c);

}

#[test]
fn number_operator() {
    
    // Declare a variable `a` and assign it an integer value
    let a = 10;
    // Declare a variable `b` and assign it an integer value
    let b = 20;
    // Perform addition on `a` and `b`, and store the result in `c`
    let c = a + b;

    // Print the result of the addition
    println!("{} + {} = {}", a, b, c);
}

#[test]
fn augmented_assignment() {


    // Declare a mutable variable `a` and assign it an integer value
    let mut a = 10;
    // Print the initial value of `a`
    println!("a: {} ", a);

    // Use the augmented assignment operator `+=` to add 10 to `a`
    a += 10;
    // Print the updated value of `a`
    println!("a: {}", a);

    // Use the augmented assignment operator `-=` to subtract 10 from `a`
    a -= 10;
    // Print the updated value of `a`
    println!("a: {}", a);

    let a = 10; 


}








