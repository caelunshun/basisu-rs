# basisu-rs

A safe Rust wrapper for the [`basis-universal`](https://github.com/BinomialLLC/basis_universal/) texture
compression library.

This crate wraps the `basisu` transcoder, used to transcode `basis`-encoded files
to various in-memory formats. It's intended for use in games,
where small textures with fast transcoding times are desired.

`.basis` files can contain:
* 2D textures, i.e. images
* 2D texture arrays
* Cubemaps
* 3D textures, used for volume data
* Videos (stored as texture arrays with additional inter-frame compression)

`basisu-rs` supports transcoding any of the above through a practical Rust interface.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
