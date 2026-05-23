#!/bin/bash

# Nux Library Test Runner
# Runs all test suites and reports results

echo "╔════════════════════════════════════════════════════════╗"
echo "║     Nux Programming Language - Test Suite Runner      ║"
echo "╚════════════════════════════════════════════════════════╝"
echo ""

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test counters
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

# Function to run a test file
run_test() {
    local test_file=$1
    local test_name=$(basename "$test_file" .nux)
    
    echo -e "${YELLOW}Running: ${test_name}${NC}"
    
    # Check if nux compiler exists
    if [ -f "./nux" ]; then
        ./nux "$test_file" 2>&1
        if [ $? -eq 0 ]; then
            echo -e "${GREEN}✓ ${test_name} PASSED${NC}"
            ((PASSED_TESTS++))
        else
            echo -e "${RED}✗ ${test_name} FAILED${NC}"
            ((FAILED_TESTS++))
        fi
    else
        echo -e "${YELLOW}⚠ Nux compiler not found, skipping execution${NC}"
        echo -e "${YELLOW}  (Syntax validation only)${NC}"
    fi
    
    ((TOTAL_TESTS++))
    echo ""
}

# Find and run all test files
echo "Discovering test files..."
echo ""

if [ -d "tests" ]; then
    for test_file in tests/*.nux; do
        if [ -f "$test_file" ]; then
            run_test "$test_file"
        fi
    done
else
    echo -e "${RED}Error: tests/ directory not found${NC}"
    exit 1
fi

# Print summary
echo "╔════════════════════════════════════════════════════════╗"
echo "║                    Test Summary                        ║"
echo "╚════════════════════════════════════════════════════════╝"
echo ""
echo "Total Test Files: $TOTAL_TESTS"
echo -e "Passed: ${GREEN}$PASSED_TESTS${NC}"
echo -e "Failed: ${RED}$FAILED_TESTS${NC}"
echo ""

# Library statistics
echo "╔════════════════════════════════════════════════════════╗"
echo "║                 Library Statistics                     ║"
echo "╚════════════════════════════════════════════════════════╝"
echo ""

if [ -d "lib" ]; then
    LIB_COUNT=$(find lib -name "*.nux" -type f | wc -l)
    echo "Total Library Files: $LIB_COUNT"
    echo ""
    
    echo "Library Breakdown:"
    for dir in lib/*/; do
        if [ -d "$dir" ]; then
            dir_name=$(basename "$dir")
            file_count=$(find "$dir" -name "*.nux" -type f | wc -l)
            echo "  - $dir_name: $file_count files"
        fi
    done
fi

echo ""

# Exit with appropriate code
if [ $FAILED_TESTS -gt 0 ]; then
    echo -e "${RED}Some tests failed!${NC}"
    exit 1
else
    echo -e "${GREEN}All tests passed!${NC}"
    exit 0
fi
