// Data types

// We'll be looking at Primitive Data Types - the fundamental ones
// There are 2 categories of data types:

// Scalar types: A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.
// Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

// Recall, Rust is a statically typed programming language which means
// it must know the types of all the variables when it gets
// to compile time. Despite that sounding like a lot of work,
// Rust can usually infer the data type if left blank.
fn main() {
    let x: i32 = 10;
    println!("Maximum i32 is {}", std::i32::MIN);
    println!("Maximum i32 is {}", std::i32::MAX);

    // Floating point types
    // a float is a number with a decimal after it e.g 10.5
    // floats can be f32 of f64 - default is f64
    let y: f32 = 10.5;
    println!("y value is {}", y);
    println!("Maximum f32 is {}", std::f32::MIN);
    println!("Maximum f32 is {}", std::f32::MAX);

    // Booleans
    // consists either true or false
    let true_or_false: bool = false;
    println!("y value is {}", true_or_false);

    // Characters 
    // These consist of ONE letter/character
    let character: char = 'a';
    let character: char = '\u{1F60A}';
    println!("y value of the character is {}", character);

    // compound types
    // tuples
    let mut tuple1: (i32, char, bool) = (22, 'e', true);
    tuple1.0 = 300;
    println!("{:?}", tuple1);

    // Arrays
    let mut arr = [10, 20, 30]; // all the elements in this array will be
    // i32 unlike any other programming language, you cant add any extra element with
    // diferent type onto this array :( BUT theres something called 'vectors' which
    // allow you to do this and we'll touch upon them later

    // modifying the elements
    arr[0] = 10000;
    println!("{:?}", arr);

    // explicitly state the types in our array
    let arr2: [i32; 4] = [1,2,3,4];
    println!("{:?}", arr2);

    
}
