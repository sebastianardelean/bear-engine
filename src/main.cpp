#include "core/context.hpp"
#include "core/engine.hpp"
#include "datatypes/texture.hpp"
#include "spdlog/sinks/basic_file_sink.h"




#undef main

auto LoadTexture() -> void {

}

auto HndlEngine(bear::Context &ctx) -> void {
    
    static int frame = 0;
    const int WALKING_ANIMATION_FRAMES = 4;
    SDL_Rect spriteClips[WALKING_ANIMATION_FRAMES];

    
    
    bear::Texture  spriteTexture;

    std::string assets_bear = "assets/bear.png";

    spriteClips[0].x = 0;
    spriteClips[0].y = 0;
    spriteClips[0].w = 512;
    spriteClips[0].h = 512;

    spriteClips[1].x = 512;
    spriteClips[1].y = 0;
    spriteClips[1].w = 512;
    spriteClips[1].h = 512;

    spriteClips[2].x = 0;
    spriteClips[2].y = 512;
    spriteClips[2].w = 512;
    spriteClips[2].h = 512;

    spriteClips[3].x = 512;
    spriteClips[3].y = 512;
    spriteClips[3].w = 512;
    spriteClips[3].h = 512;


    spriteTexture.LoadFromFile(assets_bear,ctx.g_pRenderer.get());

    SDL_Rect* currentClip = &spriteClips[frame % 4];
  
    spriteTexture.ApplyTexture(
        (ctx.g_i32WinWidth - currentClip->w) / 2, 
        (ctx.g_i32WinHeight - currentClip->h) / 2, 
        currentClip,
        ctx.g_pRenderer.get());

    SDL_Delay(100);
        
        
    /*const std::vector< SDL_Vertex > verts =
    {
        { SDL_FPoint{ 400, 150 }, SDL_Color{ 255, 0, 0, 255 }, SDL_FPoint{ 0 }, },
        { SDL_FPoint{ 200, 450 }, SDL_Color{ 0, 0, 255, 255 }, SDL_FPoint{ 0 }, },
        { SDL_FPoint{ 600, 450 }, SDL_Color{ 0, 255, 0, 255 }, SDL_FPoint{ 0 }, },
    };

    SDL_SetRenderDrawColor(ctx.g_pRenderer.get(), 0, 0, 0, SDL_ALPHA_OPAQUE);
    SDL_RenderClear(ctx.g_pRenderer.get());
    SDL_RenderGeometry(ctx.g_pRenderer.get(), nullptr, verts.data(), verts.size(), nullptr, 0);
            
        
    */    
    
    frame++;

    
}


auto main(int argc, char **argv) -> int {
    
    std::string name = "Bear Engine";
    bear::EngineInitialize(name, 800, 600);




    bear::EngineRun(&HndlEngine);

    return 0;
}
