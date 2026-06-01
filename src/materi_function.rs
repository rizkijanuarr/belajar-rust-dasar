// 1. Function tanpa parameter dan return value
fn sapa(){
    println!("Sapa");
}

// 2. Function return dengan parameter
fn sapa_nama(nama: &str){
    println!("Halo, {}", nama);
}

// 3. Function dengan return value
fn tambah(a: i32, b: i32)  {
    println!("{}, {}", a, b);
}

// 4. Function dengan Unit Test
#[test]
fn materi_function() {
    sapa();
    sapa_nama("Rizki");

    let hasil = tambah(10, 20);
    println!("Hasil: {:?}", hasil);
}