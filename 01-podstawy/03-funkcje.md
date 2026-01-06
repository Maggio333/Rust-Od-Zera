# 03. Funkcje 🦀

**Poziom:** Początkujący  
**Wymagana wiedza:** [02-zmienne-i-typy.md](02-zmienne-i-typy.md)

---

## Wprowadzenie

Funkcje to podstawowe bloki budulcowe programu Rust. W tym dokumencie nauczysz się jak definiować i używać funkcji, oraz poznasz unikalne cechy funkcji w Rust.

## Definiowanie Funkcji

### Podstawowa Składnia

```rust
fn nazwa_funkcji() {
    println!("Witaj z funkcji!");
}

fn main() {
    nazwa_funkcji();
}
```

### Terminologia

- **`fn`** - słowo kluczowe do definiowania funkcji (*function*)
- **Function** (*funkcja*) - blok kodu, który można wywołać wielokrotnie
- **Call** (*wywołanie*) - uruchomienie funkcji

## Funkcje z Parametrami

Funkcje mogą przyjmować parametry. W Rust **musisz** podać typ każdego parametru:

```rust
fn wypisz_liczbe(x: i32) {
    println!("Liczba: {}", x);
}

fn main() {
    wypisz_liczbe(42);
}
```

### Wiele Parametrów

```rust
fn dodaj(x: i32, y: i32) {
    println!("Suma: {}", x + y);
}

fn main() {
    dodaj(5, 3);
}
```

### Terminologia

- **Parameter** (*parametr*) - zmienna w definicji funkcji
- **Argument** (*argument*) - wartość przekazana do funkcji przy wywołaniu
- W praktyce te terminy są często używane zamiennie

## Funkcje Zwracające Wartości

W Rust funkcje mogą zwracać wartości. Możesz użyć słowa kluczowego **`return`**, ale częściej używa się **wyrażeń** (*expressions*).

### Używając `return`

```rust
fn pięć() -> i32 {
    return 5;
}

fn main() {
    let x = pięć();
    println!("Wartość: {}", x);
}
```

### Używając Wyrażenia (Zalecane)

```rust
fn pięć() -> i32 {
    5  // Brak średnika = wyrażenie, które jest zwracane
}

fn main() {
    let x = pięć();
    println!("Wartość: {}", x);
}
```

### Terminologia

- **`->`** - strzałka wskazująca typ zwracany
- **Return type** (*typ zwracany*) - typ wartości, którą funkcja zwraca
- **Expression** (*wyrażenie*) - kod, który zwraca wartość
- **Statement** (*instrukcja*) - kod, który wykonuje akcję, ale nie zwraca wartości

## Wyrażenia vs Instrukcje

To kluczowa różnica w Rust!

### Instrukcje (Statements)

Instrukcje wykonują akcję, ale **nie zwracają wartości**:

```rust
fn main() {
    let x = 6;  // To jest instrukcja
    // x = 6; nie zwraca wartości, tylko przypisuje
}
```

### Wyrażenia (Expressions)

Wyrażenia **zwracają wartość**:

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1  // To wyrażenie - zwraca 4
    };
    println!("y = {}", y);  // y = 4
}
```

**Kluczowa różnica:** Średnik (`;`) zamienia wyrażenie w instrukcję!

```rust
fn main() {
    let x = 5;        // Instrukcja (ma średnik)
    let y = {
        5             // Wyrażenie (brak średnika)
    };
    let z = {
        5;            // Instrukcja (ma średnik) - zwraca () (unit type)
    };
}
```

## Funkcje z Wiele Parametrów i Zwracaniem

```rust
fn dodaj(x: i32, y: i32) -> i32 {
    x + y  // Wyrażenie - zwraca sumę
}

fn main() {
    let wynik = dodaj(5, 3);
    println!("Wynik: {}", wynik);
}
```

## Funkcje z Różnymi Typami Zwracanymi

```rust
fn mnozenie(x: i32, y: i32) -> i32 {
    x * y
}

fn dzielenie(x: f64, y: f64) -> f64 {
    x / y
}

fn main() {
    let iloczyn = mnozenie(4, 5);
    let iloraz = dzielenie(10.0, 2.0);
    println!("Iloczyn: {}, Iloraz: {}", iloczyn, iloraz);
}
```

## Funkcje Zwracające `()` (Unit Type)

Funkcje, które nic nie zwracają, faktycznie zwracają **`()`** (*unit type*):

```rust
fn wypisz() -> () {
    println!("Witaj!");
}

