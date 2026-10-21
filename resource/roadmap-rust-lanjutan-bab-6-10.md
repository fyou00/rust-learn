# 🦀 Roadmap Rust — Lanjutan Bab 6–10

Kamu sudah mengenal variabel, tipe data, fungsi, control flow, dan ownership. Sekarang kita naik satu tingkat: meminjam data dengan aman, menyusun data sendiri, mengelola kumpulan nilai, dan menangani hasil yang belum tentu berhasil.

## Cara Menggunakan Roadmap

1. Selesaikan Bab 1–5 terlebih dahulu, terutama perbedaan *move*, referensi, dan `Copy`.
2. Ikuti urutan **Borrowing & References → Struct & Methods → Enum & Match → Vec → Option & Result**.
3. Pada setiap bab, baca konsep, jalankan contoh, isi jawaban, kerjakan praktik, selesaikan studi kasus serta latihan Level 1–2, lalu perbaiki debugging.

Gunakan proyek latihan Rust yang sudah kamu miliki, atau buat proyek baru:

```sh
cargo new rust_lanjutan
cd rust_lanjutan
```

Untuk setiap program, ganti seluruh isi `src/main.rs`, simpan, lalu jalankan `cargo run`. Kamu bisa memakai `cargo check` untuk memeriksa kompilasi tanpa menjalankan program.

> **Panduan latihan:** Setiap blok kode Rust adalah program terpisah. Nomor baris pada tugas praktik dihitung dari baris pertama dalam blok, termasuk baris kosong. Template studi kasus sudah dapat dijalankan, tetapi belum menyelesaikan seluruh misi. Hanya blok pada bagian debugging yang sengaja dibuat gagal dikompilasi.
>
> Gunakan nilai yang ditulis langsung di kode; input keyboard belum diperlukan. Kerjakan dengan Rust versi **stable**. Pesan compiler adalah teman diskusi—baca petunjuk pertamanya, perbaiki, lalu coba lagi!

---

## Bab 6 — Borrowing & References

### 1. Judul Materi & Penjelasan

Pada Bab 5, kamu sudah meminjam teks dengan `&`. Sekarang kita bedakan dua jenis peminjaman:

| Bentuk | Kegunaan | Aturan utama |
| --- | --- | --- |
| `&T` | Referensi bersama untuk membaca | Boleh ada beberapa peminjaman bersama sekaligus |
| `&mut T` | Referensi eksklusif untuk membaca dan mengubah | Selama masih digunakan, tidak boleh ada referensi lain ke nilai yang sama |

Bayangkan papan tulis: banyak orang boleh membaca bersama, tetapi saat seseorang sedang mengedit dengan akses eksklusif, akses lain harus menunggu.

Owner tetap memiliki nilainya; fungsi peminjam tidak mengambil ownership. Agar bisa meminjam secara mutable, variabel asal biasanya harus dideklarasikan dengan `mut`.

**Bekal sintaks:** `&mut nama` membuat peminjaman mutable. Pada referensi angka, `*angka` mengakses nilai yang dirujuk. Untuk `String`, metode `.push_str("teks")` menambahkan teks; Rust menangani akses melalui referensi secara otomatis saat memanggil metode.

> Masa aktif peminjaman bisa berakhir setelah penggunaan terakhir referensinya, tidak selalu harus menunggu kurung penutup scope. Referensi juga tidak boleh hidup lebih lama daripada nilai yang dirujuk.

### 2. Contoh Kode Dasar

```rust
fn tambah_poin(poin: &mut i32) {
    *poin = *poin + 10;
}

fn main() {
    let mut skor = 5;
    tambah_poin(&mut skor);
    println!("Skor: {}", skor);
}
```

**Hasil:** `Skor: 15`. Nilai `skor` tetap dimiliki `main`; fungsi hanya meminjamnya sementara.

### 3. 📝 Tugas Jawaban

**Pertanyaan 1:** Apa perbedaan izin yang diberikan oleh `&T` dan `&mut T`?

```text
Jawaban:


```

**Pertanyaan 2:** Mengapa Rust tidak mengizinkan referensi bersama tetap digunakan bersamaan dengan peminjaman mutable atas nilai yang sama?

```text
Jawaban:


```

### 4. 🛠️ Tugas Praktik

Jalankan kode berikut:

