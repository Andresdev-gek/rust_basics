fn main() {
    //this is how you create a variable
    // By default in Rust all variables are inmutable
    let five = 5;
    // Recall that Rust is statically and strongly typed - when you define a 
    // variable, Rust will automatically give it a type
    // this is know as "implicit" assigment 
    //five = "Elliot";

    // this creates a variable and assigns a type to it
    let ten: u32 = 10;

    //This is th way to assign a mutable variable
    let mut nine = 9;
    println!("{}", nine);

    // when using 'mut' - the variable you wish to change, must be
    // reassigned to the same type
    //nine = 12.5; // error here
    
    println!("{}", nine);
    // Anotoher way of reassigning variables - writing 'let' twice!
    let ten: u32 = 10;

    // if use let again you can chancge the variable type
    let ten = "diez";
    println!("{}", ten);
    

    // You can assign multiple variables at once
    let (name, age, country) = ("Xiao", 45, "China");
    println!("My Name is {} I am from {}, and I am {} years old", name, country, age);

    // Constants
    const ID: i32 = 001;
    println!("here the id {}", ID);

    // Shadowing
    let animal = "cow";
    println!("The animal is {}", animal);

    // {} are known as 'scopes'
    {
        let animal = "leon";
        println!("The animal in the scope is {}", animal)
    }

    println!("The animal out of the scope is {}", animal);

}
