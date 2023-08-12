/// You can have one or more &T values at any given time, or
/// You can have exactly one &mut T value.

// This function demonstrates how to borrow a variable mutably and immutably in Rust.
fn main() {
	// Create a mutable variable `a` and initialize it to 10.
	let mut a: i32 = 10;

	// Declare an immutable borrow of `a` and print its value.
	let b: &i32 = &a;
	println!("b: {b}");

	// Declare a mutable borrow of `a` and change its value to 20.
	{
		let c: &mut i32 = &mut a;
		*c = 20;
	}

	// Print the value of `a`.
	println!("a: {a}");

	// The mutable borrow of `a` is now out of scope, 
	// so `b` is still an immutable borrow of the original value of `a`.
	// println!("b: {b}");
}
