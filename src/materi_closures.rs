/*
Closure adalah function tanpa nama yang bisa disimpan di variable dan bisa menangkap nilai dari scope sekitarnya.
Kenapa penting:
- Dipakai hampir di semua iterator method
- Bikin kode lebih ringkas dan ekspresif
- Heavily dipakai di Collections processing
- Di Solana buat callback & handler
 */

#[test]
fn materi_closure() {
    // Closure sederhana
    let tambah = |a: i32, b: i32| a + b;
    println!("Hasil: {}", tambah(10, 20));

    // Closure tangkap nilai dari luar (capture)
    let diskon = 0.20;
    let hitung_diskon = |harga: f64| harga * diskon;
    println!("Diskon: ${}", hitung_diskon(3_450.75));

    // Closure multi baris
    let cek_profit = |beli: f64, jual: f64| {
        let profit = jual - beli;
        if profit > 0.0 {
            println!("Profit: ${}", profit);
        } else {
            println!("Rugi: ${}", profit);
        }
    };

    cek_profit(3_000.0, 3_450.75);
}

