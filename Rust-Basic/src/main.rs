

fn main() {
    println!("Hello, world!");
    println!("Hello, Audy!");
    println!("Hello, Wira!");

    // mengakses tuple kosong

    let result = unit();
    println!("result = {:?}", result);

    // Stack dan Heap
    function_a();
    function_b();

    // memanggil function
    say_hello();

    // memanggil function with parameters
    say_goodbye("Audy", "Wiyono");
    say_goodbye("Asep", "Johar");

    // memanggil factorial

    let result = factorial_loop(5);
    println!("result = {}", result);
  
    let result = factorial_loop(-10);
    println!("result = {}", result);


    // memanggil recursive function
    print_text(String::from("Hello Audyari"), 3);

    // memanggil factorial recursive
    let result = factorial_recursive(5);
    println!("result = {}", result);


    // test print number

    let number = 10;
    print_number(number); 
    println!("number = {}", number);   

    
   // test hi 

   let name = String::from("Audy");
   hi(name.clone());
   println!("name = {}", name);


   // test full name

   let name = full_name(String::from("Audy"), String::from("Wiyono"));
   println!("name = {}", name);

   // test full name tanpa clone

   let (first_name, last_name, name) = full_name_tanpa_clone(String::from("Audy"), String::from("Wiyono"));
   println!("name = {}", name);
   println!("first_name = {}", first_name);
   println!("last_name = {}", last_name);

   // test reference full name

   
   let first_name = String::from("Audy");
   let last_name = String::from("Wiyono");

   let name = reference_full_name(&first_name, &last_name);
   
   println!("name = {}", name);
   println!("first_name = {}", first_name);
   println!("last_name = {}", last_name);

   // change value test

   let mut value = String::from("Audyari");

   change_value(&mut value);

   // value.push_str("ASIK");

   print!("{}",value);

   // mengakses struct person

   let person = Person {
      first_name: String::from("Audy"),
      middle_name: String::from("Wiyono"),
      last_name: String::from("Wiyono"),
      age: 22,
   };
   
   println!("{} ",person.first_name);
   println!("{} ",person.middle_name);
   println!("{} ",person.last_name);
   println!("{} ",person.age);

   // test print_person

   let person = Person {
      first_name: String::from("Audy"),
      middle_name: String::from("Wiyono"),
      last_name: String::from("Wiyono"),
      age: 22,
   };
   
   print_person(&person);

   // memanggil struct_function

   struct_function();

   // memanggil tuple struct

   let geo_point = GeoPoint(-6.200000,106.816666);
   println!("{} {}", geo_point.0, geo_point.1);

   // struct nothing

   let _nothing1 = Nothing;
   let _nothing2 = Nothing;

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

// cargo test test_konversi_tipe_data_number -- --exact --nocapture
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

// cargo test test_numeric_operations -- --exact --nocapture
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

// cargo test test_augmented_assignments -- --exact --nocapture
#[test]
fn test_augmented_assignments() {
   // Augmented Assignments
   let mut a : i8 = 10;
   a += 10;
   println!("a = {}", a);
}

// cargo test test_boolean -- --exact --nocapture
#[test]
fn test_boolean() {
   // Boolean
   let a : bool = true;
   println!("a = {}", a);

}

// cargo test test_comparison_operators -- --exact --nocapture
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

// cargo test test_boolean_operators -- --exact --nocapture
#[test]
fn test_boolean_operators() {
   // Boolean-Operators
   
   let a : bool = true && false;
   println!("a = {}", a);

   let b : bool = true || false;
   println!("b = {}", b);

   let c : bool = !true;
   println!("c = {}", c);

   let absen: i32 = 75;
   let nilai_akhir: i32 =  80;

   let lulus = absen >= 75 && nilai_akhir >= 75;

   println!("Hasil lulus = {}", lulus);
   
}

// cargo test test_char -- --exact --nocapture
#[test]
fn test_char() {
   // Char

   let a : char = 'A';
   println!("a = {}", a);

   let b : char = 'b';
   println!("b = {}", b);

   
}

// cargo test test_tuple -- --exact --nocapture
#[test]
fn test_tuple() {

   // tuple

   let data: (i32, i32, bool) = (10, 20, true); 
   println!("data = {:?}", data);

   // Mengakses Tuple

   let a = data.0;
   let b = data.1;
   let c = data.2;

   println!("a = {}, b = {}, c = {}", a, b, c);

}

#[test]
fn test_destructuring_tuple() {
   // Destructuring Tuple
   let data : (i8, String, bool)  =  (10, String::from("Hello"), true);

   println!("data = {:?}", data);

   let (a, _, c) = data;
   println!("a = {}, c = {}", a, c);
}

// cargo test test_mutable_tuple -- --exact --nocapture
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

fn unit(){
   println!("Hello, world!");
}

#[test]
fn test_unit() {
   // tuple kosong
  let result = unit();
  println!("result = {:?}", result);

  let test =();
  println!("test = {:?}", test);
}

#[test]
fn test_array() {
   // Membuat Array

   let array: [i32; 5] = [1, 2, 3, 4, 5];
   println!("array = {:?}", array);  

   // Mengakses Array

   let a = array[0];
   println!("a = {}", a);

   let b = array[1];
   println!("b = {}", b);

   let c = array[2];
   println!("c = {}", c);

   let d = array[3];
   println!("d = {}", d);

   let e = array[4];
   println!("e = {}", e);


}

// cargo test test_array_mutable -- --exact --nocapture
#[test]
fn test_array_mutable() {
   
   // Mutable Array
   let mut array: [i32; 5] = [1, 2, 3, 4, 5];
   println!("array = {:?}", array);

   array[0] = 10;
   array[1] = 20;
   array[2] = 30;
   array[3] = 40;
   array[4] = 50;
   println!("array = {:?}", array);
}

#[test]
fn test_array_length() {
   // Panjang Array
  
  let array: [i32; 5] = [1, 2, 3, 4, 5];
  println!("array = {:?}", array);

  let length = array.len();
  println!("length = {}", length);
}


#[test]
fn test_two_dimensional_array() {
   // Two Dimensional Array

   let array: [[i32; 2]; 3] = [
      [1, 2],
      [3, 4],
      [5, 6],
   ];
   println!("array = {:?}", array);

   let a = array[0][0];
   println!("a = {}", a);

   let b = array[0][1];
   println!("b = {}", b);

   let c = array[1][0];
   println!("c = {}", c);

   let d = array[1][1];
   println!("d = {}", d);

   let e = array[2][0];
   println!("e = {}", e);

   let f = array[2][1];
   println!("f = {}", f);
}  

// cargo test test_constant -- --exact --nocapture
#[test]
fn test_constant() {
   // constant
   const MAX_VALUE: i32 = 100;
   const MIN_VALUE: i32 = 0;
   println!("MAX_VALUE = {}", MAX_VALUE);
   println!("MIN_VALUE = {}", MIN_VALUE);
}

// cargo test test_variable_scope -- --exact --nocapture
#[test]
fn test_variable_scope() {
   // Variable Scope
   let  audy = 1;
   {
      println!("audy = {}", audy);
      let wiyono = 2;
      println!("wiyono = {}", wiyono);
   }
   println!("audy = {}", audy);
}

#[test]
fn test_stack_dan_heap() {

   // Stack dan Heap
   function_a();
   function_b();

}

fn function_a() {
   let a = 10;
   let b = String::from("Hello");
   println!("a = {}, b = {}", a, b);
}

fn function_b() {
   let a = 20;
   let b = String::from("World");
   println!("a = {}, b = {}", a, b);
}

// cargo test test_string -- --exact --nocapture
#[test]
fn test_string_stack(){
   let mut name = " Audyari Wiyono ";
   println!("name = {}", name);

   name = "Audyari";
   println!("name = {}", name);

   // trim adalah menghapus whitespace
   let trim: &str = name.trim();
   println!("trim = {}", trim);

   let len: usize = name.len();
   println!("len = {}", len);
}

// cargo test test_string_type -- --exact --nocapture
#[test]
fn test_string_type_heap(){
   // String Type
   let mut name = String::from("Audyari");
   println!("name = {}", name);

   name.push_str(" Wiyono");
   println!("name = {}", name);

   let ubah = name.replace("Audyari", "Asep");
   println!("ubah = {}", ubah);

}

#[test]
fn test_ownership_rules() {

   //a tidak bisa diskses disini, belun dideklarasikan 
   let a = 10; // a bisa diakses mulai disini 
   
   {// b tidak bisa diskses disini, helun dideKlarasikan 
      let b = 20; // b bisa diakses mulai disini 
      println!("{}", b); 
   } // scope b selesai, b dihapus, b tidak bisa diakses lagi 
     
   println! ("{}", a); 
   
} // scope a selesai, a dihapus, a tidak bisa diskses lagi 
   

// cargo test test_data_copy -- --exact --nocapture
#[test]
fn test_data_copy() {
   
   let a = 10;
   let b = a; // copy data a ke b 
   println!("a = {}, b = {}", a, b);

}

// cargo test test_ownership_movement -- --exact --nocapture
#[test]
fn test_ownership_movement() {

   let name1 = String::from("Audyari");

   // ownership dari name1 di pindahkan ke name2
   let name2 = name1;

   // name1 tidak bisa diakses lagi
   println!("name2 = {}", name2);
   
}

#[test]
fn test_ownership_clone() {
   
   let name1 = String::from("Audyari");
   let name2 = name1.clone();
   
   println!("name1 = {}, name2 = {}", name1, name2);
   
}

// cargo test test_if_expression -- --exact --nocapture
#[test]
fn test_if_expression() {
   
   let value = 9;

   if value >= 8{
      println!("value >= 8 = {}", value); 
   } else if value == 6 {
      println!("value = 6 = {}", value);
   } else if value == 3 {
      println!("value = 3 = {}", value);
   } else {
      println!("value = {}", value);
   }
   
}

#[test]
fn test_let_statement() {
   let value = 9;
   let result: &str;

   if value >= 8 {
      result = "value >= 8";
   } else {
      result = "value < 8";
   }
   println!("{}", result);
}

// cargo test test_if_let_statement -- --exact --nocapture
#[test]
fn test_if_let_statement() {
   let value: i32 = 9;
   let result: &str = if value >=8 {
      "value >= 8"
   } else {
      "value < 8"
   };
   println!("{}", result);
}

#[test]
fn test_if_let_statement_string() {
   
   let value: i32 = 9;
   let result: String = if value >=8 {
      "value >= 8".to_string()
   } else {
      "value < 8".to_string()
   };
   println!("{}", result);

}

#[test]
fn test_loop() {

   let mut counter = 0; 
   loop { 
      counter += 1; 

      if counter > 10 { 
         break; 
      } 

      if counter % 2 == 0 { 
         continue; 
      } 

      println! ("Counter: {}", counter); 
   } 

   println! ("Counter: {}", counter);
   
}

#[test]
fn loop_return_value() {
   
   let mut counter: i32 = 0; 
  
   let result: i32 = loop { 
      counter += 1; 
      println!("counter = {}", counter);

      if counter > 10 { 
         println!("counter di if = {}", counter);
         break counter * 2; 
      } 
   };
   println!("result = {}", result);
}

#[test]
fn loop_label() {
   
    let mut number = 1;

    'outer: loop {
        let mut i = 1;

        loop {
            if number > 10 {
                break 'outer;
            }

            println!("{} x {} = {}", number, i, number * i);

            i += 1;

            if i > 10 {
                break;
            }
        }

        number += 1;
    }
}

