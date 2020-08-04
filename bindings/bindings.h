/*
 * C bindings to the C++ basis_universal transcoder.
 */

#include <stdint.h>

struct basisu_transcoder;
struct basisu_etc1_global_selector_codebook;

void basisu_transcoder_init();

struct basisu_etc1_global_selector_codebook *basisu_etc1_global_selector_codebook_new();
struct basisu_transcoder *basisu_transcoder_new(const struct basisu_etc1_global_selector_codebook *global_sel_codebook);

/*
 * Transcoder methods
*/

/* (taken directly from basisu_transcoder.h) */
typedef enum
{
    // Compressed formats

    // ETC1-2
    basisu_TFETC1_RGB = 0,  // Opaque only, returns RGB or alpha data if cDecodeFlagsTranscodeAlphaDataToOpaqueFormats flag is specified
    basisu_TFETC2_RGBA = 1, // Opaque+alpha, ETC2_EAC_A8 block followed by a ETC1 block, alpha channel will be opaque for opaque .basis files

    // BC1-5, BC7 (desktop, some mobile devices)
    basisu_TFBC1_RGB = 2,  // Opaque only, no punchthrough alpha support yet, transcodes alpha slice if cDecodeFlagsTranscodeAlphaDataToOpaqueFormats flag is specified
    basisu_TFBC3_RGBA = 3, // Opaque+alpha, BC4 followed by a BC1 block, alpha channel will be opaque for opaque .basis files
    basisu_TFBC4_R = 4,    // Red only, alpha slice is transcoded to output if cDecodeFlagsTranscodeAlphaDataToOpaqueFormats flag is specified
    basisu_TFBC5_RG = 5,   // XY: Two BC4 blocks, X=R and Y=Alpha, .basis file should have alpha data (if not Y will be all 255's)
    basisu_TFBC7_RGBA = 6, // RGB or RGBA, mode 5 for ETC1S, modes (1,2,3,5,6,7) for UASTC

    // PVRTC1 4bpp (mobile, PowerVR devices)
    basisu_TFPVRTC1_4_RGB = 8,  // Opaque only, RGB or alpha if cDecodeFlagsTranscodeAlphaDataToOpaqueFormats flag is specified, nearly lowest quality of any texture format.
    basisu_TFPVRTC1_4_RGBA = 9, // Opaque+alpha, most useful for simple opacity maps. If .basis file doesn't have alpha cTFPVRTC1_4_RGB will be used instead. Lowest quality of any supported texture format.

    // ASTC (mobile, Intel devices, hopefully all desktop GPU's one day)
    basisu_TFASTC_4x4_RGBA = 10, // Opaque+alpha, ASTC 4x4, alpha channel will be opaque for opaque .basis files. Transcoder uses RGB/RGBA/L/LA modes, void extent, and up to two ([0,47] and [0,255]) endpoint precisions.

    // ATC (mobile, Adreno devices, this is a niche format)
    basisu_TFATC_RGB = 11,  // Opaque, RGB or alpha if cDecodeFlagsTranscodeAlphaDataToOpaqueFormats flag is specified. ATI ATC (GL_ATC_RGB_AMD)
    basisu_TFATC_RGBA = 12, // Opaque+alpha, alpha channel will be opaque for opaque .basis files. ATI ATC (GL_ATC_RGBA_INTERPOLATED_ALPHA_AMD)

    // FXT1 (desktop, Intel devices, this is a super obscure format)
    basisu_TFFXT1_RGB = 17, // Opaque only, uses exclusively CC_MIXED blocks. Notable for having a 8x4 block size. GL_3DFX_texture_compression_FXT1 is supported on Intel integrated GPU's (such as HD 630).
                            // Punch-through alpha is relatively easy to support, but full alpha is harder. This format is only here for completeness so opaque-only is fine for now.
                            // See the BASISU_USE_ORIGINAL_3DFX_FXT1_ENCODING macro in basisu_transcoder_internal.h.

    basisu_TFPVRTC2_4_RGB = 18,  // Opaque-only, almost BC1 quality, much faster to transcode and supports arbitrary texture dimensions (unlike PVRTC1 RGB).
    basisu_TFPVRTC2_4_RGBA = 19, // Opaque+alpha, slower to encode than cTFPVRTC2_4_RGB. Premultiplied alpha is highly recommended, otherwise the color channel can leak into the alpha channel on transparent blocks.

    basisu_TFETC2_EAC_R11 = 20,  // R only (ETC2 EAC R11 unsigned)
    basisu_TFETC2_EAC_RG11 = 21, // RG only (ETC2 EAC RG11 unsigned), R=opaque.r, G=alpha - for tangent space normal maps

    // Uncompressed (raw pixel) formats
    basisu_TFRGBA32 = 13,   // 32bpp RGBA image stored in raster (not block) order in memory, R is first byte, A is last byte.
    basisu_TFRGB565 = 14,   // 166pp RGB image stored in raster (not block) order in memory, R at bit position 11
    basisu_TFBGR565 = 15,   // 16bpp RGB image stored in raster (not block) order in memory, R at bit position 0
    basisu_TFRGBA4444 = 16, // 16bpp RGBA image stored in raster (not block) order in memory, R at bit position 12, A at bit position 0

    cTFTotalTextureFormats = 22,
} basisu_transcoder_format;

