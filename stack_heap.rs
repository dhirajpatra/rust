fn main() {
	let mut s1 = String::from("hello");
	s1.push(' ');
	s1.push_str("world");
	// for _ in 0..10 {
	// 	s1.push_str(" world");
	// }
	println!("s1: {}", s1);

	// know about the heap
	unsafe{
		let (ptr, capacity, len): (usize, usize, usize) = std::mem::transmute(s1);
        println!("ptr = {ptr:#x}, len = {len}, capacity = {capacity}");
	}
}