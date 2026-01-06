# 02. Channels (Kanały) 🦀

**Poziom:** Zaawansowany  
**Wymagana wiedza:** [01-threads.md](01-threads.md)

---

## Wprowadzenie

**Channels** pozwalają na komunikację między wątkami.

## Podstawowe Użycie

```rust
use std::sync::mpsc;
use std::thread;

let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    tx.send(String::from("hello")).unwrap();
});

let received = rx.recv().unwrap();
println!("{}", received);
```

## Wiele Wysyłających

```rust
let (tx, rx) = mpsc::channel();
let tx1 = tx.clone();

thread::spawn(move || {
    tx.send(String::from("hi")).unwrap();
});

thread::spawn(move || {
    tx1.send(String::from("hello")).unwrap();
});

for received in rx {
    println!("{}", received);
}
```

## Ćwiczenia

1. Stwórz kanał między wątkami
2. Wyślij wiele wiadomości przez kanał

---

**Autor:** Arkadiusz Słota  
**Licencja:** CC BY-NC-SA 4.0

