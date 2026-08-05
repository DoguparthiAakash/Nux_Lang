import sys, re

def process_file(path):
    with open(path, 'r', encoding='utf-8') as f:
        content = f.read()

    # 1. Update variables
    content = re.sub(
        r'INSTALL_DIR="([^"]+)"',
        r'INSTALL_DIR_BASE="\1"\nINSTALL_DIR="$INSTALL_DIR_BASE/v$VERSION"\nCURRENT_LINK="$INSTALL_DIR_BASE/current"',
        content
    )
    
    # 2. Update do_install link creation
    content = re.sub(
        r'mkdir -p "\$\(dirname "\$BIN_LINK"\)"\n\s*ln -sf "\$target_dir/nux" "\$BIN_LINK"',
        r'ln -snf "$target_dir" "$CURRENT_LINK"\n\n        nux_print "Linking" "$CYAN" "/usr/local/bin/nux"\n        mkdir -p "$(dirname "$BIN_LINK")"\n        ln -snf "$CURRENT_LINK/nux" "$BIN_LINK"',
        content
    )

    # 3. Update environment profile
    content = re.sub(
        r'export NUX_HOME="\$target_dir"\nexport NUX_LIB_PATH="\$target_dir/lib"',
        r'export NUX_HOME="$CURRENT_LINK"\nexport NUX_LIB_PATH="$CURRENT_LINK/lib"\nexport PATH="$PATH:$NUX_HOME"',
        content
    )
    
    # 4. Update do_uninstall to use version manager logic
    new_uninstall = """do_uninstall() {
    echo ""
    if [ ! -d "$INSTALL_DIR_BASE" ]; then
        nux_error "Nux is not installed."
        exit 1
    fi

    local versions=()
    for d in "$INSTALL_DIR_BASE"/*; do
        if [ -d "$d" ] && [ "$(basename "$d")" != "current" ]; then
            versions+=("$(basename "$d")")
        fi
    done

    if [ ${#versions[@]} -eq 0 ]; then
        nux_error "No Nux versions found."
        exit 1
    fi

    printf "  %bInstalled Versions:%b\\n" "$BOLD" "$NC"
    
    local active_target=""
    if [ -L "$CURRENT_LINK" ]; then
        active_target=$(readlink "$CURRENT_LINK")
    fi

    local i=1
    for v in "${versions[@]}"; do
        local marker=""
        if [[ "$active_target" == *"$v" ]]; then
            marker=" (active)"
        fi
        printf "    %b%d)%b  %s%s\\n" "$YELLOW" "$i" "$NC" "$v" "$marker"
        ((i++))
    done
    printf "    %b0)%b  Uninstall ALL versions\\n" "$RED" "$NC"
    echo ""
    read -rp "  Select versions to uninstall (comma-separated, e.g. 1,3 or 0 for all): " choices
    
    if [ -z "$choices" ]; then
        echo "  Uninstall cancelled."
        return
    fi
    
    if [ "$choices" = "0" ]; then
        printf "  %bWARNING: This will completely remove all Nux versions.%b\\n" "$RED" "$NC"
        read -rp "  Are you sure? [y/N]: " confirm
        if [ "$confirm" = "y" ] || [ "$confirm" = "Y" ]; then
            nux_print "Removing" "$RED" "all Nux files..."
            rm -rf "$INSTALL_DIR_BASE"
            rm -f "$BIN_LINK"
            rm -f /etc/profile.d/nux.sh
            rm -f "$DESKTOP_FILE" 2>/dev/null || true
            if command -v update-desktop-database &>/dev/null; then update-desktop-database /usr/share/applications 2>/dev/null || true; fi
            nux_finish "uninstalled" "completely."
        else
            echo "  Uninstall cancelled."
        fi
        return
    fi
    
    IFS=',' read -ra choice_array <<< "$choices"
    for c in "${choice_array[@]}"; do
        c=$(echo "$c" | tr -d ' ')
        if [[ "$c" =~ ^[0-9]+$ ]]; then
            local idx=$((c-1))
            if [ $idx -ge 0 ] && [ $idx -lt ${#versions[@]} ]; then
                local v="${versions[$idx]}"
                nux_print "Removing" "$RED" "$v..."
                rm -rf "$INSTALL_DIR_BASE/$v"
                if [[ "$active_target" == *"$v" ]]; then
                    rm -f "$CURRENT_LINK"
                fi
            fi
        fi
    done
    
    if [ ! -L "$CURRENT_LINK" ]; then
        local remaining=()
        for d in "$INSTALL_DIR_BASE"/*; do
            if [ -d "$d" ] && [ "$(basename "$d")" != "current" ]; then
                remaining+=("$(basename "$d")")
            fi
        done
        
        if [ ${#remaining[@]} -gt 0 ]; then
            local latest="${remaining[-1]}"
            ln -snf "$INSTALL_DIR_BASE/$latest" "$CURRENT_LINK"
            nux_print "Switched" "$GREEN" "active version to $latest"
            ln -snf "$CURRENT_LINK/nux" "$BIN_LINK"
        else
            rm -f "$BIN_LINK"
            rm -f /etc/profile.d/nux.sh
            rm -f "$DESKTOP_FILE" 2>/dev/null || true
            rm -rf "$INSTALL_DIR_BASE"
        fi
    fi
    nux_finish "uninstalled" "Selected versions removed."
}"""

    # Replace do_uninstall
    content = re.sub(
        r'do_uninstall\(\) \{.*?\n\}(?=\n\n(?:do_build_deb|do_build_rpm|\[ "\$1" = "--update" \]|# --- CLI Mode ---))',
        new_uninstall,
        content,
        flags=re.DOTALL
    )

    # 5. Fix is_installed function to use INSTALL_DIR_BASE
    content = re.sub(
        r'is_installed\(\) \{\n\s*\[ -f "\$MARKER_FILE" \] \|\| command -v nux >/dev/null 2>&1 \|\| \[ -f "\$INSTALL_DIR/nux" \]\n\}',
        r'is_installed() {\n    [ -d "$INSTALL_DIR_BASE" ] || command -v nux >/dev/null 2>&1\n}',
        content
    )
    
    # 6. Update `do_update`
    content = re.sub(
        r'rm -f "\$INSTALL_DIR/nux" 2>/dev/null \|\| true\n\s*tar --no-same-owner -xzf "\$TEMP_TAR" -C "\$INSTALL_DIR"\n\s*echo "\$VERSION" > "\$MARKER_FILE"\n\s*nux_finish "updated" "successfully!"',
        r'do_install "$INSTALL_DIR" "yes" "yes" "yes"\n            nux_finish "updated" "successfully!"',
        content
    )

    with open(path, 'w', encoding='utf-8') as f:
        f.write(content)

process_file('linux/install.sh')
process_file('mac/install.sh')
process_file('bsd/install.sh')
