/// The main function is the entry point of the Rust program.
/// cargo run
fn main() {
    // Print a greeting message
    println!("Hello World! Audyari WIyono");

    // Print a test message
    println!("test Pencetaan");

    // Print a second test message
    println!("test Pencetaan ke dua");

    say_hello();

    function_a();

    function_b();

    println!("{}", MAXIMUM);
    unit();

    say_goodbye("BUDIONO", "SIREGAR");

    println!("factorial : {}", factorial_loop(5));

    print_text("Audyari Wiyono".to_string(), 3);

    // Declare a variable `number` and assign it the value 10
    let number = 10;
    // Call the `print_number` function to print the value of `number`
    print_number(number);

    // Declare a variable `name` and assign it a string value
    let name = String::from("Audyari Wiyono");
    // Call the `hi` function to print the value of `name`
    hi(name);

    // Declare variables for first name and last name
    let first_name = String::from("Audyari");
    let last_name = String::from("Wiyono");

    // Call the `full_name` function to combine the first and last name
    // and store the result in `full_name`
    let full_name = full_name(first_name, last_name);

    // Print the full name to the console
    println!("{}", full_name);

    let first_name = String::from("Audyari");
    let last_name = String::from("Wiyono");

    let (first_name, last_name, full_name) = full_name_dua(first_name, last_name);

    println!("{}", full_name);
    println!("{}", first_name);
    println!("{}", last_name);

    // Declare variables for first name and last name
    let first_name = String::from("Audyari");

    let last_name = String::from("Wiyono");

    // Call the `full_name_reference` function to combine the first and last name
    // and store the result in `full_name`
    let full_name = full_name_reference(&first_name, &last_name);

    // Print the full name to the console
    println!("{}", full_name);

    println!("{} {}", first_name, last_name);

    // Declare a variable `name` and assign it a string value

    let mut value = String::from("Audyari Wiyono");
    // Call the `change_value` function to change the value of `name`
    change_value(&mut value);
    // Print the value of `name` to verify that it has been changed
    println!("{}", value);

    // Declare variables for first name and last name
    let first_name = String::from("Audyari");
    let last_name = String::from("Wiyono");

    // Call the `solusi_dangling_pointer` function with references to first and last name
    // Store the result in `name`
    let name = solusi_dangling_pointer(&first_name, &last_name);

    // Print the full name to the console
    println!("{}", name);

    // The first name of the person.
    let first_name = String::from("Audyari");

    // The last name of the person.
    let last_name = String::from("Wiyono");

    // Create a new instance of the `Person` struct
    let person = Person {
        // The full name of the person.
        name: String::from("Audyari Wiyono"),
        // The first name of the person.
        first_name,
        // The last name of the person.
        last_name,
        // The age of the person.
        age: 20,
    };

    println!("Full Name: {}", person.name);
    println!("First Name: {}", person.first_name);

    // Print the details of the person using the `print_person` function
    println!("=======================================");

    print_person(&person);

    // Create a new instance of the `Person` struct by cloning the fields of the first person
    let person2 = Person {
        first_name: person.first_name.clone(),
        last_name: person.last_name.clone(),
        name: person.name.clone(),
        age: person.age,
        ..person
    };

    println!("======================================");

    print_person(&person2);

    println!("======================================");

    // Print the fields of the first person to show that they are still accessible
    println!("{}", person.first_name);
    println!("{}", person.age);

    // Create a new instance of `GeoPoint` with latitude and longitude
    let point = GeoPoint(-6.200000, 106.8166666);

    // Print the latitude of the point
    println!("{}", point.0);

    // Print the longitude of the point
    println!("{}", point.1);

       // Create a new instance of `Nothing`
       let _nothing = Nothing;

       // Create another instance of `Nothing` using the unit-like syntax
       let _nothing2 = Nothing{};
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
}

#[test]
fn boolean_test() {
    // Declare a boolean variable `a` and assign it the value true
    let a = true;
    // Declare a boolean variable `b` and assign it the value false
    let b = false;

    // Print the values of `a` and `b`
    println!("{} {}", a, b);
}

