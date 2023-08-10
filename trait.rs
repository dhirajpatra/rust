
// In Rust, a trait is a collection of methods that can be implemented by different types. 
// Traits are similar to interfaces in other programming languages, but they have some important differences.
//
// Traits can be implemented for any type, even if the type was not defined by the programmer. 
// This makes traits very flexible and powerful.
// Traits can be used to define generic functions. 
// This means that a function can be defined that can work with any type that implements a particular trait.
// Traits can be used to specify the behavior of a type. 
// This can be used to enforce certain invariants on the type, or to make it easier to write generic code.

trait Animal {
	fn make_noise(&self) -> String;

	// with default value 
	fn make_bark(&self) -> String {
		String::from("default barking sound!")
	}
}

// This struct implements the Animal trait by providing an implementation of the make_noise method. The implementation of the make_noise method simply returns a string with the name of the dog.
struct Dog  {
	name: String,
}

impl Animal for Dog {
	// must implement this trait function
	fn make_noise(&self) -> String { 
		return format!("woof! My name is {}", self.name);
	}
	// you can or not implement make_bark function as it has default value
}

fn main() {
	let dog = Dog { 
		name: String::from("Lali"), 
	};
	println!("{}", dog.make_noise());
}