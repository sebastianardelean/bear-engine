#ifndef ENGINE_TYPES_H
#define ENGINE_TYPES_H

#include <Windows.h>

typedef struct
{
  BYTE r;
  BYTE g;
  BYTE b;
  BYTE a;
} color_t;


typedef struct
{
  float x;
  float y;
  float z;
}coordinate_t;

typedef struct
{
  INT32 width;
  INT32 height;
  color_t *pixels;
}sprite_t;


#endif