#[test]
fn comparison_test() {
    // Declare a variable `result` and assign it the result of the comparison
    // 10 is greater than or equal to 10
    let result = 10 >= 10;
    // Print the value of `result`
    println!("{}", result);
}

#[test]
fn boolean_operator() {
    // Declare a variable `absen` and assign it an integer value
    let absen = 80;

    // Declare a variable `nilai_akhir` and assign it an integer value
    let nilai_akhir = 80;

    // Declare a variable `lulus` and assign it a boolean value
    // indicating whether the student is absent or not
    let lulus = absen >= 75;

    // Declare a variable `lulus_nilai_akhir` and assign it a boolean value
    // indicating whether the student has a final score greater than or equal to 75
    let lulus_nilai_akhir = nilai_akhir >= 75;

    // Declare a variable `lulus_final` and assign it a boolean value
    // indicating whether the student is able to pass the course
    // based on the values of `lulus` and `lulus_nilai_akhir`
    let lulus_final = lulus && lulus_nilai_akhir;

    // Print the value of `lulus_final` to the console
    println!("{}", lulus_final);
}

#[test]
fn char_type() {
    // Declare a variable `char1` and assign it a string containing the character A
    let char1 = "A";

    // Declare a variable `char2` and assign it a char containing the character B
    let char2 = 'B';

    // Print the values of `char1` and `char2` to the console
    println!("{} {}", char1, char2);
}

#[test]
fn tuple_test() {
    // Declare a tuple `tuple1` containing two integers
    let tuple1 = (10, 20.20, true);

    // Print the values of `tuple1` to the console
    // cara satu
    println!("{:?}", tuple1);

    // Extract the values of the tuple and assign them to new variables
    let datatuple1 = tuple1.0;
    let datatuple2 = tuple1.1;
    let datatuple3 = tuple1.2;

    // Print the values of the variables to the console
    //cara dua
    println!("{} {} {}", datatuple1, datatuple2, datatuple3);

    // Destructure the tuple and assign the values to new variables
    let (a, b, c) = tuple1;

    // Print the values of the variables to the console
    //cara tiga
    println!("{} {} {}", a, b, c);
}

#[test]
fn tuple_mutable() {
    let mut tuple1 = (10, 20.20, true);

    println!("{:?}", tuple1);

    let (a, b, c) = tuple1;

    println!("{}, {}, {}", a, b, c);

    tuple1.0 = 100;
    tuple1.1 = 200.20;
    tuple1.2 = false;

    println!("{:?}", tuple1);
}

// Fungsi yang mengembalikan nilai unit.
fn unit() -> () {
    println!("Audyari Wiyono");
}

#[test]
fn unit_test() {
    // Memanggil fungsi unit tanpa mencoba menyimpan hasilnya.
    let result = unit();

    // Mencetak `result` dengan format yang sesuai.
    println!("{:?}", result);

    // Membuat variabel `test` dan menginisialisasinya dengan nilai unit.
    let test = ();

    // Mencetak `test` dengan format yang sesuai.
    println!("{:?}", test);
}

#[test]
fn array_test() {
    // Deklarasi array
    let array1 = [10, 20, 30, 40, 50];

    // Mencetak array
    println!("{:?}", array1);

    // Mengakses dan mencetak nilai array
    let a = array1[0];
    let b = array1[1];
    let c = array1[2];
    let d = array1[3];
    let e = array1[4];

    println!("{} {} {} {} {}", a, b, c, d, e);
}

#[test]
fn mutable_array() {
    // Deklarasi array yang mutable
    let mut array1 = [10, 20, 30, 40, 50];

    // Mencetak array
    println!("{:?}", array1);

    // Mengakses dan mencetak nilai array
    let a = array1[0];
    let b = array1[1];
    let c = array1[2];
    let d = array1[3];
    let e = array1[4];

    println!("{} {} {} {} {}", a, b, c, d, e);

    // Mengubah nilai array
    array1[0] = 100;
    array1[1] = 200;
    array1[2] = 300;
    array1[3] = 400;
    array1[4] = 500;

    // Mencetak array yang telah diubah
    println!("{:?}", array1);

    // Mendapatkan panjang array
    let length = array1.len();

    // Mencetak panjang array
    println!("Panjang array: {}", length);
}

