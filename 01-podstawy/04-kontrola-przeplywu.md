# 04. Kontrola Przepływu 🦀

**Poziom:** Początkujący  
**Wymagana wiedza:** [03-funkcje.md](03-funkcje.md)

---

## Wprowadzenie

Kontrola przepływu pozwala na podejmowanie decyzji i powtarzanie akcji w programie. W Rust masz kilka sposobów na kontrolowanie przepływu: `if`, `match`, `loop`, `while`, i `for`.

## Instrukcje Warunkowe - `if`

### Podstawowy `if`

```rust
fn main() {
    let liczba = 3;
    
    if liczba < 5 {
        println!("Liczba jest mniejsza niż 5");
    }
}
```

### `if` z `else`

```rust
fn main() {
    let liczba = 7;
    
    if liczba < 5 {
        println!("Liczba jest mniejsza niż 5");
    } else {
        println!("Liczba jest większa lub równa 5");
    }
}
```

### `if` z `else if`

```rust
fn main() {
    let liczba = 6;
    
    if liczba % 4 == 0 {
        println!("Liczba jest podzielna przez 4");
    } else if liczba % 3 == 0 {
        println!("Liczba jest podzielna przez 3");
    } else if liczba % 2 == 0 {
        println!("Liczba jest podzielna przez 2");
    } else {
        println!("Liczba nie jest podzielna przez 4, 3 ani 2");
    }
}
```

### Terminologia

- **`if`** - jeśli
- **`else`** - w przeciwnym razie
- **Condition** (*warunek*) - wyrażenie logiczne, które zwraca `true` lub `false`
- **Branch** (*gałąź*) - jedna z ścieżek wykonania

## `if` jako Wyrażenie

W Rust `if` może być używany jako wyrażenie, zwracając wartość:

```rust
fn main() {
    let warunek = true;
    let liczba = if warunek {
        5
    } else {
        6
    };
    
    println!("Wartość liczby: {}", liczba);
}
```

**WAŻNE:** Oba gałęzie muszą zwracać ten sam typ!

```rust
fn main() {
    let liczba = if true {
        5      // i32
    } else {
        "sześć"  // ❌ BŁĄD! Różne typy
    };
}
```

## Early Returns (Wczesne Wyjścia)

W Rust możesz używać **early returns** (*wczesne wyjścia*) - wychodzić z funkcji wcześniej używając `return`:

```rust
fn znak(liczba: i32) -> &'static str {
    if liczba > 0 {
        return "dodatnia";  // Wychodzimy wcześnie
    }
    if liczba == 0 {
        return "zero";  // Wychodzimy wcześnie
    }
    "ujemna"  // Domyślna wartość - zawsze się wykona (jak "finally")
}
```

### Alternatywne Podejście z Zmienną

Możesz też użyć zmiennej z wartością domyślną i modyfikować ją:

```rust
fn znak(liczba: i32) -> &'static str {
    let mut result = "ujemna";  // Domyślna wartość
    if liczba > 0 {
        result = "dodatnia";
    } else if liczba == 0 {
        result = "zero";
    }
    result  // Zawsze zwracamy result (jak "finally")
}
```

### Kiedy Używać Early Returns?

- **Early returns** - gdy chcesz wyjść wcześnie z funkcji, kod jest bardziej czytelny
- **Zmienna z wartością domyślną** - gdy chcesz mieć "finally" (kod zawsze się wykona)

Oba podejścia są poprawne w Rust! Wybierz to, które jest bardziej czytelne dla Twojego przypadku.

### Uwaga o Lifetimes

W powyższych przykładach używamy `&'static str` - to jest **lifetime** (*cykl życia*). Lifetimes będą szczegółowo wyjaśnione w sekcji [02-ownership/03-lifetimes.md](../02-ownership/03-lifetimes.md). Na razie wystarczy wiedzieć, że:
- `&'static str` - literały stringowe (np. `"tekst"`) mają lifetime `'static`
- Lifetimes będą wyjaśnione później - na razie używaj `&'static str` dla literałów stringowych

## Pętle - `loop`

**`loop`** wykonuje kod w nieskończoność, dopóki nie przerwiesz go używając `break`:

```rust
fn main() {
    loop {
        println!("Nieskończona pętla!");
        break;  // Przerwanie pętli
    }
}
```

### `loop` z Wartością Zwracaną

Możesz użyć `break` z wartością, aby zwrócić wynik z pętli:

