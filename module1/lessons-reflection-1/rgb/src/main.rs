// Store RGB color values in a vector or array and print them out
fn main() {
    let colors: Vec<(u8, u8, u8)> = vec![
        (255, 0, 0),   // Red
        (0, 255, 0),   // Green
        (0, 0, 255),   // Blue
        (255, 255, 0), // Yellow
        (0, 255, 255), // Cyan
        (255, 0, 255), // Magenta
    ];

    // Print out the RGB values and align them nicely
    println!("Stored RGB Colors:");
    for (r, g, b) in &colors {
        // Print in the format: RGB(255,   0,   0)
        // Each value is right-aligned in a field of width 3
        println!("RGB({:>3}, {:>3}, {:>3})", r, g, b);
    }
}
