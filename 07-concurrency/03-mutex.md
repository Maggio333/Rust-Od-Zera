# 03. Mutex 🦀

**Poziom:** Zaawansowany  
**Wymagana wiedza:** [02-channels.md](02-channels.md)

---

## Wprowadzenie

**Mutex** (mutual exclusion) pozwala na bezpieczny dostęp do współdzielonych danych.

## Podstawowe Użycie

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();
        *num += 1;
    });
    handles.push(handle);
}

for handle in handles {
    handle.join().unwrap();
}

println!("Wynik: {}", *counter.lock().unwrap());
```

## Arc<T>

**Arc** (Atomically Reference Counted) pozwala na współdzielenie danych między wątkami.

## Ćwiczenia

1. Użyj `Mutex` do synchronizacji dostępu
2. Połącz `Arc` i `Mutex` dla shared mutable state

---

**Autor:** Arkadiusz Słota  
**Licencja:** CC BY-NC-SA 4.0

