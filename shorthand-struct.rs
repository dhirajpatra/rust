// Allow the `dead_code` warning to be suppressed for the entire struct.
#[allow(dead_code)]
#[derive(Debug)]
struct Person {
    name: String,
    age: u16,
}

impl Person {
    // Create a new instance of the `Person` struct.
    fn new(name: String, age: u16) -> Person {
        Person { name, age }
    }
}

fn main() {
    // Create a new `Person` instance using the `new` method.
    let peter = Person::new(String::from("Peter"), 27);
    // Print the `peter` instance using the `Debug` format specifier.
    println!("{peter:?}");
}
