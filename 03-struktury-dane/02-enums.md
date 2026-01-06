# 02. Enums 🦀

**Poziom:** Średniozaawansowany  
**Wymagana wiedza:** [01-struktury.md](01-struktury.md)

---

## Wprowadzenie

**Enums** (*wyliczenia*) pozwalają na definiowanie typu, który może mieć jedną z kilku możliwych wartości. W Rust enums są bardzo potężne.

## Podstawowy Enum

```rust
enum Kierunek {
    Północ,
    Południe,
    Wschód,
    Zachód,
}
```

## Enum z Danymi

```rust
enum IPAdres {
    V4(String),
    V6(String),
}

let domowy = IPAdres::V4(String::from("127.0.0.1"));
```

## Enum z Różnymi Typami

```rust
enum Wiadomosc {
    Wyjdz,
    Przesun { x: i32, y: i32 },
    Napisz(String),
    ZmienKolor(u8, u8, u8),
}
```

## Option<T>

**`Option<T>`** to enum wbudowany w Rust:

```rust
enum Option<T> {
    Some(T),
    None,
}

let liczba = Some(5);
let brak = None;
```

## Result<T, E>

**`Result<T, E>`** to enum do obsługi błędów:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

## 💡 Porównanie z C#

**C#:**
```csharp
enum Direction {
    North,
    South,
    East,
    West
}
```

**Rust:**
```rust
enum Direction {
    Północ,
    Południe,
    Wschód,
    Zachód,
}
```

## Ćwiczenia

1. Stwórz enum `Status` z wartościami: `Aktywny`, `Nieaktywny`, `Zawieszony`
2. Stwórz enum `Ksztalt` z wartościami: `Kolo(f64)`, `Prostokat(f64, f64)`, `Kwadrat(f64)`
3. Użyj `Option<i32>` do reprezentowania opcjonalnej liczby

---

**Autor:** Arkadiusz Słota  
**Licencja:** CC BY-NC-SA 4.0

