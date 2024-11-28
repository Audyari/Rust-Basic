fn main() {
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

}