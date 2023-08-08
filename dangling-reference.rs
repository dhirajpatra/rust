fn main() {
	let ref_x: mut &i32;
	{
		let x: i32 = 10;
		// dangling referece error
		// ref_x = &x;
		let ref_x = &mut x;
	}
	println!("ref_x: {ref_x}");
}