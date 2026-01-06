# 01. Struktury (Structs) 🦀

**Poziom:** Średniozaawansowany  
**Wymagana wiedza:** [04-kontrola-przeplywu.md](../01-podstawy/04-kontrola-przeplywu.md)

---

## Wprowadzenie

**Struktury** (*structs*) pozwalają na grupowanie powiązanych danych. To podstawowy sposób tworzenia własnych typów w Rust.

## Definiowanie Struktury

```rust
struct Użytkownik {
    imie: String,
    email: String,
    wiek: u32,
    aktywny: bool,
}
```

## Tworzenie Instancji

```rust
let użytkownik1 = Użytkownik {
    email: String::from("jan@example.com"),
    imie: String::from("Jan"),
    aktywny: true,
    wiek: 30,
};
```

## Dostęp do Pól

```rust
println!("Imię: {}", użytkownik1.imie);
użytkownik1.wiek = 31;  // Tylko jeśli struct jest mut
```

## Funkcje Asocjacyjne

```rust
impl Użytkownik {
    fn nowy(imie: String, email: String, wiek: u32) -> Użytkownik {
        Użytkownik {
            imie,
            email,
            wiek,
            aktywny: true,
        }
    }
    
    fn wyswietl(&self) {
        println!("{} ({})", self.imie, self.email);
    }
}
```

## Metody

```rust
impl Użytkownik {
    fn pelnoletni(&self) -> bool {
        self.wiek >= 18
    }
    
    fn urodziny(&mut self) {
        self.wiek += 1;
    }
}
```

## 💡 Porównanie z C#

**C#:**
```csharp
class User {
    public string Name { get; set; }
    public string Email { get; set; }
}
```

**Rust:**
```rust
struct User {
    name: String,
    email: String,
}
```

## 🐍 Porównanie z Pythonem

**Python:**
```python
class User:
    def __init__(self, name, email):
        self.name = name
        self.email = email
```

**Rust:**
```rust
struct User {
    name: String,
    email: String,
}
```

## Ćwiczenia

1. Stwórz strukturę `Punkt` z polami `x` i `y` (f64)
2. Dodaj metodę `odleglosc(&self, other: &Punkt) -> f64`
3. Stwórz strukturę `Prostokat` z metodą `pole(&self) -> f64`

---

**Autor:** Arkadiusz Słota  
**Licencja:** CC BY-NC-SA 4.0

