# 🦀 Roadmap Rust — Lanjutan Bab 11–15

Kamu sudah bisa meminjam data, membuat tipe sendiri, memakai vector, dan menangani `Option` serta `Result`. Sekarang saatnya mengolah teks, menyimpan data berdasarkan nama, merapikan struktur program, bekerja dengan file, dan menguji perilaku kode secara otomatis.

## Cara Menggunakan Roadmap

1. Selesaikan Bab 1–10 terlebih dahulu, terutama ownership, borrowing, `Vec`, dan `Result`.
2. Ikuti urutan **String & String Slices → HashMap → Modules & Organisasi Kode → Error Handling Lanjutan & File I/O → Testing Dasar**.
3. Pada setiap bab, baca konsep, jalankan contoh, isi jawaban, kerjakan praktik, selesaikan studi kasus serta latihan Level 1–2, lalu perbaiki debugging.

Gunakan proyek latihan Rust yang sudah kamu miliki, atau buat proyek baru:

```sh
cargo new rust_lanjutan_11_15
cd rust_lanjutan_11_15
```

Untuk setiap program, ganti seluruh isi `src/main.rs`, simpan, lalu jalankan `cargo run`. Gunakan `cargo check` untuk memeriksa kompilasi tanpa menjalankan program. Pada Bab 15, jalankan juga `cargo test`.

> **Panduan latihan:** Setiap blok kode Rust adalah program terpisah. Nomor baris pada tugas praktik dihitung dari baris pertama dalam blok, termasuk baris kosong. Template studi kasus sudah dapat dijalankan, tetapi belum menyelesaikan seluruh misi. Hanya blok pada bagian debugging yang sengaja dibuat gagal dikompilasi.
>
> Gunakan nilai yang ditulis langsung di kode; input keyboard belum diperlukan. Kerjakan dengan Rust versi **stable**. Bab 14 membuat file di folder kerja saat program dijalankan: gunakan folder latihan, bukan folder berisi dokumen penting.

---

## Bab 11 — String & String Slices

### 1. Judul Materi & Penjelasan

Rust punya dua bentuk teks yang sering kamu temui:

| Bentuk | Kegunaan | Contoh |
| --- | --- | --- |
| `String` | Memiliki data teks dan bisa bertambah jika mutable | `String::from("Halo")` |
| `&str` | Meminjam potongan teks tanpa memiliki datanya | `"Halo"` atau `nama.as_str()` |

Bayangkan `String` sebagai buku catatan milikmu, sedangkan `&str` sebagai akses baca ke sebagian atau seluruh isinya. Literal teks seperti `"Halo"` juga bertipe `&str`.

- `.push_str("teks")` menambahkan teks ke `String` mutable.
- `format!("Halo, {}", nama)` membuat `String` baru tanpa mengonsumsi `nama` pada penggunaan ini.
- `.trim()` menghasilkan `&str` tanpa spasi putih di awal dan akhir; teks asal tidak berubah.
- `.split_whitespace()` memecah teks berdasarkan spasi putih. Hasilnya bisa dipakai dalam `for`, atau dihitung dengan `.count()`.
- `.len()` menghitung **byte**, bukan jumlah karakter. `.chars().count()` menghitung Unicode scalar value, yang belum tentu sama dengan jumlah simbol visual untuk emoji gabungan atau huruf dengan tanda gabung.

Rust menyimpan teks dalam **UTF-8**: satu karakter bisa memakai beberapa byte. Karena itu, kamu tidak bisa mengambil karakter memakai `teks[0]`. `.chars().next()` menghasilkan `Option<char>` untuk karakter pertama, atau `None` jika kosong. Slice dengan rentang byte seperti `&teks[0..2]` hanya aman jika batasnya tepat pada batas karakter—jangan menebak indeksnya!

> Untuk fungsi yang hanya membaca teks, biasanya pilih parameter `&str`. Pemilik `String` bisa meminjamkan datanya melalui `&nama` karena Rust dapat menyesuaikan `&String` menjadi `&str`.

### 2. Contoh Kode Dasar

```rust
fn sapa(nama: &str) {
    println!("Halo, {}!", nama.trim());
}

fn main() {
    let mut nama = String::from("Ayu");
    nama.push_str(" Putri");
    sapa(&nama);

    let kalimat = "  Belajar Rust itu seru  ";
    println!("Teks rapi: {}", kalimat.trim());
    println!("Jumlah kata: {}", kalimat.split_whitespace().count());
    println!("Nama masih tersedia: {}", nama);
}
```

**Hasil:** sapaan `Halo, Ayu Putri!`, teks tanpa spasi pinggir, jumlah kata `4`, dan nama masih bisa dicetak di `main`.

### 3. 📝 Tugas Jawaban

**Pertanyaan 1:** Mengapa fungsi yang hanya membaca teks biasanya cukup menerima `&str`, bukan mengambil `String`?

```text
Jawaban:


```

**Pertanyaan 2:** Mengapa `.len()` pada teks `"é"` menghasilkan `2`, sedangkan `.chars().count()` menghasilkan `1`? Mengapa `teks[0]` tidak diperbolehkan?

```text
Jawaban:


```

### 4. 🛠️ Tugas Praktik

```rust
fn main() {
    let pesan = String::from("  halo Rust  ");
    let rapi = pesan.as_str();
    println!("Teks: [{}]", rapi);
    println!("Jumlah kata: {}", rapi.split_whitespace().count());
}
```

**Misi modifikasi:**