#[test]
fn two_dimensional_array() {
    // Declare a 3x3 two-dimensional array `metrix`
    let metrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

    // Print the entire array `metrix`
    println!("{:?}", metrix);

    // Access and assign elements of the array to variables
    let a = metrix[0][0];
    let b = metrix[0][1];
    let c = metrix[0][2];
    let d = metrix[1][0];
    let e = metrix[1][1];
    let f = metrix[1][2];
    let g = metrix[2][0];
    let h = metrix[2][1];
    let i = metrix[2][2];

    // Print the values of the variables
    println!("{} {} {} {} {} {} {} {} {}", a, b, c, d, e, f, g, h, i);
}

/// A constant that represents the maximum value of an i32.
const MAXIMUM: i32 = 10;

/// A test function that demonstrates the use of constants in Rust.
/// It declares a constant `MINIMUM` and prints the values of two constants, `MAXIMUM` and `MINIMUM`.
#[test]
fn constant() {
    const MINIMUM: i32 = 10;

    // Print the values of the constants
    println!("{} {}", MAXIMUM, MINIMUM);
}

// Variable Scope
#[test]
fn variabel_scope() {
    // Declare a variable `audy` and assign it an integer value
    let audy = 1;

    {
        // Print the value of `audy` in the inner scope
        println!("inner scope : {}", audy);

        // Declare a variable `wiyono` and assign it an integer value
        let wiyono = 2;
        // Print the value of `wiyono` in the inner scope
        println!("inner scope : {} ", wiyono);
    }

    // Print the value of `audy` in the outer scope
    println!("outer scope : {}", audy);
}

/// A test function that demonstrates the use of stack and heap memory
/// in Rust. It calls two functions, `function_a` and `function_b`, and prints
/// the values of two variables, `a` and `b`, in each function.
#[test]
fn stack_heap() {
    // Call the first function
    function_a();

    // Call the second function
    function_b();
}

/// A function that demonstrates the use of stack memory
/// in Rust. It declares two variables, `a` and `b`, and prints their values.
fn function_a() {
    // Declare a variable `a` and assign it an integer value
    let a = 1;

    // Declare a variable `b` and assign it a string value
    let b = "Audyari ";

    // Print the values of `a` and `b`
    println!("{} {}", a, b);
}

/// A function that demonstrates the use of heap memory
/// in Rust. It declares two variables, `a` and `b`, and prints their values.
fn function_b() {
    // Declare a variable `a` and assign it an integer value
    let a = 2;

    // Declare a variable `b` and assign it a string value
    let b = "Wiyono";

    // Print the values of `a` and `b`
    println!("{} {}", a, b);
}

#[test]
fn string_test() {
    // Declare a variable `name` and assign it a string value
    let name = " Audyari Wiyono ";

    // Trim the whitespace from the string
    let edit_name = name.trim();

    // Print the trimmed string
    println!("{}", edit_name);

    // Declare a mutable variable `nama1` and assign it a string value
    let mut nama_contoh = String::from("Audyari Wiyono");

    println!("{}", nama_contoh);

    // Reassign the value of `nama1` to a new string
    nama_contoh = String::from("Budi Wiyono");

    // Print the new value of `nama1`
    println!("{}", nama_contoh);
}

#[test]
fn string_test_2() {
    // This function demonstrates the use of stack and heap data types in Rust.
    //
    // It creates a mutable `String` on the heap, modifies it, and prints the result.
    // Then, it replaces a part of the string with another value and prints the updated string.

    // Create a mutable string on the heap
    let mut name_string = String::from("Audyari Wiyono");

    // Append a string slice to the existing string
    name_string.push_str(" Wiyono");

    // Print the modified string
    println!("{}", name_string);
    println!("");

    // Replace part of the string and store the result in a new variable
    let budi = name_string.replace("Audyari", "Budi");

    // Print the updated string
    println!("{}", budi);
}

