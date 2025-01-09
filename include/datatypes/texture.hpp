#pragma once
#include "core/auto_release.hpp"
namespace bear {
    class Texture
    {
    public:
        //Initializes variables
        Texture();
        Texture(const Texture&) = default;
        auto operator = (const Texture&)->Texture & = default;

        Texture(Texture&&) = default;
        auto operator = (Texture&&)->Texture & = default;

        
        ~Texture() = default;

        auto inline GetWidth() -> int32_t { return g_i32Width; }
        auto inline GetHeight() -> int32_t { return g_i32Height; }

        auto LoadFromFile(std::string& path, SDL_Renderer* renderer) -> void;

        auto ApplyTexture(int32_t x, int32_t y, SDL_Rect* clip, SDL_Renderer* renderer) -> void;
    private:


        core::AutoRelease<SDL_Texture*, nullptr> g_pTexture;

        int32_t g_i32Width;
        int32_t g_i32Height;
    };
};