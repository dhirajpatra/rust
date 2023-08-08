fn main() {
    // 32 bit integer variable
    let mut x: i32 = 6;
    println!("{x}");

    // loop till x = 1
    while x != 1 {
        // for even number
        if x % 2 == 0 {
            x = x / 2;
        } else {
            // increase the value of x
            x = (x * 3) + 1;
        }
        print!(" -> {x}");
    }
    // print a blank line
    println!();
}
