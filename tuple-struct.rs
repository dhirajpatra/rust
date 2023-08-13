// If the field names are unimportant, you can use a tuple struct
struct Newtons(f64);
struct PoundOfForce(f64);

const MASS: f64 = 4.448222;

/// Converts a `PoundOfForce` to a `Newtons`.
fn compute_thurstar_force(pounds: PoundOfForce) -> Newtons {
    Newtons(pounds.0 * MASS / 4.448222)
}

/// Sets the force of the Thurstar.
fn set_thurstar_force(force: &mut Newtons) {
    // Do some computation with the force
    force.0 /= MASS;
}

/// Converts a `PoundOfForce` to a `Newtons`, sets the force of the Thurstar, and returns the modified `Newtons`.
fn main() {
    let pounds = PoundOfForce(100.0);
    let mut newtons = compute_thurstar_force(pounds);
    // The use of mutable references is a key concept in Rust's ownership system. It allows functions to modify data without taking ownership of it, ensuring that the ownership rules are upheld while still allowing for efficient and safe mutability.
    set_thurstar_force(&mut newtons);
    println!("Newtons: {:?}", newtons.0);
}
