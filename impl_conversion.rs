fn multipy(x: i16, y:i16) -> i16 {
	x * y
}

fn main() {
	let x: i8 = 5;
	let y: i16 = 10;

	// conversion require here x will be i16
	println!("{x} * {y} = {}", multipy(x.into(), y));
}