#include <Windows.h>
#include <gl\gl.h>
#include <gl\glu.h>

#include <string.h>
#include "engine_types.h"
#include "resource.h"
#include "debug.h"




static INT EngineLoadResource(DWORD dwResourceId, HBITMAP *ptr_hBitmap);

extern INT LoadGlTexture(wchar_t *sFilename);
  
static GLuint g_iTextureId = 0;

/*TODO: Not finished with the textures, they just work for now*/

void DeleteGlTexture(HBITMAP *hBitmap)
{
    DeleteObject(*hBitmap);
}


INT LoadGlTexture(wchar_t *sFilename)
{
  BITMAP bitmap;
  HBITMAP hBitmap = NULL;

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
    glGenTextures(1, &g_iTextureId);
    glBindTexture(GL_TEXTURE_2D, g_iTextureId);
    glTexImage2D(GL_TEXTURE_2D, 0, GL_RGB, bitmap.bmWidth, bitmap.bmHeight, 0, GL_RGB, GL_UNSIGNED_BYTE, bitmap.bmBits);
    glTexParameteri(GL_TEXTURE_2D,GL_TEXTURE_MIN_FILTER,GL_LINEAR);
    glTexParameteri(GL_TEXTURE_2D,GL_TEXTURE_MAG_FILTER,GL_LINEAR);
    return g_iTextureId;
  }

  return -1;
  

}

INT EngineLoadResource(DWORD dwResourceId, HBITMAP *ptr_hBitmap)
{
  HINSTANCE hInstance = GetModuleHandle(NULL);
  *ptr_hBitmap =(HBITMAP)LoadImage(hInstance, MAKEINTRESOURCE(dwResourceId), IMAGE_BITMAP, 0, 0, LR_CREATEDIBSECTION);
  if (NULL == *ptr_hBitmap)
  {
    DEBUG_W(L"Failed to load bitmap %ld", GetLastError());
    return -1;
  }
  return 0;
}

void EngineLoadSpriteFromResource(DWORD dwResourceId, sprite_t *sprite)
{
  BITMAP bitmap;
  HBITMAP hBitmap = NULL;
  
  if (0 == EngineLoadResource(IDI_0, &hBitmap))
  {
    GetObject(hBitmap, sizeof(BITMAP), &bitmap);
    sprite->width = bitmap.bmWidth;
    sprite->height = bitmap.bmHeight;
    (void)memcpy(sprite->pixels, bitmap.bmBits, (sprite->width * sprite->height));
    DeleteObject(hBitmap);

  }
}
