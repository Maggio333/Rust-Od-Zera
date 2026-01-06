# 04. Modules (Moduły) 🦀

**Poziom:** Zaawansowany  
**Wymagana wiedza:** [03-smart-pointers.md](03-smart-pointers.md)

---

## Wprowadzenie

**Moduły** pozwalają na organizację kodu w logiczne grupy.

## Definiowanie Modułu

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
```

## Używanie Modułów

```rust
use crate::front_of_house::hosting;

fn main() {
    hosting::add_to_waitlist();
}
```

## Public vs Private

```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
}
```

## 💡 Porównanie z C#

**C#:**
```csharp
namespace FrontOfHouse {
    public class Hosting {
        public static void AddToWaitlist() {}
    }
}
```

**Rust:**
```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
```

## Ćwiczenia

1. Stwórz moduł z funkcjami pomocniczymi
2. Użyj `pub` do kontroli widoczności

---

**Autor:** Arkadiusz Słota  
**Licencja:** CC BY-NC-SA 4.0

