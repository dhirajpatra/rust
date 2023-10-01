#[derive(Debug)]
struct Highlight<'doc>(&'doc str);

fn erase(text: String) {
    println!("Bye {text}!");
}

fn main() {
    let text = String::from("The quick brown fox jumps over the lazy dog");
    let fox = Highlight(&text[4..19]);
    let dog = Highlight(&text[35..39]);
    // if we use here then next two line would be error
    // erase(text);
    println!("{fox:?}");
    println!("{dog:?}");
    erase(text);
}