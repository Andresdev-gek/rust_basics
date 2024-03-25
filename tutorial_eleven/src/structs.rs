// Structs

// Structs in Rust are similar to classes in other
// programming lenguages.
// Structs are similar to tuples in that you can hold
// multiple values and they can be different types
// Unlike tuples, in a struct you name each piece
// of data so you know ehat the values mean
// Structs are more flexible than tuples.

// Traditional Structs
struct Colour {
    red: u8, // u8 has range 0 to 255
    green: u8,
    blue: u8,
}

// Tuple Struct
struct Colour2(u8, u8, u8);
