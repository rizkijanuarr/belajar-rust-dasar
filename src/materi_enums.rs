// Enum sederhana
enum StatusTransaksi {
    Pending,
    Confirmed,
    Failed,
}

// Enum dengan data
enum Instruksi {
    Transfer { amount: f64, to: String },
    Stake(u64),
    Unstake,
}

#[test]
fn materi_enum() {
    let status = StatusTransaksi::Confirmed;

    match status {
        StatusTransaksi::Pending   => println!("Menunggu..."),
        StatusTransaksi::Confirmed => println!("Transaksi Berhasil!"),
        StatusTransaksi::Failed    => println!("Transaksi Gagal!"),
    }

    // Enum dengan data
    let aksi = Instruksi::Transfer {
        amount: 1.5,
        to: String::from("0xABC123"),
    };

    match aksi {
        Instruksi::Transfer { amount, to } => {
            println!("Transfer {} SOL ke {}", amount, to)
        },
        Instruksi::Stake(amount) => println!("Stake {} SOL", amount),
        Instruksi::Unstake       => println!("Unstake!"),
    }
}