```rust
fn main() {
    let mut nama = String::from("Ayu");
    nama.push_str(" Putri");
    println!("Nama: {}", nama);
}
```

**Misi modifikasi:**

1. Ganti **baris ke-3** dengan `let pinjaman = &mut nama;`.
2. Tepat setelahnya, tambahkan `pinjaman.push_str(" Putri");`.
3. Pertahankan `println!` yang mencetak `nama`. Jalankan ulang.

**Target:** tetap mencetak **`Nama: Ayu Putri`**. Referensi mutable tidak lagi digunakan saat `nama` dicetak.

### 5. 🎯 Hands-On Challenge — Editor Catatan

**Studi kasus:** Program menyimpan catatan belajar. Fungsi editor boleh mengubah catatan, sementara fungsi pratinjau hanya membacanya.

**Template:**

```rust
fn tampilkan(catatan: &str) {
    println!("Catatan: {}", catatan);
}

fn main() {
    let catatan = String::from("Belajar Rust");
    tampilkan(&catatan);
}
```

**Misi:**

1. Jadikan `catatan` mutable.
2. Buat `tambah_catatan(catatan: &mut String, tambahan: &str)` yang memanggil `catatan.push_str(tambahan)`.
3. Tampilkan catatan awal, panggil editor dengan tambahan `" selama 30 menit"`, lalu tampilkan lagi.
4. Cetak kembali `catatan` di `main` untuk membuktikan ownership tidak berpindah. Jangan gunakan `.clone()`.

**Target akhir:** `Belajar Rust selama 30 menit`.

### 6. 💻 Exercises

#### ✅ Exercise: Level 1 — Meminjam dengan Tepat

1. Buat `cetak_saldo(saldo: &i32)` yang hanya mencetak saldo. Panggil dua kali dari variabel yang sama.
2. Buat `tambah_stok(stok: &mut i32)` untuk menambah `5`. Dari stok awal `10`, **target: `15`**. Gunakan `*stok` untuk memperbarui nilai.
3. Buat dua referensi bersama ke satu `String`, lalu cetak keduanya. Setelah penggunaan terakhir keduanya, tambahkan teks melalui pemilik yang mutable dan cetak hasilnya.

#### ✅ Exercise: Level 2 — Mengatur Akses Data

1. Buat `setor(saldo: &mut i32, jumlah: i32)`. Dari saldo `10000`, lakukan setoran `5000` dan `2000` secara berurutan. **Target: `17000`.**
2. Buat `tukar(a: &mut i32, b: &mut i32)` menggunakan satu variabel sementara. Uji pada dua variabel berbeda bernilai `3` dan `8`. **Target: `8` dan `3`.**
3. Buat satu referensi mutable ke `String`, gunakan untuk menambah teks, lalu setelah penggunaan terakhirnya buat referensi bersama untuk membaca hasilnya. Jelaskan urutan akses tersebut melalui komentar.

### 7. 🐛 Tugas Debugging (Akhir Materi)

Kode berikut masih menggunakan referensi bersama setelah perubahan mutable. **Atur ulang urutan pemanggilan dan pencetakan agar penggunaan terakhir `bacaan` selesai sebelum `push_str`.** Pertahankan semua operasi, tanpa `.clone()`.

```rust
fn main() {
    let mut pesan = String::from("Halo");
    let bacaan = &pesan;
    pesan.push_str(" Rust");
    println!("Sebelum: {}", bacaan);
    println!("Sesudah: {}", pesan);
}
```

> **Target selesai:** berhasil dikompilasi; mencetak `Sebelum: Halo`, lalu `Sesudah: Halo Rust`.

---

## Bab 7 — Struct & Methods

### 1. Judul Materi & Penjelasan

**`struct`** menggabungkan beberapa data yang saling terkait menjadi satu tipe. Jika sebelumnya nama barang, harga, dan stok berdiri sendiri, sekarang ketiganya bisa masuk satu “kartu produk”. Setiap bagian disebut **field**.

- Definisi `struct` menentukan nama field dan tipenya.
- Buat sebuah nilai atau *instance* dengan mengisi seluruh field.
- Akses field menggunakan titik, misalnya `produk.stok`.
- **`impl`** berisi perilaku yang terkait dengan tipe tersebut.
- Metode dengan **`&self`** meminjam instance untuk membaca; **`&mut self`** meminjamnya untuk mengubah.