// cargo test test_while_loop -- --exact --nocapture
#[test]
fn test_while_loop() {
   
   let mut counter = 0;
   while counter <= 10 {

      if counter % 2 == 0 {
         println!("counter : {}", counter);
      }

      counter += 1;

   }
}

// cargo test test_for_loop -- --exact --nocapture
#[test]
fn test_for_loop() {

   println!("++++++++Menggunakkan While Loop++++++++++++++++++");
   
   let array = ['a', 'b', 'c', 'd', 'e']; // memory masuk ke stack
   let mut index = 0; // memory masuk ke stack

   // while loop
   while index < array.len() { // memory masuk ke stack
      println!("{}", array[index]);
      index += 1;
   }

   println!("++++++++Menggunakkan For Loop++++++++++++++++++");

   // for loop
   for item in array { // memory masuk ke heap
      println!("{}", item); // memory masuk ke stack
   }

   println!("+++++Menggunakkan Range++++++++++++++++++++");

   // range
   let range = 0..5; // memory masuk ke heap
  
   println!("start = {}", range.start); // memory masuk ke stack
   println!("end = {}", range.end); // memory masuk ke stack

   for i in range {  // memory masuk ke stack
     println!("{}", array[i]);  // memory masuk ke stack
   }

   println!("+++++++Menggunakkan Range Inclusive++++++++++++++++++");

   // menggunakkan range inclusive
   let range = 0..=4; // memory masuk ke heap
   for i in range {
     println!("{}", array[i]);
   }

}

