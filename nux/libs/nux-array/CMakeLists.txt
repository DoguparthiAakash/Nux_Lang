cmake_minimum_required(VERSION 3.15)
project(NuxArray VERSION 1.0.0 LANGUAGES CXX)

# Sub-modules
add_subdirectory(core)
add_subdirectory(linalg)
add_subdirectory(fft)
add_subdirectory(random)

# Optional GPU support
option(ENABLE_GPU "Enable GPU acceleration" OFF)
if(ENABLE_GPU)
    add_subdirectory(gpu)
endif()

# Main library combining all modules
add_library(nux_array INTERFACE)
target_link_libraries(nux_array INTERFACE
    nux_array_core
    nux_array_linalg
    nux_array_fft
    nux_array_random
)

if(ENABLE_GPU)
    target_link_libraries(nux_array INTERFACE nux_array_gpu)
endif()

message(STATUS "NuxArray modules:")
message(STATUS "  Core: ON")
message(STATUS "  Linear Algebra: ON")
message(STATUS "  FFT: ON")
message(STATUS "  Random: ON")
message(STATUS "  GPU: ${ENABLE_GPU}")
