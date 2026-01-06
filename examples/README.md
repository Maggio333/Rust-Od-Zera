# 📚 Przykłady Kodu Rust

> Działające przykłady kodu Rust, zorganizowane zgodnie z dokumentacją edukacyjną.

---

## 🎯 Cel

Ten katalog zawiera działające przykłady kodu Rust, które ilustrują koncepty opisane w dokumentach edukacyjnych. Każdy przykład można uruchomić i przeanalizować.

---

## 📁 Struktura

Przykłady są zorganizowane zgodnie z sekcjami dokumentacji:

```
examples/
├── 01-podstawy-example/     # Przykłady z sekcji "Podstawy"
│   └── 01-witaj-rust/       # Hello World i podstawy
├── 02-ownership-example/    # Przykłady z sekcji "Ownership"
├── 03-struktury-example/    # Przykłady z sekcji "Struktury Danych"
└── ...
```

---

## 🚀 Jak Używać

### Uruchomienie Przykładu

```bash
cd examples/01-podstawy-example/01-witaj-rust
cargo run
```

### Sprawdzenie Kodu

```bash
cargo check
```

### Formatowanie

```bash
cargo fmt
```

---

## 📝 Status Przykładów

### 01-podstawy-example
- ✅ [01-witaj-rust](01-podstawy-example/01-witaj-rust/) - Hello World, podstawy Cargo

---

## ⚠️ Uwagi

- Przykłady są commitowane do repo (publiczne)
- Build artifacts (`target/`) nie są commitowane (w .gitignore)
- `Cargo.lock` nie jest commitowany dla bibliotek, ale jest dla aplikacji binarnych

---

**Autor:** Arkadiusz Słota  
**Licencja:** CC BY-NC-SA 4.0

