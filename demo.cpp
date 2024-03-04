/**
 * Automatic tilling window manager for windows
 * Inspired from https://github.com/nir9/lightwm/tree/master
 */

#include <Windows.h>
#include <cstdlib>
#include <csignal>

#
#include "engine_types.h"

#include "configuration.h"
#include "debug.h"


#include <gl\gl.h>
#include <gl\glu.h>


static HMODULE g_hEngineDll = NULL;



/* ENGINE_DLL loaded functions */

static FARPROC_ENGINE_INIT HndlCreateWindow = NULL;
static FARPROC_ENGINE_RUN HndlRun = NULL;
static FARPROC_ENGINE_KEY_STATE HndlGetKeyState = NULL;
static FARPROC_ENGINE_DRAW_LINE HndlDrawLine = NULL;
static FARPROC_ENGINE_DRAW_TRIANGLE HndlDrawTriangle = NULL;
static FARPROC_ENGINE_DRAW_QUAD HndlDrawQuad = NULL;
static FARPROC_ENGINE_DRAW_POINT HndlDrawPoint = NULL;
static FARPROC_ENGINE_DRAW_SPRITE HndlDrawSprite = NULL;

static void Cleanup(void)
{

    HndlCreateWindow = NULL;
    HndlRun = NULL;
    HndlGetKeyState = NULL;
    HndlDrawLine = NULL;
    HndlDrawTriangle = NULL;
    HndlDrawQuad = NULL;
    HndlDrawPoint = NULL;
    HndlDrawSprite = NULL;
    if (g_hEngineDll)
    {
        (void)FreeLibrary(g_hEngineDll);
    }
}

static void InitializeDllHandlers(void)
{

    HndlCreateWindow = (FARPROC_ENGINE_INIT)(GetProcAddress(g_hEngineDll, "HndlEngineCreateWindow"));
    HndlRun = (FARPROC_ENGINE_RUN)(GetProcAddress(g_hEngineDll, "HndlEngineRun"));
    HndlGetKeyState = (FARPROC_ENGINE_KEY_STATE)(GetProcAddress(g_hEngineDll, "HndlEngineGetKeyState"));
    HndlDrawLine = (FARPROC_ENGINE_DRAW_LINE)(GetProcAddress(g_hEngineDll, "HndlEngineDrawLine"));
    HndlDrawTriangle = (FARPROC_ENGINE_DRAW_TRIANGLE)(GetProcAddress(g_hEngineDll, "HndlEngineDrawTriangle"));
    HndlDrawQuad = (FARPROC_ENGINE_DRAW_QUAD)(GetProcAddress(g_hEngineDll, "HndlEngineDrawQuad"));
    HndlDrawPoint = (FARPROC_ENGINE_DRAW_POINT)(GetProcAddress(g_hEngineDll, "HndlEngineDrawPoint"));
    HndlDrawSprite = (FARPROC_ENGINE_DRAW_SPRITE)(GetProcAddress(g_hEngineDll, "HndlEngineDrawSprite"));
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
        HndlDrawSprite(
            100,
            100,
            1,
            200,
            (flip_t)0);
        #endif
#if 0
        size_t i = 0;
        size_t j = 0;
        for (i = 0; i < SCREEN_WIDTH; i++)
        {
            for (j = 0; j < SCREEN_HEIGHT; j++)
            {
                coordinate_t p = { (float)i,(float)j, -1.0f };
                color_t color = { rand() % 255, rand() % 255, rand() % 255, 0 };
                HndlDrawPoint(p, color);
            }
        }

#endif
#if 0
        coordinate_t p1 = { 0.0f, 0.0f, -1.0f };
        coordinate_t p2 = { 200.0f, 200.0f, -1.0f };
        color_t color = { rand() % 255, rand() % 255, rand() % 255, 0 };
        HndlDrawLine(p1, p2, color);

        coordinate_t pr1 = { 200.0f, 200.0f, -2.0f };
        coordinate_t pr2 = { 250.0f, 200.0f, -2.0f };
        coordinate_t pr3 = { 200.0f, 150.0f, -2.0f };

        color_t cr = { rand() % 255, rand() % 255, rand() % 255, 0 };

        HndlDrawTriangle(pr1, pr2, pr3, cr);


        coordinate_t pq1 = { 300.0f, 300.0f, -2.0f };
        coordinate_t pq2 = { 350.0f, 300.0f, -2.0f };
        coordinate_t pq3 = { 350.0f, 250.0f, -2.0f };
        coordinate_t pq4 = { 300.0f, 250.0f, -2.0f };
        color_t cq = { rand() % 255, rand() % 255, rand() % 255, 0 };
        HndlDrawQuad(pq1, pq2, pq3, pq4, cq);

        //    int i = HndlLoadTexture(L"ANA");
        //    glBindTexture(GL_TEXTURE_2D, i);



#endif
        return TRUE;
    }
    return FALSE;
}

int APIENTRY wWinMain(_In_ HINSTANCE hInstance,
    _In_opt_ HINSTANCE hPrevInstance,
    _In_ LPWSTR    lpCmdLine,
    _In_ int       nCmdShow)
{
    UNREFERENCED_PARAMETER(hPrevInstance);

    SetProcessDPIAware();

    //g_hEngineDll = LoadLibraryW(L"bear-engine-dll.dll");
    g_hEngineDll = LoadLibraryW(L"bengine_dll");

    if (NULL == g_hEngineDll)
    {
        Cleanup();
        exit(GetLastError());
    }


    InitializeDllHandlers();

    signal(SIGINT, HndlSIGINT);

    if (HndlCreateWindow())
    {
      Cleanup();
      return -1;
    }

    HndlRun(&HndlDraw);

    Cleanup();
    return EXIT_SUCCESS;
}
