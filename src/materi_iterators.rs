/*
Iterator adalah cara memproses setiap elemen dari collection satu per satu. Di Rust iterator sangat powerful karena bisa di-chain.
Kenapa penting:
- Gantiin for loop yang panjang jadi satu baris
- Lebih efisien — lazy evaluation (hitung saat dibutuhkan)
- Wajib paham buat processing data di real project
- Di Solana buat filter & transform account data
 */

#[test]
fn materi_iterator() {
    let harga = vec![3_450.75, 180.50, 96_000.0, 0.85];

    // map — transform setiap elemen
    let harga_idr: Vec<f64> = harga.iter()
        .map(|h| h * 16_350.0)
        .collect();
    println!("Harga IDR: {:?}", harga_idr);

    // filter — ambil yang sesuai kondisi
    let mahal: Vec<&f64> = harga.iter()
        .filter(|h| **h > 100.0)
        .collect();
    println!("Token mahal: {:?}", mahal);

    // sum — jumlahkan semua
    let total: f64 = harga.iter().sum();
    println!("Total: ${}", total);

    // chain map + filter sekaligus
    let hasil: Vec<f64> = harga.iter()
        .filter(|h| **h > 100.0)
        .map(|h| h * 16_350.0)
        .collect();
    println!("Token mahal dalam IDR: {:?}", hasil);
}