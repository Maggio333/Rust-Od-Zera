# 🦀 Rust Od Zera

> Rust - jeden z najtrudniejszych języków na świecie. Projekt o rozwoju kompetencji, nie o pieniądzach. Gardzimy fałszywym marketingiem i falą (niby)ekspertów. Pokazujemy piękno programowania. Inspiracja do rozwoju bez wydawania pieniędzy - tylko pasja i determinacja.

**Autor:** Arkadiusz Słota

---

## 🎓 Manifest Uczenia Się

**Jestem początkujący w Rust.** Uczę się razem z Wami.

Ten projekt to mój manifest - pokazuję jak podchodzę do nauki jako samouk. Nie udaję eksperta. Pokazuję proces, nie tylko rezultat.

### Co to oznacza?

- ✅ **Transparentność** - widzicie co robię dobrze, a co źle
- ✅ **Walidacja** - materiały są weryfikowane z oficjalną dokumentacją Rust
- ✅ **Współpraca z AI** - pokazuję jak AI może pomóc w tworzeniu materiałów edukacyjnych
- ✅ **Długoterminowe aktualizacje** - dokumenty są poprawiane na podstawie feedbacku i mojej rosnącej wiedzy
- ✅ **Prawdziwy proces** - błędy, poprawki, rozwój - wszystko na żywo

### Jak to działa?

1. **Tworzenie materiałów** - AI pomaga mi przygotować dokumenty (struktura, przykłady, tłumaczenia)
2. **Walidacja** - czytam oficjalną dokumentację Rust i weryfikuję poprawność
3. **Feedback** - po publikacji zbieram opinie i poprawiam błędy
4. **Ewolucja** - dokumenty rosną wraz z moją wiedzą

**To nie jest kurs od eksperta. To jest podróż samouka, którą możesz obserwować i w której możesz uczestniczyć.**

---

## 📚 O Projekcie

Kompleksowa ścieżka nauki języka Rust od zera, stworzona z pasją i determinacją. Każdy dokument zawiera:

- ✅ **Tłumaczenia terminów** - każde specjalistyczne słowo wyjaśnione po polsku
- ✅ **Porównania z C# i Pythonem** - dla programistów z doświadczeniem
- ✅ **Przykłady kodu inline** - teoria od razu w praktyce
- ✅ **Ćwiczenia** - zadania do samodzielnego wykonania
- ✅ **Stopniowy rozwój** - od podstaw do zaawansowanych konceptów

## 🗺️ Mapa Ścieżki Nauki

### 01. Podstawy
- ✅ [01-witaj-rust.md](01-podstawy/01-witaj-rust.md) - Hello World, instalacja, Cargo
- ✅ [02-zmienne-i-typy.md](01-podstawy/02-zmienne-i-typy.md) - `let`, `mut`, typy podstawowe
- ✅ [03-funkcje.md](01-podstawy/03-funkcje.md) - Definicje funkcji, parametry, return
- ✅ [04-kontrola-przeplywu.md](01-podstawy/04-kontrola-przeplywu.md) - `if`, `match`, `loop`, `while`, `for`

### 02. Ownership i Borrowing
- ✅ [01-ownership-podstawy.md](02-ownership/01-ownership-podstawy.md) - Co to ownership, stack vs heap
- ⏳ [02-borrowing.md](02-ownership/02-borrowing.md) - Referencje (`&`), borrowing rules
- ⏳ [03-lifetimes.md](02-ownership/03-lifetimes.md) - Lifetimes (`'a`), elision
- ⏳ [04-string-vs-str.md](02-ownership/04-string-vs-str.md) - `String` vs `&str` - szczegółowe wyjaśnienie

### 03. Struktury Danych
- ⏳ [01-struktury.md](03-struktury-dane/01-struktury.md) - `struct`, pola, metody
- ⏳ [02-enums.md](03-struktury-dane/02-enums.md) - `enum`, `Option`, `Result`
- ⏳ [03-pattern-matching.md](03-struktury-dane/03-pattern-matching.md) - `match`, `if let`, `while let`
- ⏳ [04-collections.md](03-struktury-dane/04-collections.md) - `Vec`, `HashMap`, `HashSet`

