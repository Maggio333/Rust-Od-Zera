# 📔 Dzienniczek Nauki Rust

> Wspólny dzienniczek procesu uczenia się - dla autora i AI asystenta.  
> Pokazuje prawdziwy proces: odkrycia, problemy, rozwiązania, notatki.

---

## 🎯 Cel Dzienniczka

- **Dla autora:** Notatki z nauki, odkrycia, problemy do rozwiązania
- **Dla AI:** Kontekst do przyszłych rozmów, historia decyzji, preferencje
- **Dla społeczności:** Transparentny proces uczenia się

---

## 📅 2026-01-06 - Start Projektu

### Co się wydarzyło:
- ✅ Utworzono strukturę projektu `rust-od-zera/`
- ✅ Napisano README.md z manifestem uczenia się
- ✅ Utworzono pierwszy dokument: `01-witaj-rust.md`
- ✅ Dodano LICENSE (CC BY-NC-SA 4.0)
- ✅ Utworzono CHANGELOG.md
- ✅ Projekt wrzucony na GitHub: https://github.com/Maggio333/Rust-Od-Zera

### Odkrycia i Notatki:

**O Rust:**
- Rust jest językiem systemowym, kompilowanym
- Cargo to menedżer pakietów (jak npm, nuget)
- Rust ma unikalny system ownership (własności) - to kluczowy koncept

**O procesie:**
- Współpraca z AI przyspiesza tworzenie materiałów, ale wymaga walidacji
- Ważna jest transparentność - pokazywanie że jestem początkujący
- Projekt to manifest uczenia się, nie kurs od eksperta

**Narzędzia odkryte:**
- [asciinema.org](https://asciinema.org/) - do nagrywania sesji terminalowych (polecenie od kolegów z Bielik AI)
- Przyda się do pokazywania instalacji, uruchamiania programów, błędów kompilatora

**Przykłady asciinema od kolegi z Bielik AI:**
- https://asciinema.org/a/407806 - przykład nagrania (htop - monitor systemu)
- https://asciinema.org/a/403409 - przykład nagrania (Firefox na starym sprzęcie)
- **WAŻNE:** To są jego nagrania, bez jego zgody nie publikujemy ich w repo
- Dobre przykłady jak można używać asciinema do dokumentacji technicznej

### Problemy / Do rozwiązania:

**Brakująca wiedza:**
- ❓ Jak dokładnie wygląda instalacja Rust? (nie wiem jeszcze, muszę sprawdzić)
- ❓ Jak działają podstawowe komendy Cargo?
- ❓ Co to dokładnie jest ownership? (teoria w dokumentach, ale praktyka?)

**Do zrobienia:**
- [ ] Przetestować instalację Rust na własnym komputerze
- [ ] Nagrać asciinema z instalacją
- [ ] Zweryfikować `01-witaj-rust.md` z oficjalną dokumentacją
- [ ] Napisać post na LinkedIn
- [ ] Zbierać feedback po publikacji

### Decyzje projektowe:

1. **Struktura dokumentów:**
   - Każdy dokument ma sekcję "Status Dokumentu"
   - Porównania z C# i Pythonem w każdym dokumencie
   - Tłumaczenia terminów inline

2. **Ton i filozofia:**
   - Agresywny, bezpośredni opis przeciwko fałszywemu marketingowi
   - Transparentność - jestem początkujący, uczę się razem z czytelnikami
   - Manifest uczenia się, nie kurs od eksperta

3. **Narzędzia:**
   - `.gitignore` z katalogiem `/private/` na rzeczy eksperymentalne
   - Asciinema do nagrywania sesji (do przetestowania)
   - Discord na później (krok po kroku)

### Notatki techniczne:

**Git:**
- Repo zainicjalizowane lokalnie
- Remote: https://github.com/Maggio333/Rust-Od-Zera.git
- Branch: main
- Pierwszy commit: "Initial commit: Rust learning path project"

**Struktura katalogów:**
```
rust-od-zera/
├── 01-podstawy/
├── 02-ownership/
├── 03-struktury-dane/
├── 04-error-handling/
├── 05-traits-generics/
├── 06-zaawansowane/
├── 07-concurrency/
├── 08-zaawansowane-tematy/
├── porownania/
└── private/          # Nie commitowane (.gitignore)
```

### Pytania do przyszłości:

- Jak najlepiej pokazać proces uczenia się w dokumentach?
- Czy dodawać sekcję "Błędy które popełniłem" w każdym dokumencie?
- Jak strukturyzować feedback od społeczności?

---

## 📝 Format Wpisu

Każdy wpis powinien zawierać:

```markdown
## 📅 YYYY-MM-DD - [Tytuł]

### Co się wydarzyło:
- Co zrobiliśmy
- Co odkryliśmy
- Co się zmieniło

### Odkrycia i Notatki:
- Nowa wiedza o Rust
- Nowe narzędzia
- Nowe pomysły

### Problemy / Do rozwiązania:
- Co nie działa
- Czego nie wiemy
- Co trzeba sprawdzić

### Decyzje projektowe:
- Jakie decyzje podjęliśmy i dlaczego

### Notatki techniczne:
- Komendy, konfiguracje, linki

### Pytania do przyszłości:
- Co warto rozważyć później
```

---

**Pamiętaj:** Ten dzienniczek to żywy dokument. Aktualizuj go regularnie, zapisuj odkrycia, problemy i rozwiązania. To Twoja pamięć projektu i mój kontekst do przyszłych rozmów. 🦀

