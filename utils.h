#pragma once

#pragma region utils
namespace utils
{
    static std::wstring ConvertS2W(std::string s)
    {

        int count = MultiByteToWideChar(CP_UTF8, 0, s.c_str(), -1, NULL, 0);
        wchar_t* buffer = new wchar_t[count];
        MultiByteToWideChar(CP_UTF8, 0, s.c_str(), -1, buffer, count);

        std::wstring w(buffer);
        delete[] buffer;
        return w;
    }

    static void ReportError(const std::wstring& sMessage)
    {
        wprintf(L"[!] Error: %s %ld\n", sMessage.c_str(), GetLastError());
    }

    static void ReportWin32Error(const std::wstring& sMessage)
    {
        DWORD dwErr = GetLastError();

        wprintf(L"[!] Error: %s Failed.\nWin32 Last Error: %ld\n", sMessage.c_str(), dwErr);

        wchar_t sErrorCodeBuffer[sizeof(DWORD) * 8];

        swprintf(sErrorCodeBuffer, sizeof(sErrorCodeBuffer) / sizeof(wchar_t), L"%04lX", (int)dwErr);

        wchar_t sDialogMsgBuffer[100];

        swprintf(sDialogMsgBuffer, sizeof(sDialogMsgBuffer) / sizeof(wchar_t), L"Error: %s Failed. Error code: 0x%ls", sMessage.c_str(), sErrorCodeBuffer);

        MessageBoxW(
            NULL,
            sDialogMsgBuffer,
            L"BLightWM Error",
            MB_OK | MB_ICONSTOP
        );
    }
};


#pragma endregion



#pragma region debug

#ifdef DEBUG
#define BUFFER_SIZE 2048

namespace dbg
{
    static void DebugA(const char* format, ...)
    {
        char buffer[BUFFER_SIZE];
        va_list args;
        va_start(args, format);
        vsprintf_s(buffer, BUFFER_SIZE, format, args);
        va_end(args);
        OutputDebugStringA(buffer);
    }

    static void DebugW(const wchar_t* format, ...)
    {
        wchar_t buffer[BUFFER_SIZE];
        va_list args;
        va_start(args, format);
        vswprintf_s(buffer, BUFFER_SIZE, format, args);
        va_end(args);
        OutputDebugStringW(buffer);
    }

};

#define DEBUG_A(format, ...) printf("DEBUG: %s Line %d: " format "\n", __FILE__, __LINE__, ##__VA_ARGS__); \
    dbg::DebugA(format, ##__VA_ARGS__)

#define DEBUG_W(format, ...) wprintf(L"DEBUG: %ls Line %d: " format "\n", __FILE__, __LINE__, ##__VA_ARGS__); \
    dbg::DebugW(format, ##__VA_ARGS__)
#else
#define DEBUG_A(format, ...) 
#define DEBUG_W(format, ...)
#endif


#pragma endregion 