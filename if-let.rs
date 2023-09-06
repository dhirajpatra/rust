// This program prints all the command-line arguments to the console.

fn main() {
    // Get the command-line arguments.
    let mut args = std::env::args();

    // Print the program name.
    if let Some(value) = args.next() {
        println!("Program name: {value}");
    } else {
        println!("Missing name?");
    }

    // Print all the remaining arguments.
    for arg in args {
        println!("Argument: {}", arg);
    }
}
