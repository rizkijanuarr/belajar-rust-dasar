/*
    Slice adalah referensi ke sebagian data dari collection (String atau array) tanpa mengambil ownership-nya.
    Kenapa penting:
    - Bisa akses sebagian data tanpa copy
    - Tidak ambil ownership — data aslinya tetap valid
    - Sering dipakai untuk string processing & array manipulation
 */

#[test]
fn materi_slice() {
    // 1. String Slice (&str)
    let kalimat = String::from("halo bro");

    // Mengambil potongan dari indeks 0 sampai SEBELUM indeks 4 (yaitu 0, 1, 2, 3)
    let kata_pertama = &kalimat[0..4];
    // Mengambil potongan dari indeks 5 sampai akhir teks
    let kata_kedua = &kalimat[5..];

    println!("Kata pertama: {}", kata_pertama); // Output: halo
    println!("Kata kedua: {}", kata_kedua);     // Output: bro

    // 2. Array Slice
    let kumpulan_angka = [10, 20, 30, 40, 50];

    // Mengambil potongan dari indeks 1 sampai SEBELUM indeks 4 (yaitu indeks 1, 2, 3)
    let potongan_angka = &kumpulan_angka[1..4];

    println!("Potongan array: {:?}", potongan_angka); // Output: [20, 30, 40]
}