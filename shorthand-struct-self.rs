#[derive(Debug)]
struct Person {
    // The name of the person.
    name: String,
    // The age of the person.
    age: u8,
}

// This function creates a new `Person` instance.
impl Person {
    fn new(name: String, age: u8) -> Self {
        // Create a new `Person` instance with the given name and age.
        Self { name, age }
    }
}

fn main() {
    // Create a new `Person` instance named `Peter` who is 27 years old.
    let peter = Person::new(String::from("Peter"), 27);

    // Print the `Person` instance to the console.
    println!("{peter:?}");
    println!("The name of the person: {}", peter.name);
    println!("The age of the person: {}", peter.age);
}
