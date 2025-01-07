#include "core/auto_release.hpp"

#include "log/logger.hpp"
#include "core/engine.hpp"
#include "core/context.hpp"


static bear::Context g_Context;

namespace bear {
	

	auto EngineChangeLighting(bool bLightOn) -> void
	{

	}

	auto EngineGetKeyState(SDL_Keycode bKeyCode) -> bool
	{
		return true;
	}



	auto EngineInitialize(std::string& name, int32_t winWidth, int32_t winHeight) -> void
	{
		
		::g_Context = Context( name, winWidth, winHeight );

		// clear the renderer
		SDL_SetRenderDrawColor(::g_Context.g_pRenderer.get(), 0, 0, 0, 0);
		SDL_RenderClear(::g_Context.g_pRenderer.get());
		SDL_RenderPresent(::g_Context.g_pRenderer.get());
	}

	
	auto EngineRun(std::function<bool(Context &ctx)> hndlFunct) -> void
	{
		SDL_Event e;
		bool done = false;

		

		
		while(!done) {
			
			//Handle events on queue
			while (SDL_PollEvent(&e) != 0)
			{

				switch (e.type)
				{
				case ::SDL_QUIT:
					done = true;
					break;
				case ::SDL_KEYDOWN:
					break;
				case ::SDL_KEYUP:
					break;
				default:
					break;
				}
				
			}
			if (hndlFunct(::g_Context)) {
				SDL_RenderPresent(::g_Context.g_pRenderer.get());
			}
		}

	}
	
}