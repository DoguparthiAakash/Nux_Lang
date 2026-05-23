#!/bin/bash
set -e

echo "Syncing nux_dist..."

# 1. Build
echo "Building Release..."
cargo build --release --no-default-features

# 2. Update Binary
echo "Updating Binary..."
cp target/release/nux ../nux_dist/bin/

# 3. Update Libraries
# 3. Update Libraries
echo "Updating Libraries..."
# Create Dirs
mkdir -p ../nux_dist/lib/io
mkdir -p ../nux_dist/lib/util
mkdir -p ../nux_dist/lib/lang
mkdir -p ../nux_dist/lib/data

# Root / Flat (for Backward Compat + Direct Imports if handled by compiler differently)
cp -v *.nux ../nux_dist/lib/ 2>/dev/null || true

# Categorized
cp -v io.nux ../nux_dist/lib/io/console.nux
cp -v file.nux ../nux_dist/lib/io/
cp -v math.nux ../nux_dist/lib/util/
cp -v sys.nux ../nux_dist/lib/lang/
cp -v memory.nux ../nux_dist/lib/lang/
cp -v time.nux ../nux_dist/lib/lang/
cp -v testing.nux ../nux_dist/lib/lang/
cp -v log.nux ../nux_dist/lib/lang/
cp -v string.nux ../nux_dist/lib/data/
cp -v collections.nux ../nux_dist/lib/data/
cp -v util.nux ../nux_dist/lib/

cp -v nx.nux ../nux_dist/ 2>/dev/null || true

# 4. Update Examples
echo "Updating Examples..."
cp -v camera_demo.nux cv_test.nux snake.nux 3d_demo.nux ../nux_dist/examples/ 2>/dev/null || true

echo "nux_dist Synced Successfully!"
