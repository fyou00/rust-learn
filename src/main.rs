fn main() {
    let nama_produk = "Buku";
    println!("nama_produk: {}", nama_produk);
    
    let mut stok = 12;
    stok -= 3;
    println!("stok: {}", stok);
    
    let mut poin = 0;
    poin += 10;
    poin += 15;
    println!("poin: {}", poin);
}

#[test]
fn test_var() {
    let var1 = 20;
    let mut var2 = 3;
    println!("var1: {}, var2 (before): {}", var1, var2);
    var2 = 5;
    println!("var1: {}, var2 (after): {}", var1, var2);
}