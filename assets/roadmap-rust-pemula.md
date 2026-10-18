# 🦀 Roadmap Rust untuk Pemula

Belajar Rust tidak harus buru-buru. Kita mulai dari menyimpan nilai, mengenali tipe data, membuat fungsi, mengatur alur program, lalu mengenal **ownership**—konsep khas Rust yang menjaga keamanan memori.

## Cara Menggunakan Roadmap

1. Pelajari bab secara berurutan: **Variables &amp; Mutability → Tipe Data → Functions → Control Flow → Ownership**.
2. Ketik dan jalankan contoh kode, bukan sekadar membacanya.
3. Isi tugas jawaban, kerjakan praktik, lanjutkan studi kasus dan latihan bertingkat, lalu tuntaskan debugging sebelum lanjut.

**Alur latihan setiap bab:** tugas praktik melatih perubahan kecil dengan panduan baris; **Hands-On Challenge** menggabungkan konsep dalam studi kasus; **Exercises Level 1** melatih dasar, sedangkan **Level 2** mengajakmu mengembangkan solusi lebih mandiri. Terakhir, debugging melatihmu membaca pesan compiler.

> Semua template studi kasus sudah bisa dijalankan sebagai titik awal, tetapi **belum menyelesaikan seluruh misi**. Kerjakan instruksinya sampai memenuhi target hasil. Gunakan nilai yang ditulis langsung di kode; kamu belum perlu membuat input dari keyboard. Materi `enum` dan `match` tidak diperlukan untuk tantangan lima bab dasar ini.

