# 03. Pattern Matching 🦀

**Poziom:** Średniozaawansowany  
**Wymagana wiedza:** [02-enums.md](02-enums.md)

---

## Wprowadzenie

**Pattern matching** to potężne narzędzie w Rust do dekompozycji wartości i dopasowywania wzorców.

## Match z Enum

```rust
enum Kierunek {
    Północ,
    Południe,
    Wschód,
    Zachód,
}

fn kierunek_tekst(k: Kierunek) -> &'static str {
    match k {
        Kierunek::Północ => "Północ",
        Kierunek::Południe => "Południe",
        Kierunek::Wschód => "Wschód",
        Kierunek::Zachód => "Zachód",
    }
}
```

## Match z Danymi

```rust
enum IPAdres {
    V4(String),
    V6(String),
}

fn wersja(ip: IPAdres) -> String {
    match ip {
        IPAdres::V4(addr) => format!("IPv4: {}", addr),
        IPAdres::V6(addr) => format!("IPv6: {}", addr),
    }
}
```

## Match z Option

```rust
fn plus_jeden(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
```

## If Let

```rust
let some_value = Some(3);

if let Some(3) = some_value {
    println!("Trzy!");
}
```

## While Let

```rust
let mut stack = Vec::new();
stack.push(1);
stack.push(2);

while let Some(top) = stack.pop() {
    println!("{}", top);
}
```

## 💡 Porównanie z C#

**C#:**
```csharp
switch (direction) {
    case Direction.North:
        return "North";
    default:
        return "Unknown";
}
```

**Rust:**
```rust
match direction {
    Kierunek::Północ => "Północ",
    _ => "Nieznany",
}
```

## Ćwiczenia

1. Użyj `match` do obsługi `Option<i32>`
2. Użyj `if let` do sprawdzenia czy `Option` ma wartość
3. Stwórz funkcję używającą pattern matching z enum

---

**Autor:** Arkadiusz Słota  
**Licencja:** CC BY-NC-SA 4.0

