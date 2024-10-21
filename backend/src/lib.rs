//! `ndotter` - A library to convert raster images into N-dot SVG art.
//!
//! This crate provides a function to take a source image and generate an SVG file
//! with circular "dots" representing pixels based on their brightness (black or white).
//! The function supports various customizations, such as dot size, pixel inversion, and
//! optional automatic opening of the resulting SVG.
//!
//! # Example
//!
//! ```rust
//! ndot("image.png", Some("output.svg"), false, true, 10)
//! ```
//!
//! This will process `image.png`, convert it to N-dot SVG art with a dot size of 10, and
//! open the resulting `output.svg` after creation.

use std::path::Path;
use image::{GenericImageView, ImageReader, Luma, Rgba};
use svg::{node::element::Circle, Document};
use thiserror::Error;

/// Luma value for black.
const BLACK: Luma<u8> = Luma([0]);

/// Luma value for white.
const WHITE: Luma<u8> = Luma([255]);

/// Custom error type for `ndotter` operations.
#[derive(Debug, Error)]
pub enum NdotterError {
    /// Error for invalid dot size (i.e., zero).
    #[error("Dot size must not be 0")]
    ZeroDotSize,

    /// Error for image loading failures.
    #[error("Cannot load image")]
    IoError(#[from] std::io::Error),

    /// Error for image processing failures.
    #[error("Cannot process image")]
    ImageError(#[from] image::ImageError),
}

/// Convert a bitmap image into N-dot SVG art.
///
/// This function reads a source image file, processes its pixels, and generates an SVG
/// file where each black or white pixel is represented as a dot. The size of the dots,
/// inversion of black/white pixels, and automatic opening of the SVG are configurable.
///
/// # Arguments
///
/// * `source` - Path to the source image file.
/// * `destination` - Optional path to the destination SVG file. If not provided, the output file will be named after the source image with an `.svg` extension.
/// * `inversed` - If `true`, black pixels will be used as the base for creating the dots. Otherwise, white pixels will be used.
/// * `open` - If `true`, the generated SVG file will be opened after creation.
/// * `dot_size` - Size of each dot in the output SVG (must be greater than 0).
///
/// # Errors
///
/// This function returns an `NdotterError` if the image cannot be loaded, processed, or if the dot size is zero.
///
/// # Example
///
/// ```rust,no_run
/// use std::path::Path;
/// use ndotter::ndot;
///
/// let result = ndot("image.png", Some("output.svg"), false, true, 10);
/// if let Err(e) = result {
///     eprintln!("Error: {}", e);
/// }
/// ```
pub fn ndot(
    source: impl AsRef<Path>,
    destination: Option<impl AsRef<Path>>,
    inversed: bool,
    open: bool,
    dot_size: u32,
) -> Result<(), NdotterError> {
    // Ensure dot size is greater than 0
    if dot_size == 0 {
        return Err(NdotterError::ZeroDotSize);
    }

    // Get destination path or create one based on the source image
    let destination = destination
        .map(|p| p.as_ref().to_owned())
        .unwrap_or_else(|| {
            let mut path = source.as_ref().to_owned();
            path.set_extension("svg");

            path
        });

    // Open the image and decode it
    let img = ImageReader::open(source.as_ref())?.decode()?;

    // Create an empty SVG document with the appropriate viewport size
    let mut svg_document = Document::new()
        .set("xmlns", "http://www.w3.org/2000/svg")
        .set("viewBox", (
            0, 
            0, 
            img.width() * dot_size, 
            img.height() * dot_size,
        ));

    // Output some logging information
    println!("======== NDOTTER ========");
    println!("dot size:    {}", dot_size);
    println!("destination: {}", destination.display());
    println!("inversed:    {}", if inversed { "yes" } else { "no" });
    println!("open:        {}", if open { "yes" } else { "no" });
    println!("=========================");

    let len = (img.width() * img.height()) as f32;

    // Iterate over the image pixels
    for (p, (x, y, color)) in img.pixels().enumerate() {
        let bw = rgba_to_bw(color);
        // Add a dot to the SVG if the pixel matches the configuration
        if (bw == WHITE && !inversed) || (bw == BLACK && inversed) {
            svg_document = svg_document.add(
                Circle::new()
                    .set("cx", (x * dot_size) as f32 + dot_size as f32 / 2.0)
                    .set("cy", (y * dot_size) as f32 + dot_size as f32 / 2.0)
                    .set("r", dot_size as f32 / 2.0)
                    .set("fill", "white")
            );
        }

        // Print progress
        print!("\rProcessing image: {}%", (p as f32 / len * 100.).ceil() as u32);
    }

    // Save the SVG file
    println!("\nFinished.");
    svg::save(destination.as_path(), &svg_document)?;

    // Open the generated SVG file if required
    if open {
        open::that(destination.as_path())?;
    }

    Ok(())
}

/// Convert an RGBA color into a black-and-white value.
///
/// This function uses the RGB components of the color to calculate the perceived
/// brightness (luma), which is then compared against a threshold (128) to decide
/// whether the pixel is black or white.
///
/// # Arguments
///
/// * `color` - The input RGBA color.
///
/// # Returns
///
/// A `Luma<u8>` representing black or white based on the pixel's brightness.
fn rgba_to_bw(color: Rgba<u8>) -> Luma<u8> {
    let Rgba([r, g, b, _]) = color;

    // Calculate perceived brightness (luma)
    let luma = 0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32;

    // Compare luma to 128 (midpoint for perceived brightness)
    if luma > 128.0 {
        WHITE
    } else {
        BLACK
    }
}