`self` merujuk instance yang menerima pemanggilan metode. Agar metode `&mut self` bisa dipanggil pada variabel milikmu, variabel tersebut harus mutable.

### 2. Contoh Kode Dasar

```rust
struct Produk {
    nama: String,
    stok: i32,
}

impl Produk {
    fn tampilkan(&self) {
        println!("{} — stok {}", self.nama, self.stok);
    }
}

fn main() {
    let produk = Produk {
        nama: String::from("Buku"),
        stok: 10,
    };
    produk.tampilkan();
}
```

**Hasil:** `Buku — stok 10`. Metode tidak mengambil ownership atas `produk`, sehingga produk masih bisa digunakan.

### 3. 📝 Tugas Jawaban

**Pertanyaan 1:** Apa keuntungan menyimpan data terkait dalam `struct` dibandingkan variabel yang terpisah?

```text
Jawaban:


```

**Pertanyaan 2:** Kapan sebuah metode perlu menggunakan `&mut self`, bukan `&self`?

```text
Jawaban:


```

### 4. 🛠️ Tugas Praktik

```rust
struct Buku {
    halaman: i32,
}

fn main() {
    let buku = Buku { halaman: 100 };
    println!("Halaman: {}", buku.halaman);
}
```

**Misi modifikasi:**

1. Pada **baris ke-6**, ubah deklarasinya menjadi `let mut buku = Buku { halaman: 100 };`.
2. Tambahkan `buku.halaman = 120;` tepat setelah deklarasi tersebut.
3. Jalankan ulang. **Target: `Halaman: 120`.**

### 5. 🎯 Hands-On Challenge — Inventaris Toko

**Studi kasus:** Toko ingin menghitung nilai persediaan dan menambah stok produk melalui metode.

**Template:**

```rust
struct Produk {
    nama: String,
    harga: i32,
    stok: i32,
}

impl Produk {
    fn tampilkan(&self) {
        println!("{}: Rp{}; stok {}", self.nama, self.harga, self.stok);
    }
}

fn main() {
    let produk = Produk {
        nama: String::from("Buku"),
        harga: 12000,
        stok: 5,
    };
    produk.tampilkan();
}
```

**Misi:**

1. Tambahkan metode `nilai_persediaan(&self) -> i32` dengan hasil `harga * stok`.
2. Tambahkan `tambah_stok(&mut self, jumlah: i32)` yang menambah field stok.
3. Jadikan `produk` mutable, cetak nilai persediaan awal, tambah stok sebanyak `3`, lalu cetak data dan nilai persediaan terbaru.
4. Gunakan jumlah tambahan positif untuk latihan ini.

**Target:** nilai awal `60000`; stok akhir `8`; nilai akhir **`96000`**.

### 6. 💻 Exercises

#### ✅ Exercise: Level 1 — Membentuk Data

1. Buat `Siswa` dengan field `nama: String` dan `nilai: i32`. Buat dua instance dan cetak datanya.
2. Tambahkan metode `lulus(&self) -> bool` pada `Siswa` dengan batas nilai `75`. Uji nilai `74` dan `75`.
3. Buat `Persegi` dengan field `sisi: i32` dan metode `luas(&self) -> i32`. Sisi `4` harus menghasilkan **`16`**.

#### ✅ Exercise: Level 2 — Data dan Perilaku

1. Buat `Rekening` dengan field `pemilik: String` dan `saldo: i32`. Tambahkan metode `setor(&mut self, jumlah: i32)`; untuk latihan ini gunakan jumlah positif.
2. Tambahkan `tarik(&mut self, jumlah: i32) -> bool`: kurangi saldo hanya jika jumlah positif dan saldo cukup. Gunakan `&&` untuk menggabungkan kondisi. Dari saldo `20000`, tarik `5000`, lalu coba tarik `20000`. **Target akhir: saldo `15000`, hasil `true` lalu `false`.**
3. Buat `PersegiPanjang` dengan `panjang: f64` dan `lebar: f64`, serta metode `luas` dan `keliling`. Untuk ukuran `3.0 × 2.0`, **target: `6.0` dan `10.0`**.

### 7. 🐛 Tugas Debugging (Akhir Materi)

Metode berikut mengubah field, tetapi jenis peminjamannya belum sesuai. **Perbaiki penerima metode agar perubahan diizinkan.** Pertahankan penambahan stok dan deklarasi mutable di `main`.

