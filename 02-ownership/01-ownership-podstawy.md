# 01. Ownership - Podstawy 🦀

**Poziom:** Średniozaawansowany  
**Wymagana wiedza:** [04-kontrola-przeplywu.md](../01-podstawy/04-kontrola-przeplywu.md)

---

## Wprowadzenie

**Ownership** (*własność*) to najważniejszy i najbardziej unikalny koncept w Rust. To system zarządzania pamięcią, który nie wymaga garbage collectora, a jednocześnie gwarantuje bezpieczeństwo pamięci. Zrozumienie ownership jest kluczowe do programowania w Rust.

## Co to jest Ownership?

**Ownership** to zestaw reguł, które Rust używa do zarządzania pamięcią. Każda wartość w Rust ma **właściciela** (*owner*), i w danym momencie może być tylko jeden właściciel.

## 🧠 Proces Myślenia: Jak Myśleć o Ownership?

Gdy zaczynasz z Rust, ownership może wydawać się skomplikowany. Oto jak o nim myśleć:

### Mentalny Model 1: Własność jak Książka w Bibliotece

Wyobraź sobie, że wartość to książka:
- **Tylko jedna osoba może mieć książkę** w danym momencie (jeden owner)
- Gdy oddajesz książkę komuś innemu, **ty już jej nie masz** (move)
- Gdy książka wychodzi z biblioteki (scope), jest **automatycznie zwracana** (dealokacja)

### Mentalny Model 2: Stack vs Heap - Pudełka vs Magazyn

- **Stack** = małe pudełka na biurku (szybkie, znany rozmiar)
  - Liczby, booleany - możesz je łatwo kopiować
  - "Weź kopię tego pudełka" - działa szybko
  
- **Heap** = duży magazyn (wolniejszy, dynamiczny rozmiar)
  - Stringi, wektory - kopiowanie byłoby drogie
  - "Przenieś własność tego magazynu" - tylko jeden właściciel

### Jak Podejść do Problemu Ownership?

1. **Zadaj sobie pytanie:** "Czy to jest na stack czy heap?"
   - Stack (liczby, booleany) → zwykle kopiuje się automatycznie
   - Heap (String, Vec) → ownership jest przenoszony

2. **Sprawdź błąd kompilatora:**
   - "value moved here" → ownership został przeniesiony
   - "value borrowed here" → używasz referencji (omówimy w następnym dokumencie)

3. **Pomyśl o zakresie:**
   - Gdy zmienna wychodzi poza zakres `{}`, jest usuwana
   - To automatyczne - nie musisz pamiętać o zwalnianiu

### Przykład Myślenia Krok po Kroku

```rust
let s1 = String::from("hello");  // 1. Tworzę String na heap
let s2 = s1;                      // 2. Przenoszę ownership do s2
// println!("{}", s1);            // 3. ❌ s1 nie ma już dostępu - ownership jest u s2
```

**Myślenie:**
- Krok 1: `s1` jest właścicielem String na heap
- Krok 2: Ownership jest **przeniesiony** (moved) do `s2`
- Krok 3: `s1` nie jest już właścicielem - nie można go użyć

**Dlaczego?** Bo gdyby oba mogły modyfikować ten sam String, mogłyby się konfliktować. Rust zapobiega temu w czasie kompilacji.

### Terminologia

- **Ownership** (*własność*) - system zarządzania pamięcią w Rust
- **Owner** (*właściciel*) - zmienna, która "posiada" wartość
- **Memory safety** (*bezpieczeństwo pamięci*) - brak wycieków pamięci, dangling pointers, itp.

## Stack vs Heap

Aby zrozumieć ownership, musisz wiedzieć różnicę między **stack** a **heap**.

### Stack (Stos)

- **Szybki** - alokacja i dealokacja są bardzo szybkie
- **LIFO** (Last In, First Out) - ostatni dodany, pierwszy usunięty
- **Znany rozmiar** - wartości muszą mieć znany, stały rozmiar
- **Przykłady:** liczby całkowite, liczby zmiennoprzecinkowe, booleany, `char`

```rust
fn main() {
    let x = 5;        // x jest na stack
    let y = x;        // y jest kopią x (na stack)
    println!("x: {}, y: {}", x, y);  // Oba działają!
}
```

### Heap (Sterta)

- **Wolniejszy** - alokacja i dealokacja są wolniejsze
- **Dynamiczny rozmiar** - wartości mogą mieć nieznany rozmiar
- **Wymaga zarządzania** - ktoś musi zwolnić pamięć
- **Przykłady:** `String`, `Vec`, struktury o nieznanym rozmiarze

