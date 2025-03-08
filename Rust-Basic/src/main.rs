

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