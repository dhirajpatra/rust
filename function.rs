fn say_hallo(name: String) -> () {
	// with ; after println is interpreted as a complete statement with no return value.
	// so compiler produce error
	// if this function returns the value () so if you use ; to send value would error
	println!("hallow {name}");
}

fn main() {
	let name = String::from("Alice");
	say_hallo(name);
}

