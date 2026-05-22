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

/*
 * SOAL 32
 * Buat variable "is_mainnet": bool = false.
 * Jika mainnet, tampilkan "Running on Mainnet".
 * Jika tidak, tampilkan "Running on Testnet".
 * Clue: nama function -> fn soal_32(), wajib pakai attribute #[test]
 */
#[test]
fn soal_32() {
    let is_mainnet = false;

    if is_mainnet {
        println!("Running on Mainnet!");
    } else {
        println!("Running on Testnet!");
    }
}

/*
 * SOAL 33
 * Buat variable "confirmations": u32 = 3.
 * Buat variable "required_confirmations": u32 = 6.
 * Jika confirmations mencukupi, tampilkan "Transaksi Confirmed".
 * Jika tidak, tampilkan "Menunggu Konfirmasi".
 * Clue: nama function -> fn soal_33(), wajib pakai attribute #[test]
 */
#[test]
fn soal_33() {
    let confirmations = 3;
    let required_confirmations = 6;

    if confirmations >= required_confirmations {
        println!("Transaksi Confirmed!");
    } else {
        println!("Menunggu Konfirmasi!");
    }
}

/*
 * SOAL 34
 * Buat variable "token_price": f64 = 0.00045.
 * Buat variable "buy_price": f64 = 0.00030.
 * Jika harga sekarang lebih tinggi dari harga beli, tampilkan "Posisi Profit".
 * Jika tidak, tampilkan "Posisi Rugi".
 * Clue: nama function -> fn soal_34(), wajib pakai attribute #[test]
 */
#[test]
fn soal_34() {
    let token_price = 0.00045;
    let buy_price = 0.00030;

    if token_price > buy_price {
        println!("Posisi Profit!");
    } else {
        println!("Posisi Rugi");
    }
}

/*
 * SOAL 35
 * Buat variable "node_uptime": f64 = 98.5.
 * Jika uptime lebih dari atau sama dengan 99.0, tampilkan "Node Sehat".
 * Jika tidak, tampilkan "Node Perlu Perhatian".
 * Clue: nama function -> fn soal_35(), wajib pakai attribute #[test]
 */
#[test]
fn soal_35() {
    let node_uptime = 98.5;

    if node_uptime >= 99.0 {
        println!("Node Sehat!");
    } else {
        println!("Node Perlu Perhatian");
    }
}

/*
 * SOAL 36
 * Buat variable "max_supply": u64 = 21_000_000.
 * Buat variable "minted": u64 = 19_500_000.
 * Jika minted sudah melebihi 90% dari max supply, tampilkan "Supply Hampir Habis".
 * Jika tidak, tampilkan "Supply Masih Tersedia".
 * Clue: nama function -> fn soal_36(), wajib pakai attribute #[test]
 */
#[test]
fn soal_36() {
    let max_supply = 21_000_000;
    let minted = 19_500_000;
    let batas_persen_90 = max_supply as f64 * 0.90;

    if minted as f64 > batas_persen_90 {
        println!("Supply Hampir Habis");
    } else {
        println!("Supply Masih tersedia!");
    }
}

/*
 * SOAL 37
 * Buat variable "is_audited": bool = false.
 * Jika contract sudah diaudit, tampilkan "Contract Aman".
 * Jika tidak, tampilkan "WARNING: Contract Belum Diaudit!".
 * Clue: nama function -> fn soal_37(), wajib pakai attribute #[test]
 */
#[test]
fn soal_37() {
    let is_audited = false;

    if is_audited {
        println!("Contract Aman");
    } else {
        println!("Contract belum di audit!");
    }
}

/*
 * SOAL 38
 * Buat variable "tvl": f64 = 5_000_000.0.
 * Buat variable "min_tvl": f64 = 1_000_000.0.
 * Jika TVL mencukupi minimum, tampilkan "Protocol Sehat".
 * Jika tidak, tampilkan "TVL Terlalu Rendah".
 * Clue: nama function -> fn soal_38(), wajib pakai attribute #[test]
 */
#[test]
fn soal_38() {
    let tvl = 5_000_000.0;
    let min_tvl = 1_000_000.0;
    if min_tvl >= tvl {
        println!("Protocol sehat!");
    } else {
        println!("TVL Terlalu Murah");
    }
}

/*
 * SOAL 39
 * Buat variable "block_time": u32 = 400.
 * Jika block time di bawah 500ms, tampilkan "Jaringan Cepat".
 * Jika tidak, tampilkan "Jaringan Lambat".
 * Clue: nama function -> fn soal_39(), wajib pakai attribute #[test]
 */
#[test]
fn soal_39() {
    let block_time = 400;

    if block_time < 500 {
        println!("Jaringan Cepat");
    } else {
        println!("Jaringan Lambat");
    }
}

