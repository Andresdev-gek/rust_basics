// Command line arguments

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = args[1].clone();
    println!("{:?}", args);
    println!("{}", input);

    if input == "hola" {
        println!("Hey, what's up")
    }

    // Crea un vector vacío de enteros
    let mut numbers: Vec<i32> = Vec::new();

    // Agrega elementos al vector
    numbers.push(10);
    numbers.push(20);
    numbers.push(30);

    // Accede a un elemento por índice
    let first_number = numbers[0];
    println!("Primer elemento: {}", first_number);

    // Recorre el vector usando un bucle
    for number in numbers.iter() {
        println!("Número: {}", number);
    }
}
