# Minimum CMake version required
cmake_minimum_required(VERSION 3.20)

# Project name and version
project(bear VERSION 1.0)
set(PROJECT_NAME bear)




# Set the C standard to C99 (or C11 if preferred)
set(CMAKE_C_STANDARD 99)
set(CMAKE_C_STANDARD_REQUIRED True)


set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

add_library(project_options INTERFACE)


if (CMAKE_CXX_COMPILER_ID STREQUAL "Clang")
  # using Clang
elseif (CMAKE_CXX_COMPILER_ID STREQUAL "GNU")
  target_compile_options(project_options INTERFACE -Wall -Wextra -O2)
  target_compile_options(project_options INTERFACE -Wall -Wextra -g)
elseif (CMAKE_CXX_COMPILER_ID STREQUAL "Intel")
  # using Intel C++
elseif (CMAKE_CXX_COMPILER_ID STREQUAL "MSVC")
  target_compile_options(project_options INTERFACE /utf-8)
endif()


option(ENABLE_PCH "Enable Precompiled Headers" ON)

if(ENABLE_PCH)
  target_precompile_headers(
    project_options
    INTERFACE
    <atomic>
    <iostream>
   # <print>
    <memory>
    <vector>
    <exception>
    <cstdint>
    <cassert>
    <array>
    <string>
    <optional>
		

    <spdlog/spdlog.h>
    <SDL.h>
    <imgui.h>
    <imgui_impl_sdlrenderer2.h>
    <imgui_impl_sdl2.h>
    )
endif()


add_subdirectory("src")