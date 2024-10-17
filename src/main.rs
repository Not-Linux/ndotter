use std::path::PathBuf;

use anyhow::bail;
use clap::Parser;
use image::{GenericImageView, ImageReader, Luma, Rgba};
use svg::{node::element::Circle, Document};

const BLACK: Luma<u8> = Luma([0]);
const WHITE: Luma<u8> = Luma([255]);

#[derive(Debug, Parser)]
#[command(name = "ndotter")]
#[command(version = "1.0")]
#[command(about = "Convert bitmaps to N-dot SVG art")]
struct Config {
    /// Use black pixels of the image to create 
    /// N-dot art; otherwise white will be used
    #[arg(short, long)]
    inversed: bool,

    /// Size of each N-dot (changes viewport size
    /// proportionally). Minimal is 1
    #[arg(long, default_value_t = 10)]
    dot_size: u32,

    /// Open SVG image after finishing
    #[arg(long)]
    open: bool,

    /// Source image path. All major raster image
    /// formats are supporteds
    #[arg(short, long)]
    source: PathBuf,

    /// Destination image path. Default value is
    /// `<source-image-path>.svg`
    #[arg(short, long)]
    destination: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let config = Config::parse();
    if config.dot_size == 0 {
        bail!("Dot size must not be 0!");
    }

    let destination = get_destination(&config);
    let img = ImageReader::open(config.source.as_path())?.decode()?;

    let mut svg_document = Document::new()
        .set("xmlns", "http://www.w3.org/2000/svg")
        .set("viewBox", (
            0, 
            0, 
            img.width() * config.dot_size, 
            img.height() * config.dot_size,
        ));

    println!("======== NDOTTER ========");
    println!("dot size:    {}", config.dot_size);
    println!("destination: {}", destination.display());
    println!("inversed:    {}", if config.inversed { "yes" } else { "no" });
    println!("open:        {}", if config.open { "yes" } else { "no" });
    println!("=========================");

    let len = (img.width() * img.height()) as f32;

    for (p, (x, y, color)) in img.pixels().enumerate() {
        let bw = rgba_to_bw(color);
        if (bw == WHITE && !config.inversed) || (bw == BLACK && config.inversed) {
            svg_document = svg_document.add(
                Circle::new()
                    .set("cx", (x * config.dot_size) as f32 + config.dot_size as f32 / 2.0)
                    .set("cy", (y * config.dot_size) as f32 + config.dot_size as f32 / 2.0)
                    .set("r", config.dot_size as f32 / 2.0)
                    .set("fill", "white")
            );
        }

        print!("\rProcessing image: {}%", (p as f32 / len * 100.).ceil() as u32);
    }

    println!("\nFinished.");

    svg::save(destination.as_path(), &svg_document)?;

    if config.open {
        open::that(destination.as_path())?;
    }

    Ok(())
}

fn get_destination(config: &Config) -> PathBuf {
    config.destination
        .clone()
        .unwrap_or_else(|| {
            let mut path = config.source.clone();
            path.set_extension("svg");

            path
        })
}

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