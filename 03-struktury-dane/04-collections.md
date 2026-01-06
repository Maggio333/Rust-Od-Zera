# 04. Collections (Kolekcje) 🦀

**Poziom:** Średniozaawansowany  
**Wymagana wiedza:** [03-pattern-matching.md](03-pattern-matching.md)

---

## Wprowadzenie

Rust ma kilka przydatnych kolekcji do przechowywania wielu wartości.

## Vec<T> - Wektor

```rust
let mut v = Vec::new();
v.push(1);
v.push(2);
v.push(3);

let v2 = vec![1, 2, 3];  // Makro do tworzenia
```

## HashMap<K, V>

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Niebiescy"), 10);
scores.insert(String::from("Żółci"), 50);
```

## String

```rust
let mut s = String::from("hello");
s.push_str(", world");
```

## 💡 Porównanie z C#

**C#:**
```csharp
List<int> list = new List<int>();
Dictionary<string, int> dict = new Dictionary<string, int>();
```

**Rust:**
```rust
let mut vec = Vec::new();
let mut map = HashMap::new();
```

## 🐍 Porównanie z Pythonem

**Python:**
```python
list = []
dict = {}
```

**Rust:**
```rust
let mut vec = Vec::new();
let mut map = HashMap::new();
```

## Ćwiczenia

1. Stwórz `Vec<i32>` z liczbami 1-10
2. Stwórz `HashMap<String, i32>` do przechowywania wyników
3. Iteruj po kolekcjach używając `for`

---

**Autor:** Arkadiusz Słota  
**Licencja:** CC BY-NC-SA 4.0

