#include "pch.h"
#include "framework.h"
#include "Shapes.h"

extern void EngineLoadImageFromFile(sprite_t*, const std::wstring&);
extern void EngineCreateTextureFromImageFile(
	const std::wstring& sImageFile,
	texture_filter_type_t eFilterType,
	DWORD32* dwTextureId
);

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
		
		
	}
	Shape::Shape(std::vector<coordinate_t> svCoordinates, color_t sColor) :g_svCoordinates(svCoordinates)
	{
		
		g_sFillOption.fill_type = FILL_COLOR;
		g_sFillOption.color = sColor;
	}

	coordinate_t Shape::Normalize3d(const coordinate_t coordinates)
	{
		coordinate_t newCoordinates = {};
		newCoordinates.x = 2 * (coordinates.x / (float)SCREEN_WIDTH) - 1;
		newCoordinates.y = 2 * (coordinates.y / (float)SCREEN_HEIGHT) - 1;
		newCoordinates.z = coordinates.z;

		newCoordinates.tx = 2 * (coordinates.tx / (float)SCREEN_WIDTH) - 1;
		newCoordinates.ty = 2 * (coordinates.ty / (float)SCREEN_HEIGHT) - 1;
		newCoordinates.tz = coordinates.tz;

		newCoordinates.nx = 2 * (coordinates.nx / (float)SCREEN_WIDTH) - 1;
		newCoordinates.ny = 2 * (coordinates.ny / (float)SCREEN_HEIGHT) - 1;
		newCoordinates.nz = coordinates.nz;

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
				EngineCreateTextureFromImageFile(g_sFillOption.sTextureFile, g_sFillOption.filter_type, &dwTextureId);
				glBindTexture(GL_TEXTURE_2D, dwTextureId);
				glBegin(GL_TRIANGLES);

				for (coordinate_t& c : g_svCoordinates)
				{
					if (bIsLightEnabled)
						glNormal3f(c.nx, c.ny, c.nz);
					glTexCoord3f(c.tx, c.ty, c.tz);
					glVertex3f(c.x, c.y, c.z);
				}
				glEnd();
			}
			else
			{
				glBegin(GL_TRIANGLES);
				for (coordinate_t& c : g_svCoordinates)
				{
					glColor4d(g_sFillOption.color.r / 255.0f, g_sFillOption.color.g / 255.0f, g_sFillOption.color.b / 255.0f, g_sFillOption.color.a / 255.0f);
					glVertex3f(c.x, c.y, c.z);
				}
				glEnd();
			}
			break;
		}
		default://is a quad
		{
			if (g_sFillOption.fill_type == FILL_TEXTURE)
			{
				DWORD32 dwTextureId = 0;
				EngineCreateTextureFromImageFile(g_sFillOption.sTextureFile, g_sFillOption.filter_type, &dwTextureId);
				glBindTexture(GL_TEXTURE_2D, dwTextureId);
				glBegin(GL_QUADS);

				for (coordinate_t& c : g_svCoordinates)
				{
					if (bIsLightEnabled)
						glNormal3f(c.nx, c.ny, c.nz);
					glTexCoord3f(c.tx, c.ty, c.tz);
					glVertex3f(c.x, c.y, c.z);
				}
				glEnd();

			}
			else
			{
				glBegin(GL_QUADS);
				for (coordinate_t& c : g_svCoordinates)
				{
					glColor4d(g_sFillOption.color.r / 255.0f, g_sFillOption.color.g / 255.0f, g_sFillOption.color.b / 255.0f, g_sFillOption.color.a / 255.0f);
					glVertex3f(c.x, c.y, c.z);
				}
				
				glEnd();
			}
			break;
		}

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


	Quad::Quad(std::vector<coordinate_t> sP, fill_option_t sFillOption) : Shape(Normalize3d(sP), sFillOption)
	{


	}

	Sprite::Sprite(const int32_t x, const int32_t y, const std::wstring& sFilePath) :
		g_i32x(x),
		g_i32y(y),
		g_sFilePath(sFilePath)
	{

	}

	Sprite::~Sprite()
	{
		if (g_sSprite.pixels != nullptr)
		{
			delete[] g_sSprite.pixels;
		}
	}
	void Sprite::Draw()
	{
		
		EngineLoadImageFromFile(&g_sSprite, g_sFilePath.c_str());

		for (int32_t j = 0; j < g_sSprite.height; j++)
		{
			for (int32_t i = 0; i < g_sSprite.width; i++)
			{
				coordinate_t p = {
					(float)i,
					(float)j,
					-2.0f
				};
				
				Point ptr(p, g_sSprite.pixels[j * g_sSprite.width + i].color);
				ptr.Draw(false);
			}
		}
	}

};