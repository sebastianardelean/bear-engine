/**
 * Automatic tilling window manager for windows
 * Inspired from https://github.com/nir9/lightwm/tree/master
 */

#include <Windows.h>
#include <stdlib.h>
#include <signal.h>

#include <stdlib.h>
#include "engine_types.h"

#include "configuration.h"
#include "debug.h"


#include <gl\gl.h>
#include <gl\glu.h>


static HMODULE g_hEngineDll = NULL;



/* ENGINE_DLL loaded functions */

static FARPROC HndlCreateWindow = NULL;
static FARPROC HndlRun = NULL;
static FARPROC HndlGetKeyState = NULL;
static FARPROC HndlDrawLine = NULL;
static FARPROC HndlDrawTriangle = NULL;
static FARPROC HndlDrawQuad = NULL;
static FARPROC HndlDrawPoint = NULL;
static FARPROC HndlLoadTexture = NULL;

static void Cleanup(void)
{
  
  HndlCreateWindow = NULL;
  HndlRun = NULL;
  HndlGetKeyState = NULL;
  HndlDrawLine = NULL;
  HndlDrawTriangle = NULL;
  HndlDrawQuad = NULL;
  HndlDrawPoint = NULL;
  HndlLoadTexture = NULL;
  if (g_hEngineDll)
  {
    (void)FreeLibrary(g_hEngineDll);
  }
}

static void InitializeDllHandlers(void)
{
  
  HndlCreateWindow = GetProcAddress(g_hEngineDll, "HndlEngineCreateWindow");
  HndlRun = GetProcAddress(g_hEngineDll, "HndlEngineRun");
  HndlGetKeyState = GetProcAddress(g_hEngineDll, "HndlEngineGetKeyState");
  HndlDrawLine = GetProcAddress(g_hEngineDll, "HndlEngineDrawLine");
  HndlDrawTriangle = GetProcAddress(g_hEngineDll, "HndlEngineDrawTriangle");
  HndlDrawQuad = GetProcAddress(g_hEngineDll, "HndlEngineDrawQuad");
  HndlDrawPoint = GetProcAddress(g_hEngineDll, "HndlEngineDrawPoint");
  HndlLoadTexture = GetProcAddress(g_hEngineDll, "HndlEngineLoadTexture");
  if (NULL == HndlCreateWindow || NULL == HndlRun)
  {
    Cleanup();
    exit(GetLastError());
  }
  

}

static void HndlSIGINT(int signal)
{
  Cleanup();
  exit(0);
}

static BOOL HndlDraw(void)
{
  
  if (HndlGetKeyState(VK_F1))
  {
#if 1
    size_t i = 0;
    size_t j = 0;
    for (i = 0; i < SCREEN_WIDTH; i++)
    {
      for (j = 0; j < SCREEN_HEIGHT; j++)
      {
        point_t p = {(float)i,(float)j, -1.0f};
        color_t color = {rand()%255, rand()%255, rand()%255, 0};
        HndlDrawPoint(p, color);
      }
    }
#endif
#if 0
    point_t p1 = {0.0f, 0.0f, -1.0f};
    point_t p2 = {200.0f, 200.0f, -1.0f};
    color_t color = {rand()%255, rand()%255, rand()%255, 0};
    HndlDrawLine(p1, p2, color);
    
    point_t pr1 = {200.0f, 200.0f, -2.0f};
    point_t pr2 = {250.0f, 200.0f, -2.0f};
    point_t pr3 = {200.0f, 150.0f, -2.0f};

    color_t cr = {rand()%255, rand()%255, rand()%255, 0};
    
    HndlDrawTriangle(pr1, pr2, pr3, cr);


    point_t pq1 = {300.0f, 300.0f, -2.0f};
    point_t pq2 = {350.0f, 300.0f, -2.0f};
    point_t pq3 = {350.0f, 250.0f, -2.0f};
    point_t pq4 = {300.0f, 250.0f, -2.0f};
    color_t cq = {rand()%255, rand()%255, rand()%255, 0};
    HndlDrawQuad(pq1, pq2, pq3, pq4, cq);
    
    int i = HndlLoadTexture(L"ANA");
    glBindTexture(GL_TEXTURE_2D, i);
    


#endif
    return TRUE;
  }
  return FALSE;
}

int main(void)
{

  SetProcessDPIAware();   
  
  g_hEngineDll = LoadLibraryW(L"bengine_dll");

  
  if (NULL == g_hEngineDll)
  {
    Cleanup();
    exit(GetLastError());
  }


  InitializeDllHandlers();

  signal(SIGINT, HndlSIGINT);

  HndlCreateWindow();

  HndlRun(&HndlDraw);
                    
  Cleanup();
  return EXIT_SUCCESS;
}
