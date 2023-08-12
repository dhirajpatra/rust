pub trait IntoIterator {
    // The type we iterate over, such as i8.
    type Item;

    // The Iterator type returned by the into_iter method.
    type IntoIter: Iterator<Item = Self::Item>;

    // Converts self into an iterator over its elements.
    fn into_iter(self) -> Self::IntoIter;
}

// Note that IntoIter and Item are linked: the iterator must have the same Item type,
// which means that it returns Option<Item>.

fn main() {
    // Create a vector with the strings "foo" and "bar".
    let v: Vec<String> = vec![String::from("foo"), String::from("bar")];

    // Create an iterator over the vector.
    let mut iter = v.into_iter();

    // Get the first item in the iterator.
    let v0: Option<String> = iter.next();

    // Print the first item in the iterator.
    println!("v0: {v0:?}");
}
