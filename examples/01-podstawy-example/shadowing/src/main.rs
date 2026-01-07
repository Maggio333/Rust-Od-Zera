const PI: f64 = 3.14159; //w Rustcie daje się stałe z dużej litery(wszystkie) bo wypluło mi warning #[warn(non_upper_case_globals)] ~A 

fn main() {
    // Licznik - używasz mut bo modyfikujesz tę samą zmienną
    let mut i = 1;

    // Shadowing dla x - nie wymaga mut! Tworzysz nową zmienną o tej samej nazwie
    let x = 5;
    println!("iteracja {}, wartość {}", i, x);

    i = i + 1; // Modyfikujesz istniejącą zmienną (wymaga mut)
    let x = 10; // Przesłaniasz x - nowa zmienna (nie wymaga mut)
    println!("iteracja {}, wartość {}", i, x);

    i = i + 1; //moje oczy ale co zrobię, nie chce wyprzedzać, fajne DRY :P ~A
    let x = "pięć"; // Możemy nawet zmienić typ przy shadowing!
    println!("iteracja {}, wartość {}", i, x);

    //ćwiczenie 3
    i = i + 1;
    println!("iteracja {}, wartość {}", i, PI);

    //ćwiczenie 4
    let zmienna: i8 = -1;
    i = i + 1;
    println!("iteracja {}, wartość {}", i, zmienna);

    let zmienna: i32 = -2;
    i = i + 1;
    println!("iteracja {}, wartość {}", i, zmienna);

    let zmienna: i64 = -3; //tak to jest cały czas nowa zmienna bo jest let ~A
    i = i + 1;
    println!("iteracja {}, wartość {}", i, zmienna);

    let zmienna: u8 = 1; //bez znaku (unsigned) > 0 
    i = i + 1;
    println!("iteracja {}, wartość {}", i, zmienna);

    let zmienna: u32 = 2;
    i = i + 1; 
    println!("iteracja {}, wartość {}", i, zmienna);

    let zmienna: u64 = 3; 
    i = i + 1;
    println!("iteracja {}, wartość {}", i, zmienna);

    let zmienna: f32 = -0.1; //float ma to gdzieś (+/-)
    i = i + 1;
    println!("iteracja {}, wartość {}", i, zmienna);

    let zmienna: f64 = 0.2; 
    i = i + 1;
    println!("iteracja {}, wartość {}", i, zmienna);
}
