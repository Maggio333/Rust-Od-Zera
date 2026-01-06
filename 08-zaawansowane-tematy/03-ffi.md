# 03. FFI (Foreign Function Interface) 🦀

**Poziom:** Expert  
**Wymagana wiedza:** [02-macros.md](02-macros.md)

---

## Wprowadzenie

**FFI** pozwala na wywoływanie funkcji z innych języków (np. C).

## Wywoływanie C

```rust
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("{}", abs(-3));
    }
}
```

## Eksportowanie do C

```rust
#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

## ⚠️ Ostrzeżenie

FFI wymaga `unsafe` i powinien być używany ostrożnie.

## Ćwiczenia

1. Przeczytaj dokumentację o FFI
2. Zrozum jak Rust współpracuje z C

---

**Autor:** Arkadiusz Słota  
**Licencja:** CC BY-NC-SA 4.0

