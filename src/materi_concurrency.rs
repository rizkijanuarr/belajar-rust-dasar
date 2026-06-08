/*
    Concurrency adalah kemampuan menjalankan beberapa tugas secara bersamaan tanpa saling ganggu.

    Analogi dunia nyata:
    - Tanpa concurrency → kasir warung cuma satu, antri satu-satu
    - Dengan concurrency → kasir banyak, beberapa pelanggan dilayani bersamaan
 */

use std::thread;

#[test]
fn materi_concurrency() {
    // Jalankan 3 thread bersamaan
    let t1 = thread::spawn(|| {
        println!("Thread 1 - Proses transaksi ETH");
    });

    let t2 = thread::spawn(|| {
        println!("Thread 2 - Proses transaksi SOL");
    });

    let t3 = thread::spawn(|| {
        println!("Thread 3 - Proses transaksi BTC");
    });

    // Tunggu semua thread selesai
    t1.join().unwrap();
    t2.join().unwrap();
    t3.join().unwrap();
}