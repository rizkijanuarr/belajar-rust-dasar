/*
    Future adalah nilai yang belum siap sekarang, tapi akan siap di masa depan. Future adalah dasar dari async programming di Rust — mirip "nomor antrian" yang akan jadi hasil nyata setelah ditunggu dengan .await.

    Kenapa Penting?
    - Dasar dari async/await di Rust
    - Wajib paham buat bikin web server (Axum, Tokio pakai ini)
    - Bikin server bisa handle banyak request sekaligus tanpa block thread
 */

// Function async — return Future, bukan nilai langsung
async fn ambil_harga_eth() -> f64 {
    3_450.75
}

#[tokio::test]
async fn materi_future() {
    // Tanpa .await — cuma "rencana", belum dieksekusi
    let future = ambil_harga_eth();

    // Dengan .await — sekarang baru benar-benar dijalankan
    let harga = future.await;
    println!("Harga ETH: ${}", harga);
}