fn say_hello() {
   println!("Hello");
}

fn say_goodbye(first_name: &str, last_name: &str) { // memori masuk ke heap
   println!("{} {}", first_name, last_name); // memori masuk ke stack
}

#[test]
fn test_function() { // memori masuk ke heap
   say_hello(); // memori masuk ke stack
}

#[test]
fn test_function_with_parameters() { // memori masuk ke heap
   say_goodbye("Audy", "Wiyono"); // memori masuk ke stack
   say_goodbye("Asep", "Johar"); // memori masuk ke stack
}

fn factorial_loop(n: i32) -> i32 {
  
  if n  < 1 {
   return 0;
  }

  let mut result = 1;
 
  for i in 1..=n { // memory masuk ke stack
    result *= i;
  }
  result

}

#[test]
fn test_factorial_loop() {
  let result = factorial_loop(5);
  println!("result = {}", result);

  let result = factorial_loop(-10);
  println!("result = {}", result);
}

fn print_text(value: String, times: u32) {

   // recursive function   
  if times == 0 {
   return;
  }else{
   println!("{}", value); 
  }

  print_text(value, times - 1);
}

#[test]
fn test_print_text() {
   print_text(String::from("Hello Audyari"), 3);
}

fn factorial_recursive(n: u32) -> u32 {
  if n == 1 {
    return 1;
  }
  n * factorial_recursive(n - 1)
}

