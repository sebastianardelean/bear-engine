#include "framework.h"


BOOL g_baKeys[256] = {0};

extern BOOL EngineGetKeyState(const BYTE bKeyCode);

extern LRESULT CALLBACK WndProc(HWND hWnd, UINT uMsg, WPARAM wParam, LPARAM lParam);

LRESULT CALLBACK WndProc(HWND hWnd, UINT uMsg, WPARAM wParam, LPARAM lParam)
{
  switch (uMsg)
  {
  case WM_ACTIVATE://should stop rendering in background
    break;
  case WM_SYSCOMMAND:
  {
    switch (wParam)
    {
    case SC_SCREENSAVE:
    case SC_MONITORPOWER:
      return 0;
    }
    break;
  }
  case WM_CLOSE:
    PostQuitMessage(0);
    break;
  case WM_KEYDOWN:
    g_baKeys[wParam] = TRUE;
    break;
  case WM_KEYUP:
    g_baKeys[wParam] = FALSE;
    break;
  case WM_SIZE:
    break;
  default:
    break;
  }
  return DefWindowProc(hWnd, uMsg, wParam, lParam);
}

BOOL EngineGetKeyState(const BYTE bKeyCode)
{
  BOOL bKeyState = g_baKeys[bKeyCode];
  g_baKeys[bKeyCode] = FALSE;
  return bKeyState;
}

