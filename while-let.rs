// This program prints all the elements of a vector to the console.

fn main() {
    // Create a vector.
    let v = vec![10, 20, 30];

    // Iterate over the vector using an iterator.
    let mut iters = v.into_iter();

    // Print each element of the vector.
    while let Some(i) = iters.next() {
        println!("i: {i}");
    }
}
