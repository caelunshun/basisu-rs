//! Safe Rust bindings to [`basis_universal`](https://github.com/BinomialLLC/basis_universal).

#![doc(html_root_url = "https://docs.rs/basis-universal/0.1.0")]

use once_cell::sync::Lazy;

pub mod sys {
    #![allow(warnings)]

    include!("../bindings/bindings.rs");
}

// Ensure that basist::transcoder_init() is called.
static TRANSCODER_INIT: Lazy<()> = Lazy::new(|| unsafe {
    sys::basisu_transcoder_init();
});

#[derive(Debug)]
pub struct Error {
    msg: &'static str,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for Error {}

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub struct Transcoder {
    sys: *mut sys::basisu_transcoder,
    _codebook: *mut sys::basisu_etc1_global_selector_codebook,
}

impl Transcoder {
    pub fn new() -> Self {
        Lazy::force(&TRANSCODER_INIT);
        let (sys, _codebook) = unsafe {
            let codebook = sys::basisu_etc1_global_selector_codebook_new();
            let sys = sys::basisu_transcoder_new(codebook);

            (sys, codebook)
        };

        Self { sys, _codebook }
    }

    pub fn begin<'a>(&'a mut self, data: &'a [u8]) -> TranscodeOp<'a> {
        assert!(data.len() < u32::MAX as usize, "data too large");
        unsafe {
            sys::basisu_start_transcoding(self.sys, data.as_ptr().cast(), data.len() as u32);
        }

        TranscodeOp {
            transcoder: self,
            data,
        }
    }
}

impl Drop for Transcoder {
    fn drop(&mut self) {
        unsafe {
            sys::basisu_transcoder_free(self.sys);
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum TextureType {
    D2,
    D2Array,
    Cubemap,
    Video,
    Volume,
}

impl From<sys::basisu_texture_type> for TextureType {
    fn from(sys: sys::basisu_texture_type) -> Self {
        match sys {
            sys::basisu_texture_type_basisu_tex_type_2d => TextureType::D2,
            sys::basisu_texture_type_basisu_tex_type_2d_array => TextureType::D2Array,
            sys::basisu_texture_type_basisu_tex_type_cubemap_array => TextureType::Cubemap,
            sys::basisu_texture_type_basisu_tex_type_video => TextureType::Video,
            sys::basisu_texture_type_basisu_tex_type_volume => TextureType::Volume,
            _ => TextureType::D2,
        }
    }
}

#[derive(Debug)]
pub struct FileInfo {
    pub texture_type: TextureType,
    pub num_images: u32,
    pub us_per_frame: u32,

    pub has_alpha: bool,
    pub is_etc1s: bool,
}

impl From<sys::basisu_file_info> for FileInfo {
    fn from(sys: sys::basisu_file_info) -> Self {
        FileInfo {
            texture_type: TextureType::from(sys.texture_type),
            num_images: sys.total_images,
            us_per_frame: sys.us_per_frame,

            has_alpha: sys.has_alpha_slices == 1,
            is_etc1s: sys.etc1s == 1,
        }
    }
}

#[derive(Debug)]
pub struct ImageInfo {
    pub num_mipmap_levels: u32,

    pub width: u32,
    pub height: u32,

    pub total_blocks: u32,

    pub has_alpha: bool,
    pub is_iframe: bool,
}

impl From<sys::basisu_image_info> for ImageInfo {
    fn from(sys: sys::basisu_image_info) -> Self {
        ImageInfo {
            num_mipmap_levels: sys.total_levels,

            width: sys.orig_width,
            height: sys.orig_height,

            total_blocks: sys.total_blocks,

            has_alpha: sys.alpha_flag == 1,
            is_iframe: sys.iframe_flag == 1,
        }
    }
}

#[derive(Debug)]
pub struct MipmapLevelInfo {
    pub width: u32,
    pub height: u32,

    pub total_blocks: u32,

    pub has_alpha: bool,
    pub is_iframe: bool,
}

impl From<sys::basisu_image_level_info> for MipmapLevelInfo {
    fn from(sys: sys::basisu_image_level_info) -> Self {
        MipmapLevelInfo {
            width: sys.orig_width,
            height: sys.orig_height,

            total_blocks: sys.total_blocks,

            has_alpha: sys.alpha_flag == 1,
            is_iframe: sys.iframe_flag == 1,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum TextureFormat {
    Bc1Rgb,
    Bc3Rgba,
    Bc4R,
    Bc5Rg,
    Bc7Rgba,

    Rgba32,
}

impl TextureFormat {
    /// Returns the needed buffer size to store a compressed
    /// texture in this format.
    fn buffer_size(self, info: &MipmapLevelInfo) -> usize {
        match self {
            TextureFormat::Rgba32 => info.width as usize * info.height as usize * 4,
            _ => todo!("texture formats other than RGBA are not yet supported"),
        }
    }
}

impl From<TextureFormat> for sys::basisu_transcoder_format {
    fn from(format: TextureFormat) -> Self {
        match format {
            TextureFormat::Bc1Rgb => sys::basisu_transcoder_format_basisu_TFBC1_RGB,
            TextureFormat::Bc3Rgba => sys::basisu_transcoder_format_basisu_TFBC3_RGBA,
            TextureFormat::Bc4R => sys::basisu_transcoder_format_basisu_TFBC4_R,
            TextureFormat::Bc5Rg => sys::basisu_transcoder_format_basisu_TFBC5_RG,
            TextureFormat::Bc7Rgba => sys::basisu_transcoder_format_basisu_TFBC7_RGBA,
            TextureFormat::Rgba32 => sys::basisu_transcoder_format_basisu_TFRGBA32,
        }
    }
}

pub struct TranscodeOp<'a> {
    transcoder: &'a mut Transcoder,
    data: &'a [u8],
}

impl<'a> TranscodeOp<'a> {
    pub fn texture_type(&self) -> Result<TextureType> {
        let sys_type = unsafe {
            sys::basisu_get_texture_type(
                self.transcoder.sys,
                self.data.as_ptr().cast(),
                self.data.len() as u32,
            )
        };

        Ok(TextureType::from(sys_type))
    }

    pub fn num_images(&self) -> u32 {
        unsafe {
            sys::basisu_get_total_images(
                self.transcoder.sys,
                self.data.as_ptr().cast(),
                self.data.len() as u32,
            )
        }
    }

    pub fn num_mipmap_levels(&self, image_index: u32) -> u32 {
        unsafe {
            sys::basisu_get_total_image_levels(
                self.transcoder.sys,
                self.data.as_ptr().cast(),
                self.data.len() as u32,
                image_index,
            )
        }
    }

    pub fn file_info(&self) -> Result<FileInfo> {
        let info = unsafe {
            let mut info = sys::basisu_file_info {
                texture_type: 0,
                us_per_frame: 0,
                total_images: 0,
                etc1s: 0,
                has_alpha_slices: 0,
            };

            let status = sys::basisu_get_file_info(
                self.transcoder.sys,
                self.data.as_ptr().cast(),
                self.data.len() as u32,
                &mut info,
            );
            if status != 1 {
                return Err(Error {
                    msg: "failed to get file info",
                });
            }

            info
        };

        Ok(FileInfo::from(info))
    }

    pub fn image_info(&self, image_index: u32) -> Result<ImageInfo> {
        let info = unsafe {
            let mut info = sys::basisu_image_info {
                image_index: 0,
                total_levels: 0,
                orig_width: 0,
                orig_height: 0,
                width: 0,
                height: 0,
                num_blocks_x: 0,
                num_blocks_y: 0,
                total_blocks: 0,
                first_slice_index: 0,
                alpha_flag: 0,
                iframe_flag: 0,
            };

            let status = sys::basisu_get_image_info(
                self.transcoder.sys,
                self.data.as_ptr().cast(),
                self.data.len() as u32,
                &mut info,
                image_index,
            );
            if status != 1 {
                return Err(Error {
                    msg: "failed to get image info",
                });
            }

            info
        };

        Ok(ImageInfo::from(info))
    }

    pub fn mipmap_level_info(
        &self,
        image_index: u32,
        mipmap_level_index: u32,
    ) -> Result<MipmapLevelInfo> {
        let info = unsafe {
            let mut info = sys::basisu_image_level_info {
                image_index: 0,
                level_index: 0,
                orig_width: 0,
                orig_height: 0,
                width: 0,
                height: 0,
                num_blocks_x: 0,
                num_blocks_y: 0,
                total_blocks: 0,
                first_slice_index: 0,
                alpha_flag: 0,
                iframe_flag: 0,
            };

            let status = sys::basisu_get_image_level_info(
                self.transcoder.sys,
                self.data.as_ptr().cast(),
                self.data.len() as u32,
                &mut info,
                image_index,
                mipmap_level_index,
            );
            if status != 1 {
                return Err(Error {
                    msg: "failed to get mipmap level info",
                });
            }

            info
        };

        Ok(MipmapLevelInfo::from(info))
    }

    pub fn transcode(
        &mut self,
        image_index: u32,
        mipmap_level_index: u32,
        format: TextureFormat,
    ) -> Result<Vec<u8>> {
        let info = self.mipmap_level_info(image_index, mipmap_level_index)?;
        let buffer_size = format.buffer_size(&info);

        let mut buffer = vec![0u8; buffer_size];

        unsafe {
            let status = sys::basisu_transcode_image_level(
                self.transcoder.sys,
                self.data.as_ptr().cast(),
                self.data.len() as u32,
                image_index,
                mipmap_level_index,
                buffer.as_mut_ptr().cast(),
                info.width * info.height,
                format.into(),
            );

            if status != 1 {
                return Err(Error {
                    msg: "failed to transcode image data",
                });
            }
        }

        Ok(buffer)
    }
}

impl<'a> Drop for TranscodeOp<'a> {
    fn drop(&mut self) {
        unsafe {
            sys::basisu_stop_transcoding(self.transcoder.sys);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_and_drop_transcoder() {
        let _transcoder = Transcoder::new();
    }

    static CASE_1: &[u8] = include_bytes!("../tests/texture1.basis");

    #[test]
    fn texture_type_is_correct() {
        let mut transcoder = Transcoder::new();
        let op = transcoder.begin(CASE_1);

        assert_eq!(op.texture_type().unwrap(), TextureType::D2);
    }

    #[test]
    fn image_info_is_correct() {
        let mut transcoder = Transcoder::new();
        let op = transcoder.begin(CASE_1);

        let info = op.image_info(0).unwrap();

        assert_eq!(info.num_mipmap_levels, 11); // 2^10 == 1024; extra mipmap level for 2^0==1
        assert_eq!(info.width, 1024);
        assert_eq!(info.height, 1024);
    }

    #[test]
    fn mipmap_level_info_is_correct() {
        let mut transcoder = Transcoder::new();
        let op = transcoder.begin(CASE_1);

        for mipmap_level in 0..11 {
            let info = op.mipmap_level_info(0, mipmap_level).unwrap();

            let dimensions = 2u32.pow(11 - mipmap_level - 1);
            assert_eq!(info.width, dimensions);
            assert_eq!(info.height, dimensions);
        }
    }

    #[test]
    fn transcoded_buffer_has_correct_size() {
        let mut transcoder = Transcoder::new();
        let mut op = transcoder.begin(CASE_1);

        let buffer = op.transcode(0, 0, TextureFormat::Rgba32).unwrap();

        assert_eq!(buffer.len(), 1024 * 1024 * 4);
    }
}
