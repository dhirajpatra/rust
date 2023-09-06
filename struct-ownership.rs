struct Point(i32, i32);

fn main() {
	{
		let p = Point(3, 4);
		println!("x: {}", p.0);
	}

	// ownership taken away from this
	// println!("y: {}", p.1);
}