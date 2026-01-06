# 02. Generics (Generyki) 🦀

**Poziom:** Zaawansowany  
**Wymagana wiedza:** [01-traits-podstawy.md](01-traits-podstawy.md)

---

## Wprowadzenie

**Generics** pozwalają na pisanie kodu, który działa z różnymi typami.

## Funkcje Generyczne

```rust
fn najwiekszy<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}
```

## Struktury Generyczne

```rust
struct Punkt<T> {
    x: T,
    y: T,
}

let integer = Punkt { x: 5, y: 10 };
let float = Punkt { x: 1.0, y: 4.0 };
```

## Enums Generyczne

```rust
enum Option<T> {
    Some(T),
    None,
}
```

## 💡 Porównanie z C#

**C#:**
```csharp
T Largest<T>(T a, T b) where T : IComparable<T> {
    return a.CompareTo(b) > 0 ? a : b;
}
```

**Rust:**
```rust
fn najwiekszy<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}
```

## Ćwiczenia

1. Stwórz generyczną funkcję `zamien<T>(a: &mut T, b: &mut T)`
2. Stwórz generyczną strukturę `Para<T, U>`

---

**Autor:** Arkadiusz Słota  
**Licencja:** CC BY-NC-SA 4.0

