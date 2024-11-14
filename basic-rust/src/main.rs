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
fn variabel_scope(){

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