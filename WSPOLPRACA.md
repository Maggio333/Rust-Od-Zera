# 🤝 Jak Działamy - Logika Współpracy

> Dokument wyjaśniający jak działa współpraca w tym projekcie - między autorem a AI asystentem, oraz jak możesz dołączyć.

---

## 🎯 Filozofia Współpracy

Ten projekt to **manifest transparentnego uczenia się**. Pokazujemy jak można efektywnie współpracować z AI w procesie edukacji, zachowując przy tym ludzką kontrolę i walidację.

### Zasady:

1. **AI pomaga, człowiek decyduje** - AI generuje strukturę, przykłady, tłumaczenia, ale autor weryfikuje wszystko
2. **Transparentność** - wszystko jest widoczne: proces, błędy, poprawki
3. **Walidacja** - każdy dokument jest weryfikowany z oficjalną dokumentacją
4. **Ewolucja** - dokumenty rosną wraz z wiedzą autora

---

## 🔄 Proces Tworzenia Dokumentów

### Krok 1: Planowanie
- Autor określa temat i zakres
- AI proponuje strukturę dokumentu
- Wspólnie ustalamy co ma być zawarte

### Krok 2: Tworzenie
- AI generuje treść na podstawie:
  - Oficjalnej dokumentacji Rust
  - Porównań z C# i Pythonem (doświadczenie autora)
  - Wcześniejszych dokumentów z projektu
- Autor na bieżąco weryfikuje i koryguje

### Krok 3: Walidacja
- Autor sprawdza z [The Rust Book](https://doc.rust-lang.org/book/)
- Testuje przykłady kodu
- Weryfikuje poprawność tłumaczeń terminów

### Krok 4: Publikacja i Feedback
- Dokument trafia do repo
- Zbieramy feedback od społeczności
- Aktualizujemy na podstawie uwag

### Krok 5: Ewolucja
- Gdy autor uczy się więcej, dokumenty są aktualizowane
- CHANGELOG.md śledzi wszystkie zmiany
- DZIENNICZEK.md dokumentuje proces uczenia się

---

## 📁 Struktura Projektu

### Katalogi Publiczne (commitowane do repo):

- `01-podstawy/` - Podstawowe koncepty Rust
- `02-ownership/` - Ownership i Borrowing
- `03-struktury-dane/` - Struktury danych
- `04-error-handling/` - Obsługa błędów
- `05-traits-generics/` - Traits i Generics
- `06-zaawansowane/` - Zaawansowane tematy
- `07-concurrency/` - Współbieżność
- `08-zaawansowane-tematy/` - Expert level
- `porownania/` - Porównania z innymi językami

### Katalogi Prywatne (nie commitowane):

- `private/` - Rzeczy eksperymentalne, notatki prywatne, szkice
- Zawartość tego katalogu nie trafia na GitHub (`.gitignore`)
- Służy do testowania, eksperymentowania, prywatnych notatek

**Dlaczego?** Nie wszystko musi być od razu publiczne. Eksperymenty, niepewne pomysły, prywatne notatki - to wszystko może dojrzeć w `private/` zanim trafi do głównego repo.

---

## 🛠️ Narzędzia Współpracy

### AI Asystent (Cursor AI)
- Generuje strukturę dokumentów
- Tworzy przykłady kodu z komentarzami
- Tłumaczy terminy techniczne
- Proponuje porównania z C#/Pythonem
- Pomaga w formatowaniu i strukturze

### Autor (Arkadiusz Słota)
- Weryfikuje poprawność merytoryczną
- Testuje przykłady kodu
- Sprawdza z oficjalną dokumentacją
- Dodaje osobiste doświadczenia i kontekst
- Decyduje o finalnej wersji

### Społeczność
- Feedback i korekty
- Sugestie ulepszeń
- Zgłaszanie błędów (Issues)
- Pull Requesty z poprawkami

---

## 📝 Dokumenty Procesowe

### DZIENNICZEK.md
- Wspólny dzienniczek autora i AI
- Notatki z nauki, odkrycia, problemy
- Historia decyzji projektowych
- Kontekst do przyszłych rozmów

### CHANGELOG.md
- Historia wszystkich zmian w dokumentach
- Co zostało poprawione i dlaczego
- Status walidacji każdej zmiany

### WSPOLPRACA.md (ten dokument)
- Jak działa współpraca w projekcie
- Proces tworzenia dokumentów
- Struktura projektu

---

## 🎓 Jak Możesz Pomóc?

### Znalazłeś błąd?
- Otwórz [Issue](https://github.com/Maggio333/Rust-Od-Zera/issues) z opisem
- Lub stwórz Pull Request z poprawką

### Masz sugestię?
- Podziel się w Issues lub Discussions
- Każda opinia jest wartościowa

### Chcesz poprawić dokument?
- Fork repo
- Wprowadź zmiany
- Stwórz Pull Request
- Opisz co i dlaczego zmieniłeś

### Chcesz dodać nowy dokument?
- Najpierw zaproponuj w Issues
- Omówmy strukturę i zakres
- Potem możemy wspólnie stworzyć

---

## 🔒 Zasady Prywatności i Zgód

### Materiały zewnętrzne:
- Jeśli używamy przykładów od innych osób (np. nagrania asciinema), zawsze sprawdzamy zgodę
- Bez zgody autora nie publikujemy cudzych materiałów
- Szanujemy prawa autorskie i licencje

### Katalog `private/`:
- Zawartość nie jest commitowana
- Służy do eksperymentów i prywatnych notatek
- Może zawierać materiały, które jeszcze nie są gotowe do publikacji

---

## 💡 Przykłady Współpracy

### Przykład 1: Tworzenie nowego dokumentu
1. Autor: "Chcę dokument o zmiennych i typach"
2. AI: Generuje strukturę z sekcjami (zmienne, typy podstawowe, mut, inferencja)
3. Autor: Weryfikuje, dodaje swoje doświadczenia z C#
4. AI: Uzupełnia przykłady i porównania
5. Autor: Testuje kod, poprawia błędy
6. Razem: Finalna wersja gotowa do publikacji

### Przykład 2: Aktualizacja po feedbacku
1. Społeczność: "W dokumencie X jest błąd w przykładzie Y"
2. Autor: Sprawdza w dokumentacji Rust
3. AI: Pomaga poprawić przykład
4. Autor: Weryfikuje i commituje poprawkę
5. CHANGELOG: Dokumentuje zmianę

---

## 🎯 Cel Końcowy

Pokazać, że:
- **AI to narzędzie**, nie zastępstwo dla ludzkiej wiedzy
- **Współpraca** przyspiesza proces, ale wymaga walidacji
- **Transparentność** buduje zaufanie
- **Uczenie się** to proces, nie produkt

---

**Pamiętaj:** To nie jest projekt o AI. To projekt o **ludzkiej pasji do nauki**, wspieranej przez narzędzia. 🦀

---

**Autor:** Arkadiusz Słota  
**Licencja:** CC BY-NC-SA 4.0

