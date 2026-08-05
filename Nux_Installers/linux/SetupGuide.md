# Nux Setup Guide (Linux)

This guide explains how to install and update Nux on Linux.

## Installation

1. Open your terminal.
2. Run `./install.sh` for a generic installation.
3. (Optional) Run `./build_deb.sh` or `./build_rpm.sh` to create a native package for your package manager, then install it via `dpkg -i nux.deb` or `rpm -i nux.rpm`.

## Updating Nux

Run the included update script to fetch the latest versions without reinstalling manually.
```sh
./update.sh
```

## Uninstallation

If you need to remove Nux:
```sh
./uninstall.sh
```
