#pragma once

#if defined(_WIN32) || defined(WIN32) 
#ifdef BEARENGINE_EXPORTS
#define BEARENGINE_API __declspec(dllexport)
#else
#define BEARENGINE_API __declspec(dllimport)
#endif
#elif defined(__unix__)
#define LIBQPP_API 
#endif // _WIN32

#include <vector>


namespace bear {
	class BEARENGINE_API Shape
	{
	public:
		Shape(std::vector<coordinate_t> svCoordinates, fill_option_t sFillOption);
		Shape(std::vector<coordinate_t> svCoordinates, color_t sColor);
		void Draw(bool bIsLightEnabled);
	private:
	protected:
		coordinate_t Normalize3d(const coordinate_t coordinates);
		std::vector<coordinate_t> Normalize3d(const std::vector<coordinate_t> coordinates);
		std::vector<coordinate_t> g_svCoordinates;
		fill_option_t g_sFillOption;
		bool g_bIsLightEnabled;
		coordinate_t g_sNormal;
	};

	class BEARENGINE_API Point : public Shape
	{
	public:
		Point(coordinate_t sP, color_t sColor);
	};

	class BEARENGINE_API Line :public Shape
	{
	public:
		Line(coordinate_t sP1, coordinate_t sP2, color_t sColor);
		Line(std::vector<coordinate_t> sP, color_t sColor);
	};


	class BEARENGINE_API Triangle :public Shape
	{
	public:
		Triangle(coordinate_t sP1, coordinate_t sP2, coordinate_t sP3, fill_option_t sFillOption);
		Triangle(std::vector<coordinate_t> sP, fill_option_t sFillOption);
	};


	class BEARENGINE_API Quad :public Shape
	{
	public:
		Quad(coordinate_t sP1, coordinate_t sP2, coordinate_t sP3, coordinate_t sP4, fill_option_t sFillOption);
		Quad(std::vector<coordinate_t> sP, fill_option_t sFillOption);
	};

	class BEARENGINE_API Sprite
	{
	public:
		Sprite(const int32_t x, const int32_t y, const std::wstring& sFilePath);
		Sprite(const Sprite&) = default;
		Sprite(Sprite&&) = default;
		Sprite& operator=(const Sprite&) = default;
		Sprite& operator=(Sprite&&) = default;
		
		~Sprite();

		void Draw();
	private:
		int32_t g_i32x;
		int32_t g_i32y;
		std::wstring g_sFilePath;
		sprite_t g_sSprite;
	};
};
