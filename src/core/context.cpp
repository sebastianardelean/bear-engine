#include "core/context.hpp"
#include "core/safe_initialize.hpp"
#include "core/auto_release.hpp"


namespace bear
{
	std::unique_ptr<Context::SdlGlobalInitializer>
		Context::p_SdlGlobalInitializer = std::make_unique<SdlGlobalInitializer>();

	Context::Context()
	{

	}

	Context::Context(std::string& name, int32_t winWidth, int32_t winHeight) :
		g_sName(name),
		g_i32WinWidth(winWidth),
		g_i32WinHeight(winHeight)
		/*g_pWindow(core::sdl_safe_create_window(name, winWidth, winHeight), &SDL_DestroyWindow),
		g_pRenderer(core::sdl_safe_create_renderer(g_pWindow.get()), &SDL_DestroyRenderer),
		g_pTexture(core::sdl_safe_create_texture(g_pRenderer.get(), g_pWindow.get()), &SDL_DestroyTexture)*/

	{
		g_pWindow = { 
			core::sdl_safe_create_window(name, winWidth, winHeight),
			&SDL_DestroyWindow 
		};
		g_pRenderer = { 
			core::sdl_safe_create_renderer(g_pWindow.get()),
			&SDL_DestroyRenderer 
		};

		g_pTexture = {
			core::sdl_safe_create_texture(g_pRenderer.get(), g_pWindow.get()),
			&SDL_DestroyTexture

		};
	}
	

};