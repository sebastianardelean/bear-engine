#include "core/context.hpp"
#include "core/engine.hpp"

#include "spdlog/sinks/basic_file_sink.h"




#undef main


auto HndlEngine(bear::Context &ctx) -> void {
    
   
           
        
        
    const std::vector< SDL_Vertex > verts =
    {
        { SDL_FPoint{ 400, 150 }, SDL_Color{ 255, 0, 0, 255 }, SDL_FPoint{ 0 }, },
        { SDL_FPoint{ 200, 450 }, SDL_Color{ 0, 0, 255, 255 }, SDL_FPoint{ 0 }, },
        { SDL_FPoint{ 600, 450 }, SDL_Color{ 0, 255, 0, 255 }, SDL_FPoint{ 0 }, },
    };

    SDL_SetRenderDrawColor(ctx.g_pRenderer.get(), 0, 0, 0, SDL_ALPHA_OPAQUE);
    SDL_RenderClear(ctx.g_pRenderer.get());
    SDL_RenderGeometry(ctx.g_pRenderer.get(), nullptr, verts.data(), verts.size(), nullptr, 0);
            
        
        
    
    

    
}


auto main(int argc, char **argv) -> int {
    
    std::string name = "Bear Engine";
    bear::EngineInitialize(name, 800, 600);




    bear::EngineRun(&HndlEngine);

    return 0;
}
