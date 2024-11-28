fn main() {
<<<<<<< HEAD
    println!("Hello");
    print!("Audyari ");
    print!("Wiyono");
    println!("");
    println!("Salam Kenal");
}

enum Level {
    Regular,
    Premium,
    Platinum,
}

enum Bank {
    CreditCard(String),
    BankTransfer(String, String),
    EWallet(String),
}

impl Level {
    fn tampilkan(&self) {
        match self {
            Level::Regular => {
                println!("Reguler")
=======
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
>>>>>>> 1e2f3bbbb2f5be235d930777f3847c4c799d987b
            }
            Level::Premium => {
                println!("Premium")
            }
            Level::Platinum => {
                println!("Platinum")
            }
        }
    }
}

impl Bank {
    fn pay(&self, nilai: u64) {
        match self {
            Bank::CreditCard(value) => {
                println!("data credit card {} {}", value, nilai)
            }
            Bank::BankTransfer(value1, value2) => {
                println!("Bank Transfer {} {} nilai nya {}", value1, value2, nilai)
            }
            Bank::EWallet(value) => {
                println!(" EWallet {}", value)
            }
        }
    }
}

#[test]
fn test_enum() {
    let data = Level::Platinum;
    let _data1 = Level::Premium;
    let _data2 = Level::Regular;

    let data3 = Bank::BankTransfer(String::from("BCA"), String::from("123456789"));
    let _data4 = Bank::CreditCard(String::from("Audyari W"));
    let _data5 = Bank::EWallet(String::from("EMONEY"));

    data3.pay(085771263192);
    data.tampilkan();
}

#[test]
fn maching_value() {
    let nama = "ASEP";

    match nama {
        "AUDY" | "ASEP" => {
            println!("DATA AUDYARI");
        }
        "WIYO" => {
            println!("WIYO");
        }
        other => {
            println!("data nya kok {}", other);
        }
    }
}

#[test]
fn maching_range() {
    let data = 90;

    match data {
        60..100 => {
            println!("data 1 sampai 10");
        }
        0..=59 => {
            println!("data 11 sampai 20");
        }
        other => {
            println!("tidak ada data {}", other);
        }
    }

    println!("HASIL");
}

struct GeoPoint(f64, f64);

impl GeoPoint {
    fn new(x: f64, y: f64) -> Self {
        let point = GeoPoint(x, y); // Create a new GeoPoint instance

        match point {
            GeoPoint(0.0, value1) => {
                println!("Ada data kosong, ini data yg isi {}", value1);
            }
            GeoPoint(value1, 0.0) => {
                println!("Ada data kosong, ini data yg isi {}", value1);
            }
            GeoPoint(value1, value2) => {
                println!("Data keluar semua {} {}", value1, value2);
            }
        }

        point // Return the created GeoPoint instance
    }
}

#[test]
fn pemanggilan_struct() {
    let _data = GeoPoint(0.0, 15.3);

    GeoPoint::new(12.2, 14.2);
}

struct Person {
    first_nama: String,
    middle_name: String,
    last_name: String,
    age: u16,
}

#[test]
fn panggil_struck_person() {
    let data = Person {
        first_nama: String::from("Audyari"),
        last_name: String::from("midle"),
        middle_name: String::from("Belakang"),
        age: 12,
    };

    match data {
        Person {
            first_nama,
            last_name,
            middle_name,
            age,
            ..
        } => {
            println!(
                "Tampilkan semua {} {} {} {}",
                first_nama, last_name, middle_name, age
            )
        }
    }
}

#[test]
fn maching_expresi() {
    let data = 10;
    let hasil = match data {
        1 => "Satu",
        0 => "kosong",
        _ => "invalit",
    };

    println!("{}", hasil);
}

type Age = u64;
type  IdentitasKTP = String;

struct Customers {
    usia:Age,
    ktp:IdentitasKTP,
}

#[test]
fn panggil_data(){
    let data = Customers {
        usia: 12,
        ktp: String::from("12345678920").clone()
    };

    match &data {
        Customers { usia, ktp } if usia < &18 => {
            println!("Usia: {}, KTP: {} (Anak)", usia, ktp);
        }
        Customers { usia, ktp } if usia >= &18 && usia < &60 => {
            println!("Usia: {}, KTP: {} (Dewasa)", usia, ktp);
        }
        Customers { usia, ktp } if usia >= &60 => {
            println!("Usia: {}, KTP: {} (Lansia)", usia, ktp);
        }
        _ => {
            println!("Data tidak valid");
        }
    }

    println!(" {} {} ",data.ktp,data.usia);

<<<<<<< HEAD
}
=======
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
>>>>>>> 1e2f3bbbb2f5be235d930777f3847c4c799d987b
