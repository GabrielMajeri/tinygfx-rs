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

use image::ImageBuffer;
type Pixel = image::Rgb<f32>;
type RenderBuffer = ImageBuffer<Pixel, Vec<f32>>;

fn render(buffer: &mut RenderBuffer) {
    let width = buffer.width();
    let height = buffer.height();

    buffer.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        let red = 0.0;
        let green = x as f32 / width as f32;
        let blue = y as f32 / height as f32;
        *pixel = image::Rgb([red, green, blue]);
    });
}

fn main() {
    // Configurable dimensions for the canvas
    let width = 1024;
    let height = 768;
    // Solid black
    let background_color = image::Rgb([0.0f32; 3]);

    let mut framebuffer = ImageBuffer::from_pixel(width, height, background_color);

    render(&mut framebuffer);

    // Save the rendered frame to disk
    let convert_pixel = |x, y| quantize(framebuffer.get_pixel(x, y));
    let output_buffer = ImageBuffer::from_fn(width, height, convert_pixel);

    let output_path = "output.png";
    output_buffer
        .save(output_path)
        .expect("Failed to save generated framebuffer to file");
}
