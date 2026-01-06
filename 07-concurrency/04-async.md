# 04. Async/Await 🦀

**Poziom:** Expert  
**Wymagana wiedza:** [03-mutex.md](03-mutex.md)

---

## Wprowadzenie

**Async/await** pozwala na asynchroniczne programowanie w Rust.

## Podstawowe Użycie

```rust
async fn hello() {
    println!("Hello");
}

#[tokio::main]
async fn main() {
    hello().await;
}
```

## Future

```rust
use std::future::Future;

async fn async_function() -> i32 {
    42
}
```

## 💡 Porównanie z C#

**C#:**
```csharp
async Task<int> AsyncFunction() {
    return 42;
}
```

**Rust:**
```rust
async fn async_function() -> i32 {
    42
}
```

## Ćwiczenia

1. Stwórz async funkcję
2. Użyj `tokio` do uruchomienia async kodu

---

**Autor:** Arkadiusz Słota  
**Licencja:** CC BY-NC-SA 4.0

