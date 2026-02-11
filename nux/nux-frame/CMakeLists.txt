cmake_minimum_required(VERSION 3.15)
project(NuxFrame VERSION 1.0.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Include directories
include_directories(${PROJECT_SOURCE_DIR}/include)

# Source files
set(SOURCES
    src/dataframe.cpp
)

# Create library
add_library(nux_frame ${SOURCES})

target_include_directories(nux_frame PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
    $<INSTALL_INTERFACE:include>
)

# Compiler flags
if(MSVC)
    target_compile_options(nux_frame PRIVATE /W4)
else()
    target_compile_options(nux_frame PRIVATE -Wall -Wextra -O3)
endif()

# Tests
add_executable(dataframe_test tests/dataframe_test.cpp)
target_link_libraries(dataframe_test nux_frame)

# Installation
install(TARGETS nux_frame
    LIBRARY DESTINATION lib
    ARCHIVE DESTINATION lib
)

install(DIRECTORY include/nux_frame
    DESTINATION include
)
