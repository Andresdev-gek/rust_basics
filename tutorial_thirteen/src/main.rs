// We can create a function in another file
// and then run it in our main file
mod file1;

fn main() {
    file1::maths();
    //println!("hello from main")
}
