// Printing and formatting

// we can use the rustfmt command in src directory to format our rust code
fn main() {
    println!("hi brooooooooooooooooooooooou");

    let name = "Andres";
    let hobby = "read";

    // basic formatting
    println!("My name is {} and i love {}!", name, hobby);

    //you cant simply write println!(5) -it doesnt work
    println!("{}", 5);

    //Place holders in this case 'Victor' is the first index the 0, and maths de second index the 1
    println!("My best friend is {0} and {0} loves {1}", "Victor", "maths");

    // Incorporating arguments
    println!("My name is {name} and i am {age}", name="Luis", age=22);

    // Tuples - prints multiple tuples
    println!("{:?}", ("Ellie", 23, true));

    // Basic mathematics, prints the binary equivalent to 10 ----> 1010
    println!("Binary: {:b}", 10);
    // Addition
    println!("the sum of 5 plus 10 is {}", 5 + 10);
    // Division
    println!("the division of 5 plus 10 is {}", 5 / 10);
    // Multiplication
    println!("the multiplication of 5 plus 10 is {}", 5 * 10);
    // Rest
    println!("the rest of 5 plus 10 is {}", 5 % 10);

}
