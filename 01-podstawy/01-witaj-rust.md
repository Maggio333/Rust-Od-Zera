# 01. Witaj w Rust! 🦀

**Poziom:** Początkujący  
**Wymagana wiedza:** Podstawy programowania w dowolnym języku (C#, Python, itp.)

---

## Wprowadzenie

Witaj w świecie Rust! Ten dokument to Twój pierwszy krok w podróży, która zmieni sposób, w jaki myślisz o programowaniu. Rust nie jest łatwy, ale właśnie dlatego jest tak wartościowy.

## Co to jest Rust?

**Rust** to język programowania systemowego, stworzony przez Mozillę, a obecnie rozwijany przez Rust Foundation. Rust łączy w sobie:

- **Bezpieczeństwo pamięci** - kompilator zapobiega typowym błędom (wycieki pamięci, dangling pointers)
- **Wydajność** - porównywalna z C/C++, bez garbage collectora
- **Współbieżność** - bezpieczne programowanie wielowątkowe
- **Nowoczesność** - pattern matching, type inference, traits

### Terminologia

- **Rust** - nazwa języka (od grzyba rdzy - "rust")
- **Cargo** - menedżer pakietów i narzędzie do budowania projektów w Rust (jak `npm` w Node.js lub `nuget` w .NET)
- **Crate** - jednostka kompilacji w Rust (odpowiednik biblioteki/pakietu)
- **Compiler** - kompilator Rust (`rustc`)
- **Ownership** (*własność*) - unikalny system zarządzania pamięcią w Rust (omówimy szczegółowo później)

## Instalacja Rust

### Windows

1. Pobierz instalator z [rustup.rs](https://rustup.rs/)
2. Uruchom `rustup-init.exe`
3. Postępuj zgodnie z instrukcjami (domyślne opcje są zazwyczaj dobre)
4. Zrestartuj terminal

### Linux / macOS

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Weryfikacja instalacji

Otwórz terminal i wpisz:

```bash
rustc --version
cargo --version
```

Powinieneś zobaczyć coś podobnego do:
```
rustc 1.75.0 (82e1608df 2023-12-21)
cargo 1.75.0 (1d8b05cdd 2023-11-20)
```

## Twój pierwszy program: Hello World

### Metoda 1: Używając Cargo (zalecane)

**Cargo** to standardowe narzędzie do zarządzania projektami Rust. Użyjemy go do stworzenia pierwszego projektu.

```bash
# Utwórz nowy projekt
cargo new hello_world

# Wejdź do katalogu projektu
cd hello_world

# Uruchom projekt
cargo run
```

Cargo automatycznie utworzy strukturę projektu:
```
hello_world/
├── Cargo.toml    # Plik konfiguracyjny projektu (jak package.json w Node.js)
└── src/
    └── main.rs   # Główny plik źródłowy
```

### Metoda 2: Bezpośrednio z rustc

Utwórz plik `main.rs`:

```rust
fn main() {
    println!("Hello, world!");
}
```

Skompiluj i uruchom:

```bash
rustc main.rs
./main.exe    # Windows
# lub
./main        # Linux/macOS
```

## Analiza kodu Hello World

Przeanalizujmy nasz pierwszy program linijka po linijce:

```rust
fn main() {
    println!("Hello, world!");
}
```

### `fn main()`

- **`fn`** - słowo kluczowe oznaczające funkcję (*function*)
- **`main`** - nazwa funkcji. `main` to specjalna funkcja - punkt wejścia programu (jak w C#)
- **`()`** - puste nawiasy oznaczają, że funkcja nie przyjmuje parametrów
- **`{ }`** - ciało funkcji

### `println!`

- **`println!`** - makro (nie funkcja!) do wypisywania tekstu na konsolę
- **`!`** na końcu oznacza, że to makro, nie zwykła funkcja
- **`"Hello, world!"`** - string literal (literał łańcuchowy)

### Terminologia

- **Function** (*funkcja*) - blok kodu, który można wywołać
- **Macro** (*makro*) - specjalna konstrukcja, która generuje kod w czasie kompilacji
- **String literal** (*literał łańcuchowy*) - tekst zapisany bezpośrednio w kodzie

## Cargo - Twój przyjaciel

**Cargo** to więcej niż tylko kompilator. To kompleksowe narzędzie do zarządzania projektami.

### Podstawowe komendy Cargo

```bash
# Utwórz nowy projekt
cargo new nazwa_projektu

# Zbuduj projekt (kompilacja)
cargo build

# Zbuduj i uruchom
cargo run

# Sprawdź czy kod się kompiluje (bez budowania)
cargo check

# Uruchom testy
cargo test

# Formatuj kod
cargo fmt

# Sprawdź kod linterem
cargo clippy
```

### Plik Cargo.toml

Każdy projekt Rust ma plik `Cargo.toml` (TOML = Tom's Obvious Minimal Language):

```toml
[package]
name = "hello_world"
version = "0.1.0"
edition = "2021"

[dependencies]
# Tutaj dodajesz zależności (jak w package.json)
```

- **`[package]`** - sekcja z informacjami o pakiecie
- **`name`** - nazwa projektu
- **`version`** - wersja (semantic versioning)
- **`edition`** - wersja języka Rust (2021 to najnowsza stabilna)
- **`[dependencies]`** - zależności projektu (biblioteki zewnętrzne)

## 💡 Porównanie z C#

### Hello World

**C#:**
```csharp
using System;

class Program
{
    static void Main(string[] args)
    {
        Console.WriteLine("Hello, world!");
    }
}
```

**Rust:**
```rust
fn main() {
    println!("Hello, world!");
}
```

### Różnice

1. **Brak klasy** - w Rust `main` to funkcja, nie metoda klasy
2. **Brak `using`/`import`** - `println!` jest dostępne globalnie
3. **Prostsza składnia** - mniej boilerplate code
4. **Makro vs funkcja** - `println!` to makro, `Console.WriteLine` to metoda

### Zarządzanie projektami

**C# (.NET):**
- `dotnet new console` - tworzenie projektu
- `dotnet build` - kompilacja
- `dotnet run` - uruchomienie
- `dotnet add package` - dodawanie pakietów

**Rust:**
- `cargo new` - tworzenie projektu
- `cargo build` - kompilacja
- `cargo run` - uruchomienie
- Edycja `Cargo.toml` + `cargo build` - dodawanie pakietów

## 🐍 Porównanie z Pythonem

### Hello World

**Python:**
```python
print("Hello, world!")
```

**Rust:**
```rust
fn main() {
    println!("Hello, world!");
}
```

### Różnice

1. **Funkcja `main`** - Python nie wymaga funkcji `main` (choć można ją użyć)
2. **Kompilacja** - Rust kompiluje się do binarnego pliku, Python jest interpretowany
3. **Typy** - Rust wymaga kompilacji (sprawdza typy), Python sprawdza w runtime
4. **Wydajność** - Rust jest znacznie szybszy (kompilowany vs interpretowany)

### Zarządzanie projektami

**Python:**
- `pip install` - instalacja pakietów
- `requirements.txt` - lista zależności
- `virtualenv` / `venv` - środowiska wirtualne

**Rust:**
- `cargo add` lub edycja `Cargo.toml` - dodawanie pakietów
- `Cargo.toml` - lista zależności
- Cargo automatycznie zarządza zależnościami (jak `npm`)

## Ćwiczenia Praktyczne

### Ćwiczenie 1: Modyfikacja Hello World

Zmodyfikuj program `Hello, world!` tak, aby:
1. Wyświetlał Twoje imię zamiast "world"
2. Wyświetlał dwa różne komunikaty w osobnych liniach

**Wskazówka:** Możesz użyć `println!` wiele razy.

### Ćwiczenie 2: Nowy projekt

Utwórz nowy projekt o nazwie `moj_pierwszy_program` i napisz program, który:
1. Wyświetla powitanie
2. Wyświetla informację o sobie (np. "Uczę się Rust!")

### Ćwiczenie 3: Eksperymenty z Cargo

1. Uruchom `cargo check` w swoim projekcie
2. Uruchom `cargo build` i sprawdź, co się stało (szukaj pliku wykonywalnego)
3. Uruchom `cargo fmt` i zobacz, jak Cargo formatuje Twój kod

## Podsumowanie

W tym dokumencie nauczyłeś się:

- ✅ Co to jest Rust i dlaczego warto się go uczyć
- ✅ Jak zainstalować Rust i Cargo
- ✅ Jak stworzyć i uruchomić pierwszy program
- ✅ Podstawowe komendy Cargo
- ✅ Strukturę projektu Rust
- ✅ Różnice między Rust a C#/Pythonem

## Następny Krok

Gratulacje! Masz już działający program w Rust. W następnym dokumencie ([02-zmienne-i-typy.md](02-zmienne-i-typy.md)) nauczysz się:

- Zmiennych i stałych
- Typów danych
- Modyfikatora `mut`
- Inferencji typów

**Pamiętaj:** Rust jest trudny, ale każdy krok przybliża Cię do mistrzostwa. Nie spiesz się, rób ćwiczenia i eksperymentuj! 🦀

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