#[test]
fn test_factorial_recursive() {
  let result = factorial_recursive(5);
  println!("result = {}", result);
}

 
fn print_number(number: i32){ // memori masuk ke stack
   println!("number = {}", number); // memori masuk ke stack
}

fn hi(name:String){ // memori masuk ke heap
   println!("Hi {}", name); // memori masuk ke stack
}


// ownership function
#[test]
fn ownership_function() {
   
   // test print number

   let number = 10;
   print_number(number); //number dilakukan copy
   println!("number = {}", number);

   // test hi 

   let name = String::from("Audy");

   hi(name.clone()); // name di pindahkan ke hi
   println!("name = {}", name);

}

fn full_name(first_name: String, last_name: String) -> String {
   format!("{} {}", first_name, last_name)   
}

fn full_name_tanpa_clone(first_name: String, last_name: String) -> (String, String, String) { // memori masuk ke heap tipe data tuple

   let full_name_tanpa_clone = format!("{} {}", first_name, last_name);

   (first_name, last_name, full_name_tanpa_clone)

}

#[test]  
fn test_full_name() {

  let first_name = String::from("Audy");
  let last_name = String::from("Wiyono");

  let name = full_name(first_name.clone(), last_name.clone());
  println!("name = {}", name);

  println!("first_name = {}", first_name);
  println!("last_name = {}", last_name);
  
}

