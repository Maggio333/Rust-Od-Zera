# 02. Macros (Makra) 🦀

**Poziom:** Expert  
**Wymagana wiedza:** [01-unsafe-rust.md](01-unsafe-rust.md)

---

## Wprowadzenie

**Macros** pozwalają na metaprogramowanie w Rust.

## Declarative Macros

```rust
macro_rules! powitanie {
    () => {
        println!("Hello!");
    };
}

powitanie!();
```

## Procedural Macros

```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
```

## 💡 Porównanie z C#

**C#:**
```csharp
// Attributes i Reflection
[Debug]
class Point { }
```

**Rust:**
```rust
#[derive(Debug)]
struct Point { }
```

## Ćwiczenia

1. Stwórz proste makro używając `macro_rules!`
2. Użyj `#[derive]` dla automatycznej implementacji traits

---

**Autor:** Arkadiusz Słota  
**Licencja:** CC BY-NC-SA 4.0

