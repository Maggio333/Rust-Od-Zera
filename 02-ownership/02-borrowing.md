# 02. Borrowing (Pożyczanie) 🦀

**Poziom:** Średniozaawansowany  
**Wymagana wiedza:** [01-ownership-podstawy.md](01-ownership-podstawy.md)

---

## Wprowadzenie

**Borrowing** (*pożyczanie*) pozwala na używanie wartości bez przejmowania ownership. To kluczowy mechanizm w Rust, który umożliwia efektywne zarządzanie pamięcią.

## 🧠 Proces Myślenia: Jak Myśleć o Borrowing?

Borrowing to sposób na "pożyczenie" wartości bez przejmowania ownership. Oto jak o tym myśleć:

### Mentalny Model: Pożyczanie Książki

- **Ownership** = posiadasz książkę (możesz ją zniszczyć, zmienić)
- **Borrowing** = pożyczasz książkę (możesz czytać, ale musisz zwrócić w tym samym stanie)

### Kiedy Używać Borrowing?

1. **Czytasz wartość, ale nie zmieniasz** → `&T` (immutable reference)
2. **Chcesz zmienić wartość** → `&mut T` (mutable reference)
3. **Nie chcesz przejmować ownership** → użyj referencji zamiast wartości

### Proces Myślenia Krok po Kroku

```rust
fn oblicz_dlugosc(s: &String) -> usize {  // 1. Przyjmuję referencję, nie ownership
    s.len()                                // 2. Używam wartości przez referencję
}  // 3. Referencja wychodzi poza zakres, ale String nie jest usuwany (nie byłem właścicielem)

let s = String::from("hello");            // 4. s jest właścicielem
let len = oblicz_dlugosc(&s);             // 5. Pożyczam s przez referencję
println!("{}", s);                        // 6. ✅ s nadal działa - nie straciłem ownership
```

**Myślenie:**
- Krok 1-2: Funkcja używa referencji - nie przejmuje ownership
- Krok 3: Referencja znika, ale String (s) nadal istnieje
- Krok 4-6: `s` nadal jest właścicielem - można go użyć

### Zasady Borrowing - Jak Pamiętać?

1. **Wiele czytelników LUB jeden pisarz** - nie można mieszać
2. **Referencje muszą być ważne** - Rust zapobiega dangling references
3. **Myśl o zakresach** - referencje są ważne tylko w swoim zakresie

## Referencje

Zamiast przenosić ownership, możesz **pożyczyć** (*borrow*) wartość używając referencji:

```rust
fn main() {
    let s1 = String::from("hello");
    let len = oblicz_dlugosc(&s1);  // &s1 tworzy referencję
    println!("Długość '{}' wynosi {}.", s1, len);  // s1 nadal działa!
}

fn oblicz_dlugosc(s: &String) -> usize {
    s.len()
}  // s wychodzi poza zakres, ale nic nie jest usuwane (bo to tylko referencja)
```

### Terminologia

- **`&`** - operator referencji (tworzy referencję)
- **Reference** (*referencja*) - wskaźnik do wartości, nie właściciel
- **Borrowing** (*pożyczanie*) - używanie wartości przez referencję
- **Dereference** (*dereferencja*) - dostęp do wartości przez referencję (używamy `*`)

## Immutable References

Domyślnie referencje są **niezmienne** (*immutable*):

```rust
fn main() {
    let s = String::from("hello");
    zmien(&s);  // ❌ BŁĄD! Nie można zmienić przez immutable reference
}

fn zmien(s: &String) {
    s.push_str(", world");  // ❌ BŁĄD!
}
```

## Mutable References

Aby zmienić wartość, musisz użyć **mutable reference** (`&mut`):

```rust
fn main() {
    let mut s = String::from("hello");
    zmien(&mut s);
    println!("{}", s);  // "hello, world"
}

fn zmien(s: &mut String) {
    s.push_str(", world");
}
```

### Terminologia

- **`&mut`** - mutable reference (referencja modyfikowalna)
- **Immutable reference** (*referencja niezmienna*) - nie można zmienić wartości
- **Mutable reference** (*referencja modyfikowalna*) - można zmienić wartość

## Zasady Borrowing

Rust ma **dwie kluczowe zasady** borrowing:

1. **Możesz mieć wiele immutable references LUB jedną mutable reference**
2. **Referencje muszą być zawsze ważne**

### Zasada 1: Tylko jedna mutable reference

```rust
let mut s = String::from("hello");

let r1 = &mut s;  // ✅ OK
let r2 = &mut s;  // ❌ BŁĄD! Nie można mieć dwóch mutable references

println!("{}, {}", r1, r2);
```

### Zasada 2: Nie można mieszać immutable i mutable

```rust
let mut s = String::from("hello");

let r1 = &s;      // ✅ OK - immutable reference
let r2 = &s;      // ✅ OK - kolejna immutable reference
let r3 = &mut s;  // ❌ BŁĄD! Nie można mieć mutable reference gdy są immutable

println!("{}, {}, {}", r1, r2, r3);
```

### Zasada 3: Wiele immutable references jest OK

```rust
let s = String::from("hello");

let r1 = &s;  // ✅ OK
let r2 = &s;  // ✅ OK
let r3 = &s;  // ✅ OK

println!("{}, {}, {}", r1, r2, r3);
```

## Dangling References

Rust zapobiega **dangling references** (wiszącym referencjom):

```rust
fn main() {
    let reference_do_niczego = daj_referencje();
}

fn daj_referencje() -> &String {  // ❌ BŁĄD!
    let s = String::from("hello");
    &s  // s jest usuwane, referencja jest nieważna
}
```

Kompilator Rust zapobiega takim błędom w czasie kompilacji!

## 💡 Porównanie z C#

**C#:**
```csharp
string s = "hello";
string reference = s;  // Oba wskazują na ten sam obiekt
// Można mieć wiele referencji
```

**Rust:**
```rust
let s = String::from("hello");
let reference = &s;  // Referencja, nie ownership
// Zasady borrowing ograniczają użycie
```

## 🐍 Porównanie z Pythonem

**Python:**
```python
s = "hello"
reference = s  # Oba wskazują na ten sam obiekt
# Można mieć wiele referencji
```

**Rust:**
```rust
let s = String::from("hello");
let reference = &s;  // Referencja, nie ownership
// Zasady borrowing ograniczają użycie
```

## Ćwiczenia

1. Stwórz funkcję `dodaj_jeden(n: &mut i32)`, która zwiększa wartość o 1
2. Stwórz funkcję `dwa_razy(s: &String) -> usize`, która zwraca długość * 2
3. Przetestuj zasady borrowing - spróbuj stworzyć wiele mutable references

---

**Autor:** Arkadiusz Słota  
**Licencja:** CC BY-NC-SA 4.0

