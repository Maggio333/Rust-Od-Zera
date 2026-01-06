# 03. Propagacja Błędów 🦀

**Poziom:** Średniozaawansowany  
**Wymagana wiedza:** [02-option-type.md](02-option-type.md)

---

## Wprowadzenie

Operator **`?`** pozwala na łatwą propagację błędów bez boilerplate code.

## Operator ?

```rust
use std::fs::File;
use std::io;
use std::io::Read;

fn przeczytaj_plik() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

## Równoważność

```rust
// To:
let mut f = File::open("hello.txt")?;

// Jest równoważne z:
let mut f = match File::open("hello.txt") {
    Ok(file) => file,
    Err(e) => return Err(e),
};
```

## 💡 Porównanie z C#

**C#:**
```csharp
try {
    var content = File.ReadAllText("hello.txt");
} catch (Exception e) {
    throw;
}
```

**Rust:**
```rust
let content = przeczytaj_plik()?;
```

## Ćwiczenia

1. Stwórz funkcję używającą `?` do propagacji błędów
2. Połącz kilka operacji z `Result` używając `?`
3. Przetestuj różne scenariusze błędów

---

**Autor:** Arkadiusz Słota  
**Licencja:** CC BY-NC-SA 4.0

