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


		AutoRelease<SDL_Window*, nullptr> g_pWindow;
		AutoRelease<SDL_Renderer *, nullptr> g_pRenderer;
		AutoRelease<SDL_Texture*, nullptr> g_pTexture;


		std::optional<SDL_KeyboardEvent> g_KeyboardEvent;

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