```rust
struct Produk {
    stok: i32,
}

impl Produk {
    fn tambah(&self) {
        self.stok = self.stok + 1;
    }
}

fn main() {
    let mut produk = Produk { stok: 2 };
    produk.tambah();
    println!("Stok: {}", produk.stok);
}
```

> **Target selesai:** berhasil dikompilasi dan mencetak `Stok: 3`.

---

## Bab 8 — Enum & Match

### 1. Judul Materi & Penjelasan

**`enum`** menyatakan bahwa sebuah nilai memiliki salah satu dari beberapa kemungkinan, yang disebut **variant**. Bayangkan status lampu lalu lintas: pada satu saat, nilainya bisa `Red`, `Yellow`, atau `Green`.

Berbeda dengan `struct` yang menggabungkan field, `enum` memilih **salah satu bentuk**. Variant juga boleh menyimpan data, misalnya `CreditCard(String)`.

Gunakan **`match`** untuk menjalankan tindakan sesuai variant. Semua kemungkinan harus ditangani (*exhaustive*). Setiap cabang memakai `pola => ekspresi`. Jika `match` menghasilkan nilai, tipe hasil tiap cabangnya harus cocok.

> Untuk belajar, tulis setiap variant secara eksplisit. Pola `_` dapat menangkap kemungkinan yang tersisa, tetapi jangan memakainya untuk menyembunyikan variant yang seharusnya kamu tangani dengan sengaja.

### 2. Contoh Kode Dasar

```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn duration(light: TrafficLight) -> i32 {
    match light {
        TrafficLight::Red => 30,
        TrafficLight::Yellow => 5,
        TrafficLight::Green => 20,
    }
}

fn main() {
    println!("Merah: {} detik", duration(TrafficLight::Red));
    println!("Kuning: {} detik", duration(TrafficLight::Yellow));
    println!("Hijau: {} detik", duration(TrafficLight::Green));
}
```

**Hasil:** durasi merah `30`, kuning `5`, dan hijau `20` detik.

### 3. 📝 Tugas Jawaban

**Pertanyaan 1:** Kapan kamu memilih `enum`, dan kapan memilih `struct`?

```text
Jawaban:


```

**Pertanyaan 2:** Mengapa compiler meminta semua kemungkinan dalam `match` ditangani?

```text
Jawaban:


```

### 4. 🛠️ Tugas Praktik

```rust
enum Weather {
    Sunny,
}

fn main() {
    let weather = Weather::Sunny;
    match weather {
        Weather::Sunny => println!("Pakai topi."),
    }
}
```

**Misi modifikasi:**

1. Setelah **baris ke-2**, tambahkan variant `Rainy,`.
2. Setelah cabang pada **baris ke-8 kode awal**, tambahkan `Weather::Rainy => println!("Bawa payung."),`.
3. Pada **baris ke-6 kode awal**, ganti pilihan menjadi `Weather::Rainy`. Selesaikan semua perubahan sebelum menjalankan.

**Target:** `Bawa payung.`. Coba pilih `Sunny` lagi untuk menguji cabang lainnya.

### 5. 🎯 Hands-On Challenge — Metode Pembayaran

**Studi kasus:** Kasir mendukung pembayaran tunai dan kartu. Program perlu mencetak jenis pembayaran serta data yang dibawa variant tersebut.

**Template:**

```rust
enum PaymentMethod {
    CreditCard(String),
    Cash,
}

fn print_payment_method(method: PaymentMethod) {
    match method {
        PaymentMethod::CreditCard(last_four) => {
            println!("Kartu kredit berakhiran {}", last_four);
        }
        PaymentMethod::Cash => println!("Bayar tunai"),
    }
}

fn main() {
    let payment = PaymentMethod::CreditCard(String::from("3456"));
    print_payment_method(payment);
    print_payment_method(PaymentMethod::Cash);
}
```

**Misi:**

1. Tambahkan variant `DebitCard(String)` dan `PayPal`.
2. Tambahkan cabang `match` yang mencetak informasi masing-masing variant baru.
3. Uji keempat variant lewat empat pemanggilan fungsi. Pakai data fiktif dan hanya empat digit terakhir kartu, bukan nomor kartu sungguhan.
4. Jelaskan melalui komentar bahwa fungsi mengambil ownership atas nilai `PaymentMethod`; nilai itu tidak dipakai lagi setelah pemanggilan.

