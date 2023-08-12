#[derive(Debug)]

struct Point(i32, i32);

fn add(p1: &Point, p2: &Point) -> Point {
	// Create a new Point variable and set its fields to the sum of the fields of p1 and p2
	let p = Point(p1.0 + p2.0, p1.1 + p2.1);
	
	// Print the address of the 0 field of p
	println!("&p.0: {:p}", &p.0);

	// Return p
	p
}

pub fn main() {
	let p1 = Point(3, 4);
	let p2 = Point(10, 20);
	let p3 = add(&p1, &p2);
	println!("&p3.0: {:p}", &p3.0);
	println!("{p1:?} + {p2:?} = {p3:?}");
}