### 04. Error Handling
- ⏳ [01-result-type.md](04-error-handling/01-result-type.md) - `Result<T, E>`, `unwrap`, `expect`
- ⏳ [02-option-type.md](04-error-handling/02-option-type.md) - `Option<T>`, `Some`, `None`
- ⏳ [03-propagation.md](04-error-handling/03-propagation.md) - `?`, `try!`, error handling patterns

### 05. Traits i Generics
- ⏳ [01-traits-podstawy.md](05-traits-generics/01-traits-podstawy.md) - `trait`, `impl`, default methods
- ⏳ [02-generics.md](05-traits-generics/02-generics.md) - `<T>`, generic functions, structs
- ⏳ [03-trait-bounds.md](05-traits-generics/03-trait-bounds.md) - `where`, trait bounds
- ⏳ [04-common-traits.md](05-traits-generics/04-common-traits.md) - `Clone`, `Copy`, `Debug`, `Display`

### 06. Zaawansowane
- ⏳ [01-closures.md](06-zaawansowane/01-closures.md) - Closures, capture modes
- ⏳ [02-iterators.md](06-zaawansowane/02-iterators.md) - `Iterator` trait, `map`, `filter`, `collect`
- ⏳ [03-smart-pointers.md](06-zaawansowane/03-smart-pointers.md) - `Box`, `Rc`, `RefCell`, `Arc`
- ⏳ [04-modules.md](06-zaawansowane/04-modules.md) - `mod`, `use`, `pub`, crate structure

### 07. Concurrency
- ⏳ [01-threads.md](07-concurrency/01-threads.md) - `thread::spawn`, `JoinHandle`
- ⏳ [02-channels.md](07-concurrency/02-channels.md) - `mpsc`, sender, receiver
- ⏳ [03-mutex.md](07-concurrency/03-mutex.md) - `Mutex`, `RwLock`, `Arc<Mutex<T>>`
- ⏳ [04-async.md](07-concurrency/04-async.md) - `async`/`await`, `Future`, tokio

### 08. Zaawansowane Tematy
- ⏳ [01-unsafe-rust.md](08-zaawansowane-tematy/01-unsafe-rust.md) - `unsafe`, raw pointers
- ⏳ [02-macros.md](08-zaawansowane-tematy/02-macros.md) - `macro_rules!`, procedural macros
- ⏳ [03-ffi.md](08-zaawansowane-tematy/03-ffi.md) - Foreign Function Interface
- ⏳ [04-testing.md](08-zaawansowane-tematy/04-testing.md) - `#[test]`, `#[cfg(test)]`, integration tests

### Porównania
- [csharp-vs-rust.md](porownania/csharp-vs-rust.md) - Tabela porównawcza C# vs Rust
- [python-vs-rust.md](porownania/python-vs-rust.md) - Tabela porównawcza Python vs Rust

### Przykłady Kodu
- [examples/](examples/) - Działające przykłady kodu Rust, zorganizowane zgodnie z dokumentacją

## 🎯 Jak Używać

1. **Zacznij od początku** - dokumenty są numerowane i powinny być czytane po kolei
2. **Czytaj teorię** - każdy koncept jest szczegółowo wyjaśniony
3. **Analizuj przykłady** - kod jest komentowany i tłumaczony
4. **Wykonuj ćwiczenia** - na końcu każdego dokumentu znajdziesz zadania
5. **Porównuj z C#/Pythonem** - jeśli masz doświadczenie, sekcje porównawcze pomogą Ci szybciej zrozumieć

## 📖 Konwencje w Dokumentach

- **Pogrubienie** dla terminów Rust (np. **ownership**, **borrowing**)
- *Kursywa* dla tłumaczeń polskich (np. *własność*, *pożyczanie*)
- `kod` dla przykładów kodu
- Sekcje "💡 Porównanie z C#" i "🐍 Porównanie z Pythonem" w każdym dokumencie

## 📝 Licencja

Ten projekt jest dostępny na licencji [Creative Commons Attribution-NonCommercial-ShareAlike 4.0 (CC BY-NC-SA 4.0)](LICENSE).

- ✅ Możesz uczyć się i dzielić materiałami
- ✅ Możesz modyfikować i adaptować
- ❌ **Nie możesz używać komercyjnie**
- ✅ Musisz zachować licencję przy modyfikacjach