Gunakan Rust versi **stable**. Jika belum terpasang, ikuti petunjuk di [rustup.rs](https://rustup.rs/). Buat proyek latihan melalui terminal:

```sh
cargo new latihan_rust
cd latihan_rust
```

Untuk setiap contoh, **ganti seluruh isi `src/main.rs`** dengan kode yang sedang dipelajari, simpan, lalu jalankan:

```sh
cargo run
```

> **Panduan latihan:** Nomor baris pada tugas praktik dihitung dari baris pertama di dalam blok kode, termasuk baris kosong. Kode sengaja tidak diberi nomor agar bisa langsung disalin. Setiap blok merupakan program terpisah, bukan untuk digabung menjadi satu file.
>
> Kode pada bagian **debugging memang sengaja salah**. Pesan compiler adalah petunjuk, bukan tanda kamu gagal. Kamu bisa memakai `cargo check` untuk mengecek kompilasi tanpa menjalankan program.

---

## Bab 1 — Variables &amp; Mutability

### 1. Judul Materi &amp; Penjelasan

**Variabel** adalah nama untuk sebuah nilai. Bayangkan variabel seperti kotak berlabel: labelnya membantu kita menggunakan isi kotak tanpa harus menulis nilainya berulang kali.

Di Rust, variabel dibuat dengan `let` dan secara default bersifat **immutable**: setelah diberi nilai, variabel tidak bisa diberi nilai baru. Tambahkan `mut` jika nilainya memang perlu berubah.

* `let skor = 10;` → nilainya tidak boleh diganti melalui penugasan ulang.
* `let mut skor = 10;` → nilainya boleh diganti.
* `println!` mencetak teks ke terminal; `{}` menjadi tempat menyisipkan nilai.

> Biasakan memakai variabel immutable terlebih dahulu. Gunakan `mut` ketika memang diperlukan agar perubahan nilai mudah dilacak.

### 2. Contoh Kode Dasar

```rust
fn main() {
    let nama = "Ayu";
    let mut skor = 10;
    println!("Halo, {}! Skor awal: {}", nama, skor);
    skor = 20;
    println!("Skor sekarang: {}", skor);
}
```

**Hasil:** skor awal `10`, lalu berubah menjadi `20`.

### 3. 📝 Tugas Jawaban

**Pertanyaan 1:** Apa perbedaan `let` dan `let mut`? Kapan kamu membutuhkan `mut`?

```text
Jawaban:


```

**Pertanyaan 2:** Mengapa variabel yang tidak bisa diubah dapat membantu mengurangi kesalahan program?

```text
Jawaban:


```

### 4. 🛠️ Tugas Praktik

Jalankan kode awal berikut. Program ini sudah bisa dikompilasi dan dijalankan:

```rust
fn main() {
    let koin = 10;
    println!("Koin: {}", koin);
}
```

**Misi modifikasi:**

1. Pada **baris ke-2**, ganti `let koin = 10;` menjadi `let mut koin = 10;`.
2. Sisipkan `koin = 25;` tepat setelah baris tersebut, sebelum `println!`.
3. Jalankan ulang. Pastikan keluaran berubah dari `Koin: 10` menjadi **`Koin: 25`**.

### 5. 🎯 Hands-On Challenge — Dompet Uang Saku

**Studi kasus:** Kamu ingin mencatat perubahan saldo uang saku sepanjang hari. Buat program yang memperbarui satu variabel saldo, bukan membuat variabel saldo baru untuk setiap transaksi.

**Template:**

```rust
fn main() {
    let pemilik = "Ayu";
    let mut saldo = 50000;
    println!("Dompet {} — saldo awal: Rp{}", pemilik, saldo);

    // Tambahkan transaksi dan cetak saldo setelah setiap transaksi di sini.
}
```

**Misi:**

1. Tambahkan pemasukan `20000` dengan penugasan `saldo = saldo + 20000;`.
2. Kurangi saldo sebesar `15000` untuk makan dan `5000` untuk transportasi. Gunakan pola `saldo = saldo - nilai;`.
3. Cetak saldo setelah **setiap** transaksi dengan label yang jelas.
4. Pertahankan `pemilik` sebagai variabel immutable dan gunakan kembali variabel `saldo` yang sama.

**Target hasil:** saldo awal `50000` → setelah pemasukan `70000` → setelah makan `55000` → setelah transportasi **`50000`**.

> **Cek pemahaman:** Mengapa `saldo` membutuhkan `mut`, tetapi `pemilik` tidak?

### 6. 💻 Exercises

#### ✅ Exercise: Level 1 — Mengelola Nilai

1. Buat variabel immutable `nama_produk` dengan nilai `"Buku"`, lalu cetak nilainya.
2. Buat `stok` mutable dengan nilai `12`. Kurangi sebanyak `3` menggunakan penugasan ulang, lalu cetak hasilnya. **Target: `9`.**
3. Buat `poin` mutable dengan nilai `0`. Tambahkan `10`, lalu `15`, dan cetak nilai setelah masing-masing perubahan. **Target akhir: `25`.**

#### ✅ Exercise: Level 2 — Catatan Transaksi

1. Buat simulasi persediaan toko: stok awal `30`, barang masuk `10`, terjual `8`, lalu terjual lagi `7`. Cetak stok setelah tiap perubahan. **Target akhir: `25`.**
2. Simpan nilai transaksi dalam variabel immutable bernama `barang_masuk`, `penjualan_pagi`, dan `penjualan_sore`; gunakan variabel tersebut saat memperbarui stok, bukan angka langsung pada rumus.
3. Tambahkan `stok_awal` immutable dan gunakan nilainya untuk mengisi variabel mutable `stok`. Di akhir, cetak stok awal dan stok akhir untuk membandingkannya. Jangan gunakan fungsi tambahan atau percabangan dahulu.

### 7. 🐛 Tugas Debugging (Akhir Materi)

Kode berikut ingin memperbarui jumlah langkah, tetapi gagal dikompilasi. **Temukan penyebabnya dan perbaiki deklarasi variabel agar penugasan ulang diperbolehkan.** Pertahankan kedua nilai dan baris penugasan ulangnya.

```rust
fn main() {
    let langkah = 100;
    println!("Langkah awal: {}", langkah);
    langkah = 200;
    println!("Langkah sekarang: {}", langkah);
}
```

> **Target selesai:** Program berhasil dikompilasi dan mencetak langkah awal `100`, lalu langkah sekarang `200`.

---

## Bab 2 — Tipe Data

### 1. Judul Materi &amp; Penjelasan

**Tipe data** memberi tahu Rust jenis nilai yang disimpan dan operasi yang boleh dilakukan. Seperti wadah yang berbeda untuk air dan beras, tiap tipe punya kegunaan sendiri.

Beberapa tipe yang sering dipakai:

| Tipe | Kegunaan | Contoh |
| --- | --- | --- |
| `i32` | Bilangan bulat bertanda 32-bit | `-10`, `42` |
| `f64` | Bilangan pecahan | `3.5` |
| `bool` | Nilai benar atau salah | `true`, `false` |
| `char` | Satu nilai skalar Unicode; memakai petik tunggal | `'A'`, `'🦀'` |
| `&str` | Referensi ke teks; literal teks memakai petik ganda | `"Halo"` |

Rust sering bisa **menyimpulkan tipe** dari nilai dan penggunaannya. Kamu juga bisa menulis tipe secara eksplisit dengan `nama: Tipe`. Jika tidak ada konteks lain, literal bilangan bulat biasanya menjadi `i32`, sedangkan pecahan menjadi `f64`.

Rust tidak otomatis mencampur semua tipe angka. Gunakan konversi yang sesuai jika dibutuhkan. Pada latihan berikut, `f64::from(...)` mengubah nilai `i32` menjadi `f64`.

### 2. Contoh Kode Dasar

```rust
fn main() {
    let umur: i32 = 20;
    let tinggi: f64 = 1.70;
    let sedang_belajar: bool = true;
    let inisial: char = 'A';
    let nama: &str = "Ayu";
    println!("{} ({}), umur {} tahun", nama, inisial, umur);
    println!("Tinggi: {} meter; belajar: {}", tinggi, sedang_belajar);
}
```

> **Perhatikan:** `'A'` adalah `char`, sedangkan `"A"` adalah teks. Bentuk petiknya punya arti!

### 3. 📝 Tugas Jawaban

**Pertanyaan 1:** Tipe apa yang cocok untuk menyimpan jumlah buku, harga dengan pecahan, dan status lampu menyala?

```text
Jawaban:
- Jumlah buku:
- Harga dengan pecahan:
- Status lampu:

```

**Pertanyaan 2:** Apa perbedaan `'R'` dan `"Rust"` dari sisi tipe data?

```text
Jawaban:


```

### 4. 🛠️ Tugas Praktik

Jalankan program pembagian berikut:

```rust
fn main() {
    let jumlah: i32 = 7;
    let pembagi: i32 = 2;
    let hasil = jumlah / pembagi;
    println!("Hasil: {}", hasil);
}
```

**Misi modifikasi:**

1. Amati hasil awal: **`Hasil: 3`**. Pembagian bilangan bulat membuang bagian pecahan menuju nol.
2. Pada **baris ke-4**, ganti `let hasil = jumlah / pembagi;` menjadi `let hasil = f64::from(jumlah) / f64::from(pembagi);`.
3. Jalankan ulang. Hasilnya sekarang harus **`Hasil: 3.5`**.

### 5. 🎯 Hands-On Challenge — Ringkasan Pesanan Kantin

**Studi kasus:** Sebuah kantin ingin menampilkan data pesanan dan rata-rata harga per barang. Pilih tipe yang sesuai, lalu hitung hasil pecahan tanpa kehilangan bagian desimal.

**Template:**

```rust
fn main() {
    let pembeli: &str = "Ayu";
    let kode_meja: char = 'B';
    let jumlah_barang: i32 = 4;
    let total_harga: i32 = 25000;
    let sudah_bayar: bool = false;

    println!("Pembeli: {}; meja: {}", pembeli, kode_meja);
    println!("Jumlah: {}; total: Rp{}", jumlah_barang, total_harga);
    println!("Sudah bayar: {}", sudah_bayar);
    // Hitung dan cetak rata-rata harga per barang di sini.
}
```

**Misi:**

1. Buat variabel `rata_rata` bertipe `f64` dari `total_harga / jumlah_barang`. Konversikan **kedua operand sebelum membagi** dengan `f64::from(...)`.
2. Cetak hasilnya. Lalu ubah `jumlah_barang` menjadi `3` untuk memastikan bagian pecahan tetap ada.
3. Gunakan `{:.2}` di dalam `println!` untuk menampilkan dua angka di belakang titik desimal, misalnya `println!("Rata-rata: {:.2}", rata_rata);`.
4. Ubah `sudah_bayar` menjadi `true`, lalu jalankan ulang. Kamu mengedit nilai awal di kode, bukan melakukan penugasan ulang saat program berjalan; `mut` tidak diperlukan.

**Target hasil:** untuk `4` barang, rata-rata **`6250.00`**; untuk `3` barang, **`8333.33`** setelah pembulatan tampilan.

> Gunakan jumlah barang lebih dari nol. Validasi dengan percabangan akan dipelajari pada Bab 4. Angka pecahan di sini untuk latihan; sistem uang sungguhan biasanya menyimpan nilai dalam satuan terkecil menggunakan bilangan bulat.

### 6. 💻 Exercises

#### ✅ Exercise: Level 1 — Pilih Tipe yang Tepat

1. Buat profil produk dengan `nama: &str`, `kategori: char`, `stok: i32`, `berat_kg: f64`, dan `tersedia: bool`. Isi lalu cetak semua nilainya.
2. Hitung rata-rata `9` buku untuk `2` rak sebagai `f64`. **Target: `4.5`.**
3. Simpan kode kelas `'C'` sebagai `char` dan nama kelas `"C Dasar"` sebagai `&str`. Cetak keduanya, lalu jelaskan perbedaan petiknya.

#### ✅ Exercise: Level 2 — Kalkulator Sederhana

1. Hitung kecepatan rata-rata dari jarak `150` km dan waktu `4` jam. Simpan kedua input sebagai `i32`, kemudian konversikan sebelum pembagian. **Target: `37.5` km/jam.**
2. Hitung luas persegi panjang dengan panjang `2.5` meter dan lebar `1.2` meter, keduanya `f64`. Tampilkan dua angka desimal. **Target tampilan: `3.00`.**
3. Buat laporan latihan dengan `hari: i32 = 7` dan `total_menit: i32 = 200`. Hitung rata-rata menit per hari menggunakan `f64`. **Target tampilan dua desimal: `28.57`.**

### 7. 🐛 Tugas Debugging (Akhir Materi)

Program berikut ingin menyimpan nilai logika, tetapi tipe nilai awalnya tidak sesuai. **Perbaiki nilai pada deklarasi `aktif` tanpa mengubah tipe `bool`.**

```rust
fn main() {
    let aktif: bool = "true";
    println!("Status aktif: {}", aktif);
}
```

> **Target selesai:** Program berhasil dikompilasi dan mencetak `Status aktif: true`.

---

## Bab 3 — Functions

### 1. Judul Materi &amp; Penjelasan

**Fungsi** adalah kumpulan instruksi bernama yang bisa dipanggil kembali. Bayangkan mesin kecil: kamu memasukkan bahan melalui **parameter**, lalu mesin bisa menghasilkan **nilai kembalian**.

* Fungsi didefinisikan dengan `fn`.
* Parameter harus memiliki tipe, misalnya `angka: i32`.
* `-> i32` berarti fungsi mengembalikan nilai bertipe `i32`.
* Ekspresi terakhir **tanpa titik koma** dapat menjadi nilai kembalian fungsi.
* Nama fungsi umumnya memakai `snake_case`, misalnya `hitung_total`.

> **Detail penting:** `angka * 2` menghasilkan nilai. Jika diberi `;`, hasil ekspresi itu tidak lagi menjadi nilai kembalian implisit. Fungsi yang menjanjikan `i32` tetap harus mengembalikan `i32`.

### 2. Contoh Kode Dasar

```rust
fn kali_dua(angka: i32) -> i32 {
    angka * 2
}

fn main() {
    let hasil = kali_dua(6);
    println!("Hasil: {}", hasil);
}
```

**Hasil:** `Hasil: 12`. Nilai `6` adalah **argumen** saat pemanggilan, sedangkan `angka` adalah **parameter** pada definisi fungsi.

### 3. 📝 Tugas Jawaban

**Pertanyaan 1:** Apa perbedaan parameter dan nilai kembalian sebuah fungsi?

```text
Jawaban:


```

**Pertanyaan 2:** Mengapa baris `angka * 2` pada contoh tidak diakhiri `;`?

```text
Jawaban:


```

### 4. 🛠️ Tugas Praktik

Jalankan fungsi penghitung total berikut:

```rust
fn total(harga: i32, jumlah: i32) -> i32 {
    harga * jumlah
}

fn main() {
    let bayar = total(5000, 2);
    println!("Total: {}", bayar);
}
```

**Misi modifikasi:**

1. Pada **baris ke-1**, ganti definisi menjadi `fn total(harga: i32, jumlah: i32, ongkir: i32) -> i32 {`.
2. Pada **baris ke-2**, ganti ekspresi menjadi `harga * jumlah + ongkir`—tetap tanpa `;`.
3. Pada **baris ke-6**, ganti pemanggilan menjadi `let bayar = total(5000, 2, 3000);`.
4. Simpan setelah seluruh perubahan selesai, lalu jalankan. Hasilnya harus **`Total: 13000`**.

### 5. 🎯 Hands-On Challenge — Kalkulator Biaya Laundry

**Studi kasus:** Sebuah laundry menghitung biaya berdasarkan berat cucian dan tarif per kilogram. Pisahkan perhitungan ke dalam fungsi agar bisa digunakan untuk beberapa pelanggan.

**Template:**

```rust
fn hitung_biaya(berat_kg: i32, tarif_per_kg: i32) -> i32 {
    berat_kg * tarif_per_kg
}

fn main() {
    let biaya = hitung_biaya(3, 7000);
    println!("Biaya laundry: Rp{}", biaya);
}
```

**Misi:**

1. Jalankan template. Biaya awalnya adalah `21000`.
2. Tambahkan parameter `biaya_antar: i32` ke `hitung_biaya`, lalu sertakan dalam nilai kembaliannya.
3. Perbarui pemanggilan untuk cucian `3` kg, tarif `7000`, dan biaya antar `5000`. **Target: `26000`.**
4. Panggil fungsi yang sama untuk pelanggan kedua: `5` kg, tarif `7000`, dan biaya antar `0`. **Target: `35000`.**
5. Buat fungsi `total_pendapatan(pesanan_satu: i32, pesanan_dua: i32) -> i32`. Gunakan hasil kedua pemanggilan sebelumnya sebagai argumennya. **Target total: `61000`.**

> **Batas latihan:** Pakai berat bilangan bulat positif dan tarif tidak negatif. Kamu belum perlu memvalidasi input atau memakai percabangan.

### 6. 💻 Exercises

#### ✅ Exercise: Level 1 — Satu Fungsi, Satu Tugas

1. Buat fungsi `luas_persegi(sisi: i32) -> i32`. Panggil dengan sisi `6`. **Target: `36`.**
2. Buat fungsi `menit_ke_detik(menit: i32) -> i32`. Panggil dengan nilai `5`. **Target: `300`.**
3. Buat fungsi `cetak_nomor(nomor: i32)` tanpa nilai kembalian eksplisit. Di dalamnya, cetak nomor antrean. Panggil dengan `12` dan `13`.

#### ✅ Exercise: Level 2 — Menggabungkan Fungsi

1. Buat `subtotal(harga: i32, jumlah: i32) -> i32` dan `tambah_ongkir(subtotal: i32, ongkir: i32) -> i32`. Gunakan hasil fungsi pertama sebagai input fungsi kedua. Untuk harga `12000`, jumlah `3`, dan ongkir `8000`, **target: `44000`**.
2. Buat `celsius_ke_fahrenheit(celsius: f64) -> f64` dengan rumus `celsius * 9.0 / 5.0 + 32.0`. Uji `0.0` dan `100.0`. **Target: `32.0` dan `212.0`.**
3. Buat `rata_rata_dua(a: i32, b: i32) -> f64`. Konversikan masing-masing parameter sebelum menjumlahkan dan membagi dengan `2.0`. Uji `7` dan `8`. **Target: `7.5`.**

### 7. 🐛 Tugas Debugging (Akhir Materi)

Fungsi berikut menjanjikan nilai `i32`, tetapi isi fungsinya belum mengembalikan nilai itu dengan benar. **Cari satu tanda baca yang mengubah perilakunya, lalu perbaiki tanpa mengubah rumus perkalian.**

```rust
fn kuadrat(angka: i32) -> i32 {
    angka * angka;
}

fn main() {
    println!("Kuadrat: {}", kuadrat(4));
}
```

> **Target selesai:** Program berhasil dikompilasi dan mencetak `Kuadrat: 16`.

---

## Bab 4 — Control Flow

### 1. Judul Materi &amp; Penjelasan

**Control flow** mengatur instruksi mana yang dijalankan dan berapa kali instruksi diulang. Bayangkan petunjuk perjalanan: *jika* hujan, ambil payung; *selama* belum sampai, lanjut berjalan.

* **`if` / `else`** memilih tindakan berdasarkan kondisi bertipe `bool`.
* **`for`** mengulang untuk setiap nilai dalam suatu rentang atau kumpulan.
* **`while`** mengulang selama kondisi bernilai `true`.
* **`loop`** terus mengulang sampai dihentikan, misalnya dengan `break`.

Kita fokus pada `if` dan `for` terlebih dahulu. Rentang `1..4` berisi `1, 2, 3`, sedangkan `1..=4` juga menyertakan `4`.

Operator pembanding seperti `>=`, `<`, dan `==` menghasilkan nilai `bool`. Ingat: **`=` untuk penugasan**, sedangkan **`==` untuk membandingkan kesamaan**.

### 2. Contoh Kode Dasar

```rust
fn main() {
    let nilai = 80;
    if nilai >= 75 {
        println!("Lulus!");
    } else {
        println!("Coba lagi, kamu bisa!");
    }

    for putaran in 1..=3 {
        println!("Latihan putaran {}", putaran);
    }
}
```

**Hasil:** program mencetak `Lulus!`, lalu tiga baris latihan untuk putaran `1`, `2`, dan `3`.

### 3. 📝 Tugas Jawaban

**Pertanyaan 1:** Apa perbedaan nilai yang dihasilkan rentang `1..3` dan `1..=3`?

```text
Jawaban:


```

**Pertanyaan 2:** Mengapa kondisi `if` perlu berupa nilai `bool`, bukan sekadar sebuah angka?

```text
Jawaban:


```

### 4. 🛠️ Tugas Praktik

Jalankan program berikut:

```rust
fn main() {
    for angka in 1..=3 {
        println!("Angka: {}", angka);
    }
}
```

**Misi modifikasi:**

1. Pada **baris ke-2**, ganti `for angka in 1..=3 {` menjadi `for angka in 1..=5 {`.
2. Ganti **baris ke-3** dengan tiga baris berikut secara berurutan: `if angka % 2 == 0 {`, lalu `println!("Genap: {}", angka);`, lalu `}`.
3. Operator `%` menghitung sisa pembagian. Angka genap memiliki sisa `0` ketika dibagi `2`.
4. Jalankan ulang. Program harus hanya mencetak **`Genap: 2`** dan **`Genap: 4`**.

### 5. 🎯 Hands-On Challenge — Pemeriksa Kelulusan

**Studi kasus:** Kamu membuat laporan nilai beberapa peserta. Gunakan perulangan untuk memproses nilai dan percabangan untuk menentukan kategori hasil.

**Template:**

```rust
fn main() {
    for nomor in 1..=5 {
        let nilai = 40 + nomor * 10;
        if nilai >= 75 {
            println!("Peserta {}: {} — Lulus", nomor, nilai);
        } else {
            println!("Peserta {}: {} — Belum lulus", nomor, nilai);
        }
    }
}
```

**Misi:**

1. Jalankan template dan amati nilai peserta: `50`, `60`, `70`, `80`, dan `90`.
2. Tambahkan kategori tertinggi: nilai `>= 85` mendapat label `Lulus istimewa`. Gunakan pola `if ... { ... } else if ... { ... } else { ... }`. Periksa kategori tertinggi lebih dahulu agar nilai `90` tidak masuk kategori biasa.
3. Buat `jumlah_lulus` mutable bernilai `0` **sebelum** perulangan. Tambahkan `1` untuk setiap peserta dengan nilai `>= 75`, termasuk kategori istimewa. Boleh gunakan satu `if` tambahan setelah penentuan kategori, tetapi masih di dalam perulangan.
4. Cetak total peserta lulus **setelah** perulangan selesai.

**Target hasil:** peserta `1–3` belum lulus, peserta `4` lulus, peserta `5` lulus istimewa, dan total lulus **`2`**.

### 6. 💻 Exercises

#### ✅ Exercise: Level 1 — Pilihan dan Pengulangan

1. Buat pemeriksa umur dengan `if` / `else`: usia `>= 17` mencetak `Boleh membuat KTP`, selain itu `Belum cukup umur`. Uji usia `16` dan `17` secara terpisah.
2. Gunakan `for` untuk mencetak angka `1` sampai `10`, termasuk kedua batasnya.
3. Gunakan `for` dan kondisi `angka % 2 != 0` untuk mencetak angka ganjil dari `1` sampai `9`. Operator `!=` berarti tidak sama dengan. **Target: `1, 3, 5, 7, 9`.**

#### ✅ Exercise: Level 2 — Rekap dan Klasifikasi

1. Jumlahkan bilangan `1` sampai `10` menggunakan akumulator mutable yang dimulai dari `0`. Cetak hasil di luar perulangan. **Target: `55`.**
2. Buat klasifikasi suhu dengan `if` / `else if` / `else`: di bawah `20` → `Dingin`, `20` sampai kurang dari `30` → `Nyaman`, dan `30` ke atas → `Panas`. Uji batas `19`, `20`, `29`, dan `30`.
3. Simulasikan tabungan harian: saldo awal `0`, setoran `10000` per hari, target `50000`. Gunakan `while saldo < target`, tambah saldo dan jumlah hari pada setiap iterasi, lalu cetak hasil akhir. **Target: `5` hari dan saldo `50000`.**

### 7. 🐛 Tugas Debugging (Akhir Materi)

Program berikut ingin memeriksa apakah jumlah tiket lebih dari nol. **Perbaiki kondisi `if` dengan operator pembanding agar menghasilkan `bool`.** Jangan mengubah nilai `tiket`.

```rust
fn main() {
    let tiket = 2;
    if tiket {
        println!("Silakan masuk!");
    } else {
        println!("Beli tiket dahulu.");
    }
}
```

> **Target selesai:** Program berhasil dikompilasi dan mencetak `Silakan masuk!`. Kondisi yang kamu buat juga harus bernilai `false` jika jumlah tiket adalah `0`.

---

## Bab 5 — Pengenalan Dasar Ownership

### 1. Judul Materi &amp; Penjelasan

**Ownership** adalah aturan Rust tentang siapa yang bertanggung jawab atas sebuah nilai. Bayangkan buku perpustakaan: ada pihak yang memegang tanggung jawab atas buku itu, dan orang lain bisa meminjamnya dengan aturan tertentu.

Tiga ide utamanya:

1. Setiap nilai memiliki **owner** atau pemilik.
2. Pada satu waktu, hanya ada satu owner untuk nilai tersebut.
3. Saat owner keluar dari **scope**—wilayah kode tempat ia berlaku—nilainya dibersihkan otomatis.

Untuk melihat perpindahan ownership, kita memakai **`String`**: tipe teks yang memiliki data teksnya sendiri dan dapat tumbuh. `String::from("Halo")` membuat `String` dari literal teks.

* `let kedua = pertama;` pada `String` biasanya **memindahkan ownership** (*move*). Variabel `pertama` tidak bisa dipakai lagi setelahnya.
* `let kedua = &pertama;` membuat **referensi** untuk meminjam nilai, bukan memindahkan kepemilikannya.
* Tipe sederhana seperti `i32` memiliki perilaku **`Copy`**: penugasan menyalin nilai sehingga variabel asal masih dapat digunakan.

> **Batas awal:** Dengan referensi bersama `&T`, kamu bisa membaca nilai tetapi tidak mengubahnya melalui referensi itu. Referensi tidak boleh hidup lebih lama daripada nilai yang dirujuk. Peminjaman yang dapat mengubah nilai (`&mut T`) bisa dipelajari setelah dasar ini nyaman.

### 2. Contoh Kode Dasar

```rust
fn main() {
    let pesan = String::from("Halo, Rust!");
    let pinjaman = &pesan;
    println!("Melalui pinjaman: {}", pinjaman);
    println!("Melalui pemilik: {}", pesan);
}
```

**Yang terjadi:** `pesan` tetap menjadi owner. `pinjaman` hanya meminjam, sehingga keduanya bisa dipakai untuk membaca teks. Di akhir `main`, data milik `pesan` dibersihkan otomatis.

### 3. 📝 Tugas Jawaban

**Pertanyaan 1:** Apa perbedaan `let b = a;` dan `let b = &a;` jika `a` bertipe `String`?

```text
Jawaban:


```

**Pertanyaan 2:** Jika `a` bertipe `i32`, apakah `let b = a;` membuat `a` tidak bisa dipakai lagi? Jelaskan kaitannya dengan `Copy`.

```text
Jawaban:


```

### 4. 🛠️ Tugas Praktik

Jalankan kode awal berikut. Ownership dipindahkan ke `tujuan`, dan program ini tetap valid karena `asal` tidak digunakan setelah perpindahan:

```rust
fn main() {
    let asal = String::from("Belajar Rust");
    let tujuan = asal;
    println!("Tujuan: {}", tujuan);
}
```

**Misi modifikasi:**

1. Pada **baris ke-3**, ganti `let tujuan = asal;` menjadi `let tujuan = &asal;`.
2. Sisipkan `println!("Asal: {}", asal);` tepat setelah baris `println!` yang sudah ada, sebelum kurung penutup `main`.
3. Jalankan ulang. Kedua baris harus menampilkan teks **`Belajar Rust`**.
4. Jelaskan kepada dirimu sendiri: mengapa sekarang `asal` masih bisa digunakan? Petunjuknya ada pada tanda `&`.

### 5. 🎯 Hands-On Challenge — Label Paket Tanpa Memindahkan Data

**Studi kasus:** Sebuah aplikasi pengiriman menyimpan nama penerima sebagai `String`. Fungsi pencetak label hanya perlu membaca nama itu; setelah label dicetak, program utama masih harus bisa menggunakannya.

**Bekal kecil:** Parameter `nama: &str` menerima teks yang dipinjam. Jika `penerima` adalah `String`, kamu bisa mengirim `&penerima`; Rust otomatis menyesuaikan referensi tersebut menjadi `&str` pada pemanggilan ini. Fungsi tidak mengambil ownership atas `String` asal.

**Template:**

```rust
fn cetak_label(nama: &str) {
    println!("Paket untuk: {}", nama);
}

fn main() {
    let penerima = String::from("Ayu");
    cetak_label(&penerima);
    println!("Data penerima tetap tersedia: {}", penerima);
}
```

**Misi:**

1. Tambahkan `kota` sebagai `String` dengan nilai `"Bandung"` di `main`.
2. Ubah fungsi menjadi `cetak_label(nama: &str, kota: &str)` dan cetak kedua informasi itu di dalam fungsi.
3. Perbarui pemanggilannya dengan meminjam `penerima` dan `kota` menggunakan `&`.
4. Panggil fungsi dua kali, lalu cetak kembali kedua variabel asli di `main`. Jangan gunakan `.clone()`.

**Target hasil:** label `Ayu` dan `Bandung` tercetak dua kali; setelahnya kedua variabel asli masih dapat dicetak tanpa error ownership.

> **Cek pemahaman:** Mengapa fungsi pencetak cukup meminjam teks daripada mengambil kepemilikannya?

### 6. 💻 Exercises

#### ✅ Exercise: Level 1 — Pemilik dan Peminjam

1. Buat `judul` bertipe `String`, lalu buat dua referensi bersama bernama `pinjaman_satu` dan `pinjaman_dua` menggunakan `&judul`. Cetak kedua referensi dan `judul`. **Target: ketiganya menampilkan teks yang sama.**
2. Buat `stok: i32 = 10`, lalu `salinan_stok = stok`. Cetak keduanya untuk membuktikan bahwa tipe `i32` memiliki perilaku `Copy`.
3. Buat `asal` sebagai `String`, lalu pindahkan nilainya ke `pemilik_baru`. Cetak hanya `pemilik_baru` setelah perpindahan; tambahkan komentar yang menjelaskan mengapa `asal` tidak digunakan lagi.

#### ✅ Exercise: Level 2 — Ownership dalam Fungsi

1. Buat `cetak_judul(judul: &str)`. Panggil dua kali menggunakan referensi ke `String` yang sama, kemudian cetak `String` asal di `main`. **Target: ketiga pencetakan berhasil tanpa `.clone()`.**
2. Buat `terima_paket(paket: String)` yang mencetak isi paket. Panggil dengan sebuah variabel `String` tanpa `&`. Setelah pemanggilan, jangan gunakan variabel asal; jelaskan lewat komentar bahwa ownership berpindah ke parameter fungsi dan nilai dibersihkan saat fungsi berakhir.
3. Buat fungsi `kembalikan_paket(paket: String) -> String` yang mengembalikan parameter melalui ekspresi terakhir `paket` tanpa `;`. Pindahkan `String` dari `main` ke fungsi, simpan nilai kembalian dalam variabel baru, lalu cetak variabel baru itu. **Target: teks tetap sama, tetapi ownership kembali ke pemanggil melalui nilai kembalian.**

### 7. 🐛 Tugas Debugging (Akhir Materi)

Kode berikut mencoba memakai nilai setelah ownership-nya dipindahkan. **Temukan baris perpindahannya, lalu ubah menjadi peminjaman dengan referensi bersama.** Pertahankan kedua baris `println!` dan jangan gunakan `.clone()` untuk latihan ini.

```rust
fn main() {
    let nama = String::from("Ayu");
    let nama_lain = nama;
    println!("Nama awal: {}", nama);
    println!("Nama lain: {}", nama_lain);
}
```

> **Target selesai:** Program berhasil dikompilasi dan kedua baris keluaran menampilkan `Ayu`. Selamat—kamu baru saja memperbaiki masalah ownership pertamamu! 🦀

---

## ✅ Checklist Kelulusan Dasar

* [ ] Saya sudah menyelesaikan lima Hands-On Challenge dan memeriksa target hasilnya.
* [ ] Saya sudah mencoba Exercises Level 1 dan Level 2 pada setiap bab.
* \[ \] Saya bisa memilih kapan memakai `let` dan `let mut`.
* \[ \] Saya bisa membedakan bilangan bulat, pecahan, `bool`, `char`, dan teks.
* \[ \] Saya bisa membuat fungsi dengan parameter dan nilai kembalian.
* \[ \] Saya bisa menggunakan `if` dan `for` untuk mengatur alur program.
* \[ \] Saya bisa menjelaskan perbedaan *move*, peminjaman, dan `Copy` secara sederhana.
* \[ \] Saya sudah memperbaiki kelima tantangan debugging dan mengeceknya dengan compiler.

**Langkah berikutnya:** pelajari `struct` dan `enum`, pencocokan pola dengan `match`, koleksi seperti `Vec`, penanganan kegagalan dengan `Option` dan `Result`, serta pengujian menggunakan `cargo test`.

> Tidak perlu langsung hafal semua sintaks. Target awalmu adalah memahami ide dasarnya, berani bereksperimen, dan membaca pesan compiler satu per satu. Konsisten sedikit demi sedikit lebih baik daripada terburu-buru. Semangat! 🚀