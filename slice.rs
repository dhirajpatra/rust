fn main() {
	let a: [i32; 6] = [10, 20, 30, 40, 50, 60];
	println!("a: {a:?}");

	let s: &[i32] = &a[2..4];
	println!("s: {s:?}");

	let mut b: [i32; 6] = [10, 20, 30, 40, 50, 60];
	println!("b: {b:?}");

	b[3] = 400;
	println!("b: {b:?}");

	// Declare a variable named `s` of type `&[i32]`.
	// Assign the value of the slice `b[2..4]` to the variable `s`.
	let s: &[i32] = &b[2..4];
	println!("s: {s:?}");
}