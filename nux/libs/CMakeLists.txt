cmake_minimum_required(VERSION 3.15)
project(NuxLibraries VERSION 1.0.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Build options
option(BUILD_ARRAY "Build NuxArray library" ON)
option(BUILD_FRAME "Build NuxFrame library" ON)
option(BUILD_LEARN "Build NuxLearn library" ON)
option(BUILD_AI "Build NuxAI library" ON)
option(BUILD_VISION "Build NuxVision library" ON)
option(BUILD_NLP "Build NuxNLP library" ON)
option(BUILD_STATS "Build NuxStats library" ON)
option(BUILD_PLOT "Build NuxPlot library" ON)
option(BUILD_GUI "Build NuxGUI library" ON)
option(BUILD_DISTRIBUTED "Build NuxDistributed library" ON)
option(BUILD_QUANTUM "Build NuxQuantum library" ON)
option(BUILD_BLOCKCHAIN "Build NuxBlockchain library" ON)
option(BUILD_CRYPTO "Build NuxCrypto library" ON)
option(BUILD_SAFE "Build NuxSafe library" ON)

# Sub-libraries
if(BUILD_ARRAY)
    add_subdirectory(nux-array)
endif()

if(BUILD_FRAME)
    add_subdirectory(nux-frame)
endif()

if(BUILD_LEARN)
    add_subdirectory(nux-learn)
endif()

if(BUILD_AI)
    add_subdirectory(nux-ai)
endif()

if(BUILD_VISION)
    add_subdirectory(nux-vision)
endif()

if(BUILD_NLP)
    add_subdirectory(nux-nlp)
endif()

if(BUILD_STATS)
    add_subdirectory(nux-stats)
endif()

if(BUILD_PLOT)
    add_subdirectory(nux-plot)
endif()

if(BUILD_GUI)
    add_subdirectory(nux-gui)
endif()

if(BUILD_DISTRIBUTED)
    add_subdirectory(nux-distributed)
endif()

if(BUILD_QUANTUM)
    add_subdirectory(nux-quantum)
endif()

if(BUILD_BLOCKCHAIN)
    add_subdirectory(nux-blockchain)
endif()

if(BUILD_CRYPTO)
    add_subdirectory(nux-crypto)
endif()

if(BUILD_SAFE)
    add_subdirectory(nux-safe)
endif()

message(STATUS "Nux Libraries Configuration:")
message(STATUS "  NuxArray: ${BUILD_ARRAY}")
message(STATUS "  NuxFrame: ${BUILD_FRAME}")
message(STATUS "  NuxLearn: ${BUILD_LEARN}")
message(STATUS "  NuxAI: ${BUILD_AI}")
message(STATUS "  NuxVision: ${BUILD_VISION}")
message(STATUS "  NuxNLP: ${BUILD_NLP}")
message(STATUS "  NuxStats: ${BUILD_STATS}")
message(STATUS "  NuxPlot: ${BUILD_PLOT}")
message(STATUS "  NuxGUI: ${BUILD_GUI}")
message(STATUS "  NuxDistributed: ${BUILD_DISTRIBUTED}")
message(STATUS "  NuxQuantum: ${BUILD_QUANTUM}")
message(STATUS "  NuxBlockchain: ${BUILD_BLOCKCHAIN}")
message(STATUS "  NuxCrypto: ${BUILD_CRYPTO}")
message(STATUS "  NuxSafe: ${BUILD_SAFE}")