```rust
fn main() {
    let s1 = String::from("hello");  // s1 jest na heap
    let s2 = s1;  // s1 jest przeniesione do s2 (move)
    // println!("{}", s1);  // ❌ BŁĄD! s1 nie jest już właścicielem
    println!("{}", s2);  // ✅ OK
}
```

### Terminologia

- **Stack** (*stos*) - obszar pamięci dla wartości o znanym rozmiarze
- **Heap** (*sterta*) - obszar pamięci dla wartości o dynamicznym rozmiarze
- **Allocation** (*alokacja*) - przydzielenie pamięci
- **Deallocation** (*dealokacja*) - zwolnienie pamięci

## Zasady Ownership

Rust ma trzy główne zasady ownership:

1. **Każda wartość ma właściciela**
2. **Może być tylko jeden właściciel w danym momencie**
3. **Gdy właściciel wychodzi poza zakres, wartość jest usuwana**

### Zasada 1: Każda wartość ma właściciela

```rust
fn main() {
    let s = String::from("hello");  // s jest właścicielem String
}
```

### Zasada 2: Tylko jeden właściciel

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // s1 jest PRZENIESIONE (moved) do s2
    
    // println!("{}", s1);  // ❌ BŁĄD! s1 nie jest już właścicielem
    println!("{}", s2);  // ✅ OK
}
```

### Zasada 3: Wartość jest usuwana gdy właściciel wychodzi poza zakres

```rust
fn main() {
    {
        let s = String::from("hello");  // s jest właścicielem
        // s jest używane tutaj
    }  // s wychodzi poza zakres, String jest automatycznie usuwany
    // s nie jest już dostępne
}
```

## Move (Przeniesienie)

Gdy przypisujesz wartość z heap do innej zmiennej, **własność jest przenoszona** (*moved*):

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // s1 jest przeniesione do s2
    
    // s1 nie jest już właścicielem - nie można go użyć
    println!("{}", s2);  // ✅ OK
}
```

### Terminologia

- **Move** (*przeniesienie*) - transfer własności z jednej zmiennej do drugiej
- **Moved value** (*przeniesiona wartość*) - wartość, która została przeniesiona

## Copy (Kopiowanie)

Dla wartości na stack, Rust automatycznie **kopiuje** zamiast przenosić:

```rust
fn main() {
    let x = 5;        // i32 jest na stack
    let y = x;        // x jest skopiowane do y
    
    println!("x: {}, y: {}", x, y);  // ✅ Oba działają!
}
```

### Typy które są Copy

- Wszystkie typy całkowite (`i32`, `u32`, itp.)
- Typy zmiennoprzecinkowe (`f32`, `f64`)
- `bool`
- `char`
- Tuple zawierające tylko typy Copy (np. `(i32, i32)`)

### Typy które NIE są Copy

- `String`
- `Vec<T>`
- Większość innych typów

## Scope (Zakres)

Wartość jest usuwana gdy właściciel **wychodzi poza zakres**:

```rust
fn main() {
    let s = String::from("hello");  // s wchodzi w zakres
    
    // s jest używane tutaj
    
}  // s wychodzi poza zakres, String jest automatycznie usuwany
```

### Przykład z Zakresem

```rust
fn main() {
    let s1 = String::from("hello");
    
    {
        let s2 = String::from("world");
        // s2 jest używane tutaj
    }  // s2 wychodzi poza zakres, jest usuwane
    
    // s1 jest nadal dostępne
    println!("{}", s1);
}
```

## Funkcje i Ownership

Gdy przekazujesz wartość do funkcji, ownership jest **przenoszony**:

```rust
fn przyjmuje_wlasnosc(s: String) {
    println!("{}", s);
}  // s wychodzi poza zakres, String jest usuwany

fn main() {
    let s = String::from("hello");
    przyjmuje_wlasnosc(s);  // s jest przeniesione do funkcji
    // println!("{}", s);  // ❌ BŁĄD! s nie jest już właścicielem
}
```

### Zwracanie Ownership

Funkcje mogą zwracać ownership:

```rust
fn daje_wlasnosc() -> String {
    let s = String::from("hello");
    s  // s jest zwracane (przeniesione)
}

fn main() {
    let s1 = daje_wlasnosc();
    println!("{}", s1);
}
```

## 💡 Porównanie z C#

### Zarządzanie Pamięcią

**C#:**
```csharp
string s1 = "hello";
string s2 = s1;  // Oba wskazują na ten sam obiekt (reference)
// Garbage Collector zarządza pamięcią
```

