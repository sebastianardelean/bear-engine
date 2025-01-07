

set(BEAR_INCLUDE "../include")

set(SOURCE_FILES main.cpp
    core/engine.cpp
    log/logger.cpp
    core/safe_initialize.cpp
    
)



find_package(SDL2 REQUIRED)


# Find spdlog
find_path(SPDLOG_INCLUDE_DIR
          NAMES spdlog/spdlog.h
          HINTS ${CMAKE_SOURCE_DIR}/../libs/spdlog/include)

# If not found, throw an error
if (NOT SPDLOG_INCLUDE_DIR)
    message(FATAL_ERROR "spdlog headers not found in specified path.")
endif()

# Add the include directory
include_directories(${SPDLOG_INCLUDE_DIR})
include_directories(${BEAR_INCLUDE})

add_executable(${PROJECT_NAME} ${SOURCE_FILES} ${INCLUDE_FILES} "core/context.cpp")

target_link_libraries(
    ${PROJECT_NAME} PRIVATE
    project_options
    SDL2::SDL2
    SDL2::SDL2main
    
)




