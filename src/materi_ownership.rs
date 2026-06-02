#[test]
fn materi_ownership() {
    // 1. Move / Berpindah
    let mut s1 = String::from("hello");
    let s2 = &mut s1;
    // let s3 = &s1;
    println!("s1: {}", s2);

    // Pertanyaannya disini adalah :
    // apakah ada selain String::from
    // semisal Integer::from atau bahkan Float::from
    // Ada, tapi konsepnya beda. Di Rust, ::from itu adalah sebuah fungsi konversi standar bawaan (namanya Trait From)
    // Lo bisa aja nulis i32::from(10_i16) buat mengubah integer 16-bit ke 32-bit. Tapi, untuk membuat integer atau float biasa dari awal, kita nggak perlu pakai ::from. Cukup langsung tulis angkanya: let angka = 10;, karena tipe data angka ukurannya sudah pasti dan langsung disimpan di memori cepat (Stack).

    // pertanyaan kedua adalah :
    // apa bedanya String::from dan str biasa?
    // jika str biasa itu bersifat ukuran sizenya tetap (fixed), nggak bisa di edit, langsung di tanam di binary program lo.
    // jika string::from String yang bersifat dinamis data nya bisa berubah ubah di memori.

    // oiya dan perlu di ingat bahwa disini bukan hanya String::from saja melainkan kamu bisa menulis seperti ini :
    // i32::from
    // u32::from
    // Vec::from
    // Di Rust, ::from adalah fungsi universal yang berasal dari sebuah fitur bernama Trait From. Fungsinya adalah untuk mengubah (mengonversi) satu tipe data ke tipe data lain secara aman.

    /*
        kita telah mengetahui terkait ownership / kepemilikan
        daripada kita memakai ownership saja yang mana variabel aslinya bakalan hangus.

        di rust telah menyediakan solusi dengan fitur borrowing / references yaitu peminjaman data.
        cara peminjaman data di rust biasanya menggunakan simbol ampersand (&).
        nah didalam ampersand terbagi menjadi dua yaitu :
        - Mutable Borrowing = dapat dilihat dan dapat dirubah oleh peminjam dalam satu waktu
        - Imutable Borrowing = hanya dapat dilihat saja.

     */
}