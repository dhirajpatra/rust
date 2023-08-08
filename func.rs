fn main() {
	print_fizzbuzz_to(20);
}

fn is_divisible(n: u32, divisior: u32) -> bool {
	if divisior == 0 {
		return false;
	}
	n % divisior == 0
}

fn fizzbuzz(n: u32) -> String {
	let fizz = if is_divisible(n, 3) { "fizz" } else { "" };
	let buzz = if is_divisible(n, 5) { "buzz" } else { "" };
	if fizz.is_empty() && buzz.is_empty() {
		return format!("{n}");
	}
	return format!("{fizz}{buzz}");
}

fn print_fizzbuzz_to(n: u32) {
	for i in 1..=n {
		println!("{}", fizzbuzz(i));
	}
}