#[test]
fn test_full_name_tanpa_clone() {

  let first_name = String::from("Audy");
  let last_name = String::from("Wiyono");

  let (first_name, last_name, name) = full_name_tanpa_clone(first_name.clone(), last_name.clone());

  println!("name = {}", name);
  println!("first_name = {}", first_name);
  println!("last_name = {}", last_name);
  
}


fn reference_full_name(first_name: &String, last_name: &String) -> String {
   let name =  format!("{} {}", first_name, last_name);
   return name;
}

#[test]
fn test_reference_full_name() {

   let first_name = String::from("Audy");
   let last_name = String::from("Wiyono");

   let name = reference_full_name(&first_name, &last_name);

   println!("name = {}", name);
   println!("first_name = {}", first_name);
   println!("last_name = {}", last_name);
}

fn change_value(value: &mut String) {
   value.push_str(" test");
}

#[test]
fn test_change_value(){
   let mut value = String::from("Audyari");

   change_value(&mut value);

   // value.push_str("ASIK");

   print!("{}",value);
}

#[test]
fn test_slice(){
   let array = [1,2,3,4,5,6,7,8,9,10]; // memory di simpan di stack
   
   let slice1 = &array[..];             

   println!("array = {:?}", array);
   println!("slice1 = {:?}", slice1);

   let slice2 = &array[0..5];             
   println!("slice2 = {:?}", slice2);

   let slice3 = &array[5..];              
   println!("slice3 = {:?}", slice3);

   let test = slice2;                    
   println!("test = {:?}", test);
  
}


#[test]
fn string_slice() {
   
   let name = String::from("Audyari wiyono"); // memori di simpan di heap
   let first_name = &name[0..7];   

   println!("first_name = {}", first_name);

   let last_name = &name[8..14];                // memori di simpan di stack
   println!("last_name = {}", last_name);
 

}

struct Person {
   first_name: String,
   middle_name: String,
   last_name: String,
   age: u32,
}

#[test]
fn test_struct_person() {
 
   let person = Person {
      first_name: String::from("Audy"),
      middle_name: String::from("Wiyono"),
      last_name: String::from("Wiyono"),
      age: 22,
   };
   
   println!("{} ",person.first_name);
   println!("{} ",person.middle_name);
   println!("{} ",person.last_name);
   println!("{} ",person.age);

}

fn print_person(person: &Person) {
   
   println!("{} ",person.first_name);
   println!("{} ",person.middle_name);
   println!("{} ",person.last_name);
   println!("{} ",person.age);
}

#[test]
fn test_print_person() {
   
   let person = Person {
      first_name: String::from("Audy"),
      middle_name: String::from("Wiyono"),
      last_name: String::from("Wiyono"),
      age: 22,
   };
   
   print_person(&person);

}

fn struct_function() {
   
   let first_name = String::from("Audy"); // memori masuk ke heap
   let last_name = String::from("Wiyono");
   let middle_name = String::from("Wiyono");
   let age = 22;

   let person = Person {
      first_name: first_name.clone(),        // memori masuk ke heap data di clone
      middle_name: middle_name.clone(),
      last_name: last_name.clone(),
      age: age,
   };

   println!("+++++Struct Syntax+++++++++++++");

   print_person(&person);

   println!("+++++Struct Update Syntax+++++++++++++"); 

   let person2:Person = Person { ..person};

   print_person(&person2);

   println!("+++++++ Partial Struct Update Syntax ++++++++++++++++++++++++");

   let person3 = Person {
      first_name: person2.first_name.clone(),
      middle_name: person2.middle_name.clone(),
      last_name: person2.last_name.clone(),
      age: age,
   };

   print_person(&person3);

}

struct GeoPoint(f64, f64);

#[test]
fn tuple_struct(){
   let geo_point = GeoPoint(-6.200000,106.816666);
   println!("{} {}", geo_point.0, geo_point.1);
}

struct Nothing;

#[test]
fn test_nothing() {
   let _nothing1 = Nothing;
   let _nothing2 = Nothing;

}
