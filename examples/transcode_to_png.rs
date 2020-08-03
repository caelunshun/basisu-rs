//! This example transcodes a .basis file to a PNG.
//!
//! To run the example:
//! ```bash
//! cargo run --release --example transcode_to_png -- input.basis output.png
//! ```
//!
//! where `input.basis` is the input file, and `output.png` is the output file.

use basisu::{TextureFormat, Transcoder};
use image::{ImageBuffer, ImageFormat, Rgba};
use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();
    let _ = args.next();
    let input_path = args.next().expect("missing input path");
    let output_path = args.next().expect("missing output path");

    let input = std::fs::read(&input_path)?;

    // Create a transcoder to transcode the image.
    // You should reuse the same `Transcoder` when
    // transcoding multiple images.
    let mut transcoder = Transcoder::new();

    // Start transoding the input data. transcoder.begin()
    // returns a TranscodeOp which allows us to query
    // the image and transcode the data to another format.
    //
    // If we wanted to transcode another image using the
    // same Transcoder, we would drop the current TranscodeOp
    // and call Transcoder::begin() again.
    let mut op = transcoder.begin(&input);

    // Print info about the input image.
    //
    // The parameter to the image_info() function is the _index_
    // of the image to query. If the basis file being read
    // contains a texture array, cubemap, or video, there may be multiple
    // image indices available. For normal 2D textures, only
    // image index 0 exists.
    let info = op.image_info(0)?;
    println!(
        "Width: {}\nHeight: {}\nNumber of mipmap levels: {}",
        info.width, info.height, info.num_mipmap_levels,
    );

    // Transcode the image to RGBA pixels in memory.
    //
    // The first parameter to transcode() is the image index as explained above.
    // The second is the mipmap level to transcode. We'll use mipmap level 0,
    // which is the highest-resolution level.
    println!("Transcoding data...");
    let raw_pixels = op.transcode(0, 0, TextureFormat::Rgba32)?;

    // Use the image crate to write out a PNG.
    println!("Writing data to a PNG...");
    let image = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(info.width, info.height, raw_pixels)
        .expect("raw_pixels buffer is too small");
    image.save_with_format(&output_path, ImageFormat::Png)?;

    println!("Finished writing {}.", output_path);

    Ok(())
}
