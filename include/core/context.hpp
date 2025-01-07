#pragma once
#include "core/auto_release.hpp"

namespace bear
{
	struct Context
	{
		
		Context(std::string& name, int32_t winWidth, int32_t winHeight);
		Context();
		~Context() = default;


		Context(const Context&) = default;
		auto operator = (const Context&)->Context & = default;

		Context(Context&&) = default;
		auto operator = (Context&&)->Context & = default;

		std::string_view g_sName;
		int32_t g_i32WinWidth;
		int32_t g_i32WinHeight;

		/*std::unique_ptr<SDL_Window, decltype(&SDL_DestroyWindow)> g_pWindow;
		std::unique_ptr<SDL_Renderer, decltype(&SDL_DestroyRenderer)> g_pRenderer;
		std::unique_ptr<SDL_Texture, decltype(&SDL_DestroyTexture)> g_pTexture; */
		AutoRelease<SDL_Window*, nullptr> g_pWindow;
		AutoRelease<SDL_Renderer *, nullptr> g_pRenderer;
		AutoRelease<SDL_Texture*, nullptr> g_pTexture;

	private:
		class SdlGlobalInitializer
		{
		public:
			SdlGlobalInitializer() { SDL_Init(SDL_INIT_EVERYTHING); }
			~SdlGlobalInitializer() { SDL_Quit(); }
		};

		static std::unique_ptr<SdlGlobalInitializer>p_SdlGlobalInitializer;
	};
};