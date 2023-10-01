#[derive(Debug)]
struct Person {
	name: String,
	age: u8,
}

impl Person {
	fn say_hello(&self) {
		println!("Hello my name is {} and my age is {}", self.name, self.age);
	}
}

fn main() {
	let peter = Person {
		name: String::from("Peter"),
		age: 27,
	};
	peter.say_hello();
}