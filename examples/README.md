# 📚 Przykłady Kodu Rust

> Działające przykłady kodu Rust, zorganizowane zgodnie z dokumentacją edukacyjną.

---

## 🎯 Cel

Ten katalog zawiera działające przykłady kodu Rust, które ilustrują koncepty opisane w dokumentach edukacyjnych. Każdy przykład można uruchomić i przeanalizować.

---

## 📁 Struktura

Przykłady są zorganizowane zgodnie z sekcjami dokumentacji. Każda sekcja ma swój katalog, w którym są konkretne projekty:

```
examples/
├── 01-podstawy-example/          # Przykłady z sekcji "Podstawy"
│   ├── hello_world/              # Hello World
│   └── moj_pierwszy_program/     # Inne przykłady podstaw
├── 02-ownership-example/         # Przykłady z sekcji "Ownership"
├── 03-struktury-example/          # Przykłady z sekcji "Struktury Danych"
├── 04-error-handling-example/     # Przykłady z sekcji "Error Handling"
├── 05-traits-generics-example/    # Przykłady z sekcji "Traits i Generics"
├── 06-zaawansowane-example/       # Przykłady z sekcji "Zaawansowane"
├── 07-concurrency-example/        # Przykłady z sekcji "Concurrency"
└── 08-zaawansowane-tematy-example/ # Przykłady z sekcji "Zaawansowane Tematy"
```

**Nomenklatura projektów:**
- Projekty mają opisowe nazwy (np. `hello_world`, `ownership_demo`)
- Każdy projekt to osobny katalog z własnym `Cargo.toml`
- Struktura zgodna z dokumentacją - łatwe znalezienie przykładu do danego tematu

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
- ✅ [hello_world](01-podstawy-example/hello_world/) - Hello World, podstawy Cargo

### 02-ownership-example
- ⏳ Przykłady w przygotowaniu

### 03-struktury-example
- ⏳ Przykłady w przygotowaniu

### 04-error-handling-example
- ⏳ Przykłady w przygotowaniu

### 05-traits-generics-example
- ⏳ Przykłady w przygotowaniu

### 06-zaawansowane-example
- ⏳ Przykłady w przygotowaniu

### 07-concurrency-example
- ⏳ Przykłady w przygotowaniu

### 08-zaawansowane-tematy-example
- ⏳ Przykłady w przygotowaniu

---

## ⚠️ Uwagi

- Przykłady są commitowane do repo (publiczne)
- Build artifacts (`target/`) nie są commitowane (w .gitignore)
- `Cargo.lock` nie jest commitowany dla bibliotek, ale jest dla aplikacji binarnych

---

**Autor:** Arkadiusz Słota  
**Licencja:** CC BY-NC-SA 4.0