```rust
fn main() {
    let mut licznik = 0;
    
    let wynik = loop {
        licznik += 1;
        
        if licznik == 10 {
            break licznik * 2;  // Zwraca wartość
        }
    };
    
    println!("Wynik: {}", wynik);  // 20
}
```

### Terminologia

- **`loop`** - pętla nieskończona
- **`break`** - przerwanie pętli
- **`continue`** - przejście do następnej iteracji

## Pętle - `while`

**`while`** wykonuje kod dopóki warunek jest prawdziwy:

```rust
fn main() {
    let mut licznik = 3;
    
    while licznik != 0 {
        println!("{}!", licznik);
        licznik -= 1;
    }
    
    println!("Start!");
}
```

## Pętle - `for`

**`for`** iteruje po kolekcji elementów:

```rust
fn main() {
    let tablica = [10, 20, 30, 40, 50];
    
    for element in tablica.iter() {
        println!("Wartość: {}", element);
    }
}
```

### `for` z Zakresem

Możesz użyć `for` z zakresem liczb:

```rust
fn main() {
    for liczba in 1..4 {  // 1, 2, 3 (bez 4)
        println!("{}", liczba);
    }
    
    for liczba in 1..=4 {  // 1, 2, 3, 4 (z 4)
        println!("{}", liczba);
    }
}
```

### Terminologia

- **`for`** - pętla iterująca
- **Range** (*zakres*) - sekwencja wartości (np. `1..4`)
- **Iterator** (*iterator*) - obiekt pozwalający iterować po kolekcji

## Pattern Matching - `match`

**`match`** to potężne narzędzie w Rust, pozwalające na porównywanie wartości z różnymi wzorcami:

```rust
fn main() {
    let liczba = 3;
    
    match liczba {
        1 => println!("Jeden"),
        2 => println!("Dwa"),
        3 => println!("Trzy"),
        _ => println!("Coś innego"),  // _ to "catch-all"
    }
}
```

### `match` z Wartością Zwracaną

```rust
fn main() {
    let liczba = 2;
    
    let opis = match liczba {
        1 => "jeden",
        2 => "dwa",
        3 => "trzy",
        _ => "coś innego",
    };
    
    println!("Opis: {}", opis);
}
```

### Terminologia

- **`match`** - dopasowanie wzorca
- **Pattern** (*wzorzec*) - wzorzec do dopasowania
- **`_`** - wildcard, dopasowuje wszystko
- **Arm** (*ramię*) - jedna z gałęzi w `match`

## Etykiety Pętli

Możesz oznaczyć pętle etykietami, aby przerwać konkretną pętlę:

```rust
fn main() {
    let mut licznik = 0;
    
    'zewnetrzna: loop {
        println!("Zewnętrzna pętla: {}", licznik);
        let mut wewnetrzna_licznik = 0;
        
        loop {
            println!("Wewnętrzna pętla: {}", wewnetrzna_licznik);
            wewnetrzna_licznik += 1;
            
            if wewnetrzna_licznik == 2 {
                break;  // Przerwanie wewnętrznej pętli
            }
            
            if licznik == 1 {
                break 'zewnetrzna;  // Przerwanie zewnętrznej pętli
            }
        }
        
        licznik += 1;
    }
}
```

## 💡 Porównanie z C#

### Instrukcje Warunkowe

**C#:**
```csharp
int liczba = 3;
if (liczba < 5) {
    Console.WriteLine("Mniejsze niż 5");
} else {
    Console.WriteLine("Większe lub równe 5");
}
```

**Rust:**
```rust
let liczba = 3;
if liczba < 5 {
    println!("Mniejsze niż 5");
} else {
    println!("Większe lub równe 5");
}
```

### Różnice

1. **Nawiasy** - w Rust warunek nie wymaga nawiasów (choć można je użyć)
2. **`if` jako wyrażenie** - Rust pozwala na `if` zwracający wartość
3. **`match`** - Rust ma `match`, C# ma `switch` (ale `match` jest potężniejszy)

### Pętle

**C#:**
```csharp
// while
int i = 0;
while (i < 5) {
    Console.WriteLine(i);
    i++;
}

// for
for (int i = 0; i < 5; i++) {
    Console.WriteLine(i);
}

// foreach
foreach (var item in array) {
    Console.WriteLine(item);
}
```

