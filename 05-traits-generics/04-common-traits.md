# 04. Common Traits 🦀

**Poziom:** Zaawansowany  
**Wymagana wiedza:** [03-trait-bounds.md](03-trait-bounds.md)

---

## Wprowadzenie

Rust ma wiele wbudowanych traits, które są często używane.

## Clone

```rust
#[derive(Clone)]
struct Punkt {
    x: i32,
    y: i32,
}

let p1 = Punkt { x: 5, y: 10 };
let p2 = p1.clone();
```

## Copy

```rust
#[derive(Copy, Clone)]
struct Punkt {
    x: i32,
    y: i32,
}
```

## Debug

```rust
#[derive(Debug)]
struct User {
    name: String,
}

println!("{:?}", user);
```

## Display

```rust
use std::fmt;

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
```

## Ćwiczenia

1. Zaimplementuj `Debug` dla swojej struktury
2. Zaimplementuj `Display` dla swojej struktury
3. Użyj `#[derive]` dla prostych traits

---

**Autor:** Arkadiusz Słota  
**Licencja:** CC BY-NC-SA 4.0

