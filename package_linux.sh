#!/bin/bash
cd /mnt/e/nux/Nux_Lang
src='/mnt/e/nux/Nux_Lang/nux/nux_oleg/nux_dist/target/release'
lib='/mnt/e/nux/Nux_Lang/lib'
dest='/mnt/e/nux/Nux_Lang/Nux_Installers/linux/payload'

rm -rf "$dest"
mkdir -p "$dest"
cp "$src/nux" "$dest/"
cp "$src/bonfort" "$dest/"
cp "$src/nuxpm" "$dest/"
cp "$src/nuxvm" "$dest/"
cp -r "$lib" "$dest/lib"
cd "$dest/.."
tar -czf payload.tar.gz -C payload .
