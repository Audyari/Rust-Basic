

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

   // memanggil methode 

   let person = Person {
      first_name: String::from("Audy"),
      middle_name: String::from("Wiyono"),
      last_name: String::from("Wiyono"),
      age: 22,
    };
    
    person.say_hello("Audy");

    // implementasi geo point

    let geo_point = GeoPoint::new(-6.200000,106.816666);
    println!("{} {}", geo_point.0, geo_point.1);

    // mengakses enum

    let level1 = Level::Regular;
  
   match level1 {
      Level::Regular => println!("Regular"),
      Level::Premium => println!("Premium"),
      Level::Platinum => println!("Platinum"),
   }

   let level2 = Level::Premium;
   match level2 {
      Level::Regular => println!("Regular"),
      Level::Premium => println!("Premium"),
      Level::Platinum => println!("Platinum"),
   }

   let level3 = Level::Platinum;
   match level3 {
      Level::Regular => println!("Regular"),
      Level::Premium => println!("Premium"),
      Level::Platinum => println!("Platinum"),
   }

    // penganggilan enum payment

    let payment1 = Payment::CreditCard(String::from("1234567890"));

    match payment1 {
       Payment::CreditCard(card_number) => println!("Credit Card: {}", card_number),
       Payment::BankTransfer(bank_name, account_number) => println!("Bank Transfer: {} {}", bank_name, account_number),
       Payment::EWallet(ewallet_number, ewallet_name) => println!("eWallet: {} {}", ewallet_number, ewallet_name),
    }
 
    let payment2 = Payment::BankTransfer(String::from("Bank BRI"), String::from("1234567890"));
    match payment2 {
       Payment::CreditCard(card_number) => println!("Credit Card: {}", card_number),
       Payment::BankTransfer(bank_name, account_number) => println!("Bank Transfer: {} {}", bank_name, account_number),
       Payment::EWallet(ewallet_number, ewallet_name) => println!("eWallet: {} {}", ewallet_number, ewallet_name),
    }
 
    let payment3 = Payment::EWallet(String::from("1234567890"), String::from("eWallet"));
    match payment3 {
       Payment::CreditCard(card_number) => println!("Credit Card: {}", card_number),
       Payment::BankTransfer(bank_name, account_number) => println!("Bank Transfer: {} {}", bank_name, account_number),
       Payment::EWallet(ewallet_number, ewallet_name) => println!("eWallet: {} {}", ewallet_number, ewallet_name),
    }

    // memanggil impl Pay

    let payment1 = Payment::CreditCard(String::from("1234567890"));
    payment1.pay(1000);

    // memanggil ALias

    let customer = Customer {
      id: String::from("1234567890"),
      age: 22,
      name: String::from("Audy Wiyono"),
    };
   
    println!("{}", customer.name);
    println!("{}", customer.age);
    println!("{}", customer.id);


    // cara memanggil model

    let user = modul::User { 
      first_name: String::from("Audy"),
      last_name: String::from("Wiyono"),
      username: String::from("audy"),
      email: String::from("audy@audy.com"),
      age: 22,
    };
   
    user.say_hello("Audy");  

    // cara memanggil modul as
     
    first_say_hello();
    second_say_hello();
   
   // memanggil modul test

   first_say_hello_test();
   second_say_hello_test();

   first_test::second::third::say_hello();


   // memanggil trait

   let person = Person {
      first_name: String::from("Audy"),
      middle_name: String::from("Wiyono"),
      last_name: String::from("Wiyono"),
      age: 22,
   };
   
   println!("{}", person.say_hello_trait());
   println!("{}", person.say_hello_to("Audy"));
   println!("{}", person.hello());

    // pemanggilan trait 2 

    let person = Person {
      first_name: String::from("Audy"),
      middle_name: String::from("Wiyono"),
      last_name: String::from("Wiyono"),
      age: 22,
   };
   
   say_hello_trait(&person, "Audy");

   // pemanggilan trait 3

   let person = Person {
      first_name: String::from("Audy"),
      middle_name: String::from("Wiyono"),
      last_name: String::from("Wiyono"),
      age: 22,
   };
   
   hello_and_goodbye(&person);

   // pemanggilan trait 4

   let person = create_person(String::from("Audy"));
   println!("{}", person.good_bye());
   println!("{}", person.good_bye_to("Audy"));


   // memanggil traid 6

   let person = Person {
      first_name: String::from("Audyaaa"),
      middle_name: String::from("Wiyono"),
      last_name: String::from("Wiyono"),
      age: 22,
   };
   
   person.say();

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