1. Pada **baris ke-3**, ganti `pesan.as_str()` menjadi `pesan.trim()`.
2. Jalankan ulang. **Target: `Teks: [halo Rust]` dan jumlah kata `2`.**
3. Setelah **baris ke-5 kode awal**, tambahkan `println!("Asli: [{}]", pesan);`.
4. Perhatikan bahwa teks asli masih memiliki spasi pinggir. `trim()` meminjam bagian teks, bukan mengedit pemiliknya.

### 5. 🎯 Hands-On Challenge — Penganalisis Teks

**Studi kasus:** Kamu ingin merapikan kalimat belajar, menghitung kata, dan menampilkan setiap kata tanpa mengubah teks sumber.

**Template:**

```rust
fn analisis(teks: &str) {
    let rapi = teks.trim();
    println!("Teks rapi: {}", rapi);
}

fn main() {
    let catatan = String::from("  Aku sedang belajar Rust  ");
    analisis(&catatan);
    println!("Teks asli: [{}]", catatan);
}
```

**Misi:**

1. Di dalam `analisis`, cetak jumlah kata menggunakan `.split_whitespace().count()`.
2. Cetak jumlah Unicode scalar value dengan `.chars().count()`, termasuk spasi di tengah kalimat yang sudah dirapikan.
3. Gunakan `for kata in rapi.split_whitespace()` untuk mencetak setiap kata di baris terpisah.
4. Uji lagi dengan teks kosong `""` dan teks berisi spasi saja `"   "`. Keduanya harus menghasilkan jumlah kata `0` tanpa panic.

**Target input awal:** teks rapi `Aku sedang belajar Rust`, jumlah kata **`4`**, dan jumlah karakter untuk teks ASCII ini **`23`**. Teks sumber tetap dapat dicetak lengkap dengan spasi pinggir.

### 6. 💻 Exercises

#### ✅ Exercise: Level 1 — Mengolah Teks

1. Buat `String` berisi `"Belajar"`, tambahkan `" Rust"` dengan `.push_str()`, lalu cetak. **Target: `Belajar Rust`.**
2. Buat `sapa(nama: &str)`. Panggil menggunakan literal `"Budi"` dan pinjaman dari `String::from("Ayu")`. Cetak kembali `String` setelah pemanggilan.
3. Gunakan teks `"é"` untuk membandingkan `.len()` dan `.chars().count()`. **Target: `2` byte dan `1` Unicode scalar value.**

#### ✅ Exercise: Level 2 — Merangkum Input

1. Buat `hitung_kata(teks: &str) -> usize`. Uji `"Rust itu seru"`, `"   "`, dan `""`. **Target: `3`, `0`, dan `0`.**
2. Buat `buat_label(nama: &str, kategori: &str) -> String` memakai `format!`. Rapikan kedua input dengan `.trim()`. Untuk `" Buku "` dan `" Alat Tulis "`, **target: `[Alat Tulis] Buku`**.
3. Buat `tampilkan_awal(teks: &str)` yang memakai `.chars().next()` dan `match`. Pada `Some`, cetak karakter; pada `None`, cetak `Teks kosong`. Uji `"Rust"`, `"éclair"`, dan `""`. Jangan melakukan slice byte manual.

### 7. 🐛 Tugas Debugging (Akhir Materi)

Program berikut mencoba membaca karakter lewat indeks angka. **Ganti pengambilan karakter dengan `.chars().next()` dan tangani `Some` serta `None` menggunakan `match`.** Jangan memakai `.unwrap()` atau slice byte manual.

```rust
fn main() {
    let nama = String::from("Rust");
    let pertama = nama[0];
    println!("Karakter pertama: {}", pertama);
}
```

> **Target selesai:** berhasil dikompilasi dan mencetak `Karakter pertama: R`. Setelah itu, uji string kosong; program harus menampilkan pesan teks kosong tanpa panic.

---

## Bab 12 — HashMap

### 1. Judul Materi & Penjelasan

**`HashMap<K, V>`** menyimpan pasangan **kunci (*key*) dan nilai (*value*)**. Jika `Vec` cocok untuk daftar berurutan, `HashMap` cocok untuk mencari data berdasarkan identitas, misalnya nama barang → jumlah stok.

Gunakan `use std::collections::HashMap;` sebelum memakainya. Ini bagian dari pustaka standar; kamu tidak perlu menambah paket.

| Operasi | Kegunaan |
| --- | --- |
| `HashMap::new()` | Membuat map kosong; tipe bisa disimpulkan dari pemakaian |
| `.insert(kunci, nilai)` | Menambah pasangan atau mengganti nilai untuk kunci yang sama |
| `.get(kunci)` | Menghasilkan `Option<&V>` untuk membaca nilai |
| `.entry(kunci).or_insert(awal)` | Mengakses nilai yang ada atau memasukkan nilai awal; hasilnya `&mut V` |
| `.len()` | Menghitung jumlah kunci unik |

Untuk map berkunci `String`, `.get("Buku")` bisa memakai `&str`. Sebaliknya, `.insert()` dan `.entry()` membutuhkan kunci milik map, misalnya `String::from("Buku")`. `String` yang diserahkan sebagai kunci berpindah ownership ke map.

> `or_insert(0)` **tidak menimpa** nilai yang sudah ada. Karena hasilnya referensi mutable, gunakan `*nilai += tambahan` untuk mengubah angka yang dirujuk. Sintaks `+=` adalah bentuk ringkas penambahan dan penugasan. Urutan iterasi `HashMap` tidak dijamin—jangan mengandalkan urutan cetaknya.

### 2. Contoh Kode Dasar