**Rust:**
```rust
// while
let mut i = 0;
while i < 5 {
    println!("{}", i);
    i += 1;
}

// for (tylko iteracja)
for i in 0..5 {
    println!("{}", i);
}

// for (po kolekcji)
for item in array.iter() {
    println!("{}", item);
}
```

### Różnice

1. **`for`** - w Rust `for` tylko iteruje, nie ma tradycyjnej pętli `for(init; condition; increment)`
2. **Zakresy** - Rust używa `..` i `..=` dla zakresów
3. **`loop`** - Rust ma dedykowaną pętlę nieskończoną

## 🐍 Porównanie z Pythonem

### Instrukcje Warunkowe

**Python:**
```python
liczba = 3
if liczba < 5:
    print("Mniejsze niż 5")
else:
    print("Większe lub równe 5")
```

**Rust:**
```rust
let liczba = 3;
if liczba < 5 {
    println!("Mniejsze niż 5");
} else {
    println!("Większe lub równe 5");
}
```

### Różnice

1. **Nawiasy klamrowe** - Rust wymaga `{}`, Python używa wcięć
2. **`if` jako wyrażenie** - Rust pozwala, Python ma wyrażenie warunkowe `x if condition else y`
3. **`match`** - Rust ma `match`, Python 3.10+ ma `match` (ale Rust jest bardziej zaawansowany)

### Pętle

**Python:**
```python
# while
i = 0
while i < 5:
    print(i)
    i += 1

# for
for i in range(5):
    print(i)

for item in array:
    print(item)
```

**Rust:**
```rust
// while
let mut i = 0;
while i < 5 {
    println!("{}", i);
    i += 1;
}

// for
for i in 0..5 {
    println!("{}", i);
}

for item in array.iter() {
    println!("{}", item);
}
```

### Różnice

1. **Zakresy** - Python używa `range()`, Rust używa `..` i `..=`
2. **`loop`** - Rust ma dedykowaną pętlę nieskończoną, Python używa `while True`
3. **Iteratory** - Rust wymaga `.iter()`, Python iteruje bezpośrednio

## Ćwiczenia Praktyczne

### Ćwiczenie 1: Podstawowy `if`

Stwórz program, który sprawdza czy liczba jest parzysta i wyświetla odpowiedni komunikat.

### Ćwiczenie 2: `if` jako Wyrażenie

Stwórz funkcję `znak(liczba: i32) -> &str`, która zwraca:
- `"dodatnia"` jeśli liczba > 0
- `"ujemna"` jeśli liczba < 0
- `"zero"` jeśli liczba == 0

Użyj `if` jako wyrażenia!

### Ćwiczenie 3: Pętla `while`

Stwórz program, który odlicza od 5 do 1 używając `while`, a następnie wyświetla "Start!".

### Ćwiczenie 4: Pętla `for`

Stwórz program, który iteruje po tablicy `[1, 2, 3, 4, 5]` i wyświetla każdy element.

### Ćwiczenie 5: `match`

Stwórz funkcję `dzien_tygodnia(numer: u32) -> &'static str`, która zwraca nazwę dnia tygodnia używając `match`:

**Uwaga:** `&'static str` to lifetime - będzie wyjaśniony później. Na razie używaj go dla literałów stringowych.
- 1 => "Poniedziałek"
- 2 => "Wtorek"
- ...
- 7 => "Niedziela"
- _ => "Nieprawidłowy numer"

### Ćwiczenie 6: Zagnieżdżone Pętle

Stwórz program, który używa zagnieżdżonych pętli `for` do wyświetlenia tabliczki mnożenia (1x1 do 5x5).

## Podsumowanie

W tym dokumencie nauczyłeś się:

- ✅ Instrukcji warunkowych `if`, `else`, `else if`
- ✅ Używania `if` jako wyrażenia
- ✅ Early returns (wczesne wyjścia z funkcji)
- ✅ Pętli `loop`, `while`, `for`
- ✅ Pattern matching z `match`
- ✅ Etykiet pętli
- ✅ Różnic między Rust a C#/Pythonem

## Następny Krok

Świetnie! Znasz już podstawy kontroli przepływu. W następnej sekcji ([02-ownership/01-ownership-podstawy.md](../02-ownership/01-ownership-podstawy.md)) nauczysz się:

- Co to jest ownership (własność)
- Różnica między stack a heap
- Dlaczego ownership jest kluczowy w Rust

**Pamiętaj:** `match` to potężne narzędzie w Rust - użyjesz go bardzo często! 🦀

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

