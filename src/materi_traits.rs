/*
    Trait adalah kontrak/blueprint yang mendefinisikan behavior yang harus dimiliki sebuah tipe. Mirip interface di Java.
    Kenapa penting:
    - Bikin kode reusable & flexible
    - Anchor Framework heavily menggunakan traits
    - Dasar dari semua library Rust
 */


// Definisi trait
trait Info {
    fn tampilkan(&self);
    fn harga(&self) -> f64;
}

struct Token {
    nama: String,
    harga: f64,
}

struct NFT {
    nama: String,
    floor_price: f64,
}

// Implement trait untuk Token
impl Info for Token {
    fn tampilkan(&self) {
        println!("Token: {}", self.nama);
    }
    fn harga(&self) -> f64 {
        self.harga
    }
}

// Implement trait untuk NFT
impl Info for NFT {
    fn tampilkan(&self) {
        println!("NFT: {}", self.nama);
    }
    fn harga(&self) -> f64 {
        self.floor_price
    }
}

#[test]
fn materi_trait() {
    let eth = Token { nama: String::from("ETH"), harga: 3_450.75 };
    let nft = NFT { nama: String::from("DeGods"), floor_price: 500.0 };

    eth.tampilkan();
    nft.tampilkan();
    println!("Harga ETH: ${}", eth.harga());
    println!("Floor NFT: ${}", nft.harga());
}