// Function generic
fn terbesar<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

// Struct generic
struct Wallet<T> {
    address: String,
    balance: T,
}

#[test]
fn materi_generics() {
    // Function generic dengan berbagai tipe
    println!("{}", terbesar(10, 20));        // i32
    println!("{}", terbesar(3.14, 2.71));    // f64

    // Struct generic
    let wallet_sol = Wallet {
        address: String::from("ABC123"),
        balance: 150.5_f64,
    };

    let wallet_token = Wallet {
        address: String::from("DEF456"),
        balance: 1_000_u64,
    };

    println!("SOL Balance: {}", wallet_sol.balance);
    println!("Token Balance: {}", wallet_token.balance);
}