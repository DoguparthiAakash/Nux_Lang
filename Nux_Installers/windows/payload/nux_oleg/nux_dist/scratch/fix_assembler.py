import os

asm_path = r"E:\nux\Nux_Lang\nux\nux_oleg\nux_dist\src\assembler.rs"

with open(asm_path, "r", encoding="utf-8") as f:
    content = f.read()

asm_intrinsics = """            "OP_NET_LISTEN" => bytecode.push(0xB0),
            "OP_NET_ACCEPT" => bytecode.push(0xB1),
            "OP_NET_READ" => bytecode.push(0xB2),
            "OP_NET_WRITE" => bytecode.push(0xB3),
            "OP_NET_CLOSE" => bytecode.push(0xB4),
            "OP_NET_LISTEN_TLS" => bytecode.push(0xB5),
"""

if "OP_NET_LISTEN" not in content:
    content = content.replace(
        "\"OP_IMG_ALLOC\" => bytecode.push(0xA0),",
        asm_intrinsics + "            \"OP_IMG_ALLOC\" => bytecode.push(0xA0),"
    )

with open(asm_path, "w", encoding="utf-8") as f:
    f.write(content)

print("Assembler patched with net opcodes")
