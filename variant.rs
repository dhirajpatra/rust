// This enum defines three variants of web events: page load, key press, and click.
// The `PageLoad` variant does not have any payload.
// The `KeyPress` variant has a single character payload.
// The `Click` variant has two integer payloads, representing the x and y coordinates of the click.
enum WebEvent {
    // Variant without payload
    PageLoad,
    // Tuple struct variant
    KeyPress(char),
    // Full struct variant
    Click { x: i64, y: i64 },
}

// This function takes a `WebEvent` and prints a message depending on the variant.
// The `#[rustfmt::skip]` attribute tells Rust to skip formatting the code in this function.
#[rustfmt::skip]
fn inspect(event: WebEvent) {
    match event {
        // Print a message if the event is a page load.
        WebEvent::PageLoad => println!("page loaded"),
        // Print a message and the character pressed if the event is a key press.
        WebEvent::KeyPress(c) => println!("key pressed '{c}'"),
        // Print a message and the x and y coordinates of the click if the event is a click.
        WebEvent::Click { x, y } => println!("clicked at x={x}, y={y}"),
    }
}

fn main() {
    // Create three `WebEvent`s.
    let load = WebEvent::PageLoad;
    let pressed = WebEvent::KeyPress('x');
    let click = WebEvent::Click { x: 20, y: 80 };

    // Call the `inspect` function on each `WebEvent`.
    inspect(load);
    inspect(pressed);
    inspect(click);
}
