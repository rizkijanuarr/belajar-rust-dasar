// SOAL 1
// Buat variable "nilai": i32 = 75.
// Jika nilai lebih dari atau sama dengan 60, tampilkan "Lulus".
// Jika tidak, tampilkan "Tidak Lulus".
// Clue: nama function -> fn soal_1(), wajib pakai attribute #[test]]
#[test]
fn soal_1() {
    let nilai = 75;

    if nilai >= 60 {
        println!("LULUS!");
    } else {
        println!("TIDAK LULUS!");
    }
}

// SOAL 2
// Buat variable "saldo": i32 = 500_000.
// Buat variable "harga_barang": i32 = 350_000.
// Jika saldo cukup untuk membeli barang, tampilkan "Pembelian Berhasil".
// Jika tidak, tampilkan "Saldo Tidak Cukup".
// Clue: nama function -> fn soal_2(), wajib pakai attribute #[test]
#[test]
fn soal_2() {
    let saldo = 500_000;
    let harga_barang = 350_000;

    if saldo > harga_barang {
        println!("SALDO CUKUP");
    } else {
        println!("SALDO TIDAK CUKUP");
    }
}

// SOAL 3
// Buat variable "umur": u8 = 17.
// Jika umur lebih dari atau sama dengan 18, tampilkan "Boleh Masuk".
// Jika tidak, tampilkan "Dibawah Umur, Tidak Boleh Masuk".
// Clue: nama function -> fn soal_3(), wajib pakai attribute #[test]
#[test]
fn soal_3() {
    let umur = 17;

    if umur >= 18 {
        println!("BOLEH MASUK");
    } else {
        println!("dibawah umur!")
    }
}

// SOAL 1
// Buat variable "suhu": i32 = 38.
// Jika suhu di bawah 20, tampilkan "Dingin".
// Jika suhu antara 20-35, tampilkan "Normal".
// Jika suhu di atas 35, tampilkan "Panas".
// Clue: nama function -> fn soal_1(), wajib pakai attribute #[test]
#[test]
fn soal_4() {
    let suhu = 38;

    if suhu < 20 {
        println!("DINGIN");
    } else if suhu >= 20 && suhu <= 35 {
        println!("NORMAL");
    } else if suhu > 35 {
        println!("PANAS");
    }
}

#[test]
fn soal_4_range() {
    let suhu = 38;

    if suhu < 20 {
        println!("DINGIN");
    } else if (20..=35).contains(&suhu) {
        println!("NORMAL");
    } else if suhu > 35 {
        println!("PANAS");
    }
}

// SOAL 2
// Buat variable "saldo": i64 = 2_500_000.
// Jika saldo di atas 5_000_000, tampilkan "Platinum Member".
// Jika saldo di atas 2_000_000, tampilkan "Gold Member".
// Jika saldo di atas 500_000, tampilkan "Silver Member".
// Jika tidak, tampilkan "Regular Member".
// Clue: nama function -> fn soal_2(), wajib pakai attribute #[test]
#[test]
fn soal_5() {
    let saldo = 2_500_000;

    if saldo > 5_000_000 {
        println!("PLATINUM MEMBER");
    } else if saldo > 2_000_000 {
        println!("GOLD MEMBER");
    } else if saldo > 500_000 {
        println!("SILVER MEMBER");
    } else {
        println!("REGULER MEMBER");
    }
}

// SOAL 3
// Buat variable "umur": u8 = 25.
// Jika umur di bawah 13, tampilkan "Anak-anak".
// Jika umur di bawah 18, tampilkan "Remaja".
// Jika umur di bawah 60, tampilkan "Dewasa".
// Jika tidak, tampilkan "Lansia".
// Clue: nama function -> fn soal_3(), wajib pakai attribute #[test]
#[test]
fn soal_6() {
    let umur = 25;

    if umur < 13 {
        println!("anak-anak");
    } else if umur < 18 {
        println!("remaja");
    } else if umur < 60 {
        println!("dewasa");
    } else {
        println!("lansia");
    }
}

// SOAL 7
// Buat variable "password": &str = "rust123".
// Jika panjang password lebih dari atau sama dengan 8 karakter, tampilkan "Password Kuat".
// Jika tidak, tampilkan "Password Lemah".
// Clue: nama function -> fn soal_4(), wajib pakai attribute #[test]
#[test]
fn soal_7() {
    let password = "rust123";

    if password.len() >= 8 {
        println!("Password Kuat");
    } else {
        println!("Password Lemah");
    }
}

