// Control Flow: if, else if, while, etc. 

fn main() {
    // CONDITIONS - any expression that evaluates
    // to true or false (boolean data type)
    // You can create conditions using operators such
    // as < <= > >= != == (conditional operators)

    let condition1 = 5 > 7;
    println!("{}", condition1);

    // both types must be the same
    // type casting to counteract this
    let condition2 = (5 as f32) < 7.2;
    println!("{}", condition2);

    // Compound Conditions
    // Multiple conditions changed together using
    // logical operators
    // and && 
    // or || 
    // not ! negates the boolean

    let condition3 = true || condition1;
    println!("{}", condition3);


    
}
