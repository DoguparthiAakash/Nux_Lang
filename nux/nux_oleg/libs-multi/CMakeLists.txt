# Multi-Language Build System

cmake_minimum_required(VERSION 3.20)
project(NuxMultiLanguage VERSION 1.0.0)

# Enable multiple languages
enable_language(C CXX ASM)

# Find external language compilers
find_program(RUSTC rustc)
find_program(ZIGC zig)
find_program(GOC go)

# Assembly files
if(CMAKE_SYSTEM_PROCESSOR MATCHES "x86_64|AMD64")
    set(ASM_DIALECT "NASM")
    enable_language(ASM_NASM)
    
    add_library(nux_asm_simd STATIC
        asm/simd/matmul.asm
        asm/simd/vector_ops.asm
    )
endif()

# Rust libraries
if(RUSTC)
    add_custom_target(nux_rust_blockchain ALL
        COMMAND cargo build --release
        WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/rust/blockchain
        COMMENT "Building Rust blockchain library"
    )
    
    add_library(nux_blockchain SHARED IMPORTED)
    set_target_properties(nux_blockchain PROPERTIES
        IMPORTED_LOCATION ${CMAKE_CURRENT_SOURCE_DIR}/rust/blockchain/target/release/libnux_blockchain.so
    )
    add_dependencies(nux_blockchain nux_rust_blockchain)
endif()

# Zig libraries
if(ZIGC)
    add_custom_target(nux_zig_allocators ALL
        COMMAND zig build-lib -dynamic arena.zig
        WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/zig/allocators
        COMMENT "Building Zig allocator library"
    )
    
    add_library(nux_allocators SHARED IMPORTED)
    set_target_properties(nux_allocators PROPERTIES
        IMPORTED_LOCATION ${CMAKE_CURRENT_SOURCE_DIR}/zig/allocators/libarena.so
    )
    add_dependencies(nux_allocators nux_zig_allocators)
endif()

# Go libraries
if(GOC)
    add_custom_target(nux_go_distributed ALL
        COMMAND go build -buildmode=c-shared -o libnux_distributed.so
        WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/go/distributed
        COMMENT "Building Go distributed library"
    )
    
    add_library(nux_distributed SHARED IMPORTED)
    set_target_properties(nux_distributed PROPERTIES
        IMPORTED_LOCATION ${CMAKE_CURRENT_SOURCE_DIR}/go/distributed/libnux_distributed.so
    )
    add_dependencies(nux_distributed nux_go_distributed)
endif()

# C++ libraries (existing)
add_subdirectory(cpp/ai)
add_subdirectory(cpp/vision)
add_subdirectory(cpp/stats)

# C libraries (FFI bindings)
add_library(nux_ffi SHARED
    c/ffi/bindings.c
    c/ffi/conversions.c
)

# Link everything together
add_library(nux_complete INTERFACE)
target_link_libraries(nux_complete INTERFACE
    nux_asm_simd
    nux_blockchain
    nux_allocators
    nux_distributed
    nux_ffi
)

message(STATUS "Multi-Language Build Configuration:")
message(STATUS "  Assembly (SIMD): ${ASM_DIALECT}")
message(STATUS "  Rust: ${RUSTC}")
message(STATUS "  Zig: ${ZIGC}")
message(STATUS "  Go: ${GOC}")
message(STATUS "  C/C++: ON")
