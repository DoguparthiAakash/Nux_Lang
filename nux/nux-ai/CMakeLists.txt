cmake_minimum_required(VERSION 3.15)
project(NuxAI VERSION 1.0.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Options
option(BUILD_SHARED_LIBS "Build shared libraries" ON)
option(BUILD_TESTS "Build tests" ON)

# Include directories
include_directories(${PROJECT_SOURCE_DIR}/include)

# Source files
set(CORE_SOURCES
    src/core/tensor.cpp
)

set(NN_SOURCES
    src/nn/layer.cpp
    src/nn/linear.cpp
    src/nn/activation.cpp
)

set(OPTIM_SOURCES
    src/optim/optimizer.cpp
    src/optim/sgd.cpp
)

set(ALL_SOURCES
    ${CORE_SOURCES}
    ${NN_SOURCES}
    ${OPTIM_SOURCES}
)

# Create library
add_library(nux_ai ${ALL_SOURCES})

target_include_directories(nux_ai PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
    $<INSTALL_INTERFACE:include>
)

# Compiler flags
if(MSVC)
    target_compile_options(nux_ai PRIVATE /W4)
else()
    target_compile_options(nux_ai PRIVATE -Wall -Wextra -O3)
endif()

# Tests
if(BUILD_TESTS)
    add_executable(linear_regression_test tests/linear_regression_test.cpp)
    target_link_libraries(linear_regression_test nux_ai)
endif()

# Installation
install(TARGETS nux_ai
    LIBRARY DESTINATION lib
    ARCHIVE DESTINATION lib
    RUNTIME DESTINATION bin
)

install(DIRECTORY include/nux_ai
    DESTINATION include
)

# Print configuration
message(STATUS "Nux AI Library Configuration:")
message(STATUS "  Version: ${PROJECT_VERSION}")
message(STATUS "  Build type: ${CMAKE_BUILD_TYPE}")
message(STATUS "  Shared libs: ${BUILD_SHARED_LIBS}")
message(STATUS "  Build tests: ${BUILD_TESTS}")
