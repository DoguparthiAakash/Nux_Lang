cmake_minimum_required(VERSION 3.15)
project(NuxGUI VERSION 1.0.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Options
option(BUILD_SHARED_LIBS "Build shared libraries" ON)
option(BUILD_EXAMPLES "Build example applications" ON)

# Dependencies
find_package(OpenGL REQUIRED)
find_package(glfw3 3.3 REQUIRED)
find_package(Freetype REQUIRED)

# GLAD (OpenGL loader)
add_library(glad STATIC
    ${CMAKE_CURRENT_SOURCE_DIR}/external/glad/src/glad.c
)
target_include_directories(glad PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}/external/glad/include
)

# NuxGUI Library
set(NUXGUI_SOURCES
    src/core/window.cpp
    src/core/renderer.cpp
    src/core/events.cpp
    src/widgets/widget.cpp
    src/widgets/button.cpp
    src/widgets/label.cpp
    src/widgets/panel.cpp
    src/bindings/nux_ffi.cpp
)

set(NUXGUI_HEADERS
    include/nux_gui/nux_gui.h
    include/nux_gui/window.h
    include/nux_gui/renderer.h
    include/nux_gui/events.h
    include/nux_gui/widget.h
    include/nux_gui/button.h
    include/nux_gui/label.h
    include/nux_gui/panel.h
)

add_library(nuxgui ${NUXGUI_SOURCES} ${NUXGUI_HEADERS})

target_include_directories(nuxgui
    PUBLIC
        $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
        $<INSTALL_INTERFACE:include>
    PRIVATE
        ${CMAKE_CURRENT_SOURCE_DIR}/src
)

target_link_libraries(nuxgui
    PUBLIC
        OpenGL::GL
        glfw
        Freetype::Freetype
        glad
)

# Compiler warnings
if(MSVC)
    target_compile_options(nuxgui PRIVATE /W4)
else()
    target_compile_options(nuxgui PRIVATE -Wall -Wextra -Wpedantic)
endif()

# Examples
if(BUILD_EXAMPLES)
    add_subdirectory(examples)
endif()

# Installation
install(TARGETS nuxgui
    EXPORT NuxGUITargets
    LIBRARY DESTINATION lib
    ARCHIVE DESTINATION lib
    RUNTIME DESTINATION bin
    INCLUDES DESTINATION include
)

install(DIRECTORY include/nux_gui
    DESTINATION include
)

install(FILES bindings/nux/gui.nux
    DESTINATION share/nux/lib/external
)
