# 03. Smart Pointers 🦀

**Poziom:** Zaawansowany  
**Wymagana wiedza:** [02-ownership/01-ownership-podstawy.md](../02-ownership/01-ownership-podstawy.md)

---

## Wprowadzenie

**Smart pointers** to struktury działające jak wskaźniki, ale z dodatkowymi możliwościami.

## Box<T>

```rust
let b = Box::new(5);
println!("b = {}", b);
```

## Rc<T> - Reference Counting

```rust
use std::rc::Rc;

let a = Rc::new(5);
let b = Rc::clone(&a);
```

## RefCell<T>

```rust
use std::cell::RefCell;

let data = RefCell::new(5);
*data.borrow_mut() = 6;
```

## 💡 Porównanie z C#

**C#:**
```csharp
object obj = new object();
// GC zarządza pamięcią
```

**Rust:**
```rust
let b = Box::new(5);
// Ownership zarządza pamięcią
```

## Ćwiczenia

1. Użyj `Box` do przechowywania danych na heap
2. Eksperymentuj z `Rc` dla shared ownership

---

**Autor:** Arkadiusz Słota  
**Licencja:** CC BY-NC-SA 4.0

