# 01. Traits - Podstawy 🦀

**Poziom:** Zaawansowany  
**Wymagana wiedza:** [01-struktury.md](../03-struktury-dane/01-struktury.md)

---

## Wprowadzenie

**Traits** definiują wspólne zachowanie, które typy mogą dzielić. To odpowiednik interfejsów w innych językach.

## Definiowanie Trait

```rust
trait Wyswietl {
    fn wyswietl(&self);
}

struct NewsArticle {
    headline: String,
    location: String,
}

impl Wyswietl for NewsArticle {
    fn wyswietl(&self) {
        println!("{} - {}", self.headline, self.location);
    }
}
```

## Domyślne Implementacje

```rust
trait Wyswietl {
    fn wyswietl(&self) {
        println!("Domyślna implementacja");
    }
}
```

## 💡 Porównanie z C#

**C#:**
```csharp
interface IDisplay {
    void Display();
}
```

**Rust:**
```rust
trait Wyswietl {
    fn wyswietl(&self);
}
```

## Ćwiczenia

1. Stwórz trait `Powierzchnia` z metodą `pole(&self) -> f64`
2. Zaimplementuj trait dla `Kolo` i `Prostokat`

---

**Autor:** Arkadiusz Słota  
**Licencja:** CC BY-NC-SA 4.0

