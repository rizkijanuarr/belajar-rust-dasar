#[test]
fn materi_option() {
    // Option = Some(nilai) atau None
    let harga: Option<f64> = Some(3_450.75);
    let kosong: Option<f64> = None;

    // Handle dengan match
    match harga {
        Some(h) => println!("Harga ETH: ${}", h),
        None    => println!("Harga tidak tersedia"),
    }

    // Handle dengan if let (lebih ringkas)
    if let Some(h) = harga {
        println!("Harga: ${}", h);
    }

    // Contoh nyata — ambil data dari HashMap
    use std::collections::HashMap;
    let mut portfolio: HashMap<String, f64> = HashMap::new();
    portfolio.insert(String::from("ETH"), 3_450.75);

    // .get() return Option
    match portfolio.get("ETH") {
        Some(harga) => println!("ETH: ${}", harga),
        None        => println!("Token tidak ada"),
    }
}

// Function yang bisa gagal return Result
fn bagi(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Tidak bisa bagi dengan 0!"))
    } else {
        Ok(a / b)
    }
}

#[test]
fn materi_result() {
    // Handle dengan match
    match bagi(10.0, 2.0) {
        Ok(hasil)  => println!("Hasil: {}", hasil),
        Err(pesan) => println!("Error: {}", pesan),
    }

    match bagi(10.0, 0.0) {
        Ok(hasil)  => println!("Hasil: {}", hasil),
        Err(pesan) => println!("Error: {}", pesan),
    }
}

fn hitung_portfolio(harga: f64, amount: f64) -> Result<f64, String> {
    let total = bagi(harga, amount)?; // kalau error, langsung return error
    Ok(total * 1.1)
}

#[test]
fn materi_operator_tanya() {
    match hitung_portfolio(3_450.75, 2.5) {
        Ok(hasil)  => println!("Total: ${}", hasil),
        Err(pesan) => println!("Error: {}", pesan),
    }
}