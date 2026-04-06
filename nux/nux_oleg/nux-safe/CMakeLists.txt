cmake_minimum_required(VERSION 3.15)
project(NuxSafe VERSION 1.0.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Include directories
include_directories(${PROJECT_SOURCE_DIR}/include)

# Header-only library
add_library(nux_safe INTERFACE)

target_include_directories(nux_safe INTERFACE
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
    $<INSTALL_INTERFACE:include>
)

# Tests
add_executable(safety_test tests/safety_test.cpp)
target_link_libraries(safety_test nux_safe pthread)

# Installation
install(DIRECTORY include/nux_safe
    DESTINATION include
)

message(STATUS "NuxSafe Library (Header-Only)")
message(STATUS "  Thread safety enabled")
message(STATUS "  Memory bounds checking enabled")
message(STATUS "  Input validation enabled")