## 📊 Status Nauki i Aktualizacje

### 📈 Wskaźnik Postępu

**Ogólny postęp:** `[██░░░░░░░░]` 16% (5/32 dokumentów ukończonych, 27/32 w trakcie)

| Sekcja | Postęp | Status |
|--------|--------|--------|
| 01. Podstawy | `[████]` 100% | ✅ 4/4 ukończone |
| 02. Ownership i Borrowing | `[███░]` 75% | ✅ 1/4 ukończone, ⏳ 3/4 w trakcie |
| 03. Struktury Danych | `[██░░]` 50% | ⏳ 4/4 istnieją, podstawowa struktura |
| 04. Error Handling | `[██░░]` 50% | ⏳ 3/3 istnieją, podstawowa struktura |
| 05. Traits i Generics | `[██░░]` 50% | ⏳ 4/4 istnieją, podstawowa struktura |
| 06. Zaawansowane | `[██░░]` 50% | ⏳ 4/4 istnieją, podstawowa struktura |
| 07. Concurrency | `[██░░]` 50% | ⏳ 4/4 istnieją, podstawowa struktura |
| 08. Zaawansowane Tematy | `[██░░]` 50% | ⏳ 4/4 istnieją, podstawowa struktura |

**Legenda:**
- ✅ Ukończony i zweryfikowany (pełna struktura: status, podsumowanie, ćwiczenia, porównania)
- ⏳ W trakcie tworzenia (istnieje, ma podstawową strukturę: wprowadzenie, przykłady, ćwiczenia)
- 📝 Do zrobienia (nie istnieje)

**Uwaga:** Wszystkie dokumenty istnieją i mają podstawową strukturę. Tylko sekcja "01. Podstawy" ma pełną strukturę ze statusem dokumentu i szczegółowym podsumowaniem. Pozostałe dokumenty wymagają uzupełnienia o sekcję statusu i rozszerzenia podsumowań.

### 📦 Wskaźnik Postępu Przykładów

**Ogólny postęp przykładów:** `[█░░░░░░░░░]` 19% (6/32 projektów)

| Sekcja | Projekty | Status |
|--------|----------|--------|
| 01. Podstawy | `[██████]` 100% | ✅ 6/6 projektów |
| 02. Ownership i Borrowing | `[░░░░]` 0% | 📝 0/4 projektów |
| 03. Struktury Danych | `[░░░░]` 0% | 📝 0/4 projektów |
| 04. Error Handling | `[░░░]` 0% | 📝 0/3 projektów |
| 05. Traits i Generics | `[░░░░]` 0% | 📝 0/4 projektów |
| 06. Zaawansowane | `[░░░░]` 0% | 📝 0/4 projektów |
| 07. Concurrency | `[░░░░]` 0% | 📝 0/4 projektów |
| 08. Zaawansowane Tematy | `[░░░░]` 0% | 📝 0/4 projektów |

**Projekty w 01-podstawy-example:**
- ✅ `hello_world` - Hello World, podstawy Cargo
- ✅ `moj_pierwszy_program` - Podstawy println!
- ✅ `biometria` - Zmienne i typy
- ✅ `funkcje_example` - Funkcje
- ✅ `kontrola_przeplywu` - if, match, loop, while, for
- ✅ `shadowing` - Shadowing vs mut

### Aktualny Stan

**Dokumenty zweryfikowane (✅):**
- ✅ **01-witaj-rust.md** - Zweryfikowany z oficjalną dokumentacją Rust
- ✅ **02-zmienne-i-typy.md** - Zweryfikowany z oficjalną dokumentacją Rust
- ✅ **03-funkcje.md** - Zweryfikowany z oficjalną dokumentacją Rust
- ✅ **04-kontrola-przeplywu.md** - Zweryfikowany z oficjalną dokumentacją Rust
- ✅ **01-ownership-podstawy.md** - Zweryfikowany z oficjalną dokumentacją Rust

**Dokumenty w trakcie (⏳):**
- ⏳ Pozostałe 27 dokumentów - istnieją, mają podstawową strukturę, wymagają uzupełnienia

