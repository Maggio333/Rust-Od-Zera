# 04. String vs &str 🦀

**Poziom:** Średniozaawansowany  
**Wymagana wiedza:** [02-borrowing.md](02-borrowing.md)

---

## Wprowadzenie

Rust ma dwa główne typy stringów: **`String`** i **`&str`**. Zrozumienie różnicy jest kluczowe.

## String

**`String`** to typ na heap, który może być modyfikowany:

```rust
let mut s = String::from("hello");
s.push_str(", world");
println!("{}", s);  // "hello, world"
```

### Właściwości String

- Na heap (dynamiczny rozmiar)
- Modyfikowalny (z `mut`)
- Ma ownership
- Może rosnąć/skurczać się

## &str

**`&str`** to "string slice" - referencja do sekwencji UTF-8:

```rust
let s: &str = "hello";  // String literal
```

### Właściwości &str

- Referencja (nie ma ownership)
- Niezmienny
- Może wskazywać na `String` lub string literal

## Konwersje

### String -> &str

```rust
let s = String::from("hello");
let slice: &str = &s;  // Automatyczna konwersja
let slice2: &str = s.as_str();  // Explicit
```

### &str -> String

```rust
let s: &str = "hello";
let owned: String = s.to_string();
let owned2: String = String::from(s);
```

## Kiedy używać którego?

- **`String`** - gdy potrzebujesz modyfikować lub posiadać string
- **`&str`** - gdy potrzebujesz tylko czytać string (parametry funkcji)

## 💡 Porównanie z C#

**C#:**
```csharp
string s = "hello";  // String (immutable, ale reference type)
```

**Rust:**
```rust
let s = String::from("hello");  // String (mutable, owned)
let slice: &str = "hello";      // &str (immutable, borrowed)
```

## 🐍 Porównanie z Pythonem

**Python:**
```python
s = "hello"  # str (immutable)
```

**Rust:**
```rust
let s = String::from("hello");  // String (mutable, owned)
let slice: &str = "hello";      // &str (immutable, borrowed)
```

## Ćwiczenia

1. Stwórz funkcję przyjmującą `&str` i zwracającą `String`
2. Przetestuj konwersje między `String` a `&str`

---

**Autor:** Arkadiusz Słota  
**Licencja:** CC BY-NC-SA 4.0

