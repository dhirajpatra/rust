fn main() {
	// &str an immutable reference to a string slice.
	let s1: &str = "World";
	println!("s1: {s1}");

	// String a mutable string buffer
	let mut s2: String = String::from("Hello ");
	println!("s2: {s2}");
	s2.push_str(s1);
	println!("s2: {s2}");

	// no need mut if we just need to create empty string
	let mut s3: String = String::new();
	println!("s3: {s3}");
	s3.push_str(s1);
	println!("s3: {s3}");

	let s4: &str = &s2[6..];
	println!("s4: {s4}");
}