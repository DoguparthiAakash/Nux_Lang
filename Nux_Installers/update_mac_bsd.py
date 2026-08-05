import re

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
            for profile in "$HOME/.bashrc" "$HOME/.zshrc" "$HOME/.profile"; do
                [ -f "$profile" ] && sed -i.bak '/NUX_HOME\|NUX_LIB_PATH\|# Nux Programming/d' "$profile" 2>/dev/null && rm -f "${profile}.bak"
            done
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
            for profile in "$HOME/.bashrc" "$HOME/.zshrc" "$HOME/.profile"; do
                [ -f "$profile" ] && sed -i.bak '/NUX_HOME\|NUX_LIB_PATH\|# Nux Programming/d' "$profile" 2>/dev/null && rm -f "${profile}.bak"
            done
            rm -rf "$INSTALL_DIR_BASE"
        fi
    fi
    nux_finish "uninstalled" "Selected versions removed."
}"""

for f in ['mac/install.sh', 'bsd/install.sh']:
    with open(f, 'r', encoding='utf-8') as file:
        content = file.read()
    
    # Use re to replace the old uninstall function
    # It starts with do_uninstall() { and ends before the first if statement or do_build_freebsd_pkg
    content = re.sub(r'do_uninstall\(\) \{.*?\n\}(?=\n\n(?:if \[ "\$\{1:-\}" = "--update" \]|do_build_freebsd_pkg))', new_uninstall, content, flags=re.DOTALL)
    
    with open(f, 'w', encoding='utf-8') as file:
        file.write(content)
