mod materi_for;
mod materi_if_else;
mod materi_match;
mod materi_variable;
mod materi_function;

fn main() {
    println!("Hello, world!");
}

#[test]
fn materi_variable() {
    // 1. Imutable = Variable yang tidak dapat dirubah!
    let name = "Rizki Januar I.";
    println!("{}", name);

    // 2. Mutable = Variable yang dapat dirubah!
    let mut age = 20;
    println!("{}", age);

    age = 24;
    println!("{}", age);
}

#[test]
fn materi_comment() {
    // 1. Ini namanya Single Line Comment (pake dua garis miring)
    // Biasanya buat catatan pendek di atas baris kode.

    let skor = 100; // Bisa juga diletakan di samping kode kayak gini

    /* 2. Ini namanya Multi-line Comment.
       Bisa buat catatan yang panjang lebar
       sampai berbaris-baris.
    */

    println!("Cek terminal, komentar nggak bakal muncul di sini!");
    println!("Skor gue: {skor}");
}

#[test]
fn materi_data_types_scalar_integer() {
    let a: i8 = -4;
    println!("{}", a);

    let b: u32 = 1000;
    println!("{}", b);

    let c: usize = 8;
    println!("{:?}", c);
}

#[test]
fn materi_data_types_scalar_konversi_tipe_number_data() {
    // Konversi kecil ke besar
    let a: u8 = 10;
    let b: u32 = a as u32; // Konversi dari u8 ke u32
    println!("Nilai : {b}");

    // Konversi besar ke kecil
    let c: i32 = 256;
    let d: i8 = c as i8;
    println!("Nilai : {d}");
}

#[test]
fn materi_data_types_scalar_float() {
    let pi = 3.14;
    println!("Nilai : {pi}");
}

#[test]
fn materi_data_types_scalar_boolean() {
    // 1. Deklarasi eksplisit
    let is_rust_hard: bool = false;

    // 2. Deklarasi otomatis (Type Inference)
    let is_learning = true;

    // 3. Boolean dari hasil perbandingan
    let angka = 10;
    let apakah_lebih_besar = angka > 5; // Isinya bakal true

    println!("--- BOOLEAN ---");
    println!("Apakah Rust susah? {is_rust_hard}");
    println!("Lagi belajar? {is_learning}");
    println!("Apakah 10 > 5? {apakah_lebih_besar}");
}

#[test]
fn materi_data_types_scalar_char() {
    // 1. Karakter biasa
    let huruf = 'R';

    // 2. Karakter angka (tapi tipenya char, bukan integer)
    let angka_char = '7';

    // 3. Karakter Spesial / Emoji
    let emoji = '🚀';
    let mandarin = '学';

    println!("--- CHARACTER ---");
    println!("Huruf: {huruf}");
    println!("Angka (Char): {angka_char}");
    println!("Emoji: {emoji}");
    println!("Karakter Mandarin: {mandarin}");
}

#[test]
fn materi_data_types_compound_membuat_tuple() {
    let personal = ("Rizki", 24, true);
    println!("{:?}", personal);
}

#[test]
fn materi_data_types_compound_mengakses_tuple() {
    let produk = ("Kopi Susu", 15000, true);

    let nama_produk = produk.0;
    let harga = produk.1;

    println!("--- AKSES PAKE INDEX ---");
    println!("Produk: {nama_produk}, Harga: {harga}");
}

#[test]
fn materi_data_types_compound_destructuring_tuple() {
    let koordinat = (10.5, 20.0, 5.2);

    let (x, y, z) = koordinat;

    println!("--- HASIL DESTRUCTURING ---");
    println!("Titik X: {x}");
    println!("Titik Y: {y}");
    println!("Titik Z: {z}");

    // Ignored Values (Kalau cuma mau ambil X tapi sisanya dibuang/dicuekin)
    let (titik_awal, _, _) = koordinat;
    println!("--- CUMA AMBIL SATU NILAI ---");
    println!("Titik Awal: {titik_awal}");
}

#[test]
fn materi_tuple_mutable() {
    // 1. Deklarasi Tuple yang mutable pake 'mut'
    let mut status_mesin = ("OFF", 0);

    println!("--- SEBELUM DIUBAH ---");
    println!("Status: {}, Suhu: {}", status_mesin.0, status_mesin.1);

    // 2. Update nilai pake index (titik)
    status_mesin.0 = "ON";
    status_mesin.1 = 85;

    println!("--- SESUDAH DIUBAH ---");
    println!("Status: {}, Suhu: {}", status_mesin.0, status_mesin.1);

    // 3. Inget: Tipe data gak boleh diganti!
    // status_mesin.1 = "Panas"; // Ini bakal ERROR karena index .1 harus integer
}

#[test]
fn materi_tuple_unit() {
    // 1. Membuat Unit Value
    let unit_kosong = ();

    println!("--- UNIT TYPE ---");
    // Karena isinya kosong, kita pake {:?} (Debug mode) buat nampilinnya
    println!("Isi dari unit_kosong: {:?}", unit_kosong);

    // 2. Unit biasanya muncul dari fungsi yang gak balikin apa-apa
    fn fungsi_gak_jelas() {
        println!("Lagi lari kodenya...");
    }

    let hasil = fungsi_gak_jelas();
    println!("Nilai yang dibalikin fungsi: {:?}", hasil);
}

#[test]
fn materi_membuat_array() {
    // 1. Cara manual
    let bulan: [&str; 3] = ["Januari", "Februari", "Maret"];

    // 2. Cara instan (isi 10 angka 0 semua)
    let angka = [0; 10];

    println!("--- MEMBUAT ARRAY ---");
    println!("Bulan pertama: {}", bulan[0]);
    println!("Daftar angka: {:?}", angka);
}

