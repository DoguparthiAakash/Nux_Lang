import struct
import sys

def analyze():
    with open('scratch/bytecode.bin', 'rb') as f:
        code = bytearray(f.read())
        
    out = open('scratch/full_disasm.txt', 'w')
    
    ip = 64
    while ip < len(code):
        op = code[ip]
        old_ip = ip
        ip += 1
        
        name = "UNKNOWN"
        args = ""
        
        if op == 0xFF: name = "EXIT"
        elif op == 0x00: name = "NOP"
        elif op == 0x01:
            name = "PUSH"
            val = struct.unpack('<q', code[ip:ip+8])[0]
            args = str(val)
            ip += 8
        elif op == 0x02: name = "POP"
        elif op == 0x10: name = "ADD"
        elif op == 0x11: name = "SUB"
        elif op == 0x12: name = "MUL"
        elif op == 0x13: name = "DIV"
        elif op == 0x14: name = "MOD"
        elif op == 0x15: name = "POW"
        elif op == 0x16: name = "FLOORDIV"
        elif op == 0x17: name = "SWAP"
        elif op == 0x18: name = "AND"
        elif op == 0x19: name = "OR"
        elif op == 0x1A: name = "NOT"
        elif op == 0x25: name = "SHL"
        elif op == 0x26: name = "SHR"
        elif op == 0x1B: name = "FADD"
        elif op == 0x1C: name = "FMUL"
        elif op == 0x1D: name = "FDIV"
        elif op == 0x22: name = "XOR"
        elif op == 0x23: name = "XAND"
        elif op == 0x30: name = "SLEEP"
        elif op == 0x50: name = "DEBUG_PRINT"
        elif op == 0x90: name = "EQ"
        elif op == 0x91: name = "NEQ"
        elif op == 0x92: name = "LT"
        elif op == 0x93: name = "GT"
        elif op == 0x94: name = "LTE"
        elif op == 0x95: name = "GTE"
        elif op == 0x5D: name = "DUP"
        elif op == 0x60: 
            name = "JMP"
            val = struct.unpack('<q', code[ip:ip+8])[0]
            args = str(val)
            ip += 8
        elif op == 0x61: 
            name = "JE"
            val = struct.unpack('<q', code[ip:ip+8])[0]
            args = str(val)
            ip += 8
        elif op == 0x70:
            name = "CALL"
            dest = struct.unpack('<q', code[ip:ip+8])[0]
            num_args = struct.unpack('<q', code[ip+8:ip+16])[0]
            args = f"dest={dest}, args={num_args}"
            ip += 16
        elif op == 0x71: name = "RET"
        elif op == 0x51: name = "PRINT_CHAR"
        elif op == 0x52: name = "INPUT"
        elif op == 0x53: name = "PRINT_VAL"
        elif op == 0x54: name = "PRINT_FLOAT"
        elif op == 0x55: name = "FILE_OPEN"
        elif op == 0x56: name = "FILE_CLOSE"
        elif op == 0x57: name = "FILE_READ"
        elif op == 0x58: name = "FILE_WRITE"
        elif op == 0x59: name = "FILE_EXISTS"
        elif op == 0x81: name = "SYSTEM"
        elif op == 0x68:
            name = "PUSH_STR"
            assert code[ip] == 0x01
            ip += 1
            length = struct.unpack('<q', code[ip:ip+8])[0]
            ip += 8
            s = code[ip:ip+length]
            args = f"'{s.decode(errors='replace')}'"
            ip += length
        elif op == 0x69: name = "TO_UPPER"
        elif op == 0x6A: name = "TO_LOWER"
        elif op == 0x6B: name = "PRINT_STR"
        elif op == 0x6C: name = "STR_LEN"
        elif op == 0x6D: name = "STR_CHAR"
        elif op == 0x6E: name = "STR_SUB"
        elif op == 0x44:
            name = "GET_LOCAL"
            val = struct.unpack('<q', code[ip:ip+8])[0]
            args = str(val)
            ip += 8
        elif op == 0x45:
            name = "SET_LOCAL"
            val = struct.unpack('<q', code[ip:ip+8])[0]
            args = str(val)
            ip += 8
        elif op == 0x2C: name = "FSQRT"
        elif op == 0x1E: name = "ITOF"
        elif op == 0x1F: name = "FTOI"
        elif op == 0x2A: name = "FSIN"
        elif op == 0x2B: name = "FCOS"
        elif op == 0x2D: name = "FTAN"
        elif op == 0x46: name = "FPOW"
        elif op == 0x40: name = "PEEK"
        elif op == 0x41: name = "POKE"
        elif op == 0x49: name = "POKE32"
        elif op == 0x4C: name = "PEEK32"
        elif op == 0x42: name = "PEEK_PTR"
        elif op == 0x43: name = "POKE_PTR"
        elif op == 0x47: name = "SYSCALL"
        elif op == 0x4A: name = "UNSAFE_START"
        elif op == 0x4B: name = "UNSAFE_END"
        elif op == 0xEA: name = "Q_ALLOC"
        elif op == 0xEB: name = "Q_H"
        elif op == 0xEC: name = "Q_X"
        elif op == 0xED: name = "Q_Z"
        elif op == 0xEE: name = "Q_CX"
        elif op == 0xEF: name = "Q_MEASURE"
        elif op == 0xE7: name = "FFI_PYTHON"
        elif op == 0xE9: name = "EVAL_NUX"
        elif op == 0xE8: name = "FFI_C"
        elif op == 0xE0: 
            name = "SPAWN_THREAD"
            dest = struct.unpack('<q', code[ip:ip+8])[0]
            num_args = struct.unpack('<q', code[ip+8:ip+16])[0]
            args = f"dest={dest}, args={num_args}"
            ip += 16
        elif op == 0xE1: name = "JOIN_THREAD"
        elif op == 0xA0: name = "ARRAY_ALLOC"
        elif op == 0xA1: name = "ARRAY_GET"
        elif op == 0xA2: name = "ARRAY_SET"
        
        out.write(f"{old_ip:04d}: {name} {args}\n")
    out.close()

if __name__ == '__main__':
    analyze()
