fn get_value(_input: &str) -> Result<String, std::io::Error> {
    // Some code to get the value from the input
    match Some("matched value") {
        Some(value) => Ok(value.to_string()),  // Assuming you want to return "value"
        None => Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to get value")),
    }
}

fn main() {
    let result = get_value("input");

    let value = match result {
        Ok(ref v) => Some(v.as_str()),
        Err(_) => None,
    };

    match value {
        Some(value) => println!("The value is {}", value),
        None => println!("An error occurred"),
    }
}
