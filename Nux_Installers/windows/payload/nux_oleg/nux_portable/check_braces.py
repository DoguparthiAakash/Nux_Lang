
def check_braces(filename):
    with open(filename, 'r') as f:
        # Read as single string to handle multiline easier? Or line by line.
        # Line by line is safer for reporting.
        lines = f.readlines()
    
    stack = []
    
    in_string = False
    in_char = False
    string_char = '"'
    
    for i, line in enumerate(lines):
        line_num = i + 1
        
        # Simple iterator
        idx = 0
        while idx < len(line):
            c = line[idx]
            
            # Handle Strings/Chars (Ignore Escapes)
            if in_string:
                if c == string_char:
                    # Check escape
                    escapes = 0
                    back = idx - 1
                    while back >= 0 and line[back] == '\\':
                        escapes += 1
                        back -= 1
                    if escapes % 2 == 0:
                        in_string = False
            elif in_char:
                if c == '\'':
                    # Check escape
                    escapes = 0
                    back = idx - 1
                    while back >= 0 and line[back] == '\\':
                        escapes += 1
                        back -= 1
                    if escapes % 2 == 0:
                        in_char = False
            else:
                # Not in string
                if c == '"':
                    in_string = True
                    string_char = '"'
                elif c == '\'':
                    # Check if it is really a char start (not lifetime 'a)
                    # Heuristic: lifetimes usually follow a separator or &
                    # But easiest is: assume char if next char is not space?
                    # Rust lifetimes: 'a, 'static. 
                    # Char literal: 'a', '\n'.
                    # This is hard. Let's assume it IS a char for now, unless it's a lifetime.
                    # Hack: Ignored for now? No, we need to ignore '{' inside char.
                    in_char = True
                elif c == '/' and idx + 1 < len(line) and line[idx+1] == '/':
                    # Comment
                    break # Skip rest of line
                elif c == '{':
                    stack.append((line_num, idx + 1))
                elif c == '}':
                    if not stack:
                        print(f"Unexpected closing brace at line {line_num}:{idx+1}")
                        return
                    stack.pop()
            
            idx += 1

    if stack:
        print(f"Unclosed braces: {len(stack)}")
        print(f"Last unclosed brace at line {stack[-1][0]}:{stack[-1][1]}")
        if len(stack) > 1:
            print(f"Previous unclosed brace at line {stack[-2][0]}:{stack[-2][1]}")
    else:
        print("Braces are balanced (Advanced check).")

check_braces("/home/aakash/Downloads/Ainuix/custom_kernel/nux_portable/src/high_level.rs")
