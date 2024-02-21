#include <Windows.h>
#include <gl/GL.h>
#include <gl/glu.h>
#include "configuration.h"
#include "engine_types.h"

#include "debug.h"

#define NORMALIZE_3D(in, out) \
  do { \
  (out).x = 2 *((in).x/(float)SCREEN_WIDTH) - 1;           \
  (out).y = 2 *((in).y/(float)SCREEN_HEIGHT) - 1;          \
  (out).z = (in).z;                                         \
  } while (0)


extern void EngineDrawPoint(
                            const point_t p,
                            const color_t color
                            );

extern void EngineDrawLine(
                           const point_t p1,
                           const point_t p2,
                           const color_t color
                           );

extern void EngineDrawTriangle(
                               const point_t top_p,
                               const point_t bottom_left_p,
                               const point_t bottom_right_p,
                               const color_t color
                               );

extern void EngineDrawQuad(
                           const point_t top_left_p,
                           const point_t top_right_p,
                           const point_t bottom_right_p,
                           const point_t bottom_left_p,
                           const color_t color
                           );

void EngineDrawPoint(
                            const point_t p,
                            const color_t color)
{
  point_t m_p;
  NORMALIZE_3D(p, m_p);
  glBegin(GL_POINTS);
  glColor4d(color.r/255.0f, color.g/255.0f, color.b/255.0f, color.a/255.0f);
  glVertex3f(m_p.x, m_p.y, m_p.z);
  glEnd();
}


void EngineDrawLine(point_t p1, point_t p2, color_t color)
{

  point_t m_p1;
  point_t m_p2;
  NORMALIZE_3D(p1, m_p1);
  NORMALIZE_3D(p2, m_p2);
  
 


  glBegin(GL_LINES);
  glColor4d(color.r/255.0f, color.g/255.0f, color.b/255.0f, color.a/255.0f);
  glVertex3f(m_p1.x, m_p1.y, m_p1.z);
  glColor4d(color.r/255.0f, color.g/255.0f, color.b/255.0f, color.a/255.0f);
  glVertex3f(m_p2.x, m_p2.y, m_p2.z);
  glEnd();

}

void EngineDrawTriangle(
                        const point_t top_p,
                        const point_t bottom_left_p,
                        const point_t bottom_right_p,
                        const color_t color
                        )
{
  point_t m_p1;
  point_t m_p2;
  point_t m_p3;
  NORMALIZE_3D(top_p, m_p1);
  NORMALIZE_3D(bottom_left_p, m_p2);
  NORMALIZE_3D(bottom_right_p, m_p3);

  
  glBegin(GL_TRIANGLES);
  glColor4d(color.r/255.0f, color.g/255.0f, color.b/255.0f, color.a/255.0f);
  glVertex3f(m_p1.x, m_p1.y, m_p1.z);
  glColor4d(color.r/255.0f, color.g/255.0f, color.b/255.0f, color.a/255.0f);
  glVertex3f(m_p2.x, m_p2.y, m_p2.z);
  glColor4d(color.r/255.0f, color.g/255.0f, color.b/255.0f, color.a/255.0f);
  glVertex3f(m_p3.x, m_p3.y, m_p3.z);
  glEnd();
}


void EngineDrawQuad(
                    const point_t top_left_p,
                    const point_t top_right_p,
                    const point_t bottom_right_p,
                    const point_t bottom_left_p,
                    const color_t color
                    )
{
  point_t m_p1;
  point_t m_p2;
  point_t m_p3;
  point_t m_p4;
  
  NORMALIZE_3D(top_left_p, m_p1);
  NORMALIZE_3D(top_right_p, m_p2);
  NORMALIZE_3D(bottom_right_p, m_p3);
  NORMALIZE_3D(bottom_left_p, m_p4);


  glBegin(GL_QUADS);
  glColor4d(color.r/255.0f, color.g/255.0f, color.b/255.0f, color.a/255.0f);
  glVertex3f(m_p1.x, m_p1.y, m_p1.z);
  glColor4d(color.r/255.0f, color.g/255.0f, color.b/255.0f, color.a/255.0f);
  glVertex3f(m_p2.x, m_p2.y, m_p2.z);
  glColor4d(color.r/255.0f, color.g/255.0f, color.b/255.0f, color.a/255.0f);
  glVertex3f(m_p3.x, m_p3.y, m_p3.z);
  glColor4d(color.r/255.0f, color.g/255.0f, color.b/255.0f, color.a/255.0f);
  glVertex3f(m_p4.x, m_p4.y, m_p4.z);
  glEnd();
  
}
