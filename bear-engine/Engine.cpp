#include "pch.h"
#include "Engine.h"


#pragma comment (lib,"Gdiplus.lib")
#pragma comment (lib,"user32.lib")
#pragma comment (lib,"opengl32.lib")
#pragma comment (lib,"Gdi32.lib")
#pragma comment (lib,"Glu32.lib")
#pragma comment (lib,"Gdiplus.lib")

#pragma region imported_from_core
extern LRESULT CALLBACK WndProc(HWND hWnd, UINT uMsg, WPARAM wParam, LPARAM lParam);
extern INT EngineCreateWindow(
    const std::wstring& cTitle,
    const INT iWinWidth,
    const INT iWinHeight,
    const BOOL bFullScreen
);

extern void EngineDestroyWindow();

extern void EngineDrawScene();

extern BOOL EngineGetKeyState(const BYTE bKeyCode);
#pragma endregion

namespace bear
{
    void HndlEngineChangeLighting(
        bool bLightOn
    )
    {
        if (bLightOn == TRUE)
        {
            glEnable(GL_LIGHTING);
        }
        else
        {
            glDisable(GL_LIGHTING);
        }
    }



    bool HndlEngineGetKeyState(uint8_t bKeyCode)
    {
        return EngineGetKeyState(bKeyCode);
    }

    int32_t HndlEngineCreateWindow()
    {
        return EngineCreateWindow(
            L"Bear Engine",
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            FULL_SCREEN
        );

    }


    void HndlEngineRun(bool(*FctDraw)(void))
    {
        MSG msg;
        bool bDone = FALSE;
        EngineDrawScene();
        while (!bDone)
        {
            if (PeekMessage(&msg, NULL, 0, 0, PM_REMOVE))
            {
                if (msg.message == WM_QUIT)
                {
                    bDone = true;
                }
                else
                {
                    TranslateMessage(&msg);
                    DispatchMessage(&msg);
                }
            }
            else
            {
                //Draw the scene
                if (FctDraw())
                {
                    EngineDrawScene();
                }
            }
        }
        EngineDestroyWindow();
    }
}
















