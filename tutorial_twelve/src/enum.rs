// Enums

// Structs and enums are the building blocks for
// creating new types in rust

// Enums allow us to enumerate a list of variants
// but you may wondering, when is it
// appropriate to use enums over structs?

// Let's consider IP Addresses - we can enumerate
// all the variants - there's only 2! V4 or V6

enum IpAddressKind {
    // to store actual data inside your enum
    // you just write () and whatever data you
    // want inside
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,                        // a variant that stores no data
    Move { x: i32, y: i32 },     // stores anonymous struct
    Write(String),               // single string
    ChangeColour(i32, i32, i32), // 3 integers
}

struct QuitMessage; //unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); //tuple struct

// The IP Address struct allows us to group
// the version of the IP Address with the
// actuall address!
struct IpAddress {
    kind: IpAddressKind, //this is our enum
    address: String,
}

// We can also make methods and associated functions
// in enums just like we did with structs!

impl Message {
    fn function1() {
        println!("Code of the future!")
    }
}

fn main() {
    // we can create instances for each of
    // variants
    let ip_four = IpAddressKind::V4;
    let ip_six = IpAddressKind::V6;
    // So we could create a function that will
    // take in our enum type so either 4 or 6. we can do this
    // because they are both the same type
    let ipv4 = IpAddress {
        kind: IpAddressKind::V4(23, 23, 23, 23),
        address: String::from("127.0.0.1"),
    };

    let ipv6 = IpAddress {
        kind: IpAddressKind::V6("hola".to_string()),
        address: String::from("::1"),
    };

    let local_host = IpAddressKind::V4(232, 123, 234, 6);
}
