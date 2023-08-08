
#![allow(unused_variables, dead_code)]
/** 
 * by the above statement it wont create any error while compiling
 * as we put inside the function unimplemented();
 * and it will generate
 * matrix: 
 * thread 'main' panicked at 'not implemented', transpose.rs:9:5
 * note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
 * 
 * However later when implemented all code commented unimplemented()
 */

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
	// unimplemented!()
	let mut result = [[0; 3]; 3];
	for i in 0..3 {
		for j in 0..3 {
			result[j][i] = matrix[i][j];
		}
	}
	return result;
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
	// unimplemented!()
	for row in matrix {
		println!("{row:?}");
	}
}

#[test]
fn test_transpose() {
	let matrix = [
		[101, 102, 103],
		[201, 202, 203],
		[301, 302, 303],
	];
	let transposed = transpose(matrix);

	assert_eq!(
			transposed, 
			[
				[101, 201, 301],
				[201, 202, 203],
				[301, 302, 303],
			]
		);
}

fn main() {
	let matrix = [
		[101, 102, 103],
		[201, 202, 203],
		[301, 302, 303],
	];

	println!("matrix: ");
	pretty_print(&matrix);


	let transposed = transpose(matrix);
	println!("transposed: ");
	pretty_print(&transposed);
}