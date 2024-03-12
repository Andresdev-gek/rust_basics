
// Functions
fn main() {
    // functions
   do_something();
   do_something();
   do_something();
   addition(5,120);
   println!("2 - 1 is: {}", subtraction(2, 1));

    // Statements and Expressions

    // Statement
    // A statement in rust performs aan action but doesnt 
    // return a value. A statement can be a variable
    // declaration or function declaration.

    // Variable declaration
    let _x = 115; // this is a statement - it doesnt 
                      // evaluate anything

    
    // Function declaration
    // declaring a function - fn function() {}
    // Expressions
    // An expression is anything else in rust that evaaluates something
    // i.e it return a value
    // println! macro is an expression 
    // do_something is an expression (function call)
    // 115 is an expression

    // 'let' is not always a statement:
    let z = {
        let x = 5; // this is a statement - variable declaration
        x + 5 // this is an expression
    };

    println!("z variable here: {}", z);


    // Closure
    let add_numbers = |x: i32, y:i32| x + y;
    println!("Closure sum: {}", add_numbers(10,20));

    // Super handy because you can outside variables
    // which you cant do with a standard function
    // because it's block scoped

    let hi: &str = "hi ";
    let greeting = |name: &str| hi.to_string() + name;
    println!("{}", greeting("Bob"));


}

//creating functions (snakecase mandatory)
fn do_something() {
    let name: &str = "Andy";
    println!("hi {}", name);
}

// you have to specify the type of the parameter
fn addition(x: i32, y: i32) {
    println!("Addition: {}", x + y);
}

// Returning values from a function
// The -> 32 is like a return operator - this illustrate 
// the type that will be returned from our function
fn subtraction(x: i32, y: i32) -> i32 {
     x - y // this is an expression so NO semi-colon 
}

// You can also use 'return' key word and this works
// with or without a semi-colon