**Rust:**
```rust
let s1 = String::from("hello");
let s2 = s1;  // s1 jest przeniesione do s2
// println!("{}", s1);  // ❌ BŁĄD!
```

### Różnice

1. **Garbage Collector** - C# ma GC, Rust nie ma
2. **References** - C# używa referencji, Rust używa ownership
3. **Move semantics** - Rust ma explicite move, C# kopiuje referencje
4. **Bezpieczeństwo** - Rust gwarantuje bezpieczeństwo w czasie kompilacji

## 🐍 Porównanie z Pythonem

### Zarządzanie Pamięcią

**Python:**
```python
s1 = "hello"
s2 = s1  # Oba wskazują na ten sam obiekt (reference)
# Garbage Collector zarządza pamięcią
```

**Rust:**
```rust
let s1 = String::from("hello");
let s2 = s1;  // s1 jest przeniesione do s2
// println!("{}", s1);  // ❌ BŁĄD!
```

### Różnice

1. **Garbage Collector** - Python ma GC, Rust nie ma
2. **References** - Python używa referencji, Rust używa ownership
3. **Move semantics** - Rust ma explicite move, Python kopiuje referencje
4. **Bezpieczeństwo** - Rust gwarantuje bezpieczeństwo w czasie kompilacji

## Ćwiczenia Praktyczne

### Ćwiczenie 1: Stack vs Heap

Stwórz program, który:
1. Tworzy zmienną `x` typu `i32` z wartością 5
2. Tworzy zmienną `y` i przypisuje jej `x`
3. Wyświetla obie wartości

Dlaczego to działa? (Odpowiedź: `i32` jest na stack i jest Copy)

### Ćwiczenie 2: Move

Stwórz program, który:
1. Tworzy `String` z wartością "hello"
2. Próbuje przypisać go do innej zmiennej
3. Próbuje użyć obu zmiennych

Co się stanie? (Odpowiedź: pierwsza zmienna nie będzie już dostępna)

### Ćwiczenie 3: Scope

Stwórz program z zagnieżdżonym zakresem:
1. Wewnątrz `main` stwórz zakres `{}`
2. Wewnątrz zakresu stwórz `String`
3. Poza zakresem spróbuj użyć tego `String`

Co się stanie? (Odpowiedź: błąd kompilacji - wartość jest usunięta)

### Ćwiczenie 4: Funkcje i Ownership

Stwórz funkcję `przyjmuje_string(s: String)`, która wyświetla string, a następnie:
1. Wywołaj funkcję z `String`
2. Spróbuj użyć tego `String` po wywołaniu

Co się stanie?

## Podsumowanie

W tym dokumencie nauczyłeś się:

- ✅ Co to jest ownership i dlaczego jest ważny
- ✅ Różnicy między stack a heap
- ✅ Trzech zasad ownership
- ✅ Różnicy między move a copy
- ✅ Jak ownership działa z zakresami
- ✅ Jak ownership działa z funkcjami
- ✅ Różnic między Rust a C#/Pythonem

## Następny Krok

Świetnie! Znasz już podstawy ownership. W następnym dokumencie ([02-borrowing.md](02-borrowing.md)) nauczysz się:

- Co to jest borrowing (pożyczanie)
- Referencje (`&`)
- Zasady borrowing
- Mutable references

**Pamiętaj:** Ownership to fundament Rust. Na początku może być frustrujące, ale zapobiega wielu błędom! 🦀

---

## 📚 Dalsze Czytanie

- [The Rust Book - Understanding Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
- [The Rust Book - What is Ownership?](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
- [The Rust Book - References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
- [Rust by Example - Ownership](https://doc.rust-lang.org/rust-by-example/scope/ownership.html)

## 📝 Status Dokumentu

**Data utworzenia:** 2026-01-06  
**Status walidacji:** ✅ Zweryfikowany z oficjalną dokumentacją Rust (2026-01-06)  
**Ostatnia aktualizacja:** 2026-01-06

### Uwagi Autora

Ten dokument został stworzony przy współpracy z AI. Jako początkujący w Rust, będę go aktualizował na podstawie:
- Oficjalnej dokumentacji Rust ([The Rust Book](https://doc.rust-lang.org/book/))
- Feedbacku od społeczności
- Mojej rosnącej wiedzy

Jeśli znajdziesz błąd lub masz sugestię - daj znać! Uczymy się razem.

---

**Autor:** Arkadiusz Słota  
**Licencja:** CC BY-NC-SA 4.0

