#[derive(Debug)]
struct Point(i32, i32);

/// This function returns the left-most point of two points.
///
/// # Arguments
///
/// * `p1` - The first point to compare.
/// * `p2` - The second point to compare.
///
/// # Returns
///
/// The left-most point of `p1` and `p2`.
fn left_most<'a>(p1: &'a Point, p2: &'a Point) -> &'a Point {
    // The `'a` lifetime is used as generic lifetime declaration to specify that the references to `p1` and `p2` must be valid for the duration of the function call.

    // Check if p1 is to the left of p2
    if p1.0 < p2.0 {
        // Return p1
        p1
    } else {
        // Return p2
        p2
    }
}

fn main() {
    // Create two points
    let p1: Point = Point(10, 10);
    let p2: Point = Point(20, 30);

    // Get the left-most point
    let p3: &Point = left_most(&p1, &p2);

    // Print the left-most point
    println!("left-most point: {:?}", p3);
}
