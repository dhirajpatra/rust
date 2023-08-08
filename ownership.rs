struct Point(i32, i32);

fn main() {
	{
		let p = Point(3, 4);
		println!("x: {}", p.0);
	}
	// just uncomment the below line and then recompile again
	// println!("y: {}", p.1);
}