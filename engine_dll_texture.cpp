#include "framework.h"

#include "engine_types.h"





extern void EngineLoadImageResource(
    sprite_t* sprite,
    const std::wstring& sImageFile
);



void EngineLoadImageResource(sprite_t* sprite,
    const std::wstring& sImageFile)
{
    Gdiplus::Bitmap* bmp = nullptr;

    INT32 x = 0;
    INT32 y = 0;
    INT32 i32SpriteSize = 0;
    //TODO: Check if file exists
    // Load sprite from file
    bmp = Gdiplus::Bitmap::FromFile(sImageFile.c_str());

    if (bmp->GetLastStatus() == Gdiplus::Ok)
    {
        sprite->width = bmp->GetWidth();
        sprite->height = bmp->GetHeight();
        i32SpriteSize = sprite->width * sprite->height;
        sprite->pixels = (pixel_t*)malloc(i32SpriteSize*sizeof(pixel_t));
        if (sprite->pixels != nullptr)
        {
            memset(sprite->pixels, 0, i32SpriteSize * sizeof(pixel_t));
            for (y = 0; y < sprite->height; y++)
            {
                for (x = 0; x < sprite->width; x++)
                {
                    Gdiplus::Color gdiColor;
                    bmp->GetPixel(x, (sprite->height - y), &gdiColor);
                    pixel_t color = {};
                    color.color.r = gdiColor.GetR();
                    color.color.g = gdiColor.GetG();
                    color.color.b = gdiColor.GetB();
                    color.color.a = gdiColor.GetA();
                    sprite->pixels[y * sprite->width + x] = color;
                }
            }
        }

    }
    if (bmp != nullptr)
    {
        delete bmp;
    }
}




