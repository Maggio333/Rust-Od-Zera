# 02. Zmienne i Typy 🦀

**Poziom:** Początkujący  
**Wymagana wiedza:** [01-witaj-rust.md](01-witaj-rust.md)

---

## Wprowadzenie

W tym dokumencie nauczysz się podstawowych typów danych w Rust i jak pracować ze zmiennymi. To fundament, na którym zbudujesz resztę wiedzy.

## 🧠 Proces Myślenia: Jak Myśleć o Zmiennych w Rust?

Zanim przejdziemy do szczegółów, zastanówmy się jak Rust myśli o zmiennych:

### Mentalny Model: Wiązanie, Nie Przechowywanie

W Rust zmienne **wiążą** (*bind*) wartość z nazwą, a nie "przechowują" wartości jak w innych językach. To subtelna różnica, ale ważna:

- **C#/Python:** "Zmienna przechowuje wartość"
- **Rust:** "Zmienna wiąże nazwę z wartością"

### Domyślna Niezmienność - Dlaczego?

Rust domyślnie tworzy niezmienne zmienne, bo:
1. **Bezpieczeństwo** - zapobiega przypadkowym zmianom
2. **Współbieżność** - łatwiej myśleć o kodzie wielowątkowym
3. **Czytelność** - wiesz że wartość się nie zmieni (chyba że `mut`)

**Myślenie:** "Czy ta wartość musi się zmienić?" Jeśli nie - nie używaj `mut`.

## Zmienne - Podstawy

### `let` - Deklaracja Zmiennej

W Rust zmienne deklarujemy używając słowa kluczowego **`let`**.

```rust
fn main() {
    let x = 5;
    println!("Wartość x: {}", x);
}
```

### Terminologia

- **`let`** - słowo kluczowe do deklaracji zmiennej (*niech*)
- **Variable** (*zmienna*) - nazwana wartość, która może być używana w kodzie
- **Binding** (*wiązanie*) - w Rust zmienne "wiążą" wartość z nazwą (nie "przechowują")

### `mut` - Zmienne Modyfikowalne

Domyślnie w Rust wszystkie zmienne są **niezmienne** (*immutable*). To znaczy, że nie można zmienić ich wartości po przypisaniu.

```rust
fn main() {
    let x = 5;
    x = 6;  // ❌ BŁĄD! Nie można zmienić wartości
}
```

Aby zmienić wartość, musisz użyć **`mut`** (*mutable* - modyfikowalny):

```rust
fn main() {
    let mut x = 5;
    x = 6;  // ✅ OK! Zmienna jest modyfikowalna
    println!("Wartość x: {}", x);
}
```

### Terminologia

- **`mut`** - modyfikator oznaczający, że zmienna może być zmieniona
- **Immutable** (*niezmienny*) - nie można zmienić wartości
- **Mutable** (*modyfikowalny*) - można zmienić wartość

## Typy Podstawowe

Rust ma silny system typów. Oto podstawowe typy:

### Liczby Całkowite

| Typ | Rozmiar | Zakres |
|-----|---------|--------|
| `i8` | 8 bitów | -128 do 127 |
| `i16` | 16 bitów | -32,768 do 32,767 |
| `i32` | 32 bity | -2,147,483,648 do 2,147,483,647 |
| `i64` | 64 bity | -9,223,372,036,854,775,808 do 9,223,372,036,854,775,807 |
| `i128` | 128 bitów | Bardzo duży zakres |
| `isize` | Architektura | Zależy od systemu (32/64 bit) |
| `u8` | 8 bitów | 0 do 255 |
| `u16` | 16 bitów | 0 do 65,535 |
| `u32` | 32 bity | 0 do 4,294,967,295 |
| `u64` | 64 bity | 0 do 18,446,744,073,709,551,615 |
| `u128` | 128 bitów | Bardzo duży zakres |
| `usize` | Architektura | Zależy od systemu |