```rust
use std::collections::HashMap;

fn main() {
    let mut stok: HashMap<String, i32> = HashMap::new();
    stok.insert(String::from("Buku"), 10);

    let jumlah = stok.entry(String::from("Buku")).or_insert(0);
    *jumlah += 5;

    match stok.get("Buku") {
        Some(jumlah) => println!("Stok Buku: {}", jumlah),
        None => println!("Barang tidak ditemukan"),
    }
}
```

**Hasil:** `Stok Buku: 15`. Kunci `Buku` sudah tersedia, sehingga nilai awal `0` tidak dimasukkan.

### 3. 📝 Tugas Jawaban

**Pertanyaan 1:** Kapan lebih cocok menggunakan `HashMap` daripada `Vec`? Berikan satu contoh kasus.

```text
Jawaban:


```

**Pertanyaan 2:** Apa perbedaan dampak `insert(kunci, 0)` dan `entry(kunci).or_insert(0)` jika kunci tersebut sudah memiliki nilai `10`?

```text
Jawaban:


```

### 4. 🛠️ Tugas Praktik

```rust
use std::collections::HashMap;

fn main() {
    let mut stok = HashMap::new();
    stok.insert(String::from("Buku"), 10);
    println!("Jumlah jenis: {}", stok.len());
}
```

**Misi modifikasi:**

1. Setelah **baris ke-5**, tambahkan `stok.insert(String::from("Pena"), 5);`.
2. Tepat setelah tambahan tadi, tambahkan `stok.insert(String::from("Buku"), 20);`.
3. Jalankan ulang. **Target jumlah jenis: `2`, bukan `3`.** Kunci yang sama tidak membuat entri baru.
4. Sebelum penutup `main`, tambahkan `match stok.get("Buku")` dengan cabang `Some` untuk mencetak stok dan `None` untuk pesan tidak ditemukan. **Target stok Buku: `20`.**

### 5. 🎯 Hands-On Challenge — Penghitung Stok Barang

**Studi kasus:** Toko menerima kiriman beberapa kali. Setiap kiriman harus menambah stok barang, bukan mengganti total sebelumnya.

**Template:**

```rust
use std::collections::HashMap;

fn tambah_stok(stok: &mut HashMap<String, i32>, nama: &str, jumlah: i32) {
    let nilai = stok.entry(String::from(nama)).or_insert(0);
    *nilai += jumlah;
}

fn main() {
    let mut stok = HashMap::new();
    tambah_stok(&mut stok, "Buku", 10);

    match stok.get("Buku") {
        Some(jumlah) => println!("Buku: {}", jumlah),
        None => println!("Buku belum tercatat"),
    }
}
```

**Misi:**

1. Tambahkan kiriman `Buku` sebanyak `5` dan `Pena` sebanyak `8`, sebelum pencarian stok.
2. Buat `tampilkan_stok(stok: &HashMap<String, i32>, nama: &str)` yang menangani barang ada dan tidak ada dengan `match`.
3. Gunakan fungsi tersebut untuk mencari `Buku`, `Pena`, dan `Penggaris`.
4. Cetak jumlah jenis barang. Untuk latihan ini, semua jumlah kiriman harus positif.

**Target:** Buku **`15`**, Pena **`8`**, Penggaris tidak ditemukan, jumlah jenis barang **`2`**.

### 6. 💻 Exercises

#### ✅ Exercise: Level 1 — Menyimpan dan Mencari

1. Buat map `nilai_siswa` dengan kunci `String` dan nilai `i32`: Ayu `80`, Budi `75`. Cari Ayu dan Citra menggunakan `.get()` serta `match`.
2. Masukkan ulang Budi dengan nilai `90`. **Target: nilai Budi `90`, jumlah siswa tetap `2`.**
3. Iterasi map menggunakan `for (nama, nilai) in &nilai_siswa`. Cetak setiap pasangan, lalu cetak `.len()` untuk membuktikan map masih tersedia. Urutan keluaran boleh berbeda.

#### ✅ Exercise: Level 2 — Mengakumulasi Data

1. Hitung frekuensi kata dalam `"rust seru rust aman"`. Gunakan `.split_whitespace()`, kunci `String`, dan `.entry(...).or_insert(0)`. **Target: rust `2`, seru `1`, aman `1`**. Untuk latihan ini, kata dibedakan persis menurut teksnya; tanda baca dan huruf besar belum dinormalisasi.
2. Buat `catat_suara(suara: &mut HashMap<String, i32>, calon: &str)` yang menambah satu suara. Panggil untuk Ayu, Budi, dan Ayu. **Target: Ayu `2`, Budi `1`.**
3. Buat `kurangi_stok(stok: &mut HashMap<String, i32>, nama: &str, jumlah: i32) -> Result<i32, String>`. Gunakan `.get_mut(nama)`, yang menghasilkan `Option<&mut i32>`. Tolak jumlah tidak positif, barang yang tidak ada, dan jumlah melebihi stok; selain itu kurangi nilai melalui `*nilai` dan kembalikan sisa stok. **Target: stok Buku `10`, pengurangan `3` → `Ok(7)`; pengurangan berikutnya `8` → `Err` dan stok tetap `7`.**

### 7. 🐛 Tugas Debugging (Akhir Materi)

Hasil `or_insert` adalah referensi mutable, bukan angka biasa. **Perbaiki operasi penambahan dengan mengakses nilai yang dirujuk.** Pertahankan pemakaian `entry` dan `or_insert`.

