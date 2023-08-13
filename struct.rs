// This struct represents a person.
struct Person {
    // The person's name.
    name: String,
    // The person's age.
    age: u8,
}

// This function is the main entry point of the program.
fn main() {
    // Create a new person named Peter who is 27 years old.
    let mut peter = Person {
        name: String::from("Peter"),
        age: 27,
    };
    // Print Peter's name and age.
    println!("{} is {} years old", peter.name, peter.age);

    // Change Peter's age to 28.
    peter.age = 28;
    // Print Peter's name and age again.
    println!("{} is {} years old", peter.name, peter.age);

    // Create a new person named Jacki 
    // The syntax ..peter allows us to copy the majority of the fields from the old struct without having to explicitly type it all out. It must always be the last element
    let jacki = Person {
        name: String::from("Jacki"),
        // same age as Peter
        ..peter
    };
    // Print Jacki's name and age.
    println!("{} is {} years old", jacki.name, jacki.age);
}
