#pragma once
#ifdef __cplusplus
extern "C"
{
#endif
    typedef struct
    {     
        uint8_t r;
        uint8_t g;
        uint8_t b;
        uint8_t a;
    } color_t;

    typedef enum
    {
        FILL_COLOR,
        FILL_TEXTURE
    }fill_type_t;



    typedef enum
    {
        FILTER_NEAREST_TYPE,
        FILTER_LINEAR_TYPE,
        FILTER_MIPMAP_TYPE
    }texture_filter_type_t;

    typedef struct
    {
        fill_type_t fill_type;
        texture_filter_type_t filter_type;
        color_t color;
        const wchar_t* sTextureFile;
    }fill_option_t;

    typedef struct
    {
        float x;
        float y;
        float z;
        float nx;
        float ny;
        float nz;
        float tx;
        float ty;
        float tz;
    }coordinate_t;




    typedef union
    {
        color_t color;
        uint32_t rgb;
    }pixel_t;

    typedef struct
    {
        int32_t width;
        int32_t height;

        pixel_t* pixels;

    }sprite_t;



#ifdef __cplusplus
}
#endif /* __cplusplus */