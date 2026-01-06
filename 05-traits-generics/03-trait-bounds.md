# 03. Trait Bounds 🦀

**Poziom:** Zaawansowany  
**Wymagana wiedza:** [02-generics.md](02-generics.md)

---

## Wprowadzenie

**Trait bounds** ograniczają generyki do typów implementujących określone traits.

## Podstawowe Trait Bounds

```rust
fn wydrukuj<T: Wyswietl>(item: T) {
    item.wyswietl();
}
```

## Wielokrotne Bounds

```rust
fn porownaj<T: Wyswietl + Porownywalny>(a: T, b: T) {
    // ...
}
```

## Where Klauzula

```rust
fn skomplikowana<T, U>(t: T, u: U) 
where
    T: Wyswietl + Klonowalny,
    U: Wyswietl + Debug,
{
    // ...
}
```

## Ćwiczenia

1. Stwórz funkcję z wieloma trait bounds
2. Użyj `where` klauzuli dla czytelności

---

**Autor:** Arkadiusz Słota  
**Licencja:** CC BY-NC-SA 4.0

