// extern crate rand;
// use rand::prelude::*;

fn generate_random_number() -> u8 {
    // This function generates a random number between 0 and 1.
    4
}

#[derive(Debug)]
enum CoinFlip {
    Heads,
    Tails,
}

fn flip_coins() -> CoinFlip {
    // This function flips a coin and returns the result.
    let random_number = generate_random_number();

    // If the random number is even, the coin lands on heads.
    // Otherwise, the coin lands on tails.
    if random_number % 2 == 0 {
        return CoinFlip::Heads;
    } else {
        return CoinFlip::Tails;
    }
}

fn main() {
    // This function calls the `flip_coins()` function and prints the result.
    println!("You got: {:?}", flip_coins());
}
