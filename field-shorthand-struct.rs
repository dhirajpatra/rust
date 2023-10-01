#[derive(Debug)]
struct Person {
	name: String,
	age: u8,
}

impl Person{
	fn new(name: String, age: u8) -> Person {
		Self { name, age }
	}
}

fn main() {
	let peter = Person::new(String::from("Peter"), 27);
	// println!("{peter:?}");
	println!("{} is {} years old", peter.name, peter.age);
}