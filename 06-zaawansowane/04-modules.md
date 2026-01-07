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

## Standardowa Biblioteka - `std::`

**`std::`** to standardowa biblioteka Rust (*standard library*). Zawiera podstawowe typy, funkcje i struktury, które są dostępne w każdym programie Rust.

### Bezpośrednie Użycie `std::`

Możesz używać `std::` bezpośrednio w kodzie:

```rust
fn main() {
    use std::cmp::Ordering;
    
    let x = 5;
    let y = 10;
    
    match x.cmp(&y) {
        Ordering::Less => println!("x jest mniejsze"),
        Ordering::Greater => println!("x jest większe"),
        Ordering::Equal => println!("x jest równe y"),
    }
}
```

### Import z `use`

Możesz też zaimportować moduły z `std::` na początku pliku:

```rust
use std::cmp::Ordering;

fn main() {
    let x = 5;
    let y = 10;
    
    match x.cmp(&y) {
        Ordering::Less => println!("x jest mniejsze"),
        Ordering::Greater => println!("x jest większe"),
        Ordering::Equal => println!("x jest równe y"),
    }
}
```

### Często Używane Moduły z `std::`

- **`std::cmp`** - porównywanie (`Ordering`, `PartialOrd`, `Ord`)
- **`std::collections`** - kolekcje (`HashMap`, `HashSet`, `Vec`)
- **`std::io`** - operacje I/O (`stdin`, `stdout`, `File`)
- **`std::thread`** - wątki (`spawn`, `JoinHandle`)
- **`std::sync`** - synchronizacja (`Mutex`, `Arc`, `mpsc`)

### Terminologia

- **`std::`** - standardowa biblioteka Rust (*standard library*)
- **Namespace** (*przestrzeń nazw*) - sposób organizacji kodu w logiczne grupy
- **Module** (*moduł*) - jednostka organizacji kodu w Rust
- **`use`** - import modułu lub elementu do bieżącego zakresu

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
3. Użyj `std::cmp::Ordering` do porównywania dwóch liczb
4. Zaimportuj moduł z `std::` używając `use`

---

**Autor:** Arkadiusz Słota  
**Licencja:** CC BY-NC-SA 4.0

