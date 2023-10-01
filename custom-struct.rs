// This struct represents a person.
struct Person {
    // The person's name.
    name: String,
    // The person's age.
    age: u8,
}

// The main function.
fn main() {
    // Create a new Person named Peter with the age 27.
    let mut peter = Person {
        name: String::from("Peter"),
        age: 27,
    };

    // Print Peter's name and age.
    println!("{} is {} years old", peter.name, peter.age);

    // Increase Peter's age by 1.
    peter.age = 28;

    // Print Peter's name and age again.
    println!("{} is {} years old", peter.name, peter.age);

    // Create a new Person named Jackie based on Peter's data, but with the name "Jackie".
    let jackie = Person {
        name: String::from("Jackie"),
        ..peter
    };

    // Print Jackie's name and age.
    println!("{} is {} years old", jackie.name, jackie.age);
}
