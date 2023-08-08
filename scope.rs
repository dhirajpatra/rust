fn main() {
	let a = 10;
	println!("before a: {a}");

	let b = &a;
	println!("&a: {b}");

	{
		let a = "hello";
		println!("inner scope a: {a}");
		// let b = &a;
		println!("inner scope &a: {b}");

		let a = true;
		println!("shadowed in inner scope a: {a}");
		// b = &a;
		// println!("shadowed in inner scope &a: {b}");
	}

	println!("after a: {a}");
	// b = &a;
	println!("after &a: {b}");
}