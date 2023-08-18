// extern crate random_number;
use random_number::random;

fn main() {
    let num: i8 = random!(..);
    println!("{}", num);
}