```rust
use std::collections::HashMap;

fn main() {
    let mut stok = HashMap::new();
    let jumlah = stok.entry(String::from("Buku")).or_insert(0);
    jumlah += 5;
    println!("Stok: {}", jumlah);
}
```

> **Target selesai:** berhasil dikompilasi dan mencetak `Stok: 5`.

---

## Bab 13 — Modules & Organisasi Kode

### 1. Judul Materi & Penjelasan

Saat program membesar, menaruh semuanya dalam satu kumpulan fungsi akan menyulitkan pencarian kode. **Module** seperti laci berlabel: setiap laci mengelompokkan tanggung jawab tertentu.

- `mod nama { ... }` mendefinisikan module di dalam file yang sama.
- Item seperti fungsi di dalam module bersifat privat secara default. `pub fn` mengizinkan pemanggilan dari luar module, selama jalur module-nya juga dapat diakses.
- `use` menghadirkan nama atau jalur dalam scope agar pemanggilan lebih ringkas; ini tidak menyalin kode dan tidak mengubah visibilitas.
- `crate::` memulai jalur dari akar crate saat ini.
- `super::` memulai jalur dari module induk.

**Crate** adalah satu unit kompilasi Rust. Pada proyek latihan ini, `src/main.rs` menjadi akar binary crate. Module anak dapat mengakses item privat milik leluhurnya, tetapi induk tidak otomatis boleh mengakses item privat milik anak.

Untuk memindahkan module ke file lain, tulis `mod produk;` di `src/main.rs`, lalu letakkan isi module dalam `src/produk.rs` **tanpa** pembungkus `mod produk { ... }`. Module yang sama jangan didefinisikan dua kali.

> Contoh dan template bab ini sengaja memakai module dalam satu file agar mudah dicoba. Setelah alurnya jelas, latihan Level 2 mengajakmu memisahkannya menjadi beberapa file.

### 2. Contoh Kode Dasar

```rust
fn nama_toko() -> &'static str {
    "Toko Rust"
}

mod kasir {
    pub fn total(harga: i32, jumlah: i32) -> i32 {
        harga * jumlah
    }

    pub fn tampilkan_toko() {
        println!("{}", super::nama_toko());
    }
}

use crate::kasir::total;

fn main() {
    kasir::tampilkan_toko();
    println!("Total: {}", total(12000, 2));
}
```

**Hasil:** `Toko Rust` dan `Total: 24000`. `super::nama_toko()` mengakses fungsi di induk module `kasir`.

**Bekal sintaks:** `&'static str` di sini adalah referensi ke literal teks yang tersedia sepanjang program berjalan. Kita belum membahas lifetime secara mendalam; gunakan bentuk ini hanya untuk mengembalikan literal pada latihan ini.

### 3. 📝 Tugas Jawaban

**Pertanyaan 1:** Apa perbedaan peran `mod`, `pub`, dan `use`? Apakah `use` bisa membuat fungsi privat menjadi dapat diakses?

```text
Jawaban:


```

**Pertanyaan 2:** Apa perbedaan titik awal jalur `crate::` dan `super::`?

```text
Jawaban:


```

### 4. 🛠️ Tugas Praktik

```rust
mod hitung {
    pub fn total(harga: i32, jumlah: i32) -> i32 {
        harga * jumlah
    }
}

fn main() {
    println!("Total: {}", hitung::total(5000, 2));
}
```

**Misi modifikasi:**

1. Pada **baris ke-6 yang kosong**, tambahkan `use crate::hitung::total;`.
2. Pada **baris ke-8**, ganti `hitung::total(5000, 2)` menjadi `total(5000, 3)`.
3. Jalankan ulang. **Target: `Total: 15000`.**
4. Perhatikan bahwa `use` memperpendek pemanggilan; logika fungsi tetap berada di module `hitung`.

### 5. 🎯 Hands-On Challenge — Aplikasi Kasir Modular

**Studi kasus:** Aplikasi kasir memisahkan data produk, perhitungan transaksi, dan tampilan agar setiap bagian punya tanggung jawab yang jelas.

**Template:**

```rust
mod produk {
    pub fn harga_buku() -> i32 {
        12000
    }
}

mod perhitungan {
    pub fn subtotal(harga: i32, jumlah: i32) -> i32 {
        harga * jumlah
    }
}

mod tampilan {
    pub fn cetak_total(total: i32) {
        println!("Total belanja: Rp{}", total);
    }
}

fn main() {
    let total = perhitungan::subtotal(produk::harga_buku(), 2);
    tampilan::cetak_total(total);
}
```

**Misi:**

1. Tambahkan `harga_pena() -> i32` pada module `produk`, dengan hasil `5000`.
2. Hitung subtotal untuk `2` buku dan `3` pena di `main`, lalu jumlahkan hasilnya.
3. Gunakan `use crate::perhitungan::subtotal;` agar fungsi perhitungan bisa dipanggil tanpa awalan module di `main`.
4. Tetap cetak hasil melalui module `tampilan`, bukan langsung lewat `println!` di `main`.

**Target:** `Total belanja: Rp39000`. Perhitungan dan tampilan tetap terpisah.

### 6. 💻 Exercises

#### ✅ Exercise: Level 1 — Menata Fungsi

1. Buat module `konversi` dengan `pub fn menit_ke_detik(menit: i32) -> i32`. Panggil dari `main`. **Target: `3` menit → `180` detik.**
2. Buat dua module, `luas` dan `keliling`, masing-masing memiliki `pub fn persegi(sisi: i32) -> i32`. Panggil dengan jalur lengkap agar namanya tidak tertukar. Untuk sisi `5`, **target: `25` dan `20`.**
3. Buat module `sapaan` dengan fungsi privat `teks() -> &'static str` yang mengembalikan literal, serta fungsi publik `tampilkan()` yang memanggil `teks()`. Dari `main`, panggil hanya `tampilkan()`.

