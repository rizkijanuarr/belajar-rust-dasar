/*
    Collections adalah tipe data yang bisa menyimpan banyak nilai sekaligus. Di Rust ada 3 yang paling penting: Vec, HashMap, dan HashSet.
    Kenapa penting:
    - Wajib buat nyimpen & manipulasi banyak data
    - Dasar dari semua project real-world
 */

use std::collections::{HashMap, HashSet};

#[test]
fn materi_vec() {
    // Buat Vec
    let mut tokens: Vec<String> = Vec::new();

    // Tambah data
    tokens.push(String::from("ETH"));
    tokens.push(String::from("SOL"));
    tokens.push(String::from("BTC"));

    // Akses data
    println!("Token pertama: {}", tokens[0]);

    // Iterasi
    for token in &tokens {
        println!("Token: {}", token);
    }

    // Panjang Vec
    println!("Total token: {}", tokens.len());

    // Hapus data terakhir
    tokens.pop();
    println!("Setelah pop: {:?}", tokens);
}

#[test]
fn materi_hashmap() {
    let mut harga: HashMap<String, f64> = HashMap::new();

    // Insert data
    harga.insert(String::from("ETH"), 3_450.75);
    harga.insert(String::from("SOL"), 180.50);
    harga.insert(String::from("BTC"), 96_000.0);

    // Akses data
    if let Some(eth) = harga.get("ETH") {
        println!("Harga ETH: ${}", eth);
    }

    // Iterasi
    for (token, price) in &harga {
        println!("{}: ${}", token, price);
    }

    // Cek apakah key ada
    if harga.contains_key("SOL") {
        println!("SOL ada di portfolio!");
    }

    // Hapus data
    harga.remove("BTC");
    println!("Total token: {}", harga.len());
}

#[test]
fn materi_hashset() {
    let mut whitelist: HashSet<String> = HashSet::new();

    // Insert data
    whitelist.insert(String::from("wallet_A"));
    whitelist.insert(String::from("wallet_B"));
    whitelist.insert(String::from("wallet_A")); // duplikat → diabaikan!

    // Cek apakah ada
    if whitelist.contains("wallet_A") {
        println!("Wallet A ada di whitelist!");
    }

    println!("Total whitelist: {}", whitelist.len()); // output: 2 bukan 3
}