**`i`** = signed (ze znakiem, może być ujemna)  
**`u`** = unsigned (bez znaku, tylko dodatnie)

```rust
fn main() {
    let liczba: i32 = 42;
    let duza_liczba: u64 = 1_000_000;  // Można używać _ jako separator
    println!("Liczba: {}, Duża liczba: {}", liczba, duza_liczba);
}
```

### Liczby Zmiennoprzecinkowe

```rust
fn main() {
    let x: f32 = 3.14;  // 32-bitowa liczba zmiennoprzecinkowa
    let y: f64 = 2.71828;  // 64-bitowa (domyślna)
    println!("x: {}, y: {}", x, y);
}
```

- **`f32`** - 32-bitowa liczba zmiennoprzecinkowa (pojedyncza precyzja)
- **`f64`** - 64-bitowa liczba zmiennoprzecinkowa (podwójna precyzja, domyślna)

### Boolean

```rust
fn main() {
    let prawda: bool = true;
    let falsz: bool = false;
    println!("Prawda: {}, Fałsz: {}", prawda, falsz);
}
```

- **`bool`** - typ logiczny, może być `true` lub `false`

### Znaki

```rust
fn main() {
    let znak: char = 'A';
    let emoji: char = '🦀';
    println!("Znak: {}, Emoji: {}", znak, emoji);
}
```

- **`char`** - pojedynczy znak Unicode (4 bajty)
- Zawsze używamy pojedynczych cudzysłowów `'` dla `char`

## Inferencja Typów

Rust może **wywnioskować** (*infer*) typ na podstawie wartości:

```rust
fn main() {
    let x = 5;        // Rust wie, że to i32 (domyślny typ całkowity)
    let y = 3.14;    // Rust wie, że to f64 (domyślny typ zmiennoprzecinkowy)
    let z = true;    // Rust wie, że to bool
    
    // Możemy też jawnie podać typ:
    let a: i64 = 5;
    let b: f32 = 3.14;
}
```

### Terminologia

- **Type inference** (*inferencja typów*) - automatyczne wykrywanie typu przez kompilator
- **Explicit type** (*jawny typ*) - typ podany przez programistę

## Stałe

Stałe deklarujemy używając **`const`**:

```rust
const MAX_POINTS: u32 = 100_000;

fn main() {
    println!("Maksymalna liczba punktów: {}", MAX_POINTS);
}
```

Różnice między `let` a `const`:
- **`const`** - musi mieć jawny typ, nie może być `mut`, wartość musi być znana w czasie kompilacji
- **`let`** - może mieć inferencję typów, może być `mut`

## Cienie Zmiennych (Shadowing)

W Rust możesz "przesłonić" (*shadow*) zmienną nową wartością tego samego typu lub innego:

```rust
fn main() {
    let x = 5;
    let x = x + 1;      // Przesłaniamy x nową wartością
    let x = x * 2;      // Znowu przesłaniamy
    println!("Wartość x: {}", x);  // 12
    
    // Możemy nawet zmienić typ!
    let spaces = "   ";
    let spaces = spaces.len();  // Teraz spaces to liczba, nie string
    println!("Liczba spacji: {}", spaces);
}
```

### Terminologia

- **Shadowing** (*przesłanianie*) - tworzenie nowej zmiennej o tej samej nazwie, która "zasłania" starą

## 💡 Porównanie z C#

### Zmienne

**C#:**
```csharp
int x = 5;
int y = 10;
x = 15;  // Można zmienić bez mut
```

**Rust:**
```rust
let x = 5;
let mut y = 10;
y = 15;  // Tylko z mut
```

### Różnice

1. **Domyślna niezmienność** - w Rust zmienne są domyślnie niezmienne, w C# są modyfikowalne
2. **Inferencja typów** - Rust ma silniejszą inferencję (może pominąć typ częściej)
3. **Shadowing** - Rust pozwala na shadowing, C# nie (chociaż można użyć `var` w nowym scope)

