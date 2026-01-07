fn main() {
    let wiek: i32 = 30;
    #[allow(unused_assignments)]
    let mut wzrost: f64 = 180.5;
    wzrost = 181.0;
    println!("wiek {}, wzrost {}", wiek, wzrost);
}
