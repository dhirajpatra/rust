#[derive(Debug)]
struct Person {
    // The name of the person.
    name: String,
    // The age of the person.
    age: u8,
}

// This function is the default implementation for the `Default` trait for the `Person` struct.
impl Default for Person {
    // This function returns a new `Person` instance with the default values for the `name` and `age` fields.
    fn default() -> Self {
        Person {
            name: "Bot".to_string(),
            age: 0,
        }
    }
}

fn main() {
    // Create a new `Person` instance named `Peter` who is 27 years old.
    let peter = Person {
        name: String::from("Peter"),
        age: 27,
    };

    // Print the `Person` instance to the console.
    println!("{peter:?}");
    println!("The name of the person: {}", peter.name);
    println!("The age of the person: {}", peter.age);
}