// SOAL 8
// Buat variable "suhu": f64 = 38.5.
// Jika suhu di atas 37.5, tampilkan "Demam".
// Jika tidak, tampilkan "Normal".
// Clue: nama function -> fn soal_5(), wajib pakai attribute #[test]
#[test]
fn soal_8() {
    let suhu = 38.5;

    if suhu > 37.5 {
        println!("Demam");
    } else {
        println!("Normal");
    }
}

// SOAL 9
// Buat variable "eth_price": f64 = 3_450.75.
// Buat variable "budget": f64 = 5_000.0.
// Jika budget cukup untuk beli 1 ETH, tampilkan "Bisa Beli ETH".
// Jika tidak, tampilkan "Budget Tidak Cukup".
// Clue: nama function -> fn soal_6(), wajib pakai attribute #[test]
#[test]
fn soal_9() {
    let eth_price = 3_450.75;
    let budget = 5_000.0;

    if eth_price < budget {
        println!("Bisa Beli ETH");
    } else {
        println!("Budget Tidak Cukup");
    }
}

// SOAL 10
// Buat variable "stok": i32 = 0.
// Jika stok lebih dari 0, tampilkan "Stok Tersedia".
// Jika tidak, tampilkan "Stok Habis".
// Clue: nama function -> fn soal_10(), wajib pakai attribute #[test]
#[test]
fn soal_10() {
    let stok = 0;

    if stok > 0 {
        println!("Stok Tersedia");
    } else {
        println!("Stok Habis");
    }
}

// SOAL 11
// Buat variable "nilai_transfer": i64 = 150_000.
// Buat variable "limit_transfer": i64 = 100_000.
// Jika nilai transfer melebihi limit, tampilkan "Transfer Ditolak".
// Jika tidak, tampilkan "Transfer Berhasil".
// Clue: nama function -> fn soal_11(), wajib pakai attribute #[test]
#[test]
fn soal_11() {
    let nilai_transfer = 150_000;
    let limit_transfer = 100_000;

    if nilai_transfer > limit_transfer {
        println!("Transfer Ditolak!");
    } else {
        println!("Transfer Berhasil");
    }
}

// SOAL 12
// Buat variable "username": &str = "rizki".
// Jika panjang username kurang dari 5 karakter, tampilkan "Username Terlalu Pendek".
// Jika tidak, tampilkan "Username Valid".
// Clue: nama function -> fn soal_12(), wajib pakai attribute #[test]
#[test]
fn soal_12() {
    let username = "rizki";

    if username.len() < 5 {
        println!("Username Terlalu Pendek");
    } else {
        println!("Username Valid!");
    }
}

// SOAL 13
// Buat variable "is_verified": bool = false.
// Jika verified, tampilkan "Akun Terverifikasi".
// Jika tidak, tampilkan "Akun Belum Diverifikasi".
// Clue: nama function -> fn soal_13(), wajib pakai attribute #[test]
#[test]
fn soal_13() {
    let is_verified = false;

    if is_verified == true {
        println!("Akun Terverifikasi!");
    } else {
        println!("Akun Belum Terverifikasi!");
    }
}

// SOAL 14
// Buat variable "sol_price": f64 = 180.50.
// Buat variable "modal": f64 = 150.0.
// Jika harga sekarang lebih tinggi dari modal, tampilkan "Profit".
// Jika tidak, tampilkan "Rugi".
// Clue: nama function -> fn soal_14(), wajib pakai attribute #[test]
#[test]
fn soal_14() {
    let sol_price = 180.50;
    let modal = 150.0;

    if sol_price > modal {
        println!("Profit!!!");
    } else {
        println!("RUGI!!");
    }
}

// SOAL 15
// Buat variable "panjang_pin": usize = 4.
// Jika panjang pin tepat 6 karakter, tampilkan "PIN Valid".
// Jika tidak, tampilkan "PIN Harus 6 Digit".
// Clue: nama function -> fn soal_15(), wajib pakai attribute #[test]
#[test]
fn soal_15() {
    let panjang_pin = 4;

    if panjang_pin == 6 {
        println!("PIN VALID!");
    } else {
        println!("PIN HARUS 6 DIGIT!");
    }
}

// SOAL 16
// Buat variable "koneksi": bool = false.
// Jika koneksi true, tampilkan "Server Online".
// Jika tidak, tampilkan "Server Offline".
// Clue: nama function -> fn soal_16(), wajib pakai attribute #[test]
#[test]
fn soal_16() {
    let koneksi = false;

    if koneksi {
        print!("Server Online!");
    } else {
        print!("Server Offline!");
    }
}