**Target:** empat keluaran berbeda: kartu kredit, kartu debit, tunai, dan PayPal.

### 6. 💻 Exercises

#### ✅ Exercise: Level 1 — Memilih Variant

1. Buat `Weather` dengan `Sunny`, `Rainy`, `Cloudy`, dan `Windy`. Buat fungsi yang mencetak saran aktivitas untuk setiap variant; uji semuanya.
2. Buat `TrafficLight` dan fungsi durasi seperti contoh. Ubah durasi merah menjadi `40`, lalu uji tiga variant.
3. Buat `Device` dengan `Laptop(String)`, `Tablet(String)`, dan `Phone(String)`. Buat fungsi `display_device(device: Device)` yang mencetak jenis dan modelnya.

#### ✅ Exercise: Level 2 — Enum dengan Perilaku

1. Buat `OrderStatus` dengan `Pending`, `Shipped`, dan `Delivered`. Di dalam `impl OrderStatus`, buat `tampilkan(&self)` dengan `match self`. Uji semua status.
2. Buat `FileFormat` dengan `Pdf(i32)`, `Word(i32)`, dan `Excel(i32)` untuk menyimpan ukuran dalam KB. Buat fungsi yang mengambil enum sebagai input dan mencetak format serta ukuran; uji ketiganya.
3. Buat `Shape` dengan `Circle(f64)` dan `Rectangle(f64, f64)`. Buat `area(shape: Shape) -> f64` memakai `match`; gunakan `std::f64::consts::PI` untuk lingkaran. Persegi panjang `3.0 × 4.0` harus menghasilkan `12.0`; lingkaran berjari-jari `1.0` mendekati `3.14159`.

### 7. 🐛 Tugas Debugging (Akhir Materi)

Program berikut belum menangani semua kemungkinan. **Tambahkan cabang yang hilang tanpa memakai `_`**, lalu uji kedua variant.

```rust
enum Status {
    Active,
    Inactive,
}

fn print_status(status: Status) {
    match status {
        Status::Active => println!("Aktif"),
    }
}

fn main() {
    print_status(Status::Active);
    print_status(Status::Inactive);
}
```

> **Target selesai:** berhasil dikompilasi; mencetak `Aktif`, lalu `Tidak aktif`.

---

## Bab 9 — Koleksi Dinamis dengan Vec

### 1. Judul Materi & Penjelasan

**`Vec<T>`** adalah koleksi yang bisa bertambah atau berkurang dan menyimpan elemen bertipe sama. Bayangkan daftar belanja yang bisa kamu tambahi sewaktu-waktu.

- `vec![10, 20, 30]` membuat vector berisi nilai awal.
- `Vec::<i32>::new()` membuat vector kosong untuk bilangan `i32`.
- `.push(nilai)` menambahkan elemen di akhir; pemilik vector harus mutable.
- `.len()` mengembalikan jumlah elemen dengan tipe `usize`.
- `for nilai in &daftar` membaca elemen tanpa mengambil ownership vector.
- `for nilai in &mut daftar` memungkinkan perubahan tiap elemen melalui `*nilai`.

**Penting:** `for nilai in daftar` tanpa `&` mengonsumsi vector, sehingga vector asal tidak bisa dipakai lagi. Indeks dimulai dari `0`; akses `daftar[indeks]` dapat membuat program *panic* jika indeks di luar batas. Di bab ini kita fokus pada iterasi; akses aman dengan `.get()` hadir pada Bab 10.

### 2. Contoh Kode Dasar

```rust
fn main() {
    let mut nilai = vec![70, 80];
    nilai.push(90);

    for angka in &nilai {
        println!("Nilai: {}", angka);
    }

    println!("Jumlah data: {}", nilai.len());
}
```

**Hasil:** nilai `70`, `80`, dan `90` tercetak; jumlah data `3`.

### 3. 📝 Tugas Jawaban

**Pertanyaan 1:** Mengapa `Vec<i32>` tidak bisa menyimpan `String` sebagai salah satu elemennya?

```text
Jawaban:


```

**Pertanyaan 2:** Apa perbedaan dampak `for angka in nilai` dan `for angka in &nilai` terhadap penggunaan vector setelah perulangan?

