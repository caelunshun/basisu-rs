#include "basisu_transcoder.cpp"

extern "C"
{
#include "bindings.h"
}

extern "C" void basisu_transcoder_init()
{
    basist::basisu_transcoder_init();
}

extern "C" struct basisu_etc1_global_selector_codebook *basisu_etc1_global_selector_codebook_new()
{
    return (struct basisu_etc1_global_selector_codebook *)new basist::etc1_global_selector_codebook(basist::g_global_selector_cb_size, basist::g_global_selector_cb);
}

extern "C" struct basisu_transcoder *basisu_transcoder_new(const struct basisu_etc1_global_selector_codebook *global_sel_codebook)
{
    basist::etc1_global_selector_codebook *sel_codebook = (basist::etc1_global_selector_codebook *)global_sel_codebook;
    return (struct basisu_transcoder *)new basist::basisu_transcoder(sel_codebook);
}

extern "C" basisu_texture_type basisu_get_texture_type(const struct basisu_transcoder *transcoder, const void *data, uint32_t data_len)
{
    const basist::basisu_transcoder *t = (const basist::basisu_transcoder *)transcoder;
    return (basisu_texture_type)t->get_texture_type(data, data_len);
}

extern "C" uint32_t basisu_get_total_images(const struct basisu_transcoder *transcoder, const void *data, uint32_t data_len)
{
    const basist::basisu_transcoder *t = (const basist::basisu_transcoder *)transcoder;
    return t->get_total_images(data, data_len);
}

extern "C" uint32_t basisu_get_total_image_levels(const struct basisu_transcoder *transcoder, const void *data, uint32_t data_len, uint32_t image_index)
{
    const basist::basisu_transcoder *t = (const basist::basisu_transcoder *)transcoder;
    return t->get_total_image_levels(data, data_len, image_index);
}

extern "C" int basisu_get_image_level_desc(const struct basisu_transcoder *transcoder, const void *data, uint32_t data_len,
                                           uint32_t image_index, uint32_t level_index, uint32_t *orig_width, uint32_t *orig_height, uint32_t *total_blocks)
{
    const basist::basisu_transcoder *t = (const basist::basisu_transcoder *)transcoder;
    return t->get_image_level_desc(data, data_len, image_index, level_index, *orig_width, *orig_height, *total_blocks);
}

extern "C" int basisu_get_image_info(const struct basisu_transcoder *transcoder, const void *data, uint32_t data_len,
                                     basisu_image_info *image_info, uint32_t image_index)
{
    const basist::basisu_transcoder *t = (const basist::basisu_transcoder *)transcoder;

    basist::basisu_image_info info;
    int status = t->get_image_info(data, data_len, info, image_index);

    image_info->image_index = info.m_image_index;
    image_info->total_levels = info.m_total_levels;

    image_info->orig_width = info.m_orig_width;
    image_info->orig_height = info.m_orig_height;

    image_info->num_blocks_x = info.m_num_blocks_x;
    image_info->num_blocks_y = info.m_num_blocks_y;
    image_info->total_blocks = info.m_total_blocks;
    image_info->first_slice_index = info.m_first_slice_index;

    image_info->alpha_flag = (int)info.m_alpha_flag;
    image_info->iframe_flag = (int)info.m_iframe_flag;

    return status;
}

extern "C" int basisu_get_image_level_info(const struct basisu_transcoder *transcoder, const void *data, uint32_t data_len,
                                           basisu_image_level_info *level_info, uint32_t image_index, uint32_t level_index)
{
    const basist::basisu_transcoder *t = (const basist::basisu_transcoder *)transcoder;

    basist::basisu_image_level_info info;
    int status = t->get_image_level_info(data, data_len, info, image_index, level_index);

    level_info->image_index = info.m_image_index;
    level_info->level_index = info.m_level_index;

    level_info->orig_width = info.m_orig_width;
    level_info->orig_height = info.m_orig_height;

    level_info->num_blocks_x = info.m_num_blocks_x;
    level_info->num_blocks_y = info.m_num_blocks_y;
    level_info->total_blocks = info.m_total_blocks;
    level_info->first_slice_index = info.m_first_slice_index;

    level_info->alpha_flag = (int)info.m_alpha_flag;
    level_info->iframe_flag = (int)info.m_iframe_flag;

    return status;
}

extern "C" int basisu_start_transcoding(struct basisu_transcoder *transcoder, const void *data, uint32_t data_len)
{
    basist::basisu_transcoder *t = (basist::basisu_transcoder *)transcoder;
    return t->start_transcoding(data, data_len);
}

extern "C" int basisu_stop_transcoding(struct basisu_transcoder *transcoder)
{
    basist::basisu_transcoder *t = (basist::basisu_transcoder *)transcoder;
    return t->stop_transcoding();
}

extern "C" int basisu_transcode_image_level(struct basisu_transcoder *transcoder, const void *data, uint32_t data_len,
                                            uint32_t image_index, uint32_t level_index,
                                            void *output_blocks, uint32_t output_blocks_buf_size_in_blocks_or_pixels,
                                            basisu_transcoder_format format)
{
    basist::basisu_transcoder *t = (basist::basisu_transcoder *)transcoder;
    return t->transcode_image_level(data, data_len, image_index, level_index, output_blocks, output_blocks_buf_size_in_blocks_or_pixels, (basist::transcoder_texture_format)format);
}

extern "C" void basisu_transcoder_free(struct basisu_transcoder *transcoder)
{
    basist::basisu_transcoder *t = (basist::basisu_transcoder *)transcoder;
    delete t;
}
