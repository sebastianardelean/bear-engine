set(BEAR_INCLUDE "../include")


set(SOURCE_FILES main.cpp
    core/engine.cpp
    log/logger.cpp
    core/safe_initialize.cpp
    core/context.cpp
    datatypes/texture.cpp
)



set(SDL_ZIP_FILE "SDL2-2.30.11")
set(SDL_ZIP_FILE_NAME "SDL2-devel-2.30.11-VC") #because the name of the archive is different than the name of the output folder
set(ZIP_FILE_SDL "../external/${SDL_ZIP_FILE_NAME}.zip")


set(SDL_IMAGE_ZIP_FILE "SDL2_image-2.8.4")
set(SDL_IMAGE_ZIP_FILE_NAME "SDL2_image-devel-2.8.4-VC") #because the name of the archive is different than the name of the output folder
set(ZIP_FILE_SDL_IMAGE "../external/${SDL_IMAGE_ZIP_FILE_NAME}.zip")


set(SPDLOG_ZIP_FILE "spdlog-1.15.0")
set(ZIP_FILE_SPDLOG "../external/${SPDLOG_ZIP_FILE}.zip")


set(IMGUI_ZIP_FILE "imgui-1.91.6")
set(ZIP_FILE_IMGUI "../external/${IMGUI_ZIP_FILE}.zip")



set(DEST_DIR "../libs/")
file(MAKE_DIRECTORY ${DEST_DIR})



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


execute_process(
    COMMAND unzip -o ${ZIP_FILE_SDL_IMAGE} -d ${DEST_DIR}
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


# Use execute_process to call unzip
execute_process(
    COMMAND unzip -o ${ZIP_FILE_IMGUI} -d ${DEST_DIR}
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
    RESULT_VARIABLE UNZIP_RESULT
    OUTPUT_VARIABLE UNZIP_OUTPUT
    ERROR_VARIABLE UNZIP_ERROR
)

if (NOT UNZIP_RESULT EQUAL 0)
    message(FATAL_ERROR "Unzip failed: ${UNZIP_ERROR}")
endif()




set(CMAKE_PREFIX_PATH "${CMAKE_SOURCE_DIR}/libs/${SDL_ZIP_FILE}/;../libs/${SPDLOG_ZIP_FILE}/;${CMAKE_SOURCE_DIR}/libs/${SDL_IMAGE_ZIP_FILE}/")

find_package(SDL2 REQUIRED HINTS ../libs/${SDL_ZIP_FILE}/)
find_package(SDL2_image REQUIRED HINTS ../libs/${SDL_IMAGE_ZIP_FILE}/)

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
include_directories(../libs/${IMGUI_ZIP_FILE})
include_directories(../libs/${IMGUI_ZIP_FILE}/backends)

set(SOURCES_IMGUI  ../libs/${IMGUI_ZIP_FILE}/imgui.cpp
    ../libs/${IMGUI_ZIP_FILE}/imgui_draw.cpp
    ../libs/${IMGUI_ZIP_FILE}/imgui_tables.cpp
    ../libs/${IMGUI_ZIP_FILE}/imgui_demo.cpp
    ../libs/${IMGUI_ZIP_FILE}/imgui_widgets.cpp
    ../libs/${IMGUI_ZIP_FILE}/backends/imgui_impl_sdl2.cpp
    ../libs/${IMGUI_ZIP_FILE}/backends/imgui_impl_sdlrenderer2.cpp)

add_executable(${PROJECT_NAME} ${SOURCE_FILES} ${SOURCES_IMGUI} ${INCLUDE_FILES} )

# Define IMGUI_DEFINE_MATH_OPERATORS
target_compile_definitions(${PROJECT_NAME} PRIVATE IMGUI_DEFINE_MATH_OPERATORS)

target_link_libraries(
    ${PROJECT_NAME} PRIVATE
    project_options
    SDL2::SDL2
    SDL2::SDL2main
    SDL2_image::SDL2_image

)




