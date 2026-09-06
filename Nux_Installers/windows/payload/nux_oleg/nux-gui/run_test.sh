#!/bin/bash

# Nux GUI Library - Test Runner Script
# This script compiles and runs the architecture test

set -e  # Exit on error

echo "======================================"
echo "  Nux GUI Library - Test Runner"
echo "======================================"
echo ""

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Check if g++ is installed
if ! command -v g++ &> /dev/null; then
    echo -e "${RED}Error: g++ compiler not found${NC}"
    echo "Please install g++: sudo apt install g++"
    exit 1
fi

echo -e "${BLUE}[1/3] Compiling test...${NC}"
cd tests

# Compile the test
if g++ -std=c++17 -Wall -Wextra architecture_test.cpp -o arch_test 2>&1; then
    echo -e "${GREEN}✓ Compilation successful${NC}"
else
    echo -e "${RED}✗ Compilation failed${NC}"
    exit 1
fi

echo ""
echo -e "${BLUE}[2/3] Running test...${NC}"
echo "--------------------------------------"

# Run the test
if ./arch_test; then
    TEST_RESULT=$?
else
    TEST_RESULT=$?
fi

echo "--------------------------------------"
echo ""

# Check result
if [ $TEST_RESULT -eq 0 ]; then
    echo -e "${GREEN}[3/3] ✓ All tests passed!${NC}"
    echo ""
    echo "The Nux GUI library architecture is validated."
    echo "FFI interface, widgets, and rendering pipeline are working correctly."
else
    echo -e "${RED}[3/3] ✗ Tests failed${NC}"
    exit 1
fi

echo ""
echo "======================================"
echo "  Test Complete"
echo "======================================"