#[test]
fn ownership_rules() {
    // The variable `a` is declared and assigned the value 10.
    // It is valid in the outer scope.
    let a = 10;

    {
        // The variable `b` is declared and assigned the value 20.
        // It is valid within the inner scope.
        let b = 20;

        // Print the values of `a` and `b` within the inner scope.
        println!("a is {}", a);
        println!("b is {}", b);
    }

    // Print the value of `a` again in the outer scope.
    // The variable `b` is no longer valid here.
    println!("a is {}", a);
}

#[test]
fn data_copy() {
    // Declare a variable `a` and assign it the value 10.
    let a = 10;

    // Declare a variable `b` and assign it the value of `a`.
    // This creates a copy of the value of `a`.
    let b = a;

    // Print the values of `a` and `b`.
    // The output will be "10 10".
    println!("{} {}", a, b);
}

#[test]
fn ownership_movement() {
    // Create a new String on the heap and assign it to `name1`
    let name1 = String::from("Audyari Wiyono");

    // Clone the value of `name1` and assign it to `name2`
    let name2 = name1.clone();

    // Print the values of `name1` and `name2`
    // Both strings are accessible and remain unchanged
    println!("{} {}", name2, name1);
}

/// Tests for if expressions.
#[test]
fn if_expression() {
    let a = 10;

    if a > 5 {
        // If a is greater than 5, print a message
        println!("a lebih besar dari 5");
    } else if a < 5 {
        // If a is less than 5, print a message
        println!("a kurang dari atau sama dengan 5");
    } else if a == 5 {
        // If a is equal to 5, print a message
        println!("a sama dengan 5");
    }
}

#[test]

fn let_statement() {
    // Declare a variable `value` and assign it an integer value
    let value = 10;

    // Declare a variable `result` and assign it a string value
    // based on a condition
    let result;

    // Check if `value` is greater than 5
    if value > 5 {
        // If true, assign the value of `result` to a string
        // indicating that `value` is greater than 5
        result = "value lebih besar dari 5";
    } else {
        // If false, assign the value of `result` to a string
        // indicating that `value` is less than or equal to 5
        result = "value kurang dari atau sama dengan 5";
    }

    // Print the value of `result` to the console
    println!("{}", result);
}

#[test]
fn if_di_let_statement() {
    // Declare a variable `a` and assign it an integer value
    let a = 10;

    // Declare a variable `result` and assign it a string value
    // based on a condition using an if expression
    let result = if a > 5 {
        // If `a` is greater than 5, assign the value of `result`
        // to a string indicating that `a` is greater than 5
        "a lebih besar dari 5"
    } else {
        // If `a` is less than or equal to 5, assign the value of `result`
        // to a string indicating that `a` is less than or equal to 5
        "a kurang dari atau sama dengan 5"
    };

    // Print the value of `result` to the console
    println!("{}", result);
}

#[test]
fn loop_expression() {
    let mut counter = 0;

    // Use a loop to print the values of counter from 1 to 10
    loop {
        // Increment the value of counter
        counter += 1;

        // If counter is greater than 10, exit the loop
        if counter > 10 {
            break;
        } else if counter % 2 == 0 {
            // If counter is even, skip to the next iteration
            continue;
        }

        // Print the value of counter
        println!("{}", counter);
    }
}

#[test]
fn loop_return_value() {
    let mut counter = 0;

    let result = loop {
        // Increment the value of counter
        counter += 1;
        println!("counter: {}", counter);

        // Exit the loop if counter is greater than 10
        if counter > 10 {
            // Return the value of counter
            break counter * 2; // Return the value of counter multiplied by 2
        }
    };

    // Print the result to the console
    println!("result: {}", result);
}

#[test]
fn loop_label() {
    let mut number = 1;

    // Use a labeled loop to break out of the outer loop
    'outer: loop {
        let mut i = 1;

        // Use a nested loop to print the multiplication table
        loop {
            if number > 5 {
                break 'outer;
            }

            println!("{} x {} = {}", number, i, number * i);

            i += 1;

            // Exit the inner loop if i is greater than 5
            if i > 5 {
                break;
            }
        }

        // Increment the value of number
        number += 1;
    }
}

#[test]
fn while_loop() {
    let mut counter = 0;

    // Use a while loop to print the even numbers from 0 to 10
    while counter < 10 {
        // Check if the current number is even
        if counter % 2 == 0 {
            println!("counter: {}", counter);
        }
        // Increment the counter
        counter += 1;
    }
}

