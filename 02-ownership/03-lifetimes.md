# 03. Lifetimes (Czasy Życia) 🦀

**Poziom:** Zaawansowany  
**Wymagana wiedza:** [02-borrowing.md](02-borrowing.md)

---

## Wprowadzenie

**Lifetimes** (*czasy życia*) to sposób Rust na zapewnienie, że referencje są zawsze ważne. To zaawansowany koncept, ale kluczowy dla zrozumienia Rust.

## Co to jest Lifetime?

**Lifetime** to zakres, w którym referencja jest ważna. Rust używa lifetimes, aby zapobiec dangling references.

### Podstawowa Składnia

```rust
&i32        // Referencja
&'a i32     // Referencja z lifetime 'a
&'a mut i32 // Mutable referencja z lifetime 'a
```

## Przykład z Lifetime

```rust
fn najdluzszy<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("długi string");
    let string2 = "xyz";
    
    let wynik = najdluzszy(string1.as_str(), string2);
    println!("Najdłuższy string: {}", wynik);
}
```

### Terminologia

- **`'a`** - lifetime annotation (adnotacja czasu życia)
- **Lifetime elision** (*elizja lifetimes*) - Rust automatycznie wnioskuje lifetimes w prostych przypadkach

## Lifetime Elision

W wielu przypadkach Rust automatycznie wnioskuje lifetimes:

```rust
// To:
fn pierwszy(s: &str) -> &str {
    // ...
}

// Jest automatycznie rozwijane do:
fn pierwszy<'a>(s: &'a str) -> &'a str {
    // ...
}
```

## Static Lifetime

**`'static`** oznacza, że referencja jest ważna przez cały czas działania programu:

```rust
let s: &'static str = "Mam static lifetime.";
```

## 💡 Porównanie z C#

C# nie ma explicite lifetimes - GC zarządza pamięcią automatycznie.

## 🐍 Porównanie z Pythonem

Python nie ma explicite lifetimes - GC zarządza pamięcią automatycznie.

## Ćwiczenia

1. Stwórz funkcję z explicit lifetime annotations
2. Przetestuj różne scenariusze z lifetimes

---

**Autor:** Arkadiusz Słota  
**Licencja:** CC BY-NC-SA 4.0