// SOAL 17
// Buat variable "attempts": i32 = 3.
// Jika attempts lebih dari 0, tampilkan "Silakan Login".
// Jika tidak, tampilkan "Akun Terkunci".
// Clue: nama function -> fn soal_17(), wajib pakai attribute #[test]
#[test]
fn soal_17() {
    let attempts = 3;

    if attempts > 0 {
        println!("Silahkan Login!");
    } else {
        println!("Akun Terkunci!");
    }
}

// SOAL 18
// Buat variable "gas_fee": f64 = 0.005.
// Buat variable "saldo_sol": f64 = 0.003.
// Jika saldo cukup untuk bayar gas fee, tampilkan "Transaksi Bisa Diproses".
// Jika tidak, tampilkan "Saldo SOL Tidak Cukup".
// Clue: nama function -> fn soal_18(), wajib pakai attribute #[test]
#[test]
fn soal_18() {
    let gas_fee = 0.005;
    let saldo_sol = 0.003;

    if saldo_sol > gas_fee {
        println!("Transaksi Bisa Diproses!");
    } else {
        println!("Saldo SOL tidak cukup!");
    }
}

// SOAL 19
// Buat variable "token_balance": u64 = 0.
// Jika balance lebih dari 0, tampilkan "Ada Token".
// Jika tidak, tampilkan "Wallet Kosong".
// Clue: nama function -> fn soal_19(), wajib pakai attribute #[test]
#[test]
fn soal_19() {
    let token_balance = 0;

    if token_balance > 0 {
        println!("ADA TOKEN!");
    } else {
        println!("WALLET KOSONG!");
    }
}

// SOAL 20
// Buat variable "umur_wallet": u32 = 30.
// Jika wallet berumur lebih dari 365 hari, tampilkan "Wallet Lama".
// Jika tidak, tampilkan "Wallet Baru".
// Clue: nama function -> fn soal_20(), wajib pakai attribute #[test]
#[test]
fn soal_20() {
    let umur_wallet = 30;

    if umur_wallet > 365 {
        println!("WALLET LAMA!");
    } else {
        println!("WALLET BARU!");
    }
}

// SOAL 21
// Buat variable "harga_nft": f64 = 2_500.0.
// Buat variable "offer": f64 = 2_000.0.
// Jika offer lebih besar atau sama dengan harga, tampilkan "Deal!".
// Jika tidak, tampilkan "Offer Ditolak".
// Clue: nama function -> fn soal_21(), wajib pakai attribute #[test]
#[test]
fn soal_21() {
    let harga_nft = 2_500.0;
    let offer = 2_000.0;

    if offer >= harga_nft {
        println!("DEAL!");
    } else {
        println!("OFFER DITOLAK!");
    }
}

/*
 * SOAL 22
 * Buat variable "is_whitelist": bool = true.
 * Jika masuk whitelist, tampilkan "Akses Mint NFT Diberikan".
 * Jika tidak, tampilkan "Kamu Tidak Ada di Whitelist".
 * Clue: nama function -> fn soal_22(), wajib pakai attribute #[test]
 */
#[test]
fn soal_22() {
    let is_whitelist = true;

    if is_whitelist {
        println!("Akses MINT NFT DIBERIKAN!");
    } else {
        println!("KAMU TIDAK ADA DI WHITELIST!");
    }
}

/*
 * SOAL 23
 * Buat variable "tx_size": u32 = 1200.
 * Jika ukuran transaksi lebih dari 1000 bytes, tampilkan "Transaksi Terlalu Besar".
 * Jika tidak, tampilkan "Ukuran Transaksi Valid".
 * Clue: nama function -> fn soal_23(), wajib pakai attribute #[test]
 */
#[test]
fn soal_23() {
    let tx_size = 1200;

    if tx_size > 1000 {
        println!("Transaksi Terlalu Besar!");
    } else {
        println!("Ukuran Transaksi Valid");
    }
}

/*
 * SOAL 24
 * Buat variable "apr": f64 = 12.5.
 * Jika APR lebih dari 10.0, tampilkan "High Yield".
 * Jika tidak, tampilkan "Low Yield".
 * Clue: nama function -> fn soal_24(), wajib pakai attribute #[test]
 */
#[test]
fn soal_24() {
    let apr = 12.5;

    if apr > 10.0 {
        println!("High Yield!");
    } else {
        println!("Low Yield!");
    }
}

