# 01. Unsafe Rust 🦀

**Poziom:** Expert  
**Wymagana wiedza:** [04-modules.md](../06-zaawansowane/04-modules.md)

---

## Wprowadzenie

**Unsafe Rust** pozwala na ominięcie niektórych gwarancji bezpieczeństwa Rust, gdy jest to konieczne.

## Unsafe Bloki

```rust
unsafe {
    // unsafe kod
}
```

## Unsafe Funkcje

```rust
unsafe fn niebezpieczna_funkcja() {
    // unsafe kod
}
```

## Raw Pointers

```rust
let mut num = 5;
let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;
```

## ⚠️ Ostrzeżenie

Unsafe Rust powinien być używany tylko gdy jest absolutnie konieczny i z pełną świadomością konsekwencji.

## Ćwiczenia

1. Przeczytaj dokumentację o unsafe Rust
2. Zrozum kiedy unsafe jest potrzebny

---

**Autor:** Arkadiusz Słota  
**Licencja:** CC BY-NC-SA 4.0

