# 01. Result<T, E> - Obsługa Błędów 🦀

**Poziom:** Średniozaawansowany  
**Wymagana wiedza:** [02-enums.md](../03-struktury-dane/02-enums.md)

---

## Wprowadzenie

Rust używa typu **`Result<T, E>`** do obsługi błędów zamiast wyjątków. To bezpieczniejszy i bardziej przewidywalny sposób.

## Result<T, E>

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

## Podstawowe Użycie

```rust
fn dziel(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Dzielenie przez zero!"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    match dziel(10.0, 2.0) {
        Ok(wynik) => println!("Wynik: {}", wynik),
        Err(e) => println!("Błąd: {}", e),
    }
}
```

## unwrap i expect

```rust
let wynik = dziel(10.0, 2.0).unwrap();  // Panikuje jeśli Err
let wynik2 = dziel(10.0, 0.0).expect("Dzielenie przez zero!");  // Z wiadomością
```

## 💡 Porównanie z C#

**C#:**
```csharp
try {
    var result = Divide(10, 0);
} catch (Exception e) {
    Console.WriteLine(e.Message);
}
```

**Rust:**
```rust
match dziel(10.0, 0.0) {
    Ok(r) => println!("{}", r),
    Err(e) => println!("{}", e),
}
```

## Ćwiczenia

1. Stwórz funkcję `pierwiastek(x: f64) -> Result<f64, String>`
2. Użyj `match` do obsługi `Result`
3. Przetestuj `unwrap` i `expect`

---

**Autor:** Arkadiusz Słota  
**Licencja:** CC BY-NC-SA 4.0