/*
 * SOAL 40
 * Buat variable "royalty_fee": f64 = 0.10.
 * Buat variable "sale_price": f64 = 5_000.0.
 * Hitung royalty yang didapat (sale_price * royalty_fee).
 * Jika royalty lebih dari 300.0, tampilkan "Royalty Besar".
 * Jika tidak, tampilkan "Royalty Kecil".
 * Clue: nama function -> fn soal_40(), wajib pakai attribute #[test]
 */
#[test]
fn soal_40() {
    let royalty_fee = 0.10;
    let sale_price = 5_000.0;

    let royalty = sale_price * royalty_fee;

    if royalty > 300.0 {
        println!("ROYALTY BESAR!");
    } else {
        println!("ROYALTY KECIL!");
    }
}

/*
 * SOAL 41
 * Buat variable "hash_rate": u64 = 150_000_000.
 * Buat variable "min_hash_rate": u64 = 100_000_000.
 * Jika hash rate mencukupi minimum, tampilkan "Mining Aktif".
 * Jika tidak, tampilkan "Hash Rate Tidak Cukup".
 * Clue: nama function -> fn soal_41(), wajib pakai attribute #[test]
 */
#[test]
fn soal_41() {
    let hash_rate = 150_000_000;
    let min_hash_rate = 100_000_000;

    if hash_rate >= min_hash_rate {
        println!("MINING AKTIF");
    } else {
        println!("HASH RATE TIDAK CUKUP");
    }
}

/*
 * SOAL 42
 * Buat variable "is_staking": bool = true.
 * Jika sedang staking, tampilkan "Reward Berjalan".
 * Jika tidak, tampilkan "Tidak Ada Reward".
 * Clue: nama function -> fn soal_42(), wajib pakai attribute #[test]
 */
#[test]
fn soal_42() {
    let is_staking = true;

    if is_staking {
        println!("REWARD BERJALAN!");
    } else {
        println!("TIDAK ADA REWARD");
    }
}

/*
 * SOAL 43
 * Buat variable "pending_tx": u32 = 150.
 * Buat variable "max_pending": u32 = 100.
 * Jika pending tx melebihi maksimum, tampilkan "Mempool Penuh".
 * Jika tidak, tampilkan "Mempool Normal".
 * Clue: nama function -> fn soal_43(), wajib pakai attribute #[test]
 */
#[test]
fn soal_43() {
    let pending_tx = 150;
    let max_pending = 100;

    if pending_tx > max_pending {
        println!("MEMPOOL PENUH");
    } else {
        println!("MEMPOOL NORMAL");
    }
}

/*
 * SOAL 44
 * Buat variable "apy": f64 = 8.5.
 * Buat variable "min_apy": f64 = 5.0.
 * Jika APY mencukupi minimum, tampilkan "Pool Menarik".
 * Jika tidak, tampilkan "Pool Kurang Menarik".
 * Clue: nama function -> fn soal_44(), wajib pakai attribute #[test]
 */
#[test]
fn soal_44() {
    let apy = 8.5;
    let min_apy = 5.0;

    if apy > min_apy {
        println!("POOL MENARIK!");
    } else {
        println!("POOL KURANG MENARIK");
    }
}

/*
 * SOAL 45
 * Buat variable "is_kyc": bool = false.
 * Jika KYC sudah selesai, tampilkan "Akun Full Access".
 * Jika tidak, tampilkan "Akun Terbatas, Selesaikan KYC".
 * Clue: nama function -> fn soal_45(), wajib pakai attribute #[test]
 */
#[test]
fn soal_45() {
    let is_kyc = false;

    if is_kyc {
        println!("Akun Full Akses");
    } else {
        println!("Akun Terbatas");
    }
}

/*
 * SOAL 46
 * Buat variable "locked_amount": u64 = 10_000.
 * Buat variable "unlock_amount": u64 = 10_000.
 * Jika locked amount sama dengan unlock amount, tampilkan "Siap Unlock".
 * Jika tidak, tampilkan "Belum Bisa Unlock".
 * Clue: nama function -> fn soal_46(), wajib pakai attribute #[test]
 */
#[test]
fn soal_46() {
    let locked_amount = 10_000;
    let unlock_amount = 10_000;

    if locked_amount == unlock_amount {
        println!("SIAP UNLOCK!");
    } else {
        println!("BELUM BISA UNCLOCK!");
    }
}

/*
 * SOAL 47
 * Buat variable "fee_persen": f64 = 0.03.
 * Buat variable "amount": f64 = 10_000.0.
 * Hitung fee (amount * fee_persen).
 * Jika fee lebih dari 200.0, tampilkan "Fee Mahal".
 * Jika tidak, tampilkan "Fee Wajar".
 * Clue: nama function -> fn soal_47(), wajib pakai attribute #[test]
 */
#[test]
fn soal_47() {
    let fee_persen = 0.03;
    let amount = 10_000.0;

    let fee = amount * fee_persen;
    if fee > 200.0 {
        println!("FEE MAHAL!");
    } else {
        println!("FEE WAJAR");
    }
}

