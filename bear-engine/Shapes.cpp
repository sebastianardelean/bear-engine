#include "pch.h"
#include "framework.h"
#include "Shapes.h"

extern void EngineLoadImageFromFile(sprite_t*, const std::wstring&);

namespace bear
{

	enum ShapeType_t :uint8_t
	{
		SHAPE_POINT = 1,
		SHAPE_LINE,
		SHAPE_TRIANGLE,
		SHAPE_QUAD
	};

	Shape::Shape(std::vector<coordinate_t> svCoordinates, fill_option_t sFillOption) :
		g_svCoordinates(svCoordinates),
		g_sFillOption(sFillOption)
	{
		g_bIsLightEnabled = false;
		g_sNormal = { 0 };
	}
	Shape::Shape(std::vector<coordinate_t> svCoordinates, color_t sColor) :g_svCoordinates(svCoordinates)
	{
		g_bIsLightEnabled = false;
		g_sNormal = { 0 };
		g_sFillOption.fill_type = FILL_COLOR;
		g_sFillOption.color = sColor;
	}

	coordinate_t Shape::Normalize3d(const coordinate_t coordinates)
	{
		coordinate_t newCoordinates = {};
		newCoordinates.x = 2 * (coordinates.x / (float)SCREEN_WIDTH) - 1;
		newCoordinates.y = 2 * (coordinates.y / (float)SCREEN_HEIGHT) - 1;
		newCoordinates.z = coordinates.z;


		return newCoordinates;
	}

	std::vector<coordinate_t> Shape::Normalize3d(const std::vector<coordinate_t> coordinates)
	{
		std::vector<coordinate_t> newCoordinates;
		for (coordinate_t c : coordinates) {
			newCoordinates.push_back(Normalize3d(c));
		}
		return newCoordinates;
	}

	void Shape::Draw(bool bIsLightEnabled)
	{

		switch (g_svCoordinates.size())
		{
		case ShapeType_t::SHAPE_POINT:
		{
			glBegin(GL_POINTS);
			glColor4d(g_sFillOption.color.r / 255.0f, g_sFillOption.color.g / 255.0f, g_sFillOption.color.b / 255.0f, g_sFillOption.color.a / 255.0f);
			glVertex3f(g_svCoordinates[0].x, g_svCoordinates[0].y, g_svCoordinates[0].z);
			glEnd();
			break;
		}
		case ShapeType_t::SHAPE_LINE:
		{
			glBegin(GL_LINES);
			glColor4d(g_sFillOption.color.r / 255.0f, g_sFillOption.color.g / 255.0f, g_sFillOption.color.b / 255.0f, g_sFillOption.color.a / 255.0f);
			glVertex3f(g_svCoordinates[0].x, g_svCoordinates[0].y, g_svCoordinates[0].z);
			glColor4d(g_sFillOption.color.r / 255.0f, g_sFillOption.color.g / 255.0f, g_sFillOption.color.b / 255.0f, g_sFillOption.color.a / 255.0f);
			glVertex3f(g_svCoordinates[1].x, g_svCoordinates[1].y, g_svCoordinates[1].z);
			glEnd();
			break;
		}
		case ShapeType_t::SHAPE_TRIANGLE:
		{
			if (g_sFillOption.fill_type == FILL_TEXTURE)
			{
				DWORD32 dwTextureId = 0;
				//EngineCreateTextureFromImageFile(fill.sTextureFile, &dwTextureId);
				glBindTexture(GL_TEXTURE_2D, dwTextureId);
				glBegin(GL_TRIANGLES);

				glVertex3f(g_svCoordinates[0].x, g_svCoordinates[0].y, g_svCoordinates[0].z);

				glVertex3f(g_svCoordinates[1].x, g_svCoordinates[1].y, g_svCoordinates[1].z);

				glVertex3f(g_svCoordinates[2].x, g_svCoordinates[2].y, g_svCoordinates[2].z);
				glEnd();
			}
			else
			{
				glBegin(GL_TRIANGLES);
				glColor4d(g_sFillOption.color.r / 255.0f, g_sFillOption.color.g / 255.0f, g_sFillOption.color.b / 255.0f, g_sFillOption.color.a / 255.0f);
				glVertex3f(g_svCoordinates[0].x, g_svCoordinates[0].y, g_svCoordinates[0].z);
				glColor4d(g_sFillOption.color.r / 255.0f, g_sFillOption.color.g / 255.0f, g_sFillOption.color.b / 255.0f, g_sFillOption.color.a / 255.0f);
				glVertex3f(g_svCoordinates[1].x, g_svCoordinates[1].y, g_svCoordinates[1].z);
				glColor4d(g_sFillOption.color.r / 255.0f, g_sFillOption.color.g / 255.0f, g_sFillOption.color.b / 255.0f, g_sFillOption.color.a / 255.0f);
				glVertex3f(g_svCoordinates[2].x, g_svCoordinates[2].y, g_svCoordinates[2].z);
				glEnd();
			}
			break;
		}
		case ShapeType_t::SHAPE_QUAD:
		{
			if (g_sFillOption.fill_type == FILL_TEXTURE)
			{
				DWORD32 dwTextureId = 0;
				//EngineCreateTextureFromImageFile(fill.sTextureFile, &dwTextureId);
				glBindTexture(GL_TEXTURE_2D, dwTextureId);
				glBegin(GL_QUADS);

				glVertex3f(g_svCoordinates[0].x, g_svCoordinates[0].y, g_svCoordinates[0].z);

				glVertex3f(g_svCoordinates[1].x, g_svCoordinates[1].y, g_svCoordinates[1].z);

				glVertex3f(g_svCoordinates[2].x, g_svCoordinates[2].y, g_svCoordinates[2].z);

				glVertex3f(g_svCoordinates[3].x, g_svCoordinates[3].y, g_svCoordinates[3].z);
				glEnd();

			}
			else
			{
				glBegin(GL_QUADS);
				glColor4d(g_sFillOption.color.r / 255.0f, g_sFillOption.color.g / 255.0f, g_sFillOption.color.b / 255.0f, g_sFillOption.color.a / 255.0f);
				glVertex3f(g_svCoordinates[0].x, g_svCoordinates[0].y, g_svCoordinates[0].z);
				glColor4d(g_sFillOption.color.r / 255.0f, g_sFillOption.color.g / 255.0f, g_sFillOption.color.b / 255.0f, g_sFillOption.color.a / 255.0f);
				glVertex3f(g_svCoordinates[1].x, g_svCoordinates[1].y, g_svCoordinates[1].z);
				glColor4d(g_sFillOption.color.r / 255.0f, g_sFillOption.color.g / 255.0f, g_sFillOption.color.b / 255.0f, g_sFillOption.color.a / 255.0f);
				glVertex3f(g_svCoordinates[2].x, g_svCoordinates[2].y, g_svCoordinates[2].z);
				glColor4d(g_sFillOption.color.r / 255.0f, g_sFillOption.color.g / 255.0f, g_sFillOption.color.b / 255.0f, g_sFillOption.color.a / 255.0f);
				glVertex3f(g_svCoordinates[3].x, g_svCoordinates[3].y, g_svCoordinates[3].z);
				glEnd();
			}
			break;
		}
		default:
			break;
		}

	}


