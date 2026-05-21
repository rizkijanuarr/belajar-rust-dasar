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
