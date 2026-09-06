import sys
content = open('src/high_level.rs').read()

old_assignment = 'out.push_str(&format!("PUSH {}\\nOP_ADD\\n", offset));'
new_assignment = 'out.push_str(&format!("PUSH {}\\nOP_ADD\\n", offset * 8));'
content = content.replace(old_assignment, new_assignment)

old_read = 'out.push_str(&format!("PUSH {}\\nOP_ADD\\nPEEK\\n", offset));'
new_read = 'out.push_str(&format!("PUSH {}\\nOP_ADD\\nPEEK\\n", offset * 8));'
content = content.replace(old_read, new_read)

old_new = 'out.push_str(&format!("PUSH {}\\nPUSH 1\\nOP_IMG_ALLOC\\n", size * 8));'
new_new = 'out.push_str(&format!("PUSH {}\\nOP_ALLOC\\n", size * 8));'
content = content.replace(old_new, new_new)

with open('src/high_level.rs', 'w') as f:
    f.write(content)
print('Patched')
