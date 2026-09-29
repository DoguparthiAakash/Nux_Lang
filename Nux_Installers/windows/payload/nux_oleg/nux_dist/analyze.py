import struct
import sys

def read_i64(code, ip):
    return struct.unpack('<q', code[ip:ip+8])[0]

def analyze():
    with open('scratch/bytecode.bin', 'rb') as f:
        code = bytearray(f.read())
    
    # Analyze the crash point sequence
    # 2427: OP_GET_LOCAL
    # 2436: OP_PUSH
    # 2445: OP_ADD
    # 2446: OP_PEEK
    print("Bytecode analysis:")
    
    if code[2427] != 0x44:
        print("Expected OP_GET_LOCAL at 2427")
    offset = read_i64(code, 2427 + 1)
    print(f"2427: OP_GET_LOCAL {offset}")
    
    if code[2436] != 0x01:
        print("Expected OP_PUSH at 2436")
    push_val = read_i64(code, 2436 + 1)
    print(f"2436: OP_PUSH {push_val}")
    
    if code[2445] != 0x10:
        print("Expected OP_ADD at 2445")
    print(f"2445: OP_ADD")
    
    if code[2446] != 0x40:
        print("Expected OP_PEEK at 2446")
    print(f"2446: OP_PEEK")
    
    print("\nTracing 5280 call:")
    print("5280: CALL", read_i64(code, 5280+1), "ARGS:", read_i64(code, 5280+9))
    print("6402: CALL", read_i64(code, 6402+1), "ARGS:", read_i64(code, 6402+9))

if __name__ == '__main__':
    analyze()
