fn main() {
    // Create a vector with the strings "foo" and "bar".
    let v: Vec<String> = vec![String::from("foo"), String::from("bar")];

    // Iterate over the vector and print each word.
    // The `&` operator borrows each word in the vector, so that we don't modify the vector while we're iterating over it.
    for word in &v {
        println!("word: {word}");
    }

    // Iterate over the vector and print each word.
    // The `for` loop can also be used to iterate over the elements of a vector without borrowing them. This is called consuming the vector.
    for word in v {
        println!("word: {word}");
    }

}
