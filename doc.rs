/// Determine whether first argument is divisible by the second argument
///
/// if the second argument is zero, the result is false
fn main() {
  let lhs: u32 = 5;
  let rhs: u32 = 10;

  if test(lhs, rhs) {
  	println!("good");
  } else {
  	println!("bad");
  }
}

fn test(lhs: u32, rhs: u32) -> bool {
	if rhs == 0 {
    	return false;
  	}
	return lhs % rhs == 0;
}