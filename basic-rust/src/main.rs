mod first;
mod second;
mod model;

use first::say_hello as modul_first;
use second::say_hello as modul_second;

use model::User;
fn main() {
  
    let data = User {
        first_name: String::from("Audyari"),
        last_name: String::from("Wiyono"),
        user_name: String::from("ASEP"),
        email: String::from("audy@gmail.com"),
        age: 40,
    };

    data.say_hello("ASIKKK");

    println!("{}",data.last_name);
    println!("{}",data.age);
    println!("{}",data.email);
    println!("{}",data.user_name);

    modul_first();
    modul_second();
}






#[test]
fn test_modul_alias(){
    modul_first();
    modul_second();

}
