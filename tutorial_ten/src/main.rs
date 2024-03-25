// Pointers and References

fn main() {
    // Arrays - these are primitive!
    let array1 = [1, 2, 3, 4];
    let array2 = array1;
    let array3 = array2;
    println!("{:?}", (array1, array2, array3));

    // Vectors - primitive or non-primitive?
    // Vectors are NON-PRIMITIVE. We can't do this:

    //let vector1 = vec![1, 2, 3];
    //let vector2 = vector1;
    //println!("{:?}", (vector1, vector2));

    // To make it work we make vector1 a reference
    // using the &
    let vector1 = vec![1, 2, 3];
    let vector2 = &vector1;
    let vector3 = &vector2;

    println!("{:?}", (&vector1, vector2, vector3));
}
