#!/usr/bin/env bash
# nux runner — auto-selects ovm.nxb (compiled) or boot.py (bootstrap)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
OVM_NXB="${SCRIPT_DIR}/lib/ovm/ovm.nxb"
BOOTSTRAP="${SCRIPT_DIR}/nux_oleg/bootstrap/boot.py"
if [ $# -eq 0 ]; then echo "Usage: nux <file.nux|file.nxb>"; exit 1; fi
if [ "$1" = "compile" ] || [ "$1" = "build" ]; then
    shift; INPUT="$1"; OUTPUT="${2:-${INPUT%.nux}.nxb}"
    if command -v zstd &>/dev/null; then
        zstd -19 -q "$INPUT" -o "$OUTPUT"
        BEFORE=$(wc -c < "$INPUT"); AFTER=$(wc -c < "$OUTPUT"); PCT=$(( (BEFORE-AFTER)*100/BEFORE ))
        echo "nux: $INPUT (${BEFORE}B) -> $OUTPUT (${AFTER}B) -${PCT}%"
    else gzip -9 -c "$INPUT" > "$OUTPUT"; fi; exit 0
fi
FILE="$1"
if [ ! -f "$FILE" ]; then echo "nux: file not found: $FILE"; exit 1; fi
if [ -f "$OVM_NXB" ]; then exec "$OVM_NXB" "$FILE"; fi
if [ -f "$BOOTSTRAP" ]; then exec python3 "$BOOTSTRAP" "$FILE"; fi
echo "nux: no runtime found. Run: bonfort build lib/ovm/ovm.nux"; exit 1
