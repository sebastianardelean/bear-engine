#include <Windows.h>
#include <gl\gl.h>
#include <gl\glu.h>
#include "engine_types.h"
#include "resource.h"
#include "debug.h"

extern INT LoadGlTexture(wchar_t *sFilename);

static GLuint g_iTextureId = 0;

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
