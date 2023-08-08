#[derive(Debug)]

struct Rectangle {
	width: u32,
	height: u32
}

impl Rectangle {
	// no chnage required into the obj
	fn area(&self) -> u32 {
		self.width * self.height
	}

	// it will change the width of the obj so require mutable
	fn inc_width(&mut self, delta: u32) {
		self.width += delta;
	}
}

// static method used as a constructor
fn new(width: u32, height: u32) -> Rectangle {
	Rectangle { width, height }
}

fn main() {
	// mutable so that later can chnage its attributes values
	// this will automatically call the constructor
	let mut rect = Rectangle { width: 10, height: 5 };
	println!("old area {}", rect.area());
	rect.inc_width(5);
	println!("new area {}", rect.area());

	// we dont want to change the value so default immutable
	let srect = new(20, 10);
	println!("old static area {}", srect.area());
}