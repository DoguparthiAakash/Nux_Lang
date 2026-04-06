#!/bin/bash
# Nux Test Runner
# Executes all test scenarios in the test/ directory

# Set colors
GREEN='\033[0;32m'
RED='\033[0;31m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Check for nux interpreter
NUX_BIN="/usr/local/bin/nux"
if [ ! -f "$NUX_BIN" ]; then
    # Fallback to local bin if development environment
    if [ -f "../bin/nux" ]; then
        NUX_BIN="../bin/nux"
    fi
fi

# Set NUX_LIB to local development version if available
if [ -d "nux/lib" ]; then
    export NUX_LIB="$(pwd)/nux/lib"
    echo "Using Local Libs: $NUX_LIB"
elif [ -d "../lib" ]; then
    export NUX_LIB="$(pwd)/../lib"
    echo "Using Local Libs: $NUX_LIB"
fi

echo -e "${CYAN}Running Nux Test Suite...${NC}"
echo "----------------------------------------"

TEST_DIR=$(dirname "$0")
COUNT=0
PASS=0
FAIL=0

run_test() {
    local test_file=$1
    local test_name=$(basename "$test_file")
    
    echo -n "Testing $test_name... "
    
    # Run nux and suppress output unless failed (or just check exit code)
    # Since we can't really execute nux here, this is a template script for the user
    # If we were really running it, we'd capture output.
    
    if $NUX_BIN run "$test_file" > /dev/null 2>&1; then
        echo -e "${GREEN}PASSED${NC}"
        PASS=$((PASS + 1))
    else
        echo -e "${RED}FAILED${NC}"
        FAIL=$((FAIL + 1))
        # Optional: Print error log
    fi
    COUNT=$((COUNT + 1))
}

# 1. Single Lib Tests
for t in "$TEST_DIR"/test_*_simple.nux; do
    [ -e "$t" ] || continue
    run_test "$t"
done

# 2. Multi Lib / Integrated Tests
for t in "$TEST_DIR"/test_gui_sql.nux \
         "$TEST_DIR"/test_gui_linked.nux \
         "$TEST_DIR"/test_complex_app.nux; do
    [ -e "$t" ] || continue
    run_test "$t"
done

echo "----------------------------------------"
echo -e "Tests Completed: $COUNT"
echo -e "Passed: ${GREEN}$PASS${NC}"
echo -e "Failed: ${RED}$FAIL${NC}"

if [ $FAIL -eq 0 ]; then
    exit 0
else
    exit 1
fi