#### ✅ Exercise: Level 2 — Merapikan Proyek

1. Pisahkan ketiga module studi kasus ke `src/produk.rs`, `src/perhitungan.rs`, dan `src/tampilan.rs`. Di `main.rs`, gunakan `mod produk;`, `mod perhitungan;`, dan `mod tampilan;`. Hapus definisi inline sebelumnya. **Target keluaran tetap `39000`.**
2. Di module `produk`, buat `pub struct Produk` dengan field privat `nama: String` dan `harga: i32`. Dalam `impl Produk`, sediakan `pub fn new(nama: String, harga: i32) -> Self`, `pub fn nama(&self) -> &str`, dan `pub fn harga(&self) -> i32`. `Self` berarti tipe `Produk`; konstruktor mengembalikan `Self { nama, harga }`. Dari `main`, akses data melalui metode, bukan field langsung.
3. Dalam satu file, buat module `toko` dengan fungsi privat `nama() -> &'static str`. Di dalam `toko`, buat `pub mod laporan` dengan `pub fn tampilkan()` yang memanggil `super::nama()`. Dari `main`, panggil `crate::toko::laporan::tampilkan()`.

### 7. 🐛 Tugas Debugging (Akhir Materi)

Fungsi sudah berada di module yang benar, tetapi belum bisa dipanggil oleh induknya. **Perbaiki visibilitas fungsi tanpa memindahkannya keluar dari module.**

```rust
mod kalkulator {
    fn total(harga: i32, jumlah: i32) -> i32 {
        harga * jumlah
    }
}

fn main() {
    println!("Total: {}", kalkulator::total(5000, 3));
}
```

> **Target selesai:** berhasil dikompilasi dan mencetak `Total: 15000`.

---

## Bab 14 — Error Handling Lanjutan & File I/O

### 1. Judul Materi & Penjelasan

Pada Bab 10, kamu menangani `Result` dengan `match`. Sekarang kita belajar **meneruskan error** kepada pemanggil dan menyimpan teks ke file melalui `std::fs`.

| Bentuk | Kegunaan |
| --- | --- |
| `fs::read_to_string(path)` | Membaca seluruh file sebagai UTF-8; hasilnya `io::Result<String>` |
| `fs::write(path, isi)` | Menulis data; hasilnya `io::Result<()>` |
| `hasil?` | Mengambil nilai `Ok`, atau segera mengembalikan `Err` dari fungsi saat ini |
| `match hasil` | Menangani keberhasilan/kegagalan dengan tindakan yang kamu tentukan |
| `hasil.expect("pesan")` | Mengambil nilai sukses, tetapi *panic* saat gagal |

`io::Result<T>` adalah bentuk singkat `Result<T, std::io::Error>`. Nilai `()` berarti tidak ada nilai hasil yang berarti, misalnya pada operasi menulis yang sukses.

Operator `?` **tidak mengabaikan error**. Ia menghentikan fungsi saat itu dan meneruskannya ke pemanggil. Untuk contoh `Result` di bab ini, fungsi harus mengembalikan `Result` dengan tipe error yang cocok atau dapat dikonversi. Jangan langsung menggabungkan error parsing dan error file tanpa memikirkan tipe errornya.

Pakai `match` jika ingin menampilkan pesan, mencoba pemulihan, atau memberi nilai pengganti. Pakai `?` ketika keputusan penanganan sebaiknya diserahkan ke pemanggil. `expect` cocok ketika kegagalan benar-benar melanggar asumsi yang disengaja, bukan sebagai penanganan normal file yang bisa hilang atau tidak bisa diakses. Contoh di bab ini tidak memerlukan `expect`.

> **Peringatan file:** `fs::write` membuat file jika belum ada dan **menimpa seluruh isinya** jika sudah ada. Ini bukan operasi menambahkan teks di akhir file. Path relatif dihitung dari folder kerja saat program dijalankan—biasanya akar proyek saat kamu menjalankan `cargo run` dari sana. File hilang, izin akses, dan isi non-UTF-8 bisa menyebabkan pembacaan gagal.

### 2. Contoh Kode Dasar

Jalankan di folder latihan. Nama file berikut khusus untuk demo dan boleh ditimpa.

```rust
use std::{fs, io};

fn simpan_dan_baca(path: &str, isi: &str) -> io::Result<String> {
    fs::write(path, isi)?;
    let teks = fs::read_to_string(path)?;
    Ok(teks)
}

fn main() {
    match simpan_dan_baca("demo_rust_14.txt", "Hari ini belajar file") {
        Ok(teks) => println!("Isi file: {}", teks),
        Err(error) => eprintln!("Operasi file gagal: {}", error),
    }
}
```

**Hasil jika operasi berhasil:** file `demo_rust_14.txt` dibuat atau ditimpa, lalu isinya dicetak. `eprintln!` mirip `println!`, tetapi menulis ke keluaran error standar.

### 3. 📝 Tugas Jawaban

**Pertanyaan 1:** Apa yang terjadi pada fungsi saat `fs::read_to_string(path)?` menghasilkan `Err`? Apakah baris setelahnya tetap dijalankan?

```text
Jawaban:


```

**Pertanyaan 2:** Mengapa `match` atau `?` lebih sesuai daripada `expect` untuk kegagalan file yang memang mungkin terjadi? Apa risiko `fs::write` terhadap file yang sudah ada?

