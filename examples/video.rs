//! This example plays a basis-encoded video file, displaying
//! it in a window.
//!
//! We use the minifb crate for window display.
//!
//! To run this example:
//! ```bash
//! cargo run --release --example video -- path-to-video.basis
//! ```
//!
//! See the README of the basis_universal library for how to encode
//! videos with Basis.

use basisu::{TextureFormat, Transcoder};
use minifb::{Key, Window, WindowOptions};
use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let input_path = env::args()
        .collect::<Vec<_>>()
        .get(1)
        .expect("please provide an input path")
        .clone();

    // Create a transcoder and read the input file.
    let mut transcoder = Transcoder::new();

    println!("Reading {}...", input_path);
    let input_data = std::fs::read(&input_path)?;

    let mut op = transcoder.begin(&input_data);

    // Figure out the frame count, frame rate, and video dimensions.
    let file_info = op.file_info()?;

    let num_frames = file_info.num_images;
    let us_per_frame = file_info.us_per_frame as u64; // microseconds between frame displays

    let first_image_info = op.image_info(0)?;
    let width = first_image_info.width as usize;
    let height = first_image_info.height as usize;

    // Create a window to display to.
    let mut window = Window::new("Basis Video", width, height, WindowOptions::default())?;
    // Set the frame rate.
    window.limit_update_rate(Some(std::time::Duration::from_micros(us_per_frame)));

    let mut current_frame = 0;
    while window.is_open() && !window.is_key_down(Key::Escape) && current_frame < num_frames {
        // Decode a frame and display it.
        println!("Transcoding frame {}...", current_frame);
        let raw_buffer = op.transcode(current_frame, 0, TextureFormat::Rgba32)?;

        // Convert the RGBA32 buffer to the format understood by minifb.
        let mut buffer = vec![0u32; window.get_size().0 * window.get_size().1];
        for (i, chunk) in raw_buffer.chunks_exact(4).enumerate() {
            if let &[r, g, b, _a] = chunk {
                buffer[i] = (u32::from(r) << 16) | (u32::from(g) << 8) | u32::from(b);
            }
        }

        // Display the frame.
        window.update_with_buffer(&buffer, window.get_size().0, window.get_size().1)?;

        current_frame += 1;
    }

    println!("Done.");

    Ok(())
}
