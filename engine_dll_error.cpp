#include "framework.h"


extern void ReportError(const wchar_t *sMessage);

extern void ReportWin32Error(const wchar_t* sMessage);



void ReportError(const wchar_t *sMessage)
{
  wprintf(L"[!] Error: %s\n", sMessage);
}


void ReportWin32Error(const wchar_t *sMessage)
{
  DWORD dwErr = GetLastError();

  wprintf(L"[!] Error: %s Failed.\nWin32 Last Error: %ld\n", sMessage, dwErr);

  wchar_t sErrorCodeBuffer[sizeof(DWORD) * 8];

  swprintf(sErrorCodeBuffer, sizeof(sErrorCodeBuffer)/sizeof(wchar_t), L"%04lX", (int)dwErr);

  wchar_t sDialogMsgBuffer[100];

  swprintf(sDialogMsgBuffer, sizeof(sDialogMsgBuffer) / sizeof(wchar_t), L"Error: %s Failed. Error code: 0x%ls", sMessage, sErrorCodeBuffer);

  MessageBoxW(
              NULL,
              sDialogMsgBuffer,
              L"BLightWM Error",
              MB_OK | MB_ICONSTOP
              );
}
