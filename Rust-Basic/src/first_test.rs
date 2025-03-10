use crate::third_test::say_hello as third_say_hello_test;

pub fn say_hello() {
   println!("Hello, world! from first");

   third_say_hello_test();

}

pub mod second{
   pub mod third{
      pub fn say_hello(){
         crate::first::say_hello();
         super::super::say_hello();
      }
   }
}