```text
Jawaban:


```

### 4. 🛠️ Tugas Praktik

```rust
fn main() {
    let mut harga = vec![10000, 20000];
    for nilai in &harga {
        println!("Harga: {}", nilai);
    }
}
```

**Misi modifikasi:**

1. Sisipkan `harga.push(15000);` setelah **baris ke-2**.
2. Tepat setelah deklarasi dan penambahan tadi, sebelum perulangan pencetak, tambahkan perulangan tiga baris: `for nilai in &mut harga {`, lalu `*nilai = *nilai + 1000;`, lalu `}`.
3. Pertahankan perulangan pencetak yang memakai `&harga`.

**Target:** harga `11000`, `21000`, dan `16000` tercetak berurutan.

### 5. 🎯 Hands-On Challenge — Rekap Belanja

**Studi kasus:** Kamu mencatat harga barang yang dibeli, lalu menghitung total pembayaran tanpa menghabiskan vector-nya.

**Template:**

```rust
fn main() {
    let mut harga = vec![12000, 8000];
    harga.push(5000);

    for nilai in &harga {
        println!("Barang: Rp{}", nilai);
    }
    println!("Jumlah barang: {}", harga.len());
}
```

**Misi:**

1. Buat `total: i32` mutable bernilai `0` sebelum perulangan.
2. Di dalam perulangan, tambahkan `*nilai` ke `total`.
3. Cetak total setelah perulangan. **Target awal: `25000`.**
4. Tambahkan barang baru seharga `15000` menggunakan `.push()` **sebelum** perulangan penghitungan, lalu jalankan ulang.

**Target akhir:** jumlah barang `4` dan total **`40000`**. Vector masih bisa dipakai untuk memanggil `.len()` setelah perulangan.

### 6. 💻 Exercises

#### ✅ Exercise: Level 1 — Menambah dan Membaca

1. Buat vector kosong `Vec::<i32>::new()`, lalu tambahkan angka `10`, `20`, dan `30`. Cetak jumlah elemennya. **Target: `3`.**
2. Buat `Vec<String>` berisi tiga nama menggunakan `vec!` dan `String::from`. Cetak semua nama melalui `&daftar`, lalu cetak jumlahnya.
3. Buat vector `[2, 4, 6]` dengan `vec!`, lalu gandakan setiap elemen melalui `&mut`. **Target akhir: `4`, `8`, `12`.**

#### ✅ Exercise: Level 2 — Mengolah Kumpulan Data

1. Dari `vec![60, 80, 75, 90]`, hitung peserta bernilai `>= 75`. Saat mengulang melalui referensi, gunakan `*nilai` pada perbandingan. **Target: `3`.**
2. Buat vector hasil kosong dan isi dengan angka genap dari `vec![1, 2, 3, 4, 5, 6]`. Gunakan `for`, `if`, dan `.push(*angka)`. Pertahankan vector sumber. **Target hasil: `2`, `4`, `6`.**
3. Gunakan kembali bentuk `struct Produk { nama: String, harga: i32, stok: i32 }`. Buat `Vec<Produk>` berisi Buku (`12000`, stok `2`) dan Pena (`5000`, stok `3`). Iterasi dengan referensi untuk mencetak nama dan menjumlahkan nilai persediaan. **Target total: `39000`.**

### 7. 🐛 Tugas Debugging (Akhir Materi)

Perulangan berikut mengambil ownership vector, padahal jumlah elemennya masih ingin dibaca. **Ubah sumber perulangan menjadi peminjaman bersama.** Pertahankan pencetakan jumlah setelah perulangan.

```rust
fn main() {
    let angka = vec![1, 2, 3];
    for nilai in angka {
        println!("{}", nilai);
    }
    println!("Jumlah: {}", angka.len());
}
```

> **Target selesai:** berhasil dikompilasi; angka `1`, `2`, `3` dan `Jumlah: 3` tercetak.

---

## Bab 10 — Menangani Ketiadaan dan Kegagalan dengan Option & Result

### 1. Judul Materi & Penjelasan

Tidak semua pencarian menemukan data dan tidak semua operasi berhasil. Rust membuat kemungkinan ini terlihat jelas melalui dua enum bawaan yang dapat langsung digunakan tanpa definisi atau impor tambahan:

