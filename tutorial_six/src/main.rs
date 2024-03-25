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

    let condition3 = true || !condition1;
    println!("{}", condition3);

    //Control flow - If, Else if, Else Statement
    let ten = 120;

    
    // If statement
    if ten == 10 {
        println!("the number is 10!")
    } else if ten == 120 {
        println!("the number is 120!")
    } else {
        println!("the number is not 10!")
    }

    //short hand control flow
    let age = 50;
    let teenager = if age >= 12 && age <= 17 {true} else {false};
    let children = if age < 13 {true} else {false};
    
    if teenager {
        println!("is teenager");
    } else if children {
        println!("is children");
    } else {
        println!("is adult")
    }

    // Loops
    // These are user to iterate until a condition is met

    // Infinite
    let mut count = 0;
    
    //loop {
    //    println!("Number {}", count);
    //    count += 1;
    //}

    loop {
        println!("Number is: {}", count);
        count += 1;
        if count == 10 {
            break;
        }
    }
    
    // While loop
    while count <= 100 {
        if count % 15 == 0 {
           println!("Opaaaai"); 
        } else {
            println!("yeeei")
        }

        // Increment
        count += 1;
    }

    // For range loop
    for x in 0..10 { // this will loop from 0 to 9
        println!("{}", x);
    }

    // Iterating loop 
    let a = [10, 20, 30, 40];
    for element in a.iter() {
        println!("value is {}", element);
    }

    
}
