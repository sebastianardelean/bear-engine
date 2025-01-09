#pragma once
namespace bear::core {


    SDL_Window* sdl_safe_create_window(std::string title,
        int32_t w, int32_t h);

    SDL_Renderer* sdl_safe_create_renderer(SDL_Window* window);

    SDL_Texture* sdl_safe_create_texture(SDL_Renderer* renderer, int32_t width, int32_t height);
    SDL_Texture* sdl_safe_create_texture(SDL_Renderer* renderer, SDL_Window* window);

    SDL_Surface* sdl_safe_image_load(std::string& path);

    SDL_Texture* sdl_safe_create_texture_from_surface(SDL_Renderer* renderer, SDL_Surface *surface);

    /*  SDL_Surface* sdl_safe_create_surface(const std::int32_t surfaceWidth, const std::int32_t surfaceHeight);

      SDL_Surface* sdl_safe_load_bmp(const std::string& filePath);

      void sdl_ttf_safe_init();

      TTF_Font* sdl_ttf_safe_open_font(const std::string& filePath, int ptsize);*/
};