| Tipe | Variant | Makna |
| --- | --- | --- |
| `Option<T>` | `Some(nilai)` / `None` | Ada nilai / tidak ada nilai |
| `Result<T, E>` | `Ok(nilai)` / `Err(error)` | Operasi berhasil / operasi gagal dengan informasi error |

`T` dan `E` adalah penanda tipe. Misalnya, `Option<i32>` mungkin berisi angka, sedangkan `Result<i32, String>` bisa berisi angka sukses atau pesan kesalahan.

- `daftar.get(indeks)` mengembalikan `Option<&T>`: referensi ke elemen jika tersedia, atau `None` jika indeks di luar batas.
- `teks.parse::<i32>()` mencoba mengubah teks menjadi angka `i32` dan menghasilkan `Result`.
- Gunakan `match` untuk menangani semua kemungkinan secara eksplisit.

> Hindari mengandalkan `.unwrap()` untuk data yang bisa tidak ada atau gagal: pemanggilan itu akan *panic* pada `None` atau `Err`. Pada bab ini, kita berlatih menangani kasus tersebut, bukan mengabaikannya. Operator `?` bisa dipelajari setelah kamu nyaman membaca alur `Result`.

### 2. Contoh Kode Dasar

```rust
fn main() {
    let nilai = vec![80, 90];
    match nilai.get(1) {
        Some(angka) => println!("Nilai ditemukan: {}", angka),
        None => println!("Nilai tidak ditemukan"),
    }

    match "42".parse::<i32>() {
        Ok(angka) => println!("Angka: {}", angka),
        Err(error) => println!("Gagal membaca angka: {}", error),
    }
}
```

**Hasil:** nilai ditemukan `90`, lalu angka `42`. Coba pikirkan cabang yang akan dipilih jika indeksnya `9` atau teksnya `"halo"`.

### 3. 📝 Tugas Jawaban

**Pertanyaan 1:** Apa perbedaan penggunaan `Option<T>` dan `Result<T, E>`?

```text
Jawaban:


```

**Pertanyaan 2:** Mengapa `.get()` dan `match` lebih aman daripada indeks langsung ketika kamu belum yakin elemen tersebut tersedia?

```text
Jawaban:


```

### 4. 🛠️ Tugas Praktik

```rust
fn main() {
    let teks = "12";
    match teks.parse::<i32>() {
        Ok(angka) => println!("Jumlah: {}", angka),
        Err(_) => println!("Input harus berupa angka"),
    }
}
```

**Misi modifikasi:**

1. Pada **baris ke-2**, ganti `"12"` menjadi `"dua belas"`. Jalankan; **target: `Input harus berupa angka`** tanpa panic.
2. Pada **baris ke-5**, ganti `Err(_)` menjadi `Err(error)` dan ubah isinya menjadi `println!("Input tidak valid: {}", error)` agar detail kesalahan ikut ditampilkan.
3. Kembalikan teks menjadi `"12"`. **Target: `Jumlah: 12`.**

### 5. 🎯 Hands-On Challenge — Pemilih Jumlah Pesanan

**Studi kasus:** Aplikasi menerima jumlah pesanan dalam bentuk teks. Program harus membedakan input valid, angka yang tidak diperbolehkan, dan teks yang tidak bisa dibaca sebagai angka.

**Template:**

```rust
fn baca_jumlah(teks: &str) -> Result<i32, String> {
    match teks.parse::<i32>() {
        Ok(jumlah) => Ok(jumlah),
        Err(_) => Err(String::from("Jumlah harus berupa angka")),
    }
}

fn main() {
    match baca_jumlah("3") {
        Ok(jumlah) => println!("Pesanan: {} barang", jumlah),
        Err(pesan) => println!("Pesanan ditolak: {}", pesan),
    }
}
```

**Misi:**

1. Pada cabang `Ok(jumlah)` dalam `baca_jumlah`, tambahkan `if`: jika jumlah `> 0`, kembalikan `Ok(jumlah)`; selain itu, kembalikan `Err(String::from("Jumlah harus lebih dari nol"))`.
2. Uji `"3"`, `"0"`, `"-2"`, dan `"tiga"` dengan mengganti argumen pada `main` dan menjalankan ulang setiap kali.
3. Pada cabang sukses di `main`, hitung biaya dengan harga satuan `12000` dan cetak totalnya. Untuk latihan, pakai angka kecil agar perkalian berada dalam rentang `i32`.
4. Jangan memakai `.unwrap()`; semua kegagalan harus menjadi pesan yang bisa dibaca.

