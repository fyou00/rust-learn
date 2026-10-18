fn main() {
    let stok_awal = 30;
    let mut stok = stok_awal;
    println!("stok awal: {}", stok);
    
    // barang masuk
    let barang_masuk = 10;
    stok += barang_masuk;
    println!("ada barang masuk! (+{}) stok: {}", barang_masuk, stok);
    
    print!("memasuki sesi penjualan pagi... ");
    let penjualan_pagi = 8;
    stok -= penjualan_pagi;
    println!("(terjual: {})", penjualan_pagi);
    println!("stok: {}", stok);
    
    print!("memasuki sesi penjualan sore... ");
    let penjualan_sore = 7;
    stok -= penjualan_sore;
    println!("(terjual: {})", penjualan_sore);
    println!("stok: {}", stok);
    
    println!("===================================");
    println!("perbandingan stok_awal dan stok");
    println!("stok_awal: {}", stok_awal);
    println!("stok (akhir): {}", stok);
}