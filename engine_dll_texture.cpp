#include <Windows.h>
#include <gl\gl.h>
#include <gl\glu.h>
#include <stdlib.h>
#include <string.h>
#include "engine_types.h"
//#include "resource.h"
#include "debug.h"




static INT EngineLoadResource(
  DWORD dwResourceId,
  HBITMAP *ptr_hBitmap
  );

extern void EngineLoadSpriteFromResource(
  const DWORD dwResourceId,
  const flip_t flip,
  const sprite_t *sprite
  );


INT EngineLoadResource(
  DWORD dwResourceId,
  HBITMAP *ptr_hBitmap
  )
{
  HINSTANCE hInstance = GetModuleHandle(NULL);
  *ptr_hBitmap =(HBITMAP)LoadImage(hInstance,
                                   MAKEINTRESOURCE(dwResourceId),
                                   IMAGE_BITMAP,
                                   0, 0,
                                   LR_CREATEDIBSECTION);
  /*if (NULL == *ptr_hBitmap)
  {
    DEBUG_W(L"Failed to load bitmap %ld", GetLastError());
    return -1;
  }*/
  return 0;
}

void EngineLoadSpriteFromResource(
  const DWORD dwResourceId,
  const flip_t flip,
  const sprite_t *sprite  
  )
{
  /*BITMAP bitmap;
  HBITMAP hBitmap = NULL;
  size_t i = 0;
  if (0 == EngineLoadResource(dwResourceId, &hBitmap))
  {
    GetObject(hBitmap, sizeof(BITMAP), &bitmap);
    sprite->width = bitmap.bmWidth;
    sprite->height = bitmap.bmHeight;
    sprite->flip = flip;

    HDC dcBitmap = CreateCompatibleDC ( NULL );
    SelectObject( dcBitmap, hBitmap );

    BITMAPINFO bmpInfo;
    bmpInfo.bmiHeader.biSize = sizeof(BITMAPINFOHEADER);
    bmpInfo.bmiHeader.biWidth = bitmap.bmWidth;
    bmpInfo.bmiHeader.biHeight = bitmap.bmHeight;
    bmpInfo.bmiHeader.biPlanes = 1;
    bmpInfo.bmiHeader.biBitCount = 32;
    bmpInfo.bmiHeader.biCompression = BI_RGB;        
    bmpInfo.bmiHeader.biSizeImage = 0;
    
    COLORREF* pixel = (COLORREF*)malloc(sizeof(COLORREF)*(bitmap.bmWidth * bitmap.bmHeight));
    if (pixel != NULL)
    {
      GetDIBits(
        dcBitmap ,
        hBitmap ,
        0 ,
        bitmap.bmHeight ,
        pixel ,
        &bmpInfo ,
        DIB_RGB_COLORS
        );

      sprite->pixels = malloc(sizeof(COLORREF)*(sprite->width * sprite->height));
      if (sprite->pixels != NULL)
      {
        for (i = 0; i < (sprite->width * sprite->height); i++)
        {
          sprite->pixels[i].rgb = pixel[i];
        }
      }
      else
      {
        DEBUG_W(L"Could not allocate memory for sprite");
      }
    }
    free(pixel);
    DeleteObject(hBitmap);

  }*/
}



/*TODO: Not finished with the textures, they just work for now*/

void DeleteGlTexture(HBITMAP *hBitmap)
{
    //DeleteObject(*hBitmap);
}


INT LoadGlTexture(wchar_t *sFilename)
{
  /*BITMAP bitmap;
  HBITMAP hBitmap = NULL;
  GLuint iTextureId = 0;

  HINSTANCE hInstance = GetModuleHandle(NULL);
  hBitmap =(HBITMAP)LoadImage(hInstance, MAKEINTRESOURCE(IDI_0), IMAGE_BITMAP, 0, 0, LR_CREATEDIBSECTION);

 
  if (NULL == hBitmap)
  {
    DEBUG_W(L"Failed to load bitmap %ld", GetLastError());
    return -1;
  }
  else
  {
    GetObject(hBitmap, sizeof(BITMAP), &bitmap);
    glGenTextures(1, &iTextureId);
    glBindTexture(GL_TEXTURE_2D, iTextureId);
    glTexImage2D(GL_TEXTURE_2D, 0, GL_RGB, bitmap.bmWidth, bitmap.bmHeight, 0, GL_RGB, GL_UNSIGNED_BYTE, bitmap.bmBits);
    glTexParameteri(GL_TEXTURE_2D,GL_TEXTURE_MIN_FILTER,GL_LINEAR);
    glTexParameteri(GL_TEXTURE_2D,GL_TEXTURE_MAG_FILTER,GL_LINEAR);
    return iTextureId;
  }*/

  return -1;
  

}




