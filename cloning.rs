/*
After the assignment, both p1 and p2 own their own data.
We can also use p1.clone() to explicitly copy the data.
*/

#[derive(Copy, Clone, Debug)]
struct Point(i32, i32);

fn main() {
    let p1 = Point(3, 4);
    let p2 = p1;
    println!("p1: {p1:?}");
    println!("p2: {p2:?}");
}