typedef enum
{
    basisu_tex_type_2d = 0,            // An arbitrary array of 2D RGB or RGBA images with optional mipmaps, array size = # images, each image may have a different resolution and # of mipmap levels
    basisu_tex_type_2d_array = 1,      // An array of 2D RGB or RGBA images with optional mipmaps, array size = # images, each image has the same resolution and mipmap levels
    basisu_tex_type_cubemap_array = 2, // an array of cubemap levels, total # of images must be divisable by 6, in X+, X-, Y+, Y-, Z+, Z- order, with optional mipmaps
    basisu_tex_type_video = 3,         // An array of 2D video frames, with optional mipmaps, # frames = # images, each image has the same resolution and # of mipmap levels
    basisu_tex_type_volume = 4,        // A 3D texture with optional mipmaps, Z dimension = # images, each image has the same resolution and # of mipmap levels

    cBASISTexTypeTotal
} basisu_texture_type;

typedef struct
{
    basisu_texture_type texture_type;

    uint32_t us_per_frame;

    uint32_t total_images;

    int etc1s;
    int has_alpha_slices;
} basisu_file_info;

typedef struct
{
    uint32_t image_index;
    uint32_t total_levels;

    uint32_t orig_width;
    uint32_t orig_height;

    uint32_t width;
    uint32_t height;

    uint32_t num_blocks_x;
    uint32_t num_blocks_y;
    uint32_t total_blocks;

    uint32_t first_slice_index;

    int alpha_flag;  // true if the image has alpha data
    int iframe_flag; // true if the image is an I-Frame
} basisu_image_info;

typedef struct
{
    uint32_t image_index;
    uint32_t level_index;

    uint32_t orig_width;
    uint32_t orig_height;

    uint32_t width;
    uint32_t height;

    uint32_t num_blocks_x;
    uint32_t num_blocks_y;
    uint32_t total_blocks;

    uint32_t first_slice_index;

    int alpha_flag;  // true if the image has alpha data
    int iframe_flag; // true if the image is an I-Frame
} basisu_image_level_info;

basisu_texture_type basisu_get_texture_type(const struct basisu_transcoder *transcoder, const void *data, uint32_t data_len);

uint32_t basisu_get_total_images(const struct basisu_transcoder *transcoder, const void *data, uint32_t data_len);
uint32_t basisu_get_total_image_levels(const struct basisu_transcoder *transcoder, const void *data, uint32_t data_len, uint32_t image_index);

int basisu_get_image_level_desc(const struct basisu_transcoder *transcoder, const void *data, uint32_t data_len,
                                uint32_t image_index, uint32_t level_index, uint32_t *orig_width, uint32_t *orig_height, uint32_t *total_blocks);

int basisu_get_image_info(const struct basisu_transcoder *transcoder, const void *data, uint32_t data_len,
                          basisu_image_info *info, uint32_t image_index);

int basisu_get_image_level_info(const struct basisu_transcoder *transcoder, const void *data, uint32_t data_len,
                                basisu_image_level_info *info, uint32_t image_index, uint32_t level_index);

int basisu_get_file_info(const struct basisu_transcoder *transcoder, const void *data, uint32_t data_len, basisu_file_info *file_info);

int basisu_start_transcoding(struct basisu_transcoder *transcoder, const void *data, uint32_t data_len);
int basisu_stop_transcoding(struct basisu_transcoder *transcoder);

int basisu_transcode_image_level(struct basisu_transcoder *transcoder, const void *data, uint32_t data_len,
                                 uint32_t image_index, uint32_t level_index,
                                 void *output_blocks, uint32_t output_blocks_buf_size_in_blocks_or_pixels,
                                 basisu_transcoder_format format);

void basisu_transcoder_free(struct basisu_transcoder *transcoder);
