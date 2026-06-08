/*
    Lifetime adalah cara Rust memastikan referensi selalu valid — tidak ada dangling reference (referensi ke data yang sudah dihapus).
    Kenapa penting:
    - Compiler butuh tahu seberapa lama referensi hidup
    - Mencegah bug memori yang berbahaya
    - Wajib di security — dangling pointer adalah celah exploit
    - Sering muncul saat function return referensi
 */


// Tanpa lifetime annotation — compiler bingung
// fn terpanjang(s1: &str, s2: &str) -> &str { ... } ❌

// Dengan lifetime annotation 'a
fn terpanjang<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}

// Struct dengan lifetime
struct Transaksi<'a> {
    hash: &'a str,
    amount: f64,
}

#[test]
fn materi_lifetime() {
    let s1 = String::from("Ethereum");
    let hasil;

    {
        let s2 = String::from("SOL");
        hasil = terpanjang(&s1, &s2);
        println!("Terpanjang: {}", hasil);
    }

    // Struct dengan lifetime
    let hash = String::from("0xABC123DEF456");
    let tx = Transaksi {
        hash: &hash,
        amount: 1.5,
    };
    println!("TX Hash: {}, Amount: {}", tx.hash, tx.amount);
}