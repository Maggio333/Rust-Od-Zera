# 02. Option<T> - Wartości Opcjonalne 🦀

**Poziom:** Średniozaawansowany  
**Wymagana wiedza:** [01-result-type.md](01-result-type.md)

---

## Wprowadzenie

**`Option<T>`** reprezentuje wartość, która może być obecna (`Some`) lub nieobecna (`None`).

## Option<T>

```rust
enum Option<T> {
    Some(T),
    None,
}
```

## Podstawowe Użycie

```rust
fn znajdz_indeks(tablica: &[i32], wartosc: i32) -> Option<usize> {
    for (i, &item) in tablica.iter().enumerate() {
        if item == wartosc {
            return Some(i);
        }
    }
    None
}

fn main() {
    let tablica = [1, 2, 3, 4, 5];
    match znajdz_indeks(&tablica, 3) {
        Some(i) => println!("Znaleziono na indeksie {}", i),
        None => println!("Nie znaleziono"),
    }
}
```

## unwrap i expect

```rust
let wartosc = Some(5);
let wynik = wartosc.unwrap();  // 5

let brak = None;
// brak.unwrap();  // Panika!
```

## 💡 Porównanie z C#

**C#:**
```csharp
int? value = 5;  // Nullable
if (value.HasValue) {
    Console.WriteLine(value.Value);
}
```

**Rust:**
```rust
let value = Some(5);
if let Some(v) = value {
    println!("{}", v);
}
```

## Ćwiczenia

1. Stwórz funkcję zwracającą `Option<i32>`
2. Użyj `match` i `if let` z `Option`
3. Przetestuj `unwrap_or` i `unwrap_or_else`

---

**Autor:** Arkadiusz Słota  
**Licencja:** CC BY-NC-SA 4.0

