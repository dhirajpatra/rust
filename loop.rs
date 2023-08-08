
fn main() {
	let array = [10, 20, 30];
	println!("array: {array:?}");

	print!("iterating over the array");
	for n in array {
		print!(" {n} ");
	}
	println!();

	print!("iterating over a range");
	for i in 0..3 {
		print!(" {}", array[i]);
	}
	println!();
}