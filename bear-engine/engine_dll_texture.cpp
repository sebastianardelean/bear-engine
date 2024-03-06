#include "pch.h"
#include "framework.h"

#include "engine_types.h"


extern void EngineLoadImageFromFile(sprite_t* sprite,
    const std::wstring& sImageFile);

extern void EngineCreateTextureFromImageFile(
    const std::wstring& sImageFile,
    DWORD32* dwTextureId
);


void EngineLoadImageFromFile(sprite_t* sprite,
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




void EngineCreateTextureFromImageFile(
    const std::wstring& sImageFile,
    DWORD32 *dwTextureId
)
{
   
    sprite_t sprite = {};
    EngineLoadImageFromFile(&sprite, sImageFile);


    GLuint iTextureId = 0;

    glGenTextures(1, &iTextureId);

    glBindTexture(GL_TEXTURE_2D, iTextureId);

    glTexImage2D(GL_TEXTURE_2D, 0, GL_RGB, sprite.width, sprite.height, 0, GL_RGB, GL_UNSIGNED_BYTE, sprite.pixels);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR);

    *dwTextureId = iTextureId;
}

