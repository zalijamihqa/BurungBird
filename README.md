
# BurungBird

Aplikasi komunitas pecinta burung untuk berbagi informasi, jual beli, dan mencari event lomba burung.

## Fitur

- **Forum Diskusi:**
  - Diskusi antar pengguna.
- **Marketplace:**
  - Tempat jual beli burung dan perlengkapan.
- **Event Finder:**
  - Cari lomba burung terdekat.

## Cara Menjalankan

### Backend

1. Masuk ke direktori `backend/`.
2. Jalankan:
   ```bash
   cargo build --release
   cargo run

###Frontend

1. Instal wasm-pack jika belum:
```bash
cargo install wasm-pack
```
2. Masuk ke direktori frontend/.
3. Bangun frontend:
```bash
wasm-pack build --target web
```
4. Jalankan server lokal:
```bash
python3 -m http.server 8080
```
5. Akses aplikasi di browser: http://localhost:8080.
