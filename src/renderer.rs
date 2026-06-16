//! Renderer: Image rendering and PPM output

use std::fs::File;
use std::io::{self, Write};
use crate::math::Color;
use crate::scene::{Scene, Camera};

pub fn render(scene: &Scene, camera: &Camera, width: u32, height: u32) -> Vec<Color> {
    let mut pixels = Vec::with_capacity((width * height) as usize);

    for j in (0..height).rev() {
        for i in 0..width {
            let u = (i as f64 + 0.5) / width as f64;
            let v = (j as f64 + 0.5) / height as f64;

            let ray = camera.get_ray(u, v);
            let color = scene.trace(&ray, 5);

            pixels.push(color);
        }
    }
    pixels
}

// ============ PPM FILE OUTPUT ============

/// Write pixels to PPM format (ASCII, P3)
/// This is the core PPM output function
fn write_ppm<W: Write>(writer: &mut W, pixels: &[Color], width: u32, height: u32) -> io::Result<()> {
    // Write PPM header
    writeln!(writer, "P3")?;                    // Format: ASCII color PPM
    writeln!(writer, "{} {}", width, height)?; // Image dimensions
    writeln!(writer, "255")?;                  // Maximum color value

    // Write color for each pixel
    for color in pixels {
        // Convert from [0.0, 1.0] float to [0, 255] byte
        let r = (color.x.clamp(0.0, 1.0) * 255.0) as u8;
        let g = (color.y.clamp(0.0, 1.0) * 255.0) as u8;
        let b = (color.z.clamp(0.0, 1.0) * 255.0) as u8;

        // Write RGB values (one per line for clarity)
        writeln!(writer, "{} {} {}", r, g, b)?;
    }
    Ok(())
}

/// Save rendered pixels to PPM file
///
/// # Arguments
/// * `filename` - Output file path
/// * `pixels` - Pixel color data
/// * `width` - Image width
/// * `height` - Image height
pub fn save_ppm(filename: &str, pixels: &[Color], width: u32, height: u32) -> io::Result<()> {
    let mut file = File::create(filename)?;
    write_ppm(&mut file, pixels, width, height)
}
