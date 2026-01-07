# 🏗️ Architektura Repozytorium

> Dokument opisujący strukturę, organizację i filozofię projektu "Rust Od Zera"

**Ostatnia aktualizacja:** 2026-01-06

---

## 📋 Spis Treści

1. [Przegląd](#przegląd)
2. [Struktura Katalogów](#struktura-katalogów)
3. [Komponenty Projektu](#komponenty-projektu)
4. [Konwencje Nazewnictwa](#konwencje-nazewnictwa)
5. [Zależności i Relacje](#zależności-i-relacje)
6. [Przepływ Pracy](#przepływ-pracy)

---

## 📊 Przegląd

Projekt "Rust Od Zera" to kompleksowa ścieżka nauki języka Rust, zorganizowana w sposób modułowy i przejrzysty. Repozytorium składa się z kilku głównych komponentów:

- **Dokumentacja edukacyjna** - 32 dokumenty w 8 sekcjach tematycznych
- **Przykłady kodu** - działające projekty Rust ilustrujące koncepty
- **Proces pracy** - transparentny dziennik nauki i współpracy z AI
- **Weryfikacja** - raporty zgodności z oficjalną dokumentacją
- **Metadane** - licencja, changelog, współpraca

---

## 📁 Struktura Katalogów

```
rust-od-zera/
├── 01-podstawy/              # Sekcja 1: Podstawy Rust
├── 02-ownership/             # Sekcja 2: Ownership i Borrowing
├── 03-struktury-dane/        # Sekcja 3: Struktury Danych
├── 04-error-handling/        # Sekcja 4: Obsługa Błędów
├── 05-traits-generics/       # Sekcja 5: Traits i Generics
├── 06-zaawansowane/          # Sekcja 6: Zaawansowane Tematy
├── 07-concurrency/           # Sekcja 7: Współbieżność
├── 08-zaawansowane-tematy/   # Sekcja 8: Zaawansowane Tematy
├── examples/                 # Przykłady kodu Rust
├── proces/                   # Transparentny dziennik pracy
├── porownania/               # Porównania z innymi językami
├── private/                  # Prywatne notatki (nie w repo)
├── README.md                 # Główny plik projektu
├── ARCHITECTURE.md           # Ten dokument
├── WERYFIKACJA.md            # Raport weryfikacji z dokumentacją
├── WSPOLPRACA.md             # Zasady współpracy
├── CHANGELOG.md              # Historia zmian
├── LICENSE                   # Licencja CC BY-NC-SA 4.0
└── main.rs                   # Plik pomocniczy (detekcja języka na GitHub)
```

---

## 🧩 Komponenty Projektu

### 1. Dokumentacja Edukacyjna (`01-podstawy/` - `08-zaawansowane-tematy/`)

**Cel:** Główne materiały edukacyjne projektu.

**Struktura:**
- Każda sekcja ma numerację `01-`, `02-`, itd.
- Każdy dokument ma numerację `01-`, `02-`, itd.
- Dokumenty są czytane sekwencyjnie (numeracja wskazuje kolejność)

**Format dokumentu:**
```
# [Numer]. [Tytuł] 🦀

**Poziom:** [Początkujący/Średniozaawansowany/Zaawansowany]
**Wymagana wiedza:** [Link do poprzedniego dokumentu]

## Wprowadzenie
## [Główne tematy]
## 💡 Porównanie z C#
## 🐍 Porównanie z Pythonem
## Ćwiczenia Praktyczne
## Podsumowanie
## Następny Krok
## 📚 Dalsze Czytanie
## 📝 Status Dokumentu
```

**Status dokumentów:**
- ✅ **Ukończony** - pełna struktura, zweryfikowany, ma linki do dokumentacji
- ⏳ **W trakcie** - istnieje, ma podstawową strukturę, wymaga uzupełnienia
- 📝 **Do zrobienia** - nie istnieje

**Aktualny stan:** 5/32 ukończonych (16%)

### 2. Przykłady Kodu (`examples/`)

**Cel:** Działające przykłady kodu Rust ilustrujące koncepty z dokumentacji.

**Struktura:**
```
examples/
├── 01-podstawy-example/      # Przykłady dla sekcji 01
│   ├── hello_world/          # Projekt Cargo
│   ├── biometria/            # Projekt Cargo
│   └── ...
├── 02-ownership-example/     # Przykłady dla sekcji 02
└── ...
```

**Konwencje:**
- Każdy przykład to osobny projekt Cargo
- Nazwy projektów są opisowe (np. `hello_world`, `shadowing`)
- Projekty mają własny `Cargo.toml` i `src/main.rs`
- `target/` nie jest commitowany (w `.gitignore`)

**Aktualny stan:** 6/32 projektów (19%) - tylko sekcja 01-podstawy-example

### 3. Proces Pracy (`proces/`)

**Cel:** Transparentny dziennik pokazujący rzeczywisty proces nauki i współpracy z AI.

**Struktura:**
```
proces/
├── README.md                 # Indeks i opis procesu
└── YYYY-MM-DD/               # Wpisy organizowane po datach
    ├── temat-1.md
    ├── temat-2.md
    └── ...
```

**Zawartość:**
- Rzeczywiste rozmowy z AI
- Proces myślenia przy rozwiązywaniu problemów
- Decyzje projektowe i ich uzasadnienie
- Błędy i poprawki
- Ewolucja zrozumienia

**Format wpisu:**
- Data i temat
- O czym rozmawialiśmy
- Rzeczywiste rozmowy
- Co się wydarzyło
- Decyzje projektowe
- Spostrzeżenia AI

**Aktualny stan:** 11 wpisów z 2026-01-06

### 4. Weryfikacja (`WERYFIKACJA.md`)

**Cel:** Raport weryfikacji dokumentów z oficjalną dokumentacją Rust.

**Zawartość:**
- Lista zweryfikowanych dokumentów
- Porównanie z The Rust Book
- Znalezione problemy i sugestie poprawek
- Plan dalszej weryfikacji

**Aktualny stan:** 5/32 dokumentów zweryfikowanych

### 5. Metadane Projektu

#### `README.md`
- Główny punkt wejścia do projektu
- Manifest uczenia się
- Mapa ścieżki nauki
- Wskaźniki postępu
- Linki do wszystkich komponentów

#### `CHANGELOG.md`
- Historia zmian w dokumentach
- Data, dokument, typ zmiany
- Status walidacji

#### `WSPOLPRACA.md`
- Zasady współpracy
- Proces tworzenia dokumentów
- Logika projektu

#### `LICENSE`
- Licencja CC BY-NC-SA 4.0
- Zasady użytkowania

#### `main.rs`
- Plik pomocniczy dla GitHub
- Wykrywa Rust jako główny język projektu

### 6. Porównania (`porownania/`)

**Cel:** Tabele porównawcze Rust z innymi językami.

**Pliki:**
- `csharp-vs-rust.md` - Porównanie z C#
- `python-vs-rust.md` - Porównanie z Pythonem

### 7. Private (`private/`)

**Cel:** Prywatne notatki i projekty (nie powinny być w publicznym repo).

**Uwaga:** Ten katalog zawiera prywatne materiały i nie jest częścią publicznego projektu.

---

## 📝 Konwencje Nazewnictwa

### Dokumenty Edukacyjne
- Format: `NN-nazwa-tematu.md` (np. `01-witaj-rust.md`)
- Numeracja: `01-`, `02-`, `03-`, `04-` w każdej sekcji
- Nazwy: małe litery, myślniki zamiast spacji
- Język: polski

### Przykłady Kodu
- Format: `nazwa_projektu/` (snake_case)
- Opisowe nazwy (np. `hello_world`, `shadowing`, `kontrola_przeplywu`)
- Każdy projekt to osobny katalog Cargo

### Proces Pracy
- Format: `temat-rozmowy.md` (małe litery, myślniki)
- Organizacja: `YYYY-MM-DD/temat.md`
- Opisowe nazwy tematów

### Sekcje
- Format: `NN-nazwa-sekcji/` (np. `01-podstawy/`)
- Numeracja: `01-` do `08-`
- Nazwy: małe litery, myślniki

---

## 🔗 Zależności i Relacje

### Hierarchia Dokumentów

```
01-podstawy/
  └── 01-witaj-rust.md (punkt startowy)
       └── 02-zmienne-i-typy.md
            └── 03-funkcje.md
                 └── 04-kontrola-przeplywu.md
                      └── 02-ownership/01-ownership-podstawy.md
                           └── ...
```

**Zasady:**
- Każdy dokument wskazuje wymaganą wiedzę (poprzedni dokument)
- Dokumenty są czytane sekwencyjnie
- Sekcje są numerowane i czytane po kolei

### Relacje Dokumenty ↔ Przykłady

```
01-podstawy/01-witaj-rust.md
  └── examples/01-podstawy-example/hello_world/

01-podstawy/02-zmienne-i-typy.md
  └── examples/01-podstawy-example/biometria/
  └── examples/01-podstawy-example/shadowing/

01-podstawy/03-funkcje.md
  └── examples/01-podstawy-example/funkcje_example/

01-podstawy/04-kontrola-przeplywu.md
  └── examples/01-podstawy-example/kontrola_przeplywu/
```

**Zasady:**
- Przykłady odpowiadają sekcjom dokumentacji
- Nazwy projektów są opisowe i powiązane z tematem
- Przykłady ilustrują koncepty z dokumentów

### Proces Pracy ↔ Dokumenty

```
proces/2026-01-06/shadowing-vs-mut-rozmowa.md
  └── 01-podstawy/02-zmienne-i-typy.md (wpływ na dokument)

proces/2026-01-06/funkcje-to-string-i-enumerator.md
  └── 01-podstawy/03-funkcje.md (wpływ na dokument)
```

**Zasady:**
- Proces dokumentuje rzeczywiste rozmowy podczas tworzenia dokumentów
- Wpisy w procesie mogą wpływać na dokumenty
- Proces pokazuje ewolucję myślenia

### Weryfikacja ↔ Dokumenty

```
WERYFIKACJA.md
  └── 01-podstawy/01-witaj-rust.md (zweryfikowany)
  └── 01-podstawy/02-zmienne-i-typy.md (zweryfikowany)
  └── ...
```

**Zasady:**
- Weryfikacja sprawdza zgodność z oficjalną dokumentacją
- Zweryfikowane dokumenty mają status ✅
- Weryfikacja wpływa na aktualizacje dokumentów

---

## 🔄 Przepływ Pracy

### Tworzenie Nowego Dokumentu

1. **Planowanie** (w `proces/`)
   - Rozmowa z AI o temacie
   - Dokumentacja w `proces/YYYY-MM-DD/`

2. **Tworzenie** (w odpowiedniej sekcji)
   - AI pomaga w strukturze i przykładach
   - Tworzenie dokumentu `NN-temat.md`

3. **Walidacja** (w `WERYFIKACJA.md`)
   - Sprawdzenie z oficjalną dokumentacją
   - Aktualizacja statusu

4. **Przykłady** (w `examples/`)
   - Tworzenie działających projektów
   - Ilustracja konceptów z dokumentu

5. **Aktualizacja** (w `README.md`, `CHANGELOG.md`)
   - Aktualizacja wskaźników postępu
   - Wpis w changelog

### Aktualizacja Istniejącego Dokumentu

1. **Identyfikacja potrzeby**
   - Feedback od społeczności
   - Wykryty błąd
   - Nowa wiedza

2. **Dokumentacja zmiany** (w `proces/`)
   - Rozmowa o zmianie
   - Uzasadnienie

3. **Wprowadzenie zmiany**
   - Aktualizacja dokumentu
   - Weryfikacja z dokumentacją

4. **Aktualizacja metadanych**
   - `CHANGELOG.md`
   - Status w `README.md`

### Weryfikacja Dokumentu

1. **Sprawdzenie** (w `WERYFIKACJA.md`)
   - Porównanie z The Rust Book
   - Sprawdzenie przykładów

2. **Aktualizacja dokumentu**
   - Dodanie linków do dokumentacji
   - Poprawki błędów
   - Aktualizacja statusu

3. **Aktualizacja raportu**
   - `WERYFIKACJA.md`
   - Status w `README.md`

---

## 🎯 Filozofia Projektu

### Zasady Organizacji

1. **Sekwencyjność** - dokumenty są numerowane i czytane po kolei
2. **Modularność** - każda sekcja jest niezależna, ale powiązana
3. **Transparentność** - cały proces jest dokumentowany
4. **Weryfikacja** - wszystko jest sprawdzane z oficjalną dokumentacją
5. **Ewolucja** - dokumenty rosną wraz z wiedzą

### Zasady Nazewnictwa

1. **Spójność** - jednolite konwencje w całym projekcie
2. **Opisowość** - nazwy mówią o zawartości
3. **Numeracja** - jasna kolejność i hierarchia
4. **Język** - polski dla dokumentacji, angielski dla kodu

### Zasady Struktury

1. **Separacja** - dokumentacja, przykłady, proces są oddzielone
2. **Organizacja** - logiczne grupowanie po tematach
3. **Dostępność** - łatwe znalezienie potrzebnych materiałów
4. **Skalowalność** - łatwe dodawanie nowych materiałów

---

## 📊 Statystyki Projektu

**Dokumenty:**
- 32 dokumenty edukacyjne
- 5 ukończonych (16%)
- 27 w trakcie (84%)

**Przykłady:**
- 32 projekty planowanych
- 6 istniejących (19%)
- 26 do zrobienia (81%)

**Proces:**
- 11 wpisów z 2026-01-06
- Transparentny dziennik pracy

**Weryfikacja:**
- 5 dokumentów zweryfikowanych
- Raport w `WERYFIKACJA.md`

---

## 🔮 Przyszłe Rozszerzenia

### Możliwe Dodatki

1. **Ćwiczenia** (`exercises/`)
   - Osobny katalog z ćwiczeniami
   - Rozwiązania w `exercises/solutions/`

2. **Testy** (`tests/`)
   - Testy dla przykładów
   - Weryfikacja poprawności kodu

3. **Slajdy** (`slides/`)
   - Prezentacje do dokumentów
   - Wizualizacje konceptów

4. **Wideo** (`videos/`)
   - Linki do nagrań
   - Wyjaśnienia wizualne

---

## 📚 Powiązane Dokumenty

- [README.md](README.md) - Główny punkt wejścia
- [WERYFIKACJA.md](WERYFIKACJA.md) - Raport weryfikacji
- [WSPOLPRACA.md](WSPOLPRACA.md) - Zasady współpracy
- [CHANGELOG.md](CHANGELOG.md) - Historia zmian
- [proces/README.md](proces/README.md) - Opis procesu pracy

---

**Autor:** Arkadiusz Słota  
**Licencja:** CC BY-NC-SA 4.0  
**Status:** 🔄 Dokument na żywo - aktualizowany wraz z rozwojem projektu

