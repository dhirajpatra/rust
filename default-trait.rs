#[derive(Debug)]
struct Person {
	name: String,
	age: u8,
}

impl Default for Person { 
	fn default() -> Self {
		Person { 
			name: "Bot".to_string(),
			age: 25,
		}
	}
}

fn create_default() -> Person {
	let tmp = Person {
		..Person::default()
	};
	let john = Person {
		name: "John".to_string(),
		..tmp
	};
	john
}

fn main() {
	let john = create_default();
	println!("{:#?}", john);
	println!("{:#?} is {:#?} years old", john.name, john.age);
}