/*
 * SOAL 25
 * Buat variable "block_number": u64 = 1_000_000.
 * Jika block number lebih dari 500_000, tampilkan "Block Sudah Lama".
 * Jika tidak, tampilkan "Block Masih Baru".
 * Clue: nama function -> fn soal_25(), wajib pakai attribute #[test]
 */
#[test]
fn soal_25() {
    let block_number = 1_000_000;

    if block_number > 500_000 {
        println!("Block Sudah Lama");
    } else {
        println!("Block Masih Baru!");
    }
}

/*
 * SOAL 26
 * Buat variable "liquidity": f64 = 50_000.0.
 * Buat variable "min_liquidity": f64 = 100_000.0.
 * Jika liquidity mencukupi minimum, tampilkan "Pool Aktif".
 * Jika tidak, tampilkan "Liquidity Tidak Cukup".
 * Clue: nama function -> fn soal_26(), wajib pakai attribute #[test]
 */
#[test]
fn soal_26() {
    let liquidity = 50_000.0;
    let min_liquidity = 100_000.0;

    if liquidity >= min_liquidity {
        println!("POOL AKTIF!");
    } else {
        println!("LIQUIDITY TIDAK CUKUP!");
    }
}

/*
 * SOAL 27
 * Buat variable "is_contract_paused": bool = true.
 * Jika contract sedang pause, tampilkan "Contract Sedang Dihentikan".
 * Jika tidak, tampilkan "Contract Berjalan Normal".
 * Clue: nama function -> fn soal_27(), wajib pakai attribute #[test]
 */
#[test]
fn soal_27() {
    let is_contract_paused = true;

    if is_contract_paused {
        println!("CONTRACT SEDANG DIHENTIKAN!");
    } else {
        println!("CONTRACT BERJALAN NORMAL");
    }
}

/*
 * SOAL 28
 * Buat variable "wallet_age": u32 = 7.
 * Buat variable "min_age": u32 = 30.
 * Jika wallet age mencukupi minimum, tampilkan "Wallet Eligible".
 * Jika tidak, tampilkan "Wallet Belum Eligible".
 * Clue: nama function -> fn soal_28(), wajib pakai attribute #[test]
 */
#[test]
fn soal_28() {
    let wallet_age = 7;
    let min_age = 30;

    if wallet_age >= min_age {
        println!("WALLET ELIGIBLE");
    } else {
        println!("WALLET BELUM ELIGIBLE");
    }
}

/*
 * SOAL 29
 * Buat variable "total_supply": u64 = 1_000_000_000.
 * Buat variable "circulating": u64 = 800_000_000.
 * Jika circulating lebih dari 50% total supply, tampilkan "Mayoritas Token Beredar".
 * Jika tidak, tampilkan "Token Masih Banyak Terkunci".
 * Clue: nama function -> fn soal_29(), wajib pakai attribute #[test]
 */
#[test]
fn soal_29() {
    let total_supply = 1_000_000_000;
    let circulating = 800_000_000;
    let setengah_supply = total_supply / 2;

    if circulating > setengah_supply {
        println!("Mayoritas Token Beredar");
    } else {
        println!("Token masih banyak terkunci!");
    }
}

/*
 * SOAL 30
 * Buat variable "slippage": f64 = 0.8.
 * Buat variable "max_slippage": f64 = 0.5.
 * Jika slippage melebihi maksimum, tampilkan "Transaksi Gagal - Slippage Terlalu Tinggi".
 * Jika tidak, tampilkan "Slippage Aman".
 * Clue: nama function -> fn soal_30(), wajib pakai attribute #[test]
 */
#[test]
fn soal_30() {
    let slippage = 0.8;
    let max_slippage = 0.5;

    if slippage > max_slippage {
        println!("TRANSAKSI GAGAL - SLIPPAGE TERLALU TINGGI");
    } else {
        println!("SLIPPAGE AMAN!");
    }
}

/*
 * SOAL 31
 * Buat variable "validator_stake": u64 = 32_000.
 * Buat variable "min_stake": u64 = 32_000.
 * Jika stake mencukupi minimum, tampilkan "Validator Aktif".
 * Jika tidak, tampilkan "Stake Tidak Cukup".
 * Clue: nama function -> fn soal_31(), wajib pakai attribute #[test]
 */
#[test]
fn soal_31() {
    let validator_stake = 32_000;
    let min_stake = 32_000;

    if validator_stake >= min_stake {
        println!("VALIDATOR AKTIF");
    } else {
        println!("STAKE TIDAK CUKUP!");
    }
}
