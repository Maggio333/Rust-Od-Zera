# 02. Iterators (Iteratory) 🦀

**Poziom:** Zaawansowany  
**Wymagana wiedza:** [01-closures.md](01-closures.md)

---

## Wprowadzenie

**Iteratory** pozwalają na przetwarzanie sekwencji elementów w sposób funkcjonalny.

## Podstawowe Użycie

```rust
let v1 = vec![1, 2, 3];
let v1_iter = v1.iter();

for val in v1_iter {
    println!("{}", val);
}
```

## Metody Iteratora

```rust
let v1: Vec<i32> = vec![1, 2, 3];
let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
```

## map, filter, collect

```rust
let liczby = vec![1, 2, 3, 4, 5];
let parzyste: Vec<_> = liczby
    .iter()
    .filter(|x| *x % 2 == 0)
    .map(|x| x * 2)
    .collect();
```

## 💡 Porównanie z C#

**C#:**
```csharp
var result = numbers
    .Where(x => x % 2 == 0)
    .Select(x => x * 2)
    .ToList();
```

**Rust:**
```rust
let result: Vec<_> = numbers
    .iter()
    .filter(|x| *x % 2 == 0)
    .map(|x| x * 2)
    .collect();
```

## Ćwiczenia

1. Użyj iteratora do filtrowania i mapowania
2. Stwórz własny iterator

---

**Autor:** Arkadiusz Słota  
**Licencja:** CC BY-NC-SA 4.0

