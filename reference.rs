fn main() {
	// mut means mutable so this variable values can be changed later
    let mut x: i32 = 10;
    // providing reference or pointer as c or c++
    let ref_x: &mut i32 = &mut x;
    // change the value of x by reference
    *ref_x = 20;

    let address = format!("{:p}", ref_x); // this produces something like '0x7f06092ac6d0'
 	println!("{}", address);
    println!("x: {x}");
}