struct Person {
   first_name: String,
   middle_name: String,
   last_name: String,
   age: u32,
}

impl Person {
   fn say_hello(&self , name: &str) {
      println!("Hello, {} {} {}", name, self.middle_name, self.last_name);
   }
   
}


#[test]
fn test_method() {
  let person = Person {
    first_name: String::from("Audy"),
    middle_name: String::from("Wiyono"),
    last_name: String::from("Wiyono"),
    age: 22,
  };
  
  person.say_hello("Audy");
}


struct GeoPoint(f64, f64);

impl GeoPoint {
   fn new(lat: f64, lng: f64) -> GeoPoint {
      GeoPoint(lat, lng)
   }
}

#[test]
fn test_geo_point() {
   let geo_point = GeoPoint::new(-6.200000,106.816666);
   println!("{} {}", geo_point.0, geo_point.1);
}


enum Level {
  Regular,
  Premium,
  Platinum,
}

#[test]
fn test_level() {
   let level1 = Level::Regular;
  
   match level1 {
      Level::Regular => println!("Regular"),
      Level::Premium => println!("Premium"),
      Level::Platinum => println!("Platinum"),
   }

   let level2 = Level::Premium;
   match level2 {
      Level::Regular => println!("Regular"),
      Level::Premium => println!("Premium"),
      Level::Platinum => println!("Platinum"),
   }

   let level3 = Level::Platinum;
   match level3 {
      Level::Regular => println!("Regular"),
      Level::Premium => println!("Premium"),
      Level::Platinum => println!("Platinum"),
   }

}

enum Payment {
   // card number
   CreditCard(String),
   // bank name akun number
   BankTransfer(String, String),
   // ewallet number, ewallet name
   EWallet(String, String),
   
}

impl Payment {
   fn pay(&self, amount: u32) {    
    
      match self {
      Payment::CreditCard(card_number) => {
         println!("Paying with card number {} amount {}", card_number ,amount);
      },
      Payment::BankTransfer(bank_name, account_number) => {
         println!("Paying with bank transfer {} dan account number {} amount {}", bank_name, account_number, amount);
      },
      Payment::EWallet(ewallet_number, ewallet_name) => {
         println!("Paying with ewallet {} dan ewallet name {} amount {}", ewallet_number, ewallet_name, amount);
      },
      }
   }
}


#[test]
fn test_payment_method() {
   let payment1 = Payment::CreditCard(String::from("1234567890"));
   payment1.pay(1000);

   let payment2 = Payment::BankTransfer(String::from("Bank BRI"), String::from("1234567890"));
   payment2.pay(1000);

   let payment3 = Payment::EWallet(String::from("1234567890"), String::from("eWallet"));
   payment3.pay(1000);
  
}


#[test]
fn test_payment() {
   let payment1 = Payment::CreditCard(String::from("1234567890"));

   match payment1 {
      Payment::CreditCard(card_number) => println!("Credit Card: {}", card_number),
      Payment::BankTransfer(bank_name, account_number) => println!("Bank Transfer: {} {}", bank_name, account_number),
      Payment::EWallet(ewallet_number, ewallet_name) => println!("eWallet: {} {}", ewallet_number, ewallet_name),
   }

   let payment2 = Payment::BankTransfer(String::from("Bank BRI"), String::from("1234567890"));
   match payment2 {
      Payment::CreditCard(card_number) => println!("Credit Card: {}", card_number),
      Payment::BankTransfer(bank_name, account_number) => println!("Bank Transfer: {} {}", bank_name, account_number),
      Payment::EWallet(ewallet_number, ewallet_name) => println!("eWallet: {} {}", ewallet_number, ewallet_name),
   }

   let payment3 = Payment::EWallet(String::from("1234567890"), String::from("eWallet"));
   match payment3 {
      Payment::CreditCard(card_number) => println!("Credit Card: {}", card_number),
      Payment::BankTransfer(bank_name, account_number) => println!("Bank Transfer: {} {}", bank_name, account_number),
      Payment::EWallet(ewallet_number, ewallet_name) => println!("eWallet: {} {}", ewallet_number, ewallet_name),
   }

}

#[test]
fn test_enum_pattern_matching() {
   
   let level = Level::Regular;
   
   match level {
      Level::Regular => println!("Regular"),
      Level::Premium => println!("Premium"),
      Level::Platinum => println!("Platinum"),
   }
  
}