#[test]
fn materi_mengakses_array() {
    let daftar_harga = [15000, 20000, 25000];

    let harga_satu = daftar_harga[0];
    let harga_dua = daftar_harga[1];

    println!("--- AKSES ARRAY ---");
    println!("Harga 1: {harga_satu}, Harga 2: {harga_dua}");
}

#[test]
fn materi_mutable_array() {
    let mut stok = [10, 20, 30];

    println!("Stok awal: {:?}", stok);

    // Ganti nilai di index ke-1
    stok[1] = 50;

    println!("Stok baru: {:?}", stok);
}

#[test]
fn materi_two_dimensional_array() {
    // Array 2x3 (2 baris, 3 kolom)
    let matriks = [[1, 2, 3], [4, 5, 6]];

    println!("--- ARRAY 2D ---");
    // Akses baris ke-0, kolom ke-2 (angka 3)
    println!("Isi baris 1 kolom 3: {}", matriks[0][2]);
}

// 1. Syarat Promo Kopi: Buat variabel harga_kopi (integer) dan status_member (boolean).
// Cek apakah harga_kopi lebih dari 25000 DAN status_member bernilai benar.
#[test]
fn soal_1() {
    let harga_kopi = 100000;
    let status_number = true;

    let result = harga_kopi > 25000 && status_number == true;

    println!("Harga Kopi : {harga_kopi} dan Status Member : {status_number}");
    println!("Hasil cek : {result}");
}

// 2. Validasi Login: Ada variabel email_input (string/&str) dan password_input (string/&str).
// Cek apakah email_input TIDAK kosong (tidak sama dengan "") DAN panjang password_input lebih dari 8 karakter.
#[test]
fn soal_2() {
    let email_input = "jondoe@gmail.com";
    let password_input = "Pwdpwd88";

    let result = email_input != "" && password_input.len() >= 8;

    println!("Email : {email_input}, Password: {password_input}");
    println!("{result}");
}

// 3. Akses Admin: Buat variabel role (string/&str).
// Cek apakah role sama dengan "admin" ATAU role sama dengan "super_user".
#[test]
fn soal_3() {
    let mut role = "admin";
    println!("{role}");

    role = "super_user";

    let result = role == "admin" || role == "super_user";

    println!("{result}");
}

// 4. Cek Stok & Toko: Variabel jumlah_stok (integer) dan is_toko_tutup (boolean).
// Cek apakah jumlah_stok lebih dari 0 DAN toko TIDAK tutup (gunakan operator !).
#[test]
fn soal_4() {
    let jumlah_stok: i8 = 5;
    let is_toko_tutup: bool = false;

    let result = jumlah_stok > 0 && !is_toko_tutup;

    println!("Stok? {jumlah_stok}, Toko buka? {is_toko_tutup}");
    print!("Boleh belanja? {result}");
}

// Diskon Pelajar: Variabel umur (integer) dan is_pelajar (boolean).
// Cek apakah umur di bawah 20 tahun ATAU is_pelajar bernilai benar.
#[test]
fn soal_5() {
    let umur = 20;
    let is_pelajar = false;

    let result = umur < 20 || !is_pelajar;

    println!("{umur}, {is_pelajar}");
    println!("{result}");
}

// SOAL 1
// Buat variable imutable "token_price": f64 dengan nilai 3_450.75.
// Coba tampilkan, lalu coba ubah nilainya — perhatikan apa yang terjadi!
// Clue: nama function -> fn soal_1(), wajib pakai attribute #[test]
#[test]
fn soal_6() {
    let token_price = 3_450.75;

    println!("Token Price: {}", token_price);
}

// SOAL 2
// Buat variable mutable "stok_barang": i32 = 100.
// Tampilkan stok awal, kurangi 30, tampilkan lagi,
// lalu kurangi 25 lagi dan tampilkan.
// Clue: nama function -> fn soal_2(), wajib pakai attribute #[test]
#[test]
fn soal_7() {
    let mut stok_barang = 100;
    stok_barang = stok_barang - 30;
    stok_barang = stok_barang - 25;
    println!("{}", stok_barang);
}

// SOAL 3
// Buat variable imutable "nama": &str = "Rizki".
// Lakukan shadowing dengan nama yang sama tapi tambahkan " - Rustacean" di belakangnya.
// Tampilkan keduanya.
// Clue: nama function -> fn soal_3(), wajib pakai attribute #[test]
#[test]
fn soal_8() {
    let nama = "Rizki";
    println!("{}", nama);
    let nama = format!("{} - Rustocean", nama);
    println!("{}", nama);
}

// SOAL 4
// Buat variable mutable "saldo": i64 = 1_000_000.
// Tampilkan saldo awal.
// Tambahkan gaji 5_000_000, tampilkan.
// Kurangi tagihan 2_500_000, tampilkan.
// Kurangi belanja 750_000, tampilkan saldo akhir.
// Clue: nama function -> fn soal_4(), wajib pakai attribute #[test]
#[test]
fn soal_9() {
    let mut saldo = 1_000_000;
    println!("Saldo Awal : {}", saldo);

    saldo += 5_000_000;
    println!("Saldo ditambah : {}", saldo);
    saldo -= 2_500_000;
    println!("Saldo dikurang : {}", saldo);

    saldo -= 750_000;
    println!("Saldo dikurang belanja : {}", saldo);
}

// SOAL 5
// Buat variable imutable "is_active": bool = false.
// Lakukan shadowing menjadi true.
// Tampilkan sebelum dan sesudah shadowing.
// Clue: nama function -> fn soal_5(), wajib pakai attribute #[test]
#[test]
fn soal_10() {
    let is_active = false;
    println!("Sebelum : {}", is_active);
    let is_active = true;
    println!("Sesudah : {}", is_active);
}
