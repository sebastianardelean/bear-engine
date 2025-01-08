#include "core/safe_initialize.hpp"
#include "core/graphics_exceptions.hpp"
namespace bear::core {

    SDL_Window* sdl_safe_create_window(std::string title,
        int32_t w, int32_t h) {
        SDL_WindowFlags window_flags = (SDL_WindowFlags)(SDL_WINDOW_RESIZABLE | SDL_WINDOW_ALLOW_HIGHDPI);
        SDL_Window* hdlWindow = SDL_CreateWindow(title.c_str(), SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED, w, h, window_flags);
        if (hdlWindow == nullptr) {
            throw SdlCreateWindowException(std::string(SDL_GetError()));
        }
        return hdlWindow;
    }

    SDL_Renderer* sdl_safe_create_renderer(SDL_Window* window) {
        int rendererIndex = -1;
        SDL_Renderer* hdlRenderer = SDL_CreateRenderer(window, rendererIndex, SDL_RENDERER_ACCELERATED | SDL_RENDERER_PRESENTVSYNC);
        if (hdlRenderer == nullptr) {
            throw SdlRendererCreateException(std::string(SDL_GetError()));
        }
        return hdlRenderer;
    }

    SDL_Texture* sdl_safe_create_texture(SDL_Renderer* renderer, int32_t width, int32_t height) {

        SDL_Texture* hdlTexture = SDL_CreateTexture(renderer, SDL_PIXELFORMAT_RGBA32,
            SDL_TEXTUREACCESS_TARGET, width,
            height);
        if (hdlTexture == nullptr) {
            throw SdlTextureCreateException(std::string(SDL_GetError()));
        }
        return hdlTexture;
    }

    SDL_Texture* sdl_safe_create_texture(SDL_Renderer* renderer, SDL_Window* window) {
        int windowHeight = 0;
        int windowWidth = 0;
        SDL_GetWindowSize(window, &windowWidth, &windowHeight);
        SDL_Texture* hdlTexture = SDL_CreateTexture(renderer, SDL_PIXELFORMAT_RGBA32,
            SDL_TEXTUREACCESS_TARGET, windowWidth,
            windowHeight);
        if (hdlTexture == nullptr) {
            throw SdlTextureCreateException(std::string(SDL_GetError()));
        }
        return hdlTexture;
    }

    /* SDL_Surface* sdl_safe_create_surface(const std::int32_t surfaceWidth, const std::int32_t surfaceHeight) {

         SDL_Surface* hdlSurface = SDL_CreateRGBSurface(0, surfaceWidth, surfaceHeight, 32, 0, 0, 0, 0);
         if (hdlSurface == nullptr) {
             throw cubeexcept::SdlSurfaceCreateException(std::string(SDL_GetError()));
         }
         return hdlSurface;
     }

     SDL_Surface* sdl_safe_load_bmp(const std::string& filePath) {
         SDL_Surface* hdlSurface = SDL_LoadBMP(filePath.c_str());
         if (hdlSurface == nullptr) {
             throw cubeexcept::SdlLoadBmpException(std::string(SDL_GetError()));
         }
         return hdlSurface;
     }

     void sdl_ttf_safe_init() {
         if (TTF_Init() == -1) {
             throw cubeexcept::SdlInitTTFException(std::string(TTF_GetError()));
         }
     }

     TTF_Font* sdl_ttf_safe_open_font(const std::string& filePath, int ptsize) {
         TTF_Font* hdlFont = TTF_OpenFont(filePath.c_str(), ptsize);
         if (hdlFont == nullptr) {
             throw cubeexcept::SdlTTFOpenFontException(std::string(TTF_GetError()));
         }
         return hdlFont;
     }*/
};