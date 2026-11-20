<p align="center">
  <img src="img/logo/png/logo_mbo_256.png" alt="MusicBox Organizer Logo" width="128" height="128" />
</p>

<h1 align="center">MusicBox Organizer</h1>

<p align="center">
  <b>Manajer playlist MP3 & tag ID3 desktop yang ringan, cepat, dan portabel untuk perangkat audio jadul/primitif.</b>
</p>

---

## 📌 Ringkasan

**MusicBox Organizer** dibuat khusus untuk mengatasi masalah pengurutan lagu pada pemutar audio primitif (pemutar MP3 portable, head unit mobil, modul mikrokontroler/Arduino, speaker USB/SD) yang membaca file berdasarkan urutan byte ASCII murni tanpa smart-sorting.

Aplikasi secara otomatis menyematkan kode prefix base-26 4-huruf (`[AAAA-namafile.mp3]` hingga `[ZZZZ-namafile.mp3]`) agar lagu selalu diputar berurutan 100% tanpa acak.

## ✨ Fitur Utama

- 🎵 **Penyusunan Playlist Intuitif**: Tambah lagu, pindahkan urutan ke atas/bawah (Up/Down) dengan seleksi tetap aktif.
- 🏷️ **Edit ID3 Tag Langsung**: Edit Title dan Artist secara inline langsung di tabel.
- 💾 **Pemisahan Save & Export**:
  - **Save Project (`.txt`)**: Simpan metadata susunan playlist lokal untuk diedit kembali tanpa menyentuh flashdisk.
  - **Export Media**: Salin lagu dengan penamaan `[XXYY-filename.mp3]` ke USB/SD card.
- 💽 **Deteksi Flashdisk/SD Card**: Otomatis mendeteksi drive USB yang terpasang, kapasitas, sisa ruang, dan sistem berkas (FAT32/exFAT).
- ⚠️ **Format Flashdisk Aman (Two-Step Confirmation)**: Opsi format FAT32/exFAT sebelum ekspor dengan dialog modal konfirmasi eksplisit (`CONFIRM`) demi mencegah kehilangan data tidak sengaja.
- 📄 **Ekspor Multi-Format**: Ekspor daftar putar ke folder `playlist-text/` dalam format `.txt` dan `.rtf` (Rich Text Format).
- 🎧 **Audio Preview**: Dengarkan cuplikan audio langsung di aplikasi dengan tombol Play, Pause, dan Stop.
- ↺ **Undo / Redo Penuh**: Riwayat perubahan hingga 10 langkah untuk pengeditan dan pemindahan lagu yang aman.
- 🌙 **Tema Dark & Light**: Tampilan gelap (Dark Mode) aktif secara default dan tersimpan otomatis tanpa terpengaruh tema sistem OS.
- 🔍 **Audit Kompatibilitas Perangkat**: Periksa apakah susunan file pada drive USB memenuhi standar urutan ASCII murni dan batas panjang path FAT32.

---

## 🚀 Cara Menjalankan

### 1. Prasyarat Sistem
- **Rust**: 1.75+ (`cargo`, `rustc`)
- **Linux**: `libgtk-3-dev`, `libasound2-dev`, `libdbus-1-dev`
- **Windows / macOS**: Didukung kompilasi native

### 2. Jalankan Mode GUI Desktop
```bash
cargo run
# atau
cargo run -- gui
```

### 3. Perintah CLI (Command Line Interface)
Aplikasi menyediakan perintah CLI lengkap tanpa perlu membuka jendela grafis:

| Perintah | Deskripsi |
|---|---|
| `cargo run -- status` | Tampilkan proyek aktif dan daftar USB/SD card yang terhubung |
| `cargo run -- scan [dir]` | Pindai folder untuk file MP3 dan baca ID3 tag |
| `cargo run -- tag <file.mp3> [title] [artist]` | Lihat atau perbarui tag ID3 file audio secara langsung |
| `cargo run -- save [path]` | Simpan proyek playlist secara lokal (`.txt`) |
| `cargo run -- export` | Ekspor playlist langsung ke media USB/SD card yang aktif |
| `cargo run -- export-text [dir]` | Ekspor daftar playlist `.txt` & `.rtf` ke folder `playlist-text/` |
| `cargo run -- check-compat [dir]` | Audit folder/flashdisk untuk kepatuhan urutan ASCII dan FAT32 |

---

## 🛠️ Kompilasi Binary Release

Untuk menghasilkan satu file binary mandiri berukuran sangat kecil (~1.7 MB) tanpa dependensi runtime tambahan:

```bash
cargo build --release
```

Binary hasil build berada di:
```text
target/release/musicbox
```

Jalankan langsung:
```bash
./target/release/musicbox
```

---

## 🧪 Pengujian (Unit & Integration Tests)

Semua logika bisnis, algoritma base-26, parser ID3, verifikasi kompatibilitas, dan alur ekspor telah diuji:

```bash
cargo test
```

---

## 📐 Prinsip Desain (Konstitusi Proyek)

1. **Lightweight First**: Ukuran binary minimal (<50 MB, aktual ~1.7 MB), tanpa Electron, tanpa runtime web.
2. **Cross-Platform Native**: Antarmuka asli per platform (GTK3 di Linux, Win32 di Windows, Cocoa di macOS).
3. **Offline-First & Portabel**: Tanpa koneksi internet, tidak meninggalkan file sampah, single executable.
4. **Device Compatibility**: Penamaan `[XXYY-filename.mp3]` menjamin pemutaran berurutan pada semua jenis firmware audio primitif.

---

## 📄 Lisensi

Distributed under the MIT License. See [LICENSE](LICENSE) for more information.
