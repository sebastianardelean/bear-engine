#include "core/safe_initialize.hpp"
#include "core/auto_release.hpp"
#include "datatypes/texture.hpp"

namespace bear {
	Texture::Texture():g_i32Width(0),g_i32Height(0)
	{
		
	}

	auto Texture::LoadFromFile(std::string& path, SDL_Renderer *renderer) -> void {
		

		//The final texture
	
		core::AutoRelease<SDL_Surface*, nullptr> loadedSurface={
			core::sdl_safe_image_load(path),
			&::SDL_FreeSurface
		};
		
		SDL_SetColorKey(loadedSurface, SDL_TRUE, SDL_MapRGB(loadedSurface->format, 0, 0xFF, 0xFF));
	
		g_pTexture = {
			core::sdl_safe_create_texture_from_surface(renderer, loadedSurface.get()),
			&::SDL_DestroyTexture
		};

		g_i32Width = loadedSurface.get()->w;
		g_i32Height = loadedSurface.get()->h;

		

	}

	auto Texture::ApplyTexture(int32_t x, int32_t y, SDL_Rect* clip, SDL_Renderer* renderer) -> void
	{
		SDL_Rect renderQuad = { x, y, g_i32Width, g_i32Height };

		//Set clip rendering dimensions
		if (clip != nullptr)
		{
			renderQuad.w = clip->w;
			renderQuad.h = clip->h;
		}

		//Render to screen
		SDL_RenderCopy(renderer, g_pTexture.get(), clip, &renderQuad);

	}
}