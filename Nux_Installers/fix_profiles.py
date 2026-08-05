import re

def fix_linux():
    with open('linux/install.sh', 'r', encoding='utf-8') as f:
        content = f.read()
    content = content.replace(
        'export PATH="$PATH:$NUX_HOME"',
        'export PATH="\\$PATH:\\$NUX_HOME"'
    )
    with open('linux/install.sh', 'w', encoding='utf-8', newline='\n') as f:
        f.write(content)

def fix_mac():
    with open('mac/install.sh', 'r', encoding='utf-8') as f:
        content = f.read()
    
    # 1. Update link creation logic for Mac
    content = re.sub(
        r'nux_print "Linking" "\$CYAN" "\$BIN_LINK"\n\s*mkdir -p "\$\(dirname "\$BIN_LINK"\)"\n\s*ln -sf "\$target_dir/nux" "\$BIN_LINK"',
        r'ln -snf "$target_dir" "$CURRENT_LINK"\n\n        nux_print "Linking" "$CYAN" "$BIN_LINK"\n        mkdir -p "$(dirname "$BIN_LINK")"\n        ln -snf "$CURRENT_LINK/nux" "$BIN_LINK"',
        content
    )

    # 2. Update profile append for Mac
    content = re.sub(
        r'export NUX_HOME="\$target_dir"\n\s*export NUX_LIB_PATH="\$target_dir/lib"\n\s*export PATH="\$PATH:\$NUX_HOME"',
        r'export NUX_HOME="$CURRENT_LINK"\nexport NUX_LIB_PATH="$CURRENT_LINK/lib"\nexport PATH="\\$PATH:\\$NUX_HOME"',
        content
    )

    with open('mac/install.sh', 'w', encoding='utf-8', newline='\n') as f:
        f.write(content)

def fix_bsd():
    with open('bsd/install.sh', 'r', encoding='utf-8') as f:
        content = f.read()
    
    # 1. Update link creation logic for BSD
    content = re.sub(
        r'nux_print "Linking" "\$CYAN" "\$BIN_LINK"\n\s*mkdir -p "\$\(dirname "\$BIN_LINK"\)"\n\s*ln -sf "\$target_dir/nux" "\$BIN_LINK"',
        r'ln -snf "$target_dir" "$CURRENT_LINK"\n\n        nux_print "Linking" "$CYAN" "$BIN_LINK"\n        mkdir -p "$(dirname "$BIN_LINK")"\n        ln -snf "$CURRENT_LINK/nux" "$BIN_LINK"',
        content
    )

    # 2. Update profile append for BSD
    content = re.sub(
        r'export NUX_HOME="\$target_dir"\n\s*export NUX_LIB_PATH="\$target_dir/lib"\n\s*export PATH="\$PATH:\$NUX_HOME"',
        r'export NUX_HOME="$CURRENT_LINK"\nexport NUX_LIB_PATH="$CURRENT_LINK/lib"\nexport PATH="\\$PATH:\\$NUX_HOME"',
        content
    )

    with open('bsd/install.sh', 'w', encoding='utf-8', newline='\n') as f:
        f.write(content)

fix_linux()
fix_mac()
fix_bsd()
