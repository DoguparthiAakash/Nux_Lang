#!/usr/bin/env python3
"""
Nux Syntax Migration Tool
Converts old-style syntax (fn/let) to new-style (func/var)
"""

import re
import sys
import os

def migrate_syntax(code):
    """Convert old Nux syntax to new syntax"""
    
    # // -> # (Comments)
    # Heuristic: Replace // with # if not preceded by : (to avoid URLs)
    # and not inside quotes.
    lines = []
    for line in code.split('\n'):
        # Skip if // is part of an URL (heuristic: starts with http:// or https://)
        if 'http://' in line or 'https://' in line or 'sqlite://' in line:
            # We must only replace // that are not part of the URL
            # But the most common case is // used as comments.
            # Fix: Only replace // if it has space before it or at start of line
            # AND it's not preceded by :
            line = re.sub(r'(?<![:])//', '#', line)
        else:
            line = re.sub(r'//', '#', line)
        lines.append(line)
    code = '\n'.join(lines)
    
    # fn -> func
    code = re.sub(r'\bfn\b', 'func', code)
    
    # let -> var
    code = re.sub(r'\blet\b', 'var', code)
    
    # Add type annotations where missing (basic heuristic)
    # var x = 10; -> var x: int = 10;
    code = re.sub(
        r'var\s+(\w+)\s*=\s*(\d+);',
        r'var \1: int = \2;',
        code
    )
    
    # var x = "string"; -> var x: string = "string";
    code = re.sub(
        r'var\s+(\w+)\s*=\s*"([^"]*)";',
        r'var \1: string = "\2";',
        code
    )
    
    # var x = true/false; -> var x: bool = true/false;
    code = re.sub(
        r'var\s+(\w+)\s*=\s*(true|false);',
        r'var \1: bool = \2;',
        code
    )
    
    return code

def migrate_file(filepath):
    """Migrate a single file"""
    print(f"Migrating {filepath}...")
    
    with open(filepath, 'r') as f:
        code = f.read()
    
    migrated = migrate_syntax(code)
    
    # Create backup
    backup_path = filepath + '.backup'
    with open(backup_path, 'w') as f:
        f.write(code)
    
    # Write migrated code
    with open(filepath, 'w') as f:
        f.write(migrated)
    
    print(f"  ✓ Migrated (backup: {backup_path})")

def migrate_directory(dirpath):
    """Migrate all .nux files in directory"""
    for root, dirs, files in os.walk(dirpath):
        for file in files:
            if file.endswith('.nux'):
                filepath = os.path.join(root, file)
                migrate_file(filepath)

if __name__ == '__main__':
    if len(sys.argv) < 2:
        print("Usage: python3 migrate_syntax.py <file_or_directory>")
        sys.exit(1)
    
    path = sys.argv[1]
    
    if os.path.isfile(path):
        migrate_file(path)
    elif os.path.isdir(path):
        migrate_directory(path)
    else:
        print(f"Error: {path} not found")
        sys.exit(1)
    
    print("\n✓ Migration complete!")
