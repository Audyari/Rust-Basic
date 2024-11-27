fn main() {
<<<<<<< HEAD
    println!("Hello word");
=======
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
>>>>>>> 16bc7b17962bd6952927b568ce25bd2eae304ce0
}

struct Person<'a> {
    nama_depan: &'a str,
    nama_belakang: &'a str,
}

impl<'a> Person<'a> {
    fn fungsi_satu(&self, name: &str) {
        println!("{} kamu {}", name, self.nama_depan);
    }
    fn fungsi_dua(&self, nama: &str) {
        println!("{} {}", nama, self.nama_belakang);
    }
    fn test_lain(&self) {
        println!("{} {}", self.nama_depan, self.nama_belakang);
    }
}

#[test]
fn test_panggil() {
    let nama_depan = "Audy";
    let nama_belakang = "Wiyo";

    let data = Person {
        nama_depan,
        nama_belakang,
    };

    data.fungsi_satu("Testing");
    data.fungsi_dua("Testing");
    data.test_lain();
}

struct DataKordinat(f32, f32);

impl DataKordinat {
    fn new(long: f32, lat: f32) -> DataKordinat {
        DataKordinat(long, lat)
    }
    fn test(data: String) {
        println!("{}", data)
    }
}

#[test]
fn pangil_data() {
    let coba = DataKordinat::new(1.2, 12.1);

    DataKordinat::test(String::from("ASIKKK"));

    println!("{}", coba.0);
    println!("{}", coba.1);

<<<<<<< HEAD
    let hasil = DataKordinat(1.2,12.2);
 
=======
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
>>>>>>> 16bc7b17962bd6952927b568ce25bd2eae304ce0
}
