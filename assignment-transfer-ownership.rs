fn main() {
	let s1: String = String::from("hello");
	// let s1_address = &s1;
	let s2: String = s1;
	// let s2_address = &s2;
	println!("s2: {s2}");
	// println!("s1 address: {:?}", s1_address);
	// println!("s2 address: {:?}", s2_address);
	// now s1 freed as goes off the scope and copied to s2
	// heap data for s1 now used for s2 it will error out
	// println!("s1: {s1}");
}