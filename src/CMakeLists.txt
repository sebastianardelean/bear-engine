

set(BEAR_INCLUDE "../include")

set(SOURCE_FILES main.cpp
    core/engine.cpp
    log/logger.cpp
    core/safe_initialize.cpp
    
)

set(SDL_ZIP_FILE "SDL2-devel-2.30.11-VC")
set(SPDLOG_ZIP_FILE "spdlog-1.15.0")

# Set the path to the zip file
set(ZIP_FILE_SDL "../libs/${SDL_ZIP_FILE}.zip")
set(ZIP_FILE_SPDLOG "../libs/${SPDLOG_ZIP_FILE}.zip")
set(DEST_DIR "../libs/")


# Use execute_process to call unzip
execute_process(
    COMMAND unzip -o ${ZIP_FILE_SDL} -d ${DEST_DIR}
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
    RESULT_VARIABLE UNZIP_RESULT
    OUTPUT_VARIABLE UNZIP_OUTPUT
    ERROR_VARIABLE UNZIP_ERROR
)

if (NOT UNZIP_RESULT EQUAL 0)
    message(FATAL_ERROR "Unzip failed: ${UNZIP_ERROR}")
endif()


# Use execute_process to call unzip
execute_process(
    COMMAND unzip -o ${ZIP_FILE_SPDLOG} -d ${DEST_DIR}
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
    RESULT_VARIABLE UNZIP_RESULT
    OUTPUT_VARIABLE UNZIP_OUTPUT
    ERROR_VARIABLE UNZIP_ERROR
)

if (NOT UNZIP_RESULT EQUAL 0)
    message(FATAL_ERROR "Unzip failed: ${UNZIP_ERROR}")
endif()

find_package(SDL2 REQUIRED)


# Find spdlog
find_path(SPDLOG_INCLUDE_DIR
          NAMES spdlog/spdlog.h
          HINTS ../libs/${SPDLOG_ZIP_FILE}/include)

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




