#!/bin/bash
# Test script to simulate git sparse-checkout behavior locally
# This doesn't actually hit GitHub but verifies syntax and commands

set -e

echo "Testing Linux repair script syntax..."
bash -n /home/aakash/Downloads/Nux_Lang/nux/nux_lang_linux_v1.0/repair.sh
echo "Linux syntax OK"

echo "Testing macOS repair script syntax..."
bash -n /home/aakash/Downloads/Nux_Lang/nux/nux_lang_macos_v1.0/repair.sh
echo "macOS syntax OK"

echo "Testing BSD repair script syntax..."
sh -n /home/aakash/Downloads/Nux_Lang/nux/nux_lang_bsd_v1.0/repair.sh
echo "BSD syntax OK"

echo "Verification complete."
