const NUMBER_TO_CHECK: i32 = 5;
const NUM_TABLE: [i32; 5] = [1, 2, 3, 4, 5]; //kompilator mi powiedział że tak sie zapisuje tablice

fn main() {
    //ćwiczenie 1
    print_even_info(NUMBER_TO_CHECK);

    //ćwiczenie 2
    znak(NUMBER_TO_CHECK);

    //ćwiczenie 3
    start_counter();

    //ćwiczenie 4
    iter_table(NUM_TABLE);

    //ćwiczenie 5
    println!("{}", dzien_tygodnia(4));

    //ćwiczenie 6
    print_multiplication_tables();
}

fn print_multiplication_tables() {
    for i in 0..=5 {
        for j in 0..=5 {
            println!("{}x{}", i, j);
        }
    }
}

// ' to cykl życia, 'static to cykl życia przez całą aplikację. Doczytałem to wczoraj na gpt.. Potem pewnie będzie ale błąd dokumentu. Można też chyba dać String ale ta wersja co dałem chyba najlepsza
fn dzien_tygodnia(numer: u32) -> &'static str {
    let result = match numer {
        1 => "Poniedziałek",
        2 => "Wtorek",
        3 => "Środa",
        4 => "Czwartek",
        5 => "Piątek",
        6 => "Sobota",
        7 => "Niedziela",
        _ => "Nieprawidłowy numer",
    };

    result
}

//kompilator mi powiedział że tak sie zapisuje tablice
fn iter_table(table: [i32; 5]) {
    for number in table.iter() {
        println!("{}", number);
    }
}

fn start_counter() {
    let mut counter = 5;
    while counter > 1 {
        counter = counter - 1;
    }

    println!("Start!");
}

// ' to cykl życia, 'static to cykl życia przez całą aplikację. Doczytałem to wczoraj na gpt.. Potem pewnie będzie ale błąd dokumentu. 
//Można też chyba dać String ale ta wersja co dałem chyba najlepsza
fn znak(liczba: i32) -> &'static str {
    let mut result: &str = "ujemna";
    if liczba > 0 {
        //nie lubie zagnieżdżać i robię robić wyjścia
        result = "dodatnia"
    } else if liczba == 0 {
        result = "zero"
    }
    result //bez else ifa
}

fn is_even(number: i32) -> bool {
    number % 2 == 0
}

fn print_even_info(number: i32) {
    if is_even(number) {
        println!("Liczba {} jest parzysta", number);
        return;
    }
    println!("Liczba {} jest nieparzysta", number)
}
