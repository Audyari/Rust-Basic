

fn main() {
    println!("Hello, world!");
    println!("Hello, Audy!");
    println!("Hello, Wira!");
}

#[test]
fn test_main() {

    //membuat main test 

    println!("Hello, world!");
    println!("Hello, Audy!");
    println!("Hello, Wira!");
}

#[test]
fn test_variabel() {

    // membuat variabel 
    let x = 5;
    let y = x + 1;
    
    println!("x = {}, y = {}", x, y);

    let mut name = "Asep";
    println!("name = {}", name);

    name = "Audy";
    println!("name = {}", name);


    // Shadowing
    let name = 10;
    println!("name = {}", name);
}

#[test]
fn test_data_type() {
    // Data Type
    let age : u8 = 42;
    println!("age = {}", age);

}

#[test]
fn test_number() {
    // Number
   let a  : i32 = 10;
   println!("a = {}", a);

   let b : f64 = 10.5;
   println!("b = {}", b);
}

#[test]
fn test_konversi_tipe_data_number() {
   // Konversi Tipe Data Number
   let a : i8 = 10;
   let b : i16 = a as i16;
   println!("b = {}", b);

   let c : i32 = b as i32;
   println!("c = {}", c);

   let d : i64 = c as i64;
   println!("d = {}", d);

   let e : i128 = d as i128;
   println!("e = {}", e);

   //problem convert dari besar ke kecil 
   let f: i128= 10000000000000;
   let g : i8 = f as i8;
   println!("g = {}", g);
}

#[test]
fn test_numeric_operations() {
   // Numeric Operations
   let a : i8 = 10;
   let b : i8 = 20;
   let c : i8 = a + b;
   println!("c = {}", c);

   let d : i8 = a - b;
   println!("d = {}", d);
}

#[test]
fn test_augmented_assignments() {
   // Augmented Assignments
   let mut a : i8 = 10;
   a += 10;
   println!("a = {}", a);
}

#[test]
fn test_boolean() {
   // Boolean
   let a : bool = true;
   println!("a = {}", a);

}

#[test]
fn test_comparison_operators() {
   // Comparison-Operators
   let a : bool = 10 == 10;
   println!("a = {}", a);

   let b : bool = 10 != 10;
   println!("b = {}", b);

   let c : bool = 10 > 10;
   println!("c = {}", c);

   let d : bool = 10 < 10;
   println!("d = {}", d);

}

#[test]
fn test_boolean_operators() {
   // Boolean-Operators
   
   let a : bool = true && false;
   println!("a = {}", a);

   let b : bool = true || false;
   println!("b = {}", b);

   let c : bool = !true;
   println!("c = {}", c);

   let absen = 70;
   let nilai_akhir =  800;

   let lulus = absen > 75 && nilai_akhir > 700;

   println!("Hasil lulus = {}", lulus);
   
}

#[test]
fn test_char() {
   // Char

   let a : char = 'A';
   println!("a = {}", a);

   let b : char = 'b';
   println!("b = {}", b);

   
}

#[test]
fn test_tuple() {
   // Tuple
   let a : (i8, i8) = (10, 20);
   println!("a = {}, {}", a.0, a.1);

   let b : (u8, char) = (10, 'a');
   println!("b = {}, {}", b.0, b.1);

   let c : (String, bool) = (String::from("Hello"), true);
   println!("c = {:?}, {}", c.0, c.1);

   let d : (String, u8, bool) = (String::from("Hello"), 10, true);
   println!("d = {:?}, {}, {}", d.0, d.1, d.2);
}

#[test]
fn test_destructuring_tuple() {
   // Destructuring Tuple
   let data : (i8, String, bool)  =  (10, String::from("Hello"), true);

   println!("data = {:?}", data);

   let (a, _, c) = data;
   println!("a = {}, c = {}", a, c);
}

#[test]
fn test_mutable_tuple() {
   // Mutable Tuple
   let mut data : (i8, String, bool)  =  (10, String::from("Hello"), true);

   let (a, _, c) = data;
   println!("a = {}, c = {}", a, c);

   data.0 = 30;
   data.1 = String::from("World");
   data.2 = false;
   println!("data = {:?}", data);

}
