pub fn luhn(cc_number: &str) -> bool {
    // Remove spaces and reverse the string
    let cc_number = cc_number
        .chars()
        .filter(|&c| !c.is_whitespace())  // Filter out whitespace characters
        .collect::<String>()
        .chars()
        .rev()
        .collect::<String>();  // Reverse the string

    let mut sum = 0;
    let mut double = false;

    // Iterate over the characters in the reversed string
    for c in cc_number.chars() {
        if let Some(digit) = c.to_digit(10) {
            let mut value = digit;
            if double {
                value *= 2;
                if value > 9 {
                    value -= 9;
                }
            }
            sum += value;
            double = !double;  // Toggle the "double" flag for every digit
        } else {
            // If a non-digit character is encountered, return false immediately
            return false;
        }
    }

    // Check if the sum is divisible by 10
    sum % 10 == 0
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}

fn main() {}