#[test]
fn iterasi_array() {
    // Create an array of strings
    let arr_contoh = ["A", "B", "C", "D", "E"];

    // Initialize a variable to keep track of the index
    let mut index_arry = 0;

    // Use a while loop to iterate through the array
    while index_arry < arr_contoh.len() {
        // Print the value of the current element
        println!(" Value: {}", arr_contoh[index_arry]);

        // Increment the index
        index_arry += 1;
    }
}

#[test]
fn for_loop() {
    // Create an array of strings
    let arr_contoh = ["A", "B", "C", "D", "E"];

    // Use a for loop to iterate through the array
    for element in arr_contoh {
        // Print the value of the current element
        println!(" Value: {}", element);
    }
}

#[test]

fn range() {
    let array = ["A", "B", "C", "D", "E"];

    // Create a range from 0 to 5 (exclusive)
    let range = 0..5;

    // Print the start and end values of the range
    println!("Start : {}", range.start);
    println!("End : {}", range.end);

    // Iterate through the range and print the values
    for i in range {
        // Print the value of the current element
        println!("Value: {}", array[i]);
    }
}

#[test]
fn range_inclusive() {
    // Create an array of strings
    let array = ["A", "B", "C", "D", "E"];

    // Create a range that is inclusive of the end value
    let range = 0..=4;

    // Print the start and end values of the range
    println!("Start : {}", range.start());
    println!("End : {}", range.end());

    // Iterate through the range and print the values
    for i in range {
        println!("Value: {}", array[i]);
    }
}

/// Prints a greeting message
fn say_hello() {
    println!("Hello, Audyari Wiyono!");
}

/// A test function that calls the `say_hello` function three times
#[test]
fn test_say_hello() {
    // Call the `say_hello` function three times
    say_hello();
    say_hello();
    say_hello();
}

/// Prints a goodbye message
///
/// This function takes two string arguments: `first_name` and `last_name`.
/// It prints a message to the console that says "Goodbye, [first_name] [last_name]!"
fn say_goodbye(first_name: &str, last_name: &str) {
    println!("Goodbye, {} {}!", first_name, last_name);
}

#[test]
fn test_say_goodbye() {
    say_goodbye("Audyari", "Wiyono");
    say_goodbye("Audyari", "Wiyono");
    say_goodbye("Audyari", "Wiyono");
}

/// Calculates the factorial of a number
///
/// This function takes a single u64 argument: `n`.
/// It calculates the factorial of `n` using a loop and returns the result.
fn factorial_loop(n: u64) -> u64 {
    if n < 1 {
        return 0;
    }

    let mut result = 1;

    for i in 1..=n {
        result *= i;
    }

    return result;
}

#[test]
fn test_factorial_loop() {
    // Display the factorial of 5
    println!("Factorial of 5: {}", factorial_loop(5));

    // Display the factorial of 6
    println!("Factorial of 6: {}", factorial_loop(6));

    // Display the factorial of 7
    println!("Factorial of 7: {}", factorial_loop(7));

    // Calculate the factorial of 10 and store it in `hasil`
    let hasil = factorial_loop(10);

    // Display the factorial of 10
    println!("Factorial of 10: {}", hasil);
}

fn print_text(value: String, times: u32) {
    // Prints the given string a specified number of times
    //
    // This function takes two arguments: `value` and `times`.
    // `value` is the string to be printed, and `times` is the number of times to print it.
    // The function prints the string `times` times, and then returns.
    if times == 0 {
        return;
    } else {
        println!("{}", value);
    }

    // Recursively call the `print_text` function to print the string `times - 1` more times
    print_text(value, times - 1);
}

#[test]
fn test_print_text() {
    // This test function calls the `print_text` function to print the given string
    // a specified number of times.
    //
    // It tests the `print_text` function by printing "Audyari Wiyono" three times.
    print_text("Audyari Wiyono".to_string(), 3);
}

// Prints a given number to the console
fn print_number(value: u32) {
    // Print the given number to the console
    println!("{}", value);
}

