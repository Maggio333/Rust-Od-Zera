const IMIE: &str = "Arek";

//ćwiczenie 1
fn main() {
    powitanie();
    wypisz_imie(IMIE);
    // kwadrat(3);
    // oblicz(2, 3);
    // srednia(2.0, 3.0, 4.0);

    let mut count = 3; 
    count = invoke_example(count, kwadrat(3).to_string());
    count = invoke_example(count, oblicz(2, 3).to_string());
    invoke_example(count, srednia(2.0, 3.0, 4.0).to_string());
}

//ćwiczenie 1
fn powitanie() {
    println!("Witaj w Rust!");
}

//ćwiczenie 2
fn wypisz_imie(imie: &str) {
    //przydałaby się wartość domyślna ale nie chce wyprzedzać faktów, sklejam tylko to co da się zrobić z poprzednich informacji
    println!("Witaj, {}", imie);
}

//ćwiczenie 3
fn kwadrat(x: i32) -> i32 {
    x * x
}

//ćwiczenie 4
fn oblicz(x: i32, y: i32) -> i32 {
    (x + y) * 2
}

//ćwiczenie 5
fn srednia(a: f64, b: f64, c: f64) -> f64 {
    (a + b + c) / 3.0
}

fn print_with_enumenator(count: i32, value: String) {
    println!("id ćwiczenia:{}, value:{}", count, value);
}

fn invoke_example(count: i32, value: String) -> i32 {
    //jakieś generyki by się przydały a najlepiej func<T>
    print_with_enumenator(count, value);
    seq_add(count)
}

fn seq_add(count: i32) -> i32 {
    count + 1
}