#[test]
fn test_match_value(){
   
   let name: &str = "Audyaa";

   match name {
      "Audy" => println!("Audy"),
      "Wiyono"=> println!("Wiyono"),
      other  => println!("hello {}",other),
   }  
}

#[test]
fn test_multiple_patterns(){
   let name = "audy";

   match name {
       "audy" | "budi" => {
         println!("Hello Bos")
       }
       other   => {
         println!("hello {} ", other)
       }
   }
}

#[test]
fn test_range_pattern(){

   let value = 100;

   match value {
       75..=100 => {
         println!("Good");
       }
       50..=74 => {
         println!("Not Bad");
       }
       0..=49 => {
         println!("Bad");
       }
       _ => {
         println!("Unknown");
       }
   }
}

#[test]
fn test_struct_pattern_matching() {
   
   let point  = GeoPoint(-6.200000,106.816666);
   //let point = GeoPoint(1.0, 0.0);

   match point {
      GeoPoint(0.0, 0.0) => {
         println!("Data Kosong");
      }
      GeoPoint(lat, 0.0) => {
         println!("{:.6} ", lat);
      }
      GeoPoint(0.0, lng) => {
         println!("data lng : {:.6} ", lng); 
      }  
      GeoPoint(lat, lng) => {
         println!("{:.6} dan {:.6} ", lat, lng);
      }
    
   }

}

#[test]
fn test_destructuring_struct_patterns(){
   
   let person = Person {
      first_name: String::from("Audy"),
      middle_name: String::new(),
      last_name: String::from("Wiyono"),
      age: 22,
   };

   match person {
      Person { first_name, last_name, ..} => {
         println!("nama depan : {} dan nama belakang : {}", first_name, last_name);
      }
   }
   
}

#[test]
fn test_ignore_values(){
   
   let point = GeoPoint(-6.200000,106.816666);

   match point {
       GeoPoint(lat, _) => {
           println!(" lat : {} ", lat);
       }
     
   }

   let value = 100;

   match value {
       75..=100 => {
         println!("Good");
       }
       50..=74 => {
         println!("Not Bad");
       }
       0..=49 => {
         println!("Bad");
       }
       _ => {
         println!("Unknown");
       }
   }

}

#[test]
fn test_match_expression(){

   let value = 4;

   let result = match value {
       1 => "One",
       2 => "Two",
       3 => "Three",
       _ => "Other",
   };

   println!("{}", result);
   
}

type Age = u8;
type IdentityNumber = String;

struct Customer {
  id : IdentityNumber,
  age: Age,
  name: String,
}

#[test]
fn test_type_alias() {

 let customer = Customer {
   id: String::from("1234567890"),
   age: 22,
   name: String::from("Audyari Wiyono"),
 };

 println!("{}", customer.name);
 println!("{}", customer.age);
 println!("{}", customer.id);

 match customer {
   Customer { id, age: 22, name: n } => {
     println!("Hello, my name is {} and my id is {}", n, id);
   }
   _ => {
     println!("I don't know who you are");
   }
 }

}


pub mod modul {

   pub struct User{
      pub first_name: String,
      pub last_name: String,
      pub username: String,
      pub email: String,
      pub age: u8,
   }

   impl User {
      pub fn say_hello(&self, name: &str) {
         println!("Hello, {} My Name is {} {}", name, self.first_name, self.last_name);
      }
   }  

}

#[test]
fn test_method_modul() {
   
 let user = modul::User { 
   first_name: String::from("Audy"),
   last_name: String::from("Wiyono"),
   username: String::from("audy"),
   email: String::from("audy@audy.com"),
   age: 22,
 };

 user.say_hello("Audy");  
}


mod first{
   pub fn say_hello() {
      println!("say hello from first module");
   }
}

mod second{
   pub fn say_hello() {
      println!("say hello from second module");
   }
}

use first::say_hello as first_say_hello;
use second::say_hello as second_say_hello;

#[test]
fn test_method_modul_second() {
  first_say_hello();
  second_say_hello();
}

mod first_test;
mod second_test;
mod third_test;

use first_test::say_hello as first_say_hello_test;
use second_test::say_hello as second_say_hello_test;

#[test]
fn test_method_modul_second_test() {
  //first_say_hello_test();
  //second_say_hello_test();

  first_test::second::third::say_hello();
}


