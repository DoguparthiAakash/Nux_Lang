cmake_minimum_required(VERSION 3.15)
project(NuxAI VERSION 1.0.0 LANGUAGES C CXX ASM)

set(CMAKE_C_STANDARD 11)
set(CMAKE_CXX_STANDARD 17)

# Core tensor library (C)
add_library(nux_tensor STATIC
    core/tensor.c
)

target_include_directories(nux_tensor PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}/core
)

# SIMD kernels (Assembly)
# Support both GAS (.S) and NASM (.asm)
if(CMAKE_SYSTEM_PROCESSOR MATCHES "x86_64|AMD64|aarch64|arm64")
    # Prefer GAS (.S) for better portability
    add_library(nux_kernels STATIC
        kernels/matmul.S
    )
    
    # Enable AVX2 for x86-64
    if(CMAKE_SYSTEM_PROCESSOR MATCHES "x86_64|AMD64")
        target_compile_options(nux_kernels PRIVATE -mavx2 -mfma)
    endif()
    
    # Alternative: NASM version (if preferred)
    # enable_language(ASM_NASM)
    # add_library(nux_kernels STATIC kernels/matmul_avx2.asm)
    # set_target_properties(nux_kernels PROPERTIES NASM_OBJ_FORMAT elf64)
endif()

# Neural network library (C++)
add_library(nux_nn STATIC
    nn/layer.cpp
    nn/model.cpp
)

target_link_libraries(nux_nn PUBLIC nux_tensor)
target_include_directories(nux_nn PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}
)

# Rust training library
find_program(CARGO cargo)
if(CARGO)
    add_custom_target(nux_training ALL
        COMMAND cargo build --release
        WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/training
        COMMENT "Building Rust training library"
    )
    
    # Determine library extension
    if(WIN32)
        set(RUST_LIB_EXT "dll")
    elseif(APPLE)
        set(RUST_LIB_EXT "dylib")
    else()
        set(RUST_LIB_EXT "so")
    endif()
    
    add_library(nux_training_lib SHARED IMPORTED)
    set_target_properties(nux_training_lib PROPERTIES
        IMPORTED_LOCATION ${CMAKE_CURRENT_SOURCE_DIR}/training/target/release/libnux_training.${RUST_LIB_EXT}
    )
    add_dependencies(nux_training_lib nux_training)
endif()

# Complete AI library
add_library(nux_ai INTERFACE)
target_link_libraries(nux_ai INTERFACE
    nux_tensor
    nux_nn
)

if(TARGET nux_kernels)
    target_link_libraries(nux_ai INTERFACE nux_kernels)
endif()

if(TARGET nux_training_lib)
    target_link_libraries(nux_ai INTERFACE nux_training_lib)
endif()

# Example programs
add_executable(tensor_test examples/tensor_test.c)
target_link_libraries(tensor_test nux_tensor)

add_executable(nn_test examples/nn_test.cpp)
target_link_libraries(nn_test nux_nn)

# Installation
install(TARGETS nux_tensor nux_nn
    ARCHIVE DESTINATION lib
    LIBRARY DESTINATION lib
)

install(DIRECTORY core/ nn/
    DESTINATION include/nux_ai
    FILES_MATCHING PATTERN "*.h"
)

message(STATUS "Nux AI Configuration:")
message(STATUS "  Architecture: ${CMAKE_SYSTEM_PROCESSOR}")
message(STATUS "  C Tensor: ON")
message(STATUS "  C++ Neural Network: ON")
message(STATUS "  Assembly Format: GAS (.S)")
if(CMAKE_SYSTEM_PROCESSOR MATCHES "x86_64|AMD64")
    message(STATUS "  SIMD: AVX2 + FMA")
elseif(CMAKE_SYSTEM_PROCESSOR MATCHES "aarch64|arm64")
    message(STATUS "  SIMD: NEON")
endif()
message(STATUS "  Rust Training: ${CARGO}")