### Typy

**C#:**
```csharp
int liczba = 42;
double zmiennoprzecinkowa = 3.14;
bool prawda = true;
char znak = 'A';
```

**Rust:**
```rust
let liczba: i32 = 42;
let zmiennoprzecinkowa: f64 = 3.14;
let prawda: bool = true;
let znak: char = 'A';
```

## 🐍 Porównanie z Pythonem

### Zmienne

**Python:**
```python
x = 5
x = 6  # Można zmienić zawsze
x = "tekst"  # Można nawet zmienić typ!
```

**Rust:**
```rust
let mut x = 5;
x = 6;  // Tylko z mut
// x = "tekst";  // ❌ BŁĄD! Nie można zmienić typu
```

### Różnice

1. **Typy** - Python jest dynamicznie typowany, Rust jest statycznie typowany
2. **Niezmienność** - Python nie ma konceptu immutable variables (oprócz `tuple`)
3. **Shadowing** - W Rust shadowing jest bardziej kontrolowany

### Typy

**Python:**
```python
liczba = 42  # int
zmiennoprzecinkowa = 3.14  # float
prawda = True  # bool
znak = 'A'  # str (string, nie char!)
```

**Rust:**
```rust
let liczba: i32 = 42;
let zmiennoprzecinkowa: f64 = 3.14;
let prawda: bool = true;
let znak: char = 'A';  // Pojedynczy znak, nie string!
```

## Ćwiczenia Praktyczne

### Ćwiczenie 1: Podstawowe Typy

Stwórz program, który:
1. Deklaruje zmienną całkowitą `wiek` z wartością 30
2. Deklaruje modyfikowalną zmienną `wzrost` z wartością 180.5
3. Zmienia wartość `wzrost` na 181.0
4. Wyświetla obie wartości

**Uwaga:** Rust może wyświetlić warning o nieużywanej wartości `180.5` (bo od razu ją zmieniasz). To normalne w tym ćwiczeniu - możesz zignorować warning lub użyć `#[allow(unused_assignments)]` nad deklaracją `wzrost`.

### Ćwiczenie 2: Shadowing

Stwórz program, który:
1. Deklaruje zmienną `x` z wartością 5
2. Przesłania `x` wartością 10
3. Przesłania `x` wartością "pięć" (string)
4. Wyświetla wszystkie wartości

**Wskazówka:** Użyj shadowing, nie `mut`!

### Ćwiczenie 3: Stałe

Stwórz stałą `PI` z wartością 3.14159 i użyj jej w funkcji `main`.

### Ćwiczenie 4: Różne Typy Liczbowe

Zadeklaruj zmienne różnych typów liczbowych:
- `i8`, `i32`, `i64`
- `u8`, `u32`, `u64`
- `f32`, `f64`

Wyświetl wszystkie wartości.

## Podsumowanie

W tym dokumencie nauczyłeś się:

- ✅ Jak deklarować zmienne używając `let`
- ✅ Różnicy między zmiennymi niezmiennymi a modyfikowalnymi (`mut`)
- ✅ Podstawowych typów danych w Rust
- ✅ Inferencji typów
- ✅ Stałych (`const`)
- ✅ Shadowing zmiennych
- ✅ Różnic między Rust a C#/Pythonem

## Następny Krok

Gratulacje! Znasz już podstawy zmiennych i typów. W następnym dokumencie ([03-funkcje.md](03-funkcje.md)) nauczysz się:

- Jak definiować funkcje
- Parametry i wartości zwracane
- Różnicę między wyrażeniami a instrukcjami

**Pamiętaj:** Rust wymusza bezpieczeństwo typów. To może być frustrujące na początku, ale zapobiega wielu błędom! 🦀

---

## 📝 Status Dokumentu

**Data utworzenia:** 2026-01-06  
**Status walidacji:** ⏳ Czeka na weryfikację z oficjalną dokumentacją Rust  
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

