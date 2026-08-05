#!/usr/bin/env bash
DIR="$(cd "$(dirname "$0")" && pwd)"
if [ -f "$DIR/install.sh" ]; then
    exec "$DIR/install.sh" --uninstall
else
    echo "Installer not found at $DIR/install.sh"
    exit 1
fi
