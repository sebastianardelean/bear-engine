#include "core/context.hpp"
#include "core/engine.hpp"

#include "spdlog/sinks/basic_file_sink.h"


#undef main


auto HndlEngine(bear::Context &ctx) -> bool {
    
    if (ctx.g_KeyboardEvent.has_value()) {
        if (ctx.g_KeyboardEvent->keysym.sym==SDLK_a) {
            
        }
        
        if (ctx.g_KeyboardEvent->keysym.sym == SDLK_t) {
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
        
    }
    else {
        for (size_t i = 0; i < ctx.g_i32WinWidth; i++) {
            for (size_t j = 0; j < ctx.g_i32WinHeight; j++) {
                SDL_SetRenderDrawColor(ctx.g_pRenderer.get(), rand() % 255, rand() % 255, rand() % 255, rand() % 255);
                SDL_RenderDrawPoint(ctx.g_pRenderer.get(), i, j);
            }
        }
    }

    return true;
}


auto main(int argc, char **argv) -> int {
    
    std::string name = "Bear Engine";
    bear::EngineInitialize(name, 800, 600);
    bear::EngineRun(&HndlEngine);

    return 0;
}
