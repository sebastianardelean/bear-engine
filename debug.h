#ifndef DEBUG_H
#define DEBUG_H

#ifdef DEBUG
#include <Windows.h>
#include <stdio.h>
#include <stdarg.h>

#define BUFFER_SIZE 2048

inline void DebugA(const char * format,...)
{
  char buffer[BUFFER_SIZE];
  va_list args;
  va_start(args, format);
  vsprintf_s(buffer, BUFFER_SIZE, format, args);
  va_end(args);
  OutputDebugStringA(buffer);
}

inline void DebugW(const wchar_t *format, ...)
{
  wchar_t buffer[BUFFER_SIZE];
  va_list args;
  va_start(args, format);
  vswprintf_s(buffer, BUFFER_SIZE, format, args);
  va_end(args);
  OutputDebugStringW(buffer);
}

#define DEBUG_A(format, ...) printf("DEBUG: %s Line %d: " format "\n", __FILE__, __LINE__, ##__VA_ARGS__); \
  DebugA(format, ##__VA_ARGS__)

#define DEBUG_W(format, ...) wprintf(L"DEBUG: %ls Line %d: " format "\n", __FILE__, __LINE__, ##__VA_ARGS__); \
	DebugW(format, ##__VA_ARGS__)
#else
#define DEBUG_A(format, ...) 
#define DEBUG_W(format, ...)
#endif

#endif