```text
Jawaban:


```

### 4. 🛠️ Tugas Praktik

```rust
use std::{fs, io};

fn main() -> io::Result<()> {
    let path = "praktik_rust_14.txt";
    fs::write(path, "Belajar Rust")?;
    let isi = fs::read_to_string(path)?;
    println!("Isi: {}", isi);
    Ok(())
}
```

**Misi modifikasi:**

1. Pada **baris ke-5**, ganti isi teks menjadi `"Belajar Rust\nBelajar file"`. `\n` membuat baris baru. Jalankan; **target: dua baris teks tercetak setelah label `Isi:`.**
2. Setelah **baris ke-7 kode awal**, tambahkan `println!("Jumlah baris: {}", isi.lines().count());`. **Target: `2`.** `.lines()` menghasilkan bagian teks per baris, yang bisa dihitung dengan `.count()`.
3. Untuk menguji kegagalan baca, ubah **hanya argumen pembacaan pada baris ke-6 kode awal** menjadi `"file_yang_belum_ada_14.txt"`. Pastikan nama tersebut memang tidak ada.
4. Jalankan ulang: `?` meneruskan error keluar dari `main`, dan kode pencetakan setelah pembacaan tidak dijalankan. Setelah pengamatan, kembalikan argumen pembacaan menjadi `path`.

### 5. 🎯 Hands-On Challenge — Catatan Harian CLI

**Studi kasus:** Buat aplikasi terminal sederhana untuk menyimpan dan membaca catatan. Pada tahap ini, catatan masih ditulis di kode, belum diketik lewat keyboard.

**Template:**

```rust
use std::{fs, io};

fn simpan_catatan(path: &str, isi: &str) -> io::Result<()> {
    fs::write(path, isi)?;
    Ok(())
}

fn baca_catatan(path: &str) -> io::Result<String> {
    let isi = fs::read_to_string(path)?;
    Ok(isi)
}

fn main() {
    let path = "catatan_latihan_14.txt";
    match simpan_catatan(path, "Hari 1: belajar Rust") {
        Ok(()) => match baca_catatan(path) {
            Ok(isi) => println!("{}", isi),
            Err(error) => eprintln!("Gagal membaca: {}", error),
        },
        Err(error) => eprintln!("Gagal menyimpan: {}", error),
    }
}
```

**Misi:**

1. Ubah catatan menjadi dua baris: `Hari 1: belajar Rust` dan `Hari 2: belajar file`, dipisahkan `\n`.
2. Buat `jalankan(path: &str, isi: &str) -> io::Result<String>` yang memanggil `simpan_catatan` dan `baca_catatan`, memakai `?` untuk meneruskan error.
3. Ganti `match` bertingkat di `main` dengan satu `match jalankan(...)`. Pada sukses, tampilkan teks dan jumlah baris. Pada gagal, tampilkan pesan yang mudah dipahami.
4. Uji `baca_catatan` secara terpisah memakai nama file yang dipastikan belum ada. Jangan menjalankan penyimpanan terlebih dahulu untuk pengujian ini, karena penyimpanan justru membuat file tersebut.

**Target:** catatan tersimpan dan terbaca, jumlah baris **`2`**, serta file yang tidak ada menghasilkan pesan error tanpa `unwrap`/`expect`. Program ini menyimpan **ulang seluruh catatan**; belum menambahkan catatan ke akhir file lama.

### 6. 💻 Exercises

#### ✅ Exercise: Level 1 — Membaca dan Menulis

1. Tulis teks `"Ayu\nBudi\nCitra"` ke `nama_latihan_14.txt`, lalu baca dan tampilkan. Gunakan fungsi yang mengembalikan `io::Result` dan tangani hasilnya.
2. Buat `hitung_baris_file(path: &str) -> io::Result<usize>` dengan `read_to_string`, `?`, dan `.lines().count()`. **Target untuk file latihan pertama: `3`.**
3. Panggil fungsi tadi dengan path yang dipastikan tidak ada. Tangani `Err` melalui `match`; program harus menampilkan kegagalan tanpa panic. Jangan menuntut teks error persis sama karena pesan dapat berbeda antar sistem operasi.

#### ✅ Exercise: Level 2 — Menggabungkan Operasi

1. Buat `salin_teks(sumber: &str, tujuan: &str) -> io::Result<()>`. Baca sumber, lalu tulis ke tujuan dengan `?`. Gunakan dua nama file latihan berbeda dan pastikan tujuan boleh ditimpa. **Target: isi sumber dan tujuan sama.** Jika pembacaan sumber gagal, penulisan tujuan tidak boleh dijalankan.
2. Buat `ringkasan_file(path: &str) -> io::Result<String>` yang mengembalikan teks berisi jumlah baris dan kata. Gunakan `.lines().count()`, `.split_whitespace().count()`, serta `format!`. Untuk isi `"Belajar Rust\nItu seru"`, **target: `2` baris dan `4` kata.**
3. Pisahkan `simpan_catatan` dan `baca_catatan` ke module `penyimpanan` dengan fungsi publik. `main` hanya mengatur alur dan menampilkan hasil. Pertahankan tipe `io::Result` serta penanganan error tanpa panic.

### 7. 🐛 Tugas Debugging (Akhir Materi)

Operator `?` dipakai pada fungsi yang belum memiliki tipe hasil yang sesuai. **Ubah tanda tangan `main` menjadi `fn main() -> io::Result<()>` dan tambahkan `Ok(())` sebagai ekspresi terakhir.** Pertahankan `?` pada kedua operasi file.