/*
 * SOAL 48
 * Buat variable "epoch": u64 = 450.
 * Buat variable "max_epoch": u64 = 500.
 * Jika epoch sudah melebihi 90% dari max epoch, tampilkan "Epoch Hampir Selesai".
 * Jika tidak, tampilkan "Epoch Masih Berjalan".
 * Clue: nama function -> fn soal_48(), wajib pakai attribute #[test]
 */
#[test]
fn soal_48() {
    let epoch = 450;
    let max_epoch = 500;
    let batas_90_persen = epoch as f64 * 0.90;

    if batas_90_persen > max_epoch as f64 {
        println!("EPOCH HAMPIR SELESAI");
    } else {
        println!("EPOCH MASIH BERJALAN");
    }
}

/*
 * SOAL 49
 * Buat variable "is_blacklisted": bool = true.
 * Jika address masuk blacklist, tampilkan "Transaksi Diblokir".
 * Jika tidak, tampilkan "Transaksi Diizinkan".
 * Clue: nama function -> fn soal_49(), wajib pakai attribute #[test]
 */
#[test]
fn soal_49() {
    let is_blacklisted = true;

    if is_blacklisted {
        println!("TRANSAKSI DIBLOKIR");
    } else {
        println!("TRANSAKSI DIIZINKAN");
    }
}

/*
 * SOAL 50
 * Buat variable "nft_floor_price": f64 = 2.5.
 * Buat variable "offer_price": f64 = 2.5.
 * Jika offer sama dengan atau lebih dari floor price, tampilkan "Offer Diterima".
 * Jika tidak, tampilkan "Offer Dibawah Floor Price".
 * Clue: nama function -> fn soal_50(), wajib pakai attribute #[test]
 */
#[test]
fn soal_50() {
    let nft_floor_price = 2.5;
    let offer_price = 2.5;

    if offer_price == nft_floor_price || offer_price >= nft_floor_price {
        println!("OFFER DITERIMA!");
    } else {
        println!("OFFER DIBAWAH FLOOR PRICE");
    }
}

/*
 * SOAL 51
 * Buat variable "total_voter": u32 = 1_000.
 * Buat variable "quorum": u32 = 500.
 * Jika total voter mencapai quorum, tampilkan "Voting Valid".
 * Jika tidak, tampilkan "Quorum Belum Tercapai".
 * Clue: nama function -> fn soal_51(), wajib pakai attribute #[test]
 */
#[test]
fn soal_51() {
    let total_voter = 1_000;
    let quorum = 500;

    if total_voter > quorum {
        println!("VOTING VALID!");
    } else {
        println!("QUORUM BELUM TERCAPAI!");
    }
}

/*
 * SOAL 52
 * Buat variable "gas_used": u64 = 21_000.
 * Buat variable "gas_limit": u64 = 30_000.
 * Jika gas used melebihi gas limit, tampilkan "Out of Gas".
 * Jika tidak, tampilkan "Gas Cukup".
 * Clue: nama function -> fn soal_52(), wajib pakai attribute #[test]
 */
#[test]
fn soal_52() {
    let gas_used = 21_000;
    let gas_limit = 30_000;

    if gas_used > gas_limit {
        println!("OUT OF GAS!");
    } else {
        println!("GAS CUKUP!");
    }
}

/*
 * SOAL 53
 * Buat variable "price_change": f64 = -5.2.
 * Jika price change lebih dari 0, tampilkan "Harga Naik".
 * Jika tidak, tampilkan "Harga Turun".
 * Clue: nama function -> fn soal_53(), wajib pakai attribute #[test]
 */
#[test]
fn soal_53() {
    let price_change = -5.2;
    let convert_to_integer = price_change as i32;

    if convert_to_integer > 0 {
        println!("HARGA NAIK");
    } else {
        println!("HARGA TURUN");
    }
}

/*
 * SOAL 54
 * Buat variable "staking_period": u32 = 90.
 * Buat variable "min_period": u32 = 30.
 * Hitung apakah staking period sudah 3x lipat dari minimum.
 * Jika iya, tampilkan "Bonus Reward Aktif".
 * Jika tidak, tampilkan "Reward Normal".
 * Clue: nama function -> fn soal_54(), wajib pakai attribute #[test]
 */
#[test]
fn soal_54() {
    let staking_period = 90;
    let min_period = 30;

    let tiga_kali_lipat = min_period * 3;
    if staking_period >= tiga_kali_lipat {
        println!("BONUS REWARD AKTIF");
    } else {
        println!("REWARD NORMAL");
    }
}

/*
 * SOAL 55
 * Buat variable "is_open_source": bool = true.
 * Jika contract open source, tampilkan "Kode Bisa Diverifikasi".
 * Jika tidak, tampilkan "Kode Tidak Transparan".
 * Clue: nama function -> fn soal_55(), wajib pakai attribute #[test]
 */
#[test]
fn soal_55() {
    let is_open_source = true;

    if is_open_source {
        println!("KODE BISA DIVERIFIKASI");
    } else {
        println!("KODE TIDAK TRANSPARAN");
    }
}
