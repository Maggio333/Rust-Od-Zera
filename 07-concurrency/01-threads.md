# 01. Threads (Wątki) 🦀

**Poziom:** Zaawansowany  
**Wymagana wiedza:** [03-smart-pointers.md](../06-zaawansowane/03-smart-pointers.md)

---

## Wprowadzenie

Rust pozwala na bezpieczne programowanie wielowątkowe dzięki systemowi ownership.

## Tworzenie Wątku

```rust
use std::thread;

let handle = thread::spawn(|| {
    println!("Hello from thread!");
});

handle.join().unwrap();
```

## Przekazywanie Danych

```rust
use std::thread;

let v = vec![1, 2, 3];

let handle = thread::spawn(move || {
    println!("{:?}", v);
});

handle.join().unwrap();
```

## 💡 Porównanie z C#

**C#:**
```csharp
var thread = new Thread(() => {
    Console.WriteLine("Hello from thread!");
});
thread.Start();
thread.Join();
```

**Rust:**
```rust
let handle = thread::spawn(|| {
    println!("Hello from thread!");
});
handle.join().unwrap();
```

## Ćwiczenia

1. Stwórz kilka wątków wykonujących różne zadania
2. Użyj `move` do przeniesienia danych do wątku

---

**Autor:** Arkadiusz Słota  
**Licencja:** CC BY-NC-SA 4.0

