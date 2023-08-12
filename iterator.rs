pub trait Iterator {
    // The type of the items produced by this iterator.
    type Item;

    // Advances the iterator and returns the next item, or `None` if there are no more items.
    fn next(&mut self) -> Option<Self::Item>;
}

fn main() {
    // Create a vector with the values 10, 20, and 30.
    let v: Vec<i8> = vec![10, 20, 30];

    // Create an iterator over the vector.
    let mut iter = v.iter();

    // Print the first three items in the vector.
    println!("v[0]: {:?}", iter.next());
    println!("v[1]: {:?}", iter.next());
    println!("v[2]: {:?}", iter.next());

    // Try to print the fourth item in the vector. This will return `None` because there are no more items.
    println!("no more items: {:?}", iter.next());

    // Get the first item in the vector without consuming it. This will return a reference to the item.
    // The `next` method can also be used to get a reference to the next item in the iterator without consuming it. This is useful if you want to iterate over the items in the iterator without removing them.
    let v0: Option<&i8> = iter.next();

    // The `{:?}` format specifier is used to print the value of a reference.
    println!("v0: {v0:?}");

    // Get the first item in the vector without consuming it. This will return a reference to the item.
    let _v0: Option<&i8> = iter.next();

    // Print the first item in the vector again. This will still be 10 because we didn't consume it.
    println!("v[0]: {:?}", v[0]);
}
