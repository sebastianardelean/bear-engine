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


#endif