**Raport weryfikacji:** Szczegółowy raport weryfikacji z oficjalną dokumentacją Rust znajduje się w [WERYFIKACJA.md](WERYFIKACJA.md).

### Historia Aktualizacji

Szczegółowa historia zmian znajduje się w [CHANGELOG.md](CHANGELOG.md).

### Jak Działamy

Szczegóły współpracy, procesu tworzenia dokumentów i logiki projektu w [WSPOLPRACA.md](WSPOLPRACA.md).

### Architektura Repozytorium

Szczegółowy opis struktury, organizacji i filozofii projektu w [ARCHITECTURE.md](ARCHITECTURE.md).

### 🔄 Proces Pracy - Transparentny Dziennik

**Cały przepływ rozmowy i proces uczenia się jest dokumentowany w [proces/](proces/README.md).**

To nie jest zwykły projekt - to **transparentny dziennik** pokazujący:
- 🗣️ **Rzeczywiste rozmowy** z AI podczas nauki Rust
- 🤔 **Proces myślenia** - jak rozwiązuję problemy, jakie pytania zadaję
- 📝 **Decyzje projektowe** - dlaczego coś zrobiłem tak, a nie inaczej
- 🐛 **Błędy i poprawki** - wszystko na żywo, bez ukrywania problemów
- 🤖 **Współpraca z AI** - jak AI pomaga, a jak ja waliduję i poprawiam

**Co znajdziesz w `proces/`:**
- 📅 Wpisy organizowane po datach (np. `2026-01-06/`)
- 💬 Dokumentacja rzeczywistych rozmów i sesji pracy
- 🎯 Decyzje projektowe i ich uzasadnienie
- 📚 Ewolucja zrozumienia - jak moja wiedza rośnie
- 🔍 Analiza problemów i ich rozwiązań

**Przykładowe tematy z procesu:**
- Start projektu i tworzenie dokumentacji
- Wybór narzędzi i workflow
- Pierwsze kroki z Cargo
- Rozwiązywanie problemów (warnings, nested repos)
- Kreatywne podejścia do ćwiczeń
- Shadowing vs mut - głęboka analiza
- Early returns i lifetimes

**Dlaczego to jest ważne?**
Pokazuję **prawdziwy proces** uczenia się - nie tylko końcowy rezultat. To może pomóc innym zobaczyć, jak można uczyć się Rust jako samouk, jak współpracować z AI, i że błędy są częścią procesu.

Dokumenty są aktualizowane na podstawie:
- Oficjalnej dokumentacji Rust ([rust-lang.org](https://doc.rust-lang.org/))
- Feedbacku od społeczności
- Mojej rosnącej wiedzy i doświadczenia
- Wykrytych błędów i nieścisłości
- **Rzeczywistych rozmów dokumentowanych w `proces/`**

**Ostatnia aktualizacja:** 2026-01-06

### Jak Pomóc?

- 🐛 Znalazłeś błąd? Otwórz Issue!
- 💡 Masz sugestię? Podziel się nią!
- ✅ Sprawdziłeś coś w dokumentacji? Daj znać!
- 📝 Chcesz poprawić dokument? Pull Request mile widziany!

## 🤖 Współpraca z AI

Ten projekt pokazuje potencjał współpracy człowieka z AI w edukacji:

- **AI pomaga** w strukturze, przykładach, tłumaczeniach
- **Człowiek waliduje** - sprawdza z dokumentacją, poprawia błędy, dodaje kontekst
- **Razem tworzymy** wartościowe materiały edukacyjne

**Cały proces współpracy jest dokumentowany w [proces/](proces/README.md)** - możesz zobaczyć rzeczywiste rozmowy, jak AI pomaga, jak ja waliduję, i jak razem rozwiązujemy problemy.

To nie zastępuje nauki - to narzędzie, które przyspiesza proces tworzenia materiałów, ale wymaga ludzkiej weryfikacji i zrozumienia.

## 🤝 Wsparcie

Jeśli masz pytania, znalazłeś błąd lub chcesz się podzielić swoimi postępami - zapraszam do kontaktu!

---

**Pamiętaj:** Rust jest trudny. Ten projekt też. Nie ma tu łatwych odpowiedzi. Jest za to pasja, determinacja i prawdziwa nauka. Uczymy się razem. 🦀

