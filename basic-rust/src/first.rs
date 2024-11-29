use crate::third::say_hello as modul_third;

pub fn say_hello() {
    println!("Hello dari First Modul");

    modul_third();
}

pub mod second {
    pub mod third {
        pub fn say_hello() {
            crate::first::say_hello();
            super::super::say_hello();
        }
    }
}
