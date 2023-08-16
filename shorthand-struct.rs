#[allow(dead_code)]
#[derive(Debug)]
struct Person {
	name: String,
	age: u16,
}

impl Person {
	fn new(name: String, age: u16) -> Person {
		Person{ name, age }
	}
}

fn main() {
	let peter = Person::new(String::from("Peter"), 27);
	println!("{peter:?}");
}