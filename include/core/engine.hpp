#pragma once

#include "core/context.hpp"




namespace bear {


    
    auto EngineRun(std::function<bool(Context& ctx)> hndlFunct) -> void;
    auto EngineInitialize(std::string& name, int32_t winWidth, int32_t winHeight) -> void;
	
	

	
}