```rust
use std::{fs, io};

fn main() {
    fs::write("debug_latihan_14.txt", "Rust itu seru")?;
    let isi = fs::read_to_string("debug_latihan_14.txt")?;
    println!("{}", isi);
}
```

> **Target selesai:** berhasil dikompilasi. Jika akses file diizinkan, mencetak `Rust itu seru`. File `debug_latihan_14.txt` adalah file latihan dan akan dibuat atau ditimpa.

---

## Bab 15 — Testing Dasar

### 1. Judul Materi & Penjelasan

Menjalankan program dan melihat hasilnya memang berguna, tetapi pemeriksaan manual mudah terlupakan. **Tes otomatis** adalah daftar pemeriksaan yang bisa kamu jalankan berulang kali setiap kali kode berubah.

Bayangkan kalkulator belanja: setelah kamu menambah fitur diskon, tes memastikan perhitungan harga normal tetap benar.

- `#[test]` menandai fungsi sebagai tes.
- `assert!(kondisi)` memeriksa bahwa kondisi bernilai `true`.
- `assert_eq!(aktual, harapan)` memeriksa bahwa dua nilai sama.
- `#[cfg(test)] mod tests` mengelompokkan kode yang hanya disertakan pada kompilasi untuk pengujian.
- `use super::*;` membawa nama dari module induk ke scope module tes.
- `cargo test` mengompilasi dan menjalankan tes. **`cargo check` saja tidak menjalankan tes.**

Jika assertion tidak terpenuhi, tes gagal. Panic pada assertion adalah mekanisme pelaporan tes, bukan alasan untuk memakai panic sebagai penanganan error normal di aplikasi.

**Apa yang perlu diuji?** Kasus normal, kondisi batas, dan kegagalan yang memang diharapkan. Contohnya: jumlah barang `2`, jumlah `0`, serta jumlah negatif. Untuk fungsi `Result`, bandingkan nilai `Ok(...)` atau gunakan `.is_err()` untuk memastikan hasilnya `Err`.

> Bab ini berfokus pada **unit test**: menguji bagian kecil seperti fungsi. Contoh tetap bisa disimpan di `src/main.rs`. Nanti, setelah memakai library crate dan proyek lebih besar, kamu bisa mempelajari integration test di folder `tests/`.

### 2. Contoh Kode Dasar

```rust
fn subtotal(harga: i32, jumlah: i32) -> i32 {
    harga * jumlah
}

fn main() {
    println!("Subtotal: {}", subtotal(12000, 2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dua_barang() {
        assert_eq!(subtotal(12000, 2), 24000);
    }

    #[test]
    fn jumlah_nol() {
        assert_eq!(subtotal(12000, 0), 0);
    }
}
```

Jalankan:

```sh
cargo test
```

**Target:** kedua tes lulus. `cargo run` tetap menampilkan `Subtotal: 24000`; kode tes tidak dijalankan oleh `cargo run`.

### 3. 📝 Tugas Jawaban

**Pertanyaan 1:** Apa perbedaan tujuan `cargo run`, `cargo check`, dan `cargo test`?

```text
Jawaban:


```

**Pertanyaan 2:** Mengapa tes tidak cukup hanya memeriksa satu input yang berhasil? Sebutkan satu kondisi batas dan satu kasus error untuk fungsi pemesanan barang.

```text
Jawaban:


```

### 4. 🛠️ Tugas Praktik

```rust
fn total(harga: i32, jumlah: i32) -> i32 {
    harga * jumlah
}
fn main() {}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dua_barang() {
        assert_eq!(total(5000, 2), 10000);
    }
}
```

**Misi modifikasi:**

1. Jalankan `cargo test` untuk memastikan tes awal lulus.
2. Pada **baris ke-10**, ganti assertion menjadi `assert_eq!(total(5000, 3), 15000);`.
3. Pada **baris ke-9**, ganti nama tes menjadi `fn tiga_barang() {`. Jalankan ulang; **target: tes tetap lulus.**
4. Setelah kurung penutup fungsi tes pada **baris ke-11 kode awal**, tambahkan tes kedua dengan `#[test]`, nama `jumlah_nol`, dan assertion `assert_eq!(total(5000, 0), 0);`.

**Target akhir:** dua tes lulus. Nama tes menjelaskan perilaku yang diperiksa, bukan sekadar `tes_1`.

### 5. 🎯 Hands-On Challenge — Pengujian Kalkulator Belanja

**Studi kasus:** Kasir memberi potongan tetap Rp10.000 ketika subtotal minimal Rp100.000. Kamu harus memastikan aturan batas dan validasinya benar.

**Template:**

```rust
fn total_bayar(harga: i32, jumlah: i32) -> Result<i32, String> {
    if harga < 0 || jumlah <= 0 {
        return Err(String::from("Harga atau jumlah tidak valid"));
    }
    let subtotal = harga * jumlah;
    Ok(subtotal)
}

fn main() {
    match total_bayar(25000, 4) {
        Ok(total) => println!("Total bayar: {}", total),
        Err(error) => eprintln!("{}", error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn belanja_tanpa_diskon() {
        assert_eq!(total_bayar(25000, 2), Ok(50000));
    }
}
```

**Misi:**