	Point::Point(coordinate_t sP, color_t sColor) :Shape({ Normalize3d(sP) }, sColor)
	{

	}

	Line::Line(coordinate_t sP1, coordinate_t sP2, color_t sColor) : Shape({ Normalize3d(sP1), Normalize3d(sP2) }, sColor)
	{

	}

	Line::Line(std::vector<coordinate_t> sP, color_t sColor) : Shape(Normalize3d(sP), sColor)
	{

	}


	Triangle::Triangle(coordinate_t sP1, coordinate_t sP2, coordinate_t sP3, fill_option_t sFillOption) :Shape({ Normalize3d(sP1),Normalize3d(sP2),Normalize3d(sP3) }, sFillOption)
	{

	}

	Triangle::Triangle(std::vector<coordinate_t> sP, fill_option_t sFillOption) : Shape(Normalize3d(sP), sFillOption)
	{

	}

	Quad::Quad(coordinate_t sP1, coordinate_t sP2, coordinate_t sP3, coordinate_t sP4, fill_option_t sFillOption) : Shape({ Normalize3d(sP1),Normalize3d(sP2),Normalize3d(sP3),Normalize3d(sP4) }, sFillOption)
	{
	}

	Quad::Quad(std::vector<coordinate_t> sP, fill_option_t sFillOption) : Shape(Normalize3d(sP), sFillOption)
	{


	}

	Sprite::Sprite(const int32_t x, const int32_t y, const std::wstring& sFilePath) :
		g_i32x(x),
		g_i32y(y),
		g_sFilePath(sFilePath)
	{

	}
	void Sprite::Draw()
	{
		sprite_t sprite = {};
		EngineLoadImageFromFile(&sprite, g_sFilePath.c_str());

		for (int32_t j = 0; j < sprite.height; j++)
		{
			for (int32_t i = 0; i < sprite.width; i++)
			{
				coordinate_t p = {
					(float)i,
					(float)j,
					-2.0f
				};
				
				Point ptr(p, sprite.pixels[j * sprite.width + i].color);
				ptr.Draw(false);
			}
		}
	}

};