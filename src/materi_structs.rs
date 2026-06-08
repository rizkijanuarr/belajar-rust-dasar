// Definisi struct
struct Token {
    nama: String,
    harga: f64,
    supply: u64,
}

// Tambah method dengan impl
impl Token {
    fn info(&self) {
        println!("Token: {}, Harga: ${}, Supply: {}",
                 self.nama, self.harga, self.supply);
    }

    fn market_cap(&self) -> f64 {
        self.harga * self.supply as f64
    }
}

#[test]
fn materi_struct() {
    let eth = Token {
        nama: String::from("Ethereum"),
        harga: 3_450.75,
        supply: 120_000_000,
    };

    eth.info();
    println!("Market Cap: ${}", eth.market_cap());
}