// To jest równoważne:
fn wypisz() {
    println!("Witaj!");
}
```

### Terminologia

- **`()`** - unit type, odpowiednik `void` w innych językach
- **Unit** - typ reprezentujący "brak wartości"

## Funkcje z Wiele Linii

```rust
fn oblicz(x: i32, y: i32) -> i32 {
    let suma = x + y;
    let iloczyn = x * y;
    suma + iloczyn  // Ostatnia linia bez średnika = wyrażenie zwracane
}

fn main() {
    let wynik = oblicz(2, 3);
    println!("Wynik: {}", wynik);  // (2+3) + (2*3) = 5 + 6 = 11
}
```

## Funkcje Wewnętrzne

Możesz definiować funkcje wewnątrz innych funkcji:

```rust
fn main() {
    fn wewnetrzna() {
        println!("To funkcja wewnętrzna!");
    }
    
    wewnetrzna();
}
```

## 💡 Porównanie z C#

### Definicja Funkcji

**C#:**
```csharp
static int Dodaj(int x, int y)
{
    return x + y;
}
```

**Rust:**
```rust
fn dodaj(x: i32, y: i32) -> i32 {
    x + y
}
```

### Różnice

1. **Brak `static`** - w Rust funkcje są domyślnie "statyczne"
2. **Typy parametrów** - w Rust typy są po dwukropku (`x: i32`), w C# przed nazwą (`int x`)
3. **Return** - Rust preferuje wyrażenia bez `return`
4. **Brak klasy** - w Rust funkcje mogą być na najwyższym poziomie

### Wyrażenia vs Instrukcje

**C#:**
```csharp
int x = 5;  // Instrukcja
int y = 5;  // Instrukcja
// W C# wszystko jest instrukcją
```

**Rust:**
```rust
let x = 5;     // Instrukcja
let y = {
    5          // Wyrażenie (brak średnika)
};
```

## 🐍 Porównanie z Pythonem

### Definicja Funkcji

**Python:**
```python
def dodaj(x, y):
    return x + y
```

**Rust:**
```rust
fn dodaj(x: i32, y: i32) -> i32 {
    x + y
}
```

### Różnice

1. **Typy** - Python nie wymaga typów (opcjonalne type hints), Rust wymaga
2. **Return** - Python wymaga `return`, Rust preferuje wyrażenia
3. **Wyrażenia** - Python nie ma tak wyraźnego podziału wyrażenia/instrukcje

### Wyrażenia vs Instrukcje

**Python:**
```python
x = 5  # Instrukcja
y = 5  # Instrukcja
# Python ma wyrażenia, ale nie tak wyraźnie jak Rust
```

**Rust:**
```rust
let x = 5;     // Instrukcja
let y = {
    5          // Wyrażenie
};
```

## Ćwiczenia Praktyczne

### Ćwiczenie 1: Podstawowa Funkcja

Stwórz funkcję `powitanie()`, która wyświetla "Witaj w Rust!".

### Ćwiczenie 2: Funkcja z Parametrem

Stwórz funkcję `wypisz_imie(imie: &str)`, która wyświetla "Witaj, [imię]!".

**Wskazówka:** `&str` to typ stringa w Rust (omówimy szczegółowo później).

### Ćwiczenie 3: Funkcja Zwracająca Wartość

Stwórz funkcję `kwadrat(x: i32) -> i32`, która zwraca kwadrat liczby.

### Ćwiczenie 4: Funkcja z Wiele Parametrów

Stwórz funkcję `oblicz(x: i32, y: i32) -> i32`, która:
1. Dodaje x i y
2. Mnoży wynik przez 2
3. Zwraca wynik

Użyj wyrażenia, nie `return`!

### Ćwiczenie 5: Funkcja z Wiele Linii

Stwórz funkcję `srednia(a: f64, b: f64, c: f64) -> f64`, która:
1. Sumuje trzy liczby
2. Dzieli przez 3
3. Zwraca średnią

## Podsumowanie

W tym dokumencie nauczyłeś się:

- ✅ Jak definiować funkcje używając `fn`
- ✅ Jak przekazywać parametry do funkcji
- ✅ Jak zwracać wartości z funkcji
- ✅ Różnicy między wyrażeniami a instrukcjami
- ✅ Jak używać wyrażeń zamiast `return`
- ✅ Różnic między Rust a C#/Pythonem

## Następny Krok

Świetnie! Znasz już podstawy funkcji. W następnym dokumencie ([04-kontrola-przeplywu.md](04-kontrola-przeplywu.md)) nauczysz się:

- Instrukcje warunkowe (`if`, `else`)
- Pętle (`loop`, `while`, `for`)
- Pattern matching (`match`)

**Pamiętaj:** Wyrażenia vs instrukcje to kluczowy koncept w Rust. Brak średnika na końcu wyrażenia oznacza, że jest ono zwracane! 🦀

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

