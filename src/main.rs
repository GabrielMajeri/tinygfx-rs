/// Limits a value to a closed interval.
///
/// Values which are out of range are clamped to the margins of the interval.
fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

/// Converts a normalized color intensity to an 8-bit unsigned integer representation.
fn quantize_channel(chan: f32) -> u8 {
    (clamp(chan, 0.0, 1.0) * 255.0) as u8
}

/// Converts a pixel with normalized floating-point intensities to a 24-bit depth color value.
fn quantize(pixel: &image::Rgb<f32>) -> image::Rgb<u8> {
    let [r, g, b] = pixel.data;
    image::Rgb([
        quantize_channel(r),
        quantize_channel(g),
        quantize_channel(b),
    ])
}

fn main() {
    // Configurable dimensions for the canvas
    let width = 1024;
    let height = 768;
    // Solid black
    let background_color = image::Rgb([0.0f32; 3]);

    use image::ImageBuffer;
    let framebuffer = ImageBuffer::from_pixel(width, height, background_color);

    // Save the rendered frame to disk
    let output_buffer =
        ImageBuffer::from_fn(width, height, |x, y| quantize(framebuffer.get_pixel(x, y)));

    let output_path = "output.png";
    output_buffer
        .save(output_path)
        .expect("Failed to save generated framebuffer to file");
}
