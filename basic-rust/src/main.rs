fn main() {
    println!("Hello word");
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

    let hasil = DataKordinat(1.2,12.2);
 
}
