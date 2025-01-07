#include "core/context.hpp"
#include "core/engine.hpp"

#include "spdlog/sinks/basic_file_sink.h"


#undef main


auto HndlEngine(bear::Context &ctx) -> bool {
    
    for (size_t i = 0; i < ctx.g_i32WinWidth; i++) {
        for (size_t j = 0; j < ctx.g_i32WinHeight; j++) {
            SDL_SetRenderDrawColor(ctx.g_pRenderer.get(), rand()%255, rand() % 255, rand() % 255, rand() % 255);
            SDL_RenderDrawPoint(ctx.g_pRenderer.get(), i, j);
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
