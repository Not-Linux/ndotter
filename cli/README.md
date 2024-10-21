# ndotter_cli

`ndotter` is a command-line tool that converts raster bitmap images into N-dot SVG art. It processes the image by mapping black or white pixels into circular "dots" on an SVG canvas. The output can be customized, and the tool supports all major raster image formats.

## Features

* Convert bitmap images into SVG art using black or white pixels as the dot base.
* Specify the size of each dot to scale the output SVG.
* Optionally open the generated SVG after conversion.
* Supports common raster image formats (e.g., PNG, JPEG).

## Installation

To use `ndotter`, ensure you have Rust installed, then clone the repository and run:

```bash
cargo install --path .
```

## Usage

```bash
ndotter [OPTIONS] --source <SOURCE>
```

### Options

* `-i, --inversed`: Use black pixels for N-dot art (default: white).
* `--dot-size <DOT_SIZE>`: Set the size of each dot (default: 10, minimum: 1).
* `--open`: Automatically open the generated SVG after creation.
* `-s, --source <SOURCE>`: The path to the source image file (required).
* `-d, --destination <DESTINATION>`: The path to save the generated SVG file. Defaults to `<source-image-path>.svg`.

### Example

Convert a PNG image to SVG art with a dot size of 15, and open the resulting file:

```bash
ndotter --source image.png --dot-size 15 --open
```

## Contributing

Feel free to submit issues or pull requests if you'd like to improve ndotter!

## License

This project is licensed under the MIT License.