/// Prints a greeting message to the console
fn hi(name: String) {
    // Prints a greeting message using the given name
    println!("Hello, {}!", name);
}

#[test]
fn test_hi() {
    // Declare a variable `number` and assign it the value 10
    let number = 10;
    // Call the `print_number` function to print the value of `number`
    print_number(number);

    // Declare a variable `name` and assign it a string value
    let name = String::from("Audyari Wiyono");
    // Call the `hi` function to print the value of `name`
    hi(name);

    // The `name` variable is moved into the `hi` function, so it cannot be used again
    // Uncomment the following line to see an error
    // println!("{}",name);
}

// Returns a full name by combining the first and last names
//
// This function takes two arguments, `first_name` and `last_name`, and returns a new string
// that combines them. The function moves the values of `first_name` and `last_name` into its
// return value, so the caller should not attempt to use `first_name` or `last_name` after calling
// `full_name`.
fn full_name(first_name: String, last_name: String) -> String {
    // Combine the first and last names into a single string
    let full_name = format!("{} {}", first_name, last_name);

    // Return the full name
    return full_name;
}

#[test]
fn test_full_name() {
    // Declare variables for first name and last name
    let first_name = String::from("Audyari");
    let last_name = String::from("Wiyono");

    // Call the `full_name` function to combine the first and last name
    // and store the result in `full_name`
    let full_name = full_name(first_name, last_name);

    // Print the full name to the console
    println!("{}", full_name);

    // The `first_name` and `last_name` variables are moved into the `full_name` function,
    // so they cannot be used again.
    // Uncomment the following line to see an error
    // println!("{} {}", first_name, last_name);
}

// A function that returns a full name by combining the first and last names
//
// This function takes two arguments, `first_name` and `last_name`, and returns a tuple
// containing the first name, last name, and the full name.
//
// The function moves the values of `first_name` and `last_name` into the tuple that is
// returned, so the caller should not attempt to use `first_name` or `last_name` after
// calling `full_name_dua`.
fn full_name_dua(first_name: String, last_name: String) -> (String, String, String) {
    // Combine the first and last names into a single string
    let full_name = format!("{} {}", first_name, last_name);

    // Return the full name in a tuple
    return (first_name, last_name, full_name);
}

/// Tests the `full_name_dua` function by calling it with sample data
/// and verifying that the returned tuple contains the correct values.
#[test]
fn test_full_name_dua() {
    // Declare variables for first name and last name
    let first_name = String::from("Audyari");
    let last_name = String::from("Wiyono");

    // Call the `full_name_dua` function with the sample data
    let (first_name, last_name, full_name) = full_name_dua(first_name, last_name);

    // Verify that the returned tuple contains the correct values
    println!("Full name: {}", full_name);
    println!("First name: {}", first_name);
    println!("Last name: {}", last_name);
}

fn full_name_reference(first_name: &String, last_name: &String) -> String {
    // Combine the first and last names into a single string

    let full_name = format!("{} {}", first_name, last_name);

    // Return the full name
    return full_name;
}

#[test]
fn test_full_name_reference() {
    // Declare variables for first name and last name
    let first_name = String::from("Audyari");

    let last_name = String::from("Wiyono");

    // Call the `full_name_reference` function to combine the first and last name
    // and store the result in `full_name`
    let full_name = full_name_reference(&first_name, &last_name);

    // Print the full name to the console
    println!("{}", full_name);

    // `first_name` and `last_name` are not moved into the `full_name_reference` function,
    // so we can use them again after calling the function
    println!("{} {}", first_name, last_name);
}

fn change_value(value: &mut String) {
    // Change the value of `name` to "Budi Wiyono"
    //
    // This function takes a mutable reference to a `String` and changes its value.
    // The function does not return a value, but the value of `name` is changed
    // after the function is called.
    value.push_str(" Budi Wiyono");
}

#[test]
fn test_change_value() {
    // Declare a variable `name` and assign it a string value

    let mut value = String::from("Audyari Wiyono");
    // Call the `change_value` function to change the value of `name`
    change_value(&mut value);
    // Print the value of `name` to verify that it has been changed
    println!("{}", value);
}

