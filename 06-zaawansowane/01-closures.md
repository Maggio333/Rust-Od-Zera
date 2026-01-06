# 01. Closures (Zamknięcia) 🦀

**Poziom:** Zaawansowany  
**Wymagana wiedza:** [03-funkcje.md](../01-podstawy/03-funkcje.md)

---

## Wprowadzenie

**Closures** to anonimowe funkcje, które mogą przechwytywać zmienne z otaczającego zakresu.

## Podstawowy Closure

```rust
let dodaj_jeden = |x| x + 1;
let wynik = dodaj_jeden(5);
```

## Closure z Typami

```rust
let dodaj = |x: i32, y: i32| -> i32 {
    x + y
};
```

## Przechwytywanie Zmiennych

```rust
let x = 4;
let równe_x = |z| z == x;
let y = 4;
assert!(równe_x(y));
```

## 💡 Porównanie z C#

**C#:**
```csharp
Func<int, int> addOne = x => x + 1;
```

**Rust:**
```rust
let dodaj_jeden = |x| x + 1;
```

## Ćwiczenia

1. Stwórz closure dodający dwie liczby
2. Użyj closure z `map` na wektorze

---

**Autor:** Arkadiusz Słota  
**Licencja:** CC BY-NC-SA 4.0