1. Setelah menghitung subtotal, terapkan potongan `10000` jika subtotal `>= 100000`; selain itu tanpa potongan. Kembalikan total setelah diskon melalui `Ok`.
2. Pertahankan tes `belanja_tanpa_diskon`.
3. Tambahkan tes batas: `total_bayar(99999, 1)` → `Ok(99999)`; `total_bayar(100000, 1)` → `Ok(90000)`; `total_bayar(100001, 1)` → `Ok(90001)`.
4. Tambahkan tes penolakan: harga `-1`, jumlah `0`, dan jumlah `-1`, masing-masing dengan pasangan input lain yang valid. Gunakan `assert!(hasil.is_err())`.
5. Jalankan `cargo test`, lalu `cargo run`.

**Target:** minimal **`7` tes lulus**, dan program mencetak `Total bayar: 90000` untuk `25000 × 4`. Harga `0` diperbolehkan; jumlah harus positif. Gunakan input kecil dengan hasil perkalian yang muat dalam `i32`—penanganan overflow belum menjadi cakupan fungsi latihan ini.

### 6. 💻 Exercises

#### ✅ Exercise: Level 1 — Assertion dan Batas

1. Buat `luas_persegi(sisi: i32) -> i32`. Tambahkan tes untuk sisi `4` dan `0`. **Target: `16` dan `0`.** Untuk latihan ini, anggap input tidak negatif dan hasilnya muat dalam `i32`.
2. Buat `lulus(nilai: i32) -> bool` dengan batas `75`. Uji `74`, `75`, dan `76` menggunakan `assert!`. Untuk memeriksa `false`, gunakan `assert!(!lulus(74))`; `!` membalik nilai boolean.
3. Buat `hitung_kata(teks: &str) -> usize`. Uji `"Rust itu seru"`, `""`, dan `"   "`. **Target: `3`, `0`, dan `0`.**

#### ✅ Exercise: Level 2 — Menguji Hasil dan Perubahan Data

1. Buat `validasi_jumlah(teks: &str) -> Result<i32, String>` yang menolak teks bukan angka serta angka `<= 0`. Tambahkan tes `"3"` → `Ok(3)`, lalu `"0"`, `"-2"`, dan `"tiga"` → `Err`. Gunakan `match` untuk parsing seperti Bab 10, agar fokus tetap pada penulisan tes.
2. Buat `tambah_stok(stok: &mut HashMap<String, i32>, nama: &str, jumlah: i32)` seperti Bab 12. Di setiap tes, buat map baru. Uji barang baru dan penambahan pada barang lama. Untuk memeriksa stok `15`, gunakan `assert_eq!(stok.get("Buku"), Some(&15));`.
3. Tambahkan tes pada kalkulator belanja untuk harga `0` dengan jumlah positif, serta subtotal di atas batas diskon: `(60000, 2)` → `Ok(110000)`. Jalankan satu tes berdasarkan nama menggunakan `cargo test nama_tes`, lalu jalankan seluruh tes dengan `cargo test`.

### 7. 🐛 Tugas Debugging (Akhir Materi)

Tes memanggil nama fungsi yang berbeda dari definisinya. **Perbaiki nama pemanggilan dalam assertion, bukan nilai hasil yang diharapkan.** Gunakan `cargo test` agar kode di module tes ikut dikompilasi; `cargo run` saja tidak memperlihatkan kesalahan ini.

```rust
fn subtotal(harga: i32, jumlah: i32) -> i32 {
    harga * jumlah
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dua_barang() {
        assert_eq!(sub_total(5000, 2), 10000);
    }
}
```

> **Target selesai:** `cargo test` berhasil dikompilasi dan satu tes lulus. Setelah itu, ubah sementara ekspektasi menjadi `9999` untuk melihat perbedaan antara **error kompilasi** dan **tes yang berhasil dikompilasi tetapi gagal saat dijalankan**. Kembalikan ekspektasi ke `10000` setelah pengamatan.

---

## ✅ Checklist Kelulusan Bab 11–15

- [ ] Saya dapat membedakan `String` dan `&str`, serta memilih parameter fungsi pembaca teks.
- [ ] Saya dapat merapikan teks, menghitung kata, dan menjelaskan perbedaan byte dengan karakter.
- [ ] Saya dapat menyimpan, mencari, dan memperbarui data di `HashMap` tanpa mengandalkan urutan iterasinya.
- [ ] Saya dapat memakai `entry().or_insert()` untuk mengakumulasi nilai.
- [ ] Saya dapat memisahkan tanggung jawab menggunakan module, `pub`, dan `use`.
- [ ] Saya dapat menjelaskan jalur `crate::` dan `super::`, serta memindahkan module ke file terpisah.
- [ ] Saya dapat membaca dan menulis file, serta memahami risiko penimpaan isi oleh `fs::write`.
- [ ] Saya dapat meneruskan error memakai `?` dan menanganinya dengan `match` di lapisan pemanggil.
- [ ] Saya dapat menulis unit test untuk kasus normal, kondisi batas, dan hasil error.
- [ ] Saya sudah menyelesaikan lima studi kasus, latihan Level 1–2, dan lima debugging.

**Arah berikutnya:** closures dan iterators, generics dan traits, pendalaman lifetime, lalu pengembangan CLI dengan input pengguna. Kamu juga bisa berhenti sejenak untuk merangkai Bab 1–15 menjadi proyek kecil: **pencatat pengeluaran**, **inventaris toko**, atau **catatan belajar berbasis file**.

> Tak perlu buru-buru menambah fitur. Program kecil yang rapi, punya penanganan error, dan dilindungi tes adalah kemajuan besar. Sekarang kamu bukan hanya membuat kode berjalan—kamu mulai membuatnya mudah dirawat. Semangat, Rustacean! 🦀
