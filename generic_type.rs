/**
 * The following code tells the compiler to copy into a certain generic container 
 * without the code ever explicitly specifying the contained type, 
 * using _ as a placeholder:
 */

fn main() {
	let mut v = Vec::new();
	v.push((10, false));
	v.push((20, true));
	println!("v: {v:?}");

	let vv = v.iter().collect::<std::collections::HashSet<_>>();
	println!("vv: {vv:?}");
}