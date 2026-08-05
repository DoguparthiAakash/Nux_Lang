import sys

lines = open('compiler/lexer.nux', 'r').readlines()
new_lines = []
for i, l in enumerate(lines):
    if i >= 933 and ("var test_src =" in l or "=== Nux Lexer Test ===" in l or "println(" in l):
        # We are at the test code, skip the rest
        # Let's just truncate at line 933 (which is `933: `)
        break
    
    # Replace self. with this.
    nl = l.replace('self.', 'this.')
    new_lines.append(nl)

with open('compiler/lexer.nux', 'w') as f:
    f.writelines(new_lines)

print(f"Fixed lexer.nux! Kept {len(new_lines)} lines.")
