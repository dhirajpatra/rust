#[derive(Debug)]
struct HighLight<'doc>(&'doc str);

/// This function erases the given text.
///
/// # Arguments
///
/// * `text` - The text to erase.
fn erase(text: String) {
    // Print the erased text to the console.
    println!("erased: {}", text);
}

/// This function creates two `HighLight` structs from the given text.
///
/// # Arguments
///
/// * `text` - The text to create `HighLight` structs from.
fn main() {
    // Create a string with the text "The quick brown fox jumps over the lazy dog".
    let text = String::from("The quick brown fox jumps over the lazy dog");

    // Create a `HighLight` struct from the substring "quick brown fox".
    let fox = HighLight(&text[4..19]);

    // Create a `HighLight` struct from the substring "lazy dog".
    let dog = HighLight(&text[35..43]);

    // Do not erase the text.
    // erase(text);

    // Print the `HighLight` structs to the console.
    println!("{fox:?}");
    println!("{dog:?}");
    erase(text);
}
