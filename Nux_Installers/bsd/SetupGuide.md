# Nux Setup Guide (BSD)

This guide explains how to install and update Nux on BSD.

## Installation

1. Open your terminal.
2. Run `./install.sh` for a generic installation.
3. (Optional) Run `./build_pkg.sh` to generate a native FreeBSD package, then install it using `pkg install`.

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
