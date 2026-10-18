fn main() {
    let pemilik = "fathur";
    let mut saldo = 50_000;
    
    // ada pemasukan
    let pemasukan = 20_000;
    saldo = saldo + pemasukan;    

    // makan
    let harga_makan = 15_000;
    saldo = saldo - harga_makan;

    println!("dompet {}, saldo: {}", pemilik, saldo);
}