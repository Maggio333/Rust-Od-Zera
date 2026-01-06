# 04. Testing (Testowanie) 🦀

**Poziom:** Zaawansowany  
**Wymagana wiedza:** [03-funkcje.md](../01-podstawy/03-funkcje.md)

---

## Wprowadzenie

Rust ma wbudowane wsparcie dla testów jednostkowych i integracyjnych.

## Testy Jednostkowe

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dodawanie() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[should_panic]
    fn test_panikuje() {
        panic!("Test paniki");
    }
}
```

## Testy Integracyjne

```rust
// tests/integration_test.rs
use rust_od_zera;

#[test]
fn test_integracja() {
    assert_eq!(rust_od_zera::dodaj(2, 2), 4);
}
```

## 💡 Porównanie z C#

**C#:**
```csharp
[Test]
public void TestAddition() {
    Assert.AreEqual(4, 2 + 2);
}
```

**Rust:**
```rust
#[test]
fn test_dodawanie() {
    assert_eq!(2 + 2, 4);
}
```

## Ćwiczenia

1. Napisz testy jednostkowe dla swoich funkcji
2. Stwórz testy integracyjne

---

**Autor:** Arkadiusz Słota  
**Licencja:** CC BY-NC-SA 4.0

