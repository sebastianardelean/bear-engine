#ifndef ENGINE_TYPES_H
#define ENGINE_TYPES_H

#include <Windows.h>

typedef struct
{
  BYTE a;
  BYTE b;
  BYTE g;
  BYTE r;

} color_t;


typedef struct
{
  float x;
  float y;
  float z;
}coordinate_t;

typedef enum
{
  FLIP_NONE = 0,
  FLIP_HORIZONTAL = 1,
  FLIP_VERTICAL = 2
}flip_t;

typedef union
{
  color_t color;
  COLORREF rgb;
}pixel_t;

typedef struct
{
  INT32 width;
  INT32 height;
  flip_t flip;
  pixel_t *pixels;
  
}sprite_t;


typedef void(FAR WINAPI* FARPROC_ENGINE_DRAW_SPRITE)(
	const INT32, 
	const INT32, 
	const DWORD, 
	const DWORD, 
	const flip_t);


typedef void(FAR WINAPI* FARPROC_ENGINE_DRAW_QUAD)(
	const coordinate_t,
	const coordinate_t,
	const coordinate_t,
	const coordinate_t,
	const color_t);

typedef void(FAR WINAPI* FARPROC_ENGINE_DRAW_TRIANGLE)(
	const coordinate_t,
	const coordinate_t,
	const coordinate_t,
	const color_t
	);	


typedef void(FAR WINAPI* FARPROC_ENGINE_DRAW_LINE)(
	const coordinate_t,
	const coordinate_t,
	const color_t
	);

typedef void(FAR WINAPI* FARPROC_ENGINE_DRAW_POINT)(
	const coordinate_t,
	const color_t
	);

typedef BOOL(FAR WINAPI* FARPROC_ENGINE_KEY_STATE)(BYTE);

typedef INT_PTR(FAR WINAPI * FARPROC_ENGINE_INIT)(void); 

typedef void(FAR WINAPI* FARPROC_ENGINE_RUN)(BOOL(*FctDraw)(void));

#endif
