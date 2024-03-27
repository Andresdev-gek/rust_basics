// Option enum

// Lots of programming languages have null values
// which represent a very useful concept - a value
// can either exist or it can be null
// i.e there is no value

//the problem is that the type system can't
// guarantee that if you use a value, it's not null.
// In Rust there NO null values.

enum Option<T> {
    None,    // stores no value
    Some(T), // stores some value in here
}

enum Movement {
    // variants
    Left,
    Right,
    Jump,
}
fn move_player(m: Movement) {
    // Perform action depending on information
    match m {
        Movement::Left => println!("Moving left"),
        Movement::Right => println!("Moving right"),
        Movement::Jump => println!("Jumping!"),
    };
}
fn main() {
    let number = Some(10);
    let boolean = Some(true);
    // since we are passing the None type, we
    // need to annotate the type - Rust can't
    // infer it.
    let nothing: std::option::Option<i32> = None;
    // this can either be 20 or it can be nothing
    let something: std::option::Option<i32> = Some(20);

    //println!("nothing {}", nothing);
    //println!("something {}", something);

    // Addition
    // We can't do this as they are not the same
    // data type!
    let y: i32 = 14;
    //let sum = x + something;
    //println!("sum  {}", sum);

    // instead we can use unwrap
    let sum = y + something.unwrap_or(12);

    // Match expression
    // we have defined an enum above called Movement
    let player1 = Movement::Jump;
    let player2 = Movement::Right;

    move_player(player1);
    move_player(player2);
}