/// A function that returns a full name by combining the first and last names.
///
/// This function takes two arguments, `first_name` and `last_name`, which are
/// references to `String`s. The function returns a new `String` that combines the
/// two names. The function does not take ownership of `first_name` or `last_name`,
/// so the caller can continue to use them after calling the function.
fn solusi_dangling_pointer(first_name: &String, last_name: &String) -> String {
    let name = format!("{} {}", first_name, last_name);
    return name;
}

#[test]
fn test_solusi_dangling_pointer() {
    // Declare variables for first name and last name
    let first_name = String::from("Audyari");
    let last_name = String::from("Wiyono");

    // Call the `solusi_dangling_pointer` function with references to first and last name
    // Store the result in `name`
    let name = solusi_dangling_pointer(&first_name, &last_name);

    // Print the full name to the console
    println!("{}", name);
}

#[test]
fn slice_reference() {
    // Create an array of integers
    let array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Create a slice that references the entire array
    let slice = &array[..];

    // Print the slice to the console
    println!("{:?}", slice);

    // Create a slice that references the first 5 elements of the array
    let slice2 = &array[0..5];

    // Print the slice to the console
    println!("{:?}", slice2);

    // Create a slice that references the last 5 elements of the array
    let slice3 = &array[5..];

    // Print the slice to the console
    println!("{:?}", slice3);
}

#[test]
fn string_slice() {
    // This function demonstrates how to create string slices from a `String`
    // and print the slices to the console.

    let name = String::from("Audyari Wiyono");

    // Create a slice for the first name using the first three characters
    let first_name = &name[0..3];

    // Print the first name slice
    println!("{}", first_name);

    // Create a slice for the last name using the characters from index 4 onward
    let last_name = &name[4..];

    // Print the last name slice
    println!("{}", last_name);
}

struct Person {
    /// The full name of the person.
    name: String,
    /// The first name of the person.
    first_name: String,
    /// The last name of the person.
    last_name: String,
    /// The age of the person.
    age: u32,
}

/// A function that prints the details of a person.
///

fn print_person(person: &Person) {
    println!("Full Name: {}", person.name);
    println!("First Name: {}", person.first_name);
    println!("Last Name: {}", person.last_name);
    println!("Age: {}", person.age);
}

/// A test function that creates a new instance of the `Person` struct and prints its details.
#[test]
fn struct_person() {
    // The first name of the person.
    let first_name = String::from("Audyari");

    // The last name of the person.
    let last_name = String::from("Wiyono");

    // Create a new instance of the `Person` struct
    let person = Person {
        // The full name of the person.
        name: String::from("Audyari Wiyono"),
        // The first name of the person.
        first_name,
        // The last name of the person.
        last_name,
        // The age of the person.
        age: 20,
    };

    println!("Full Name: {}", person.name);
    println!("First Name: {}", person.first_name);

    // Print the details of the person using the `print_person` function
    println!("=======================================");

    print_person(&person);

    // Create a new instance of the `Person` struct by cloning the fields of the first person
    let person2 = Person {
        first_name: person.first_name.clone(),
        last_name: person.last_name.clone(),
        name: person.name.clone(),
        age: person.age,
        ..person
    };

    println!("======================================");

    print_person(&person2);

    println!("======================================");

    // Print the fields of the first person to show that they are still accessible
    println!("{}", person.first_name);
    println!("{}", person.age);
}

/// A struct representing a geographical point with latitude and longitude.
struct GeoPoint(f64, f64);

/// A test function for the `GeoPoint` struct.
#[test]
fn struct_geo_point() {
    // Create a new instance of `GeoPoint` with latitude and longitude
    let point = GeoPoint(-6.200000, 106.8166666);

    // Print the latitude of the point
    println!("{}", point.0);

    // Print the longitude of the point
    println!("{}", point.1);
}

/// A struct with no fields, known as a "unit-like" struct.
///
/// This struct can be used to represent the absence of a value.
struct Nothing;

/// A test function for the `Nothing` struct.
#[test]
fn struct_nothing() {
    // Create a new instance of `Nothing`
    let _nothing = Nothing;

    // Create another instance of `Nothing` using the unit-like syntax
    let _nothing2 = Nothing{};
}
