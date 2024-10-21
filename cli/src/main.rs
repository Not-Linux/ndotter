use std::path::PathBuf;
use clap::Parser;
use ndotter_backend::ndot;

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
    
    ndot(
        config.source, 
        config.destination, 
        config.inversed, 
        config.open, 
        config.dot_size
    )?;

    Ok(())
}