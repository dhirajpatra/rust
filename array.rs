fn main() {
    // for pretty print
    let mut a: [i8; 10] = [0; 10];
    println!("a: {:#?}", a);
    a[5] = 100;
    // for normal print
    println!("a: {:?}", a);
}