**Target:** `"3"` berhasil dengan total **`36000`**; `"0"` dan `"-2"` ditolak karena tidak positif; `"tiga"` ditolak karena bukan angka.

### 6. 💻 Exercises

#### ✅ Exercise: Level 1 — Semua Kemungkinan Ditangani

1. Buat `Vec<String>` berisi dua nama. Gunakan `.get(0)` dan `.get(5)` dalam dua `match` terpisah. **Target: nama pertama ditemukan; indeks `5` menghasilkan pesan tidak ditemukan.**
2. Coba parsing `"25"` dan `"dua"` ke `i32`. Tangani `Ok` dan `Err` tanpa `.unwrap()`.
3. Buat `cari_diskon(member: bool) -> Option<i32>`. Kembalikan `Some(10)` jika member, atau `None` jika bukan. Cetak informasi diskon menggunakan `match` untuk kedua kondisi.

#### ✅ Exercise: Level 2 — Validasi dalam Fungsi

1. Buat `bagi(a: i32, b: i32) -> Result<f64, String>`. Tolak pembagi `0`; selain itu, konversikan kedua angka ke `f64` sebelum pembagian. **Target: `(7, 2)` menghasilkan `3.5`; `(7, 0)` menghasilkan pesan error.**
2. Buat `cek_stok(stok: i32, jumlah: i32) -> Result<i32, String>`. Untuk stok awal tidak negatif, tolak jumlah `<= 0` atau jumlah melebihi stok; selain itu kembalikan sisa stok. **Target: `(10, 3)` → `Ok(7)`; `(10, 12)` dan `(10, 0)` → `Err`.**
3. Buat vector teks dengan `vec!["10", "salah", "20"]`. Iterasi dengan referensi, parsing tiap elemen ke `i32`, dan jumlahkan hanya yang berhasil. Pada cabang gagal, cetak input yang dilewati. **Target total: `30`; input `salah` dilaporkan tanpa menghentikan program.**

### 7. 🐛 Tugas Debugging (Akhir Materi)

`.get()` tidak langsung memberikan angka; hasilnya mungkin berisi referensi atau mungkin tidak ada. **Ganti penugasan yang salah dengan `match angka.get(1)` dan tangani `Some` serta `None`.** Pada cabang `Some(nilai)`, cetak angkanya. Jangan memakai `.unwrap()`.

```rust
fn main() {
    let angka = vec![10, 20];
    let pilihan: i32 = angka.get(1);
    println!("Pilihan: {}", pilihan);
}
```

> **Target selesai:** berhasil dikompilasi dan mencetak `Pilihan: 20`. Setelah itu, ubah indeks menjadi `9`; program harus menampilkan pesan tidak ditemukan, bukan panic.

---

## ✅ Checklist Kelulusan Bab 6–10

- [ ] Saya dapat membedakan referensi bersama dan referensi mutable.
- [ ] Saya dapat mengubah nilai melalui peminjaman tanpa mengambil ownership.
- [ ] Saya dapat membuat `struct`, instance, dan metode `&self` serta `&mut self`.
- [ ] Saya dapat membuat `enum` dengan data dan menangani semua variant menggunakan `match`.
- [ ] Saya dapat menambah, membaca, dan mengubah elemen `Vec` dengan peminjaman yang tepat.
- [ ] Saya dapat menangani `Some` / `None` dan `Ok` / `Err` tanpa mengandalkan `.unwrap()`.
- [ ] Saya sudah menyelesaikan lima studi kasus, latihan Level 1–2, dan lima debugging.

**Arah berikutnya:** modules dan visibilitas, unit testing dengan `cargo test`, koleksi `HashMap`, closures dan iterators, lalu traits dan generics. Setelah itu, gabungkan kemampuanmu menjadi proyek CLI kecil.

> Kamu tidak harus langsung hafal semua bentuk tipe. Saat buntu, tanyakan: **siapa pemilik data ini, apakah aku perlu membaca atau mengubahnya, dan kemungkinan hasil apa saja yang harus kutangani?** Tiga pertanyaan itu akan sering membantumu. Semangat, Rustacean! 🦀
