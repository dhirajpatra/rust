#[derive(Debug)]
// The #[derive(Debug)] attribute in Rust tells the compiler to automatically generate a basic implementation of the Debug trait for the following struct. The Debug trait allows you to print the value of a struct in a human-readable format, using the {:?} formatter.
struct Point(i32, i32);

fn add(p1: &Point, p2: &Point) -> Point {
	Point(p1.0 + p2.0, p1.1 + p2.1)
}

fn main() {
	let p1 = Point(3, 4);
	let p2 = Point(10, 20);
	let p3 = add(&p1, &p2);
	println!("{p1:?} + {p2:?} = {p3:?}");
}