trait CanSayHello {
   fn hello(&self) -> String {
      String::from("Hello")
  }
  fn say_hello_trait(&self) -> String;
  fn say_hello_to(&self, name: &str) -> String;
}

// struct Person {
//    first_name: String,
//    middle_name: String,
//    last_name: String,
//    age: u32,
// }

// impl Person {
//    fn say_hello(&self , name: &str) {
//       println!("Hello, {} {} {}", name, self.middle_name, self.last_name);
//    }
   
// }


impl CanSayHello for Person {

   fn say_hello_trait(&self) -> String {
      format!("Hello, my name is {}", self.first_name)
   }

   fn say_hello_to(&self, name: &str) -> String {
      format!("Hello {}, my name is {}", name, self.first_name)
   }

//    fn hello(&self) -> String {
//       String::from("Hello")
//   }
}

#[test]
fn test_trait() {
  
   let person = Person {
      first_name: String::from("Audy"),
      middle_name: String::from("Wiyono"),
      last_name: String::from("Wiyono"),
      age: 22,
   };
   
   println!("{}", person.say_hello_trait());
   println!("{}", person.say_hello_to("Audy"));
   println!("{}", person.hello());
   person.say_hello("Audy");
}


fn say_hello_trait(person: &impl CanSayHello, name: &str) {
   println!("{}", person.say_hello_trait());

   println!("{}",person.say_hello_to(name));
}

#[test]
fn test_trait_2() {

   let person = Person {
      first_name: String::from("Audy"),
      middle_name: String::from("Wiyono"),
      last_name: String::from("Wiyono"),
      age: 22,
   };
   
   say_hello_trait(&person, "Audy");
}


trait CanSayGoodbye {
   fn good_bye(&self) -> String;
   fn good_bye_to(&self, name: &str) -> String;
}

impl CanSayGoodbye for Person {
   fn good_bye(&self) -> String {
     format!("Goodbye, my name is {}", self.first_name)
   }
   
   fn good_bye_to(&self, name: &str) -> String {
      format!("Goodbye {}, my name is {}", name, self.first_name)
   }
}

fn hello_and_goodbye(person: &(impl CanSayHello + CanSayGoodbye)) {
  println!("{}", person.say_hello_trait());
  println!("{}", person.good_bye());
  println!("{}", person.good_bye_to("Audy"));
}

#[test]
fn test_trait_3() {
   let person = Person {
      first_name: String::from("Audy"),
      middle_name: String::from("Wiyono"),
      last_name: String::from("Wiyono"),
      age: 22,
   };
   
   hello_and_goodbye(&person);
}


struct SimplePerson {
   name: String,
}

impl CanSayGoodbye for SimplePerson {
   fn good_bye(&self) -> String {
      format!("Goodbye, my name is {}", self.name)
   }
   
   fn good_bye_to(&self, name: &str) -> String {
      format!("Goodbye {}, my name is {}", name, self.name)
   }
}

fn create_person(name: String) -> impl CanSayGoodbye {
   if name.is_empty() {
      return SimplePerson { name: String::from("Unknown") };
   }

   SimplePerson { name }

}  

#[test]
fn test_trait_4() {
   let person = create_person(String::from("Audy"));
   println!("{}", person.good_bye());
   println!("{}", person.good_bye_to("Audy"));
}

#[test]
fn test_trait_5() {
   
   let person = Person {
      first_name: String::from("Audyaaa"),
      middle_name: String::from("Wiyono"),
      last_name: String::from("Wiyono"),
      age: 22,
   };
   
   person.say_hello("Audy");
   println!("++++++++++++++++++++++");
   let result = CanSayHello::say_hello_trait(&person);
   println!("{}", result);

}

trait  CanSay: CanSayHello + CanSayGoodbye {

   // sekarang kita bisa implementasi CanSay
   // kita harus mengimplementasikan  CanSayHello dan CanSayGoodBye   
   fn say(&self){
      
      println!("{}", self.say_hello_trait());
      println!("{}", self.good_bye());

   }
}

impl CanSay for Person {
   
   fn say(&self) {
      println!("{}", self.say_hello_trait());
      println!("{}", self.good_bye());
   }
}

#[test]
fn test_trait_6() {
   let person = Person {
      first_name: String::from("Audyaaa"),
      middle_name: String::from("Wiyono"),
      last_name: String::from("Wiyono"),
      age: 22,
   };
   
   person.say();
}



