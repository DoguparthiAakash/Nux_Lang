
/* 
 * MICRO NUX VM 
 * Extreme constraint bare-metal interpreter.
 * Zero dependencies. Runs on raw hardware.
 */

#ifndef NUX_MEM_SIZE
#define NUX_MEM_SIZE 32
#endif

#include <stdint.h>

/* The Compiled Nux Bytecode */
const uint8_t PROGRAM[] = { 0x41, 0x4E, 0x55, 0x58, 0x2D, 0x14, 0x68, 0x79, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2E, 0x46, 0x59, 0x5F, 0x53, 0x45, 0x43, 0x55, 0x52, 0x25, 0x38, 0x4B, 0x45, 0x59, 0x5F, 0x31, 0x32, 0x33, 0x0A, 0x54, 0x58, 0x5F, 0x53, 0x45, 0x43, 0x55, 0x52, 0x01, 0x5F, 0x4B, 0x45, 0x59, 0x5F, 0x31, 0x32, 0x33, 0xAC, 0xF5, 0x29, 0x3F, 0xD6, 0x45, 0x43, 0x55, 0x52, 0x45, 0x5F, 0x4B, 0x01, 0x58, 0x5F, 0x31, 0x32, 0x33, 0x4E, 0x55, 0x58, 0x1B, 0x53, 0x45, 0x43, 0x55, 0x52, 0x45, 0x5F, 0x4B, 0xA5, 0xF9, 0x2E, 0x51, 0xA8, 0x33, 0x4E, 0x55, 0x58, 0x5F, 0x53, 0x45, 0x07, 0x55, 0x52, 0x45, 0x5F, 0x4B, 0x45, 0x59, 0x5F, 0xDB, 0x92, 0x42, 0x2E, 0x46, 0x59, 0x5F, 0x53, 0x45, 0x43, 0x55, 0x52, 0x35, 0x0D, 0x4B, 0x45, 0x59, 0x5F, 0x31, 0x32, 0x33, 0x4C, 0x55, 0x58, 0x5F, 0x53, 0x45, 0x43, 0x55, 0x50, 0xE4, 0xFF, 0x2A, 0x54, 0x58, 0x5F, 0x31, 0x32, 0x33, 0x4E, 0x55, 0x28, 0x2F, 0x53, 0x45, 0x43, 0x55, 0x52, 0x45, 0x5F, 0x49, 0x45, 0x59, 0x5F, 0x31, 0x32, 0x33, 0x4E, 0x57, 0x28, 0xD1, 0x53, 0x45, 0x43, 0x55, 0x52, 0x45, 0x5F, 0x4A, 0x45, 0x59, 0x5F, 0x31, 0x32, 0x33, 0x4E, 0x57, 0x28, 0x2F, 0x53, 0x45, 0x43, 0x55, 0x52, 0x45, 0x5F, 0x49, 0x45, 0x59, 0x5F, 0x31, 0x32, 0x33, 0x4E, 0x57, 0x28, 0xD1, 0x53, 0x45, 0x43, 0x55, 0x52, 0x45, 0x5F, 0x4A, 0x45, 0x59, 0x5F, 0x31, 0x32, 0x33, 0x4E, 0x57, 0x38, 0xEA, 0x53, 0x45, 0x43, 0x55, 0x52, 0x45, 0x5F, 0xEB, 0x34, 0xA6 };
const int PROGRAM_LEN = sizeof(PROGRAM);
const uint8_t KEY[] = "NUX_SECURE_KEY_123";

/* VM State */
int32_t stack[NUX_MEM_SIZE];
int32_t vars[NUX_MEM_SIZE];
int sp = -1;
int pc = 0;

#define PUSH(x) stack[++sp] = (x)
#define POP() stack[sp--]

/* Hardware I/O Hooks (Override these for specific hardware) */
#ifndef NUX_PUTC
#define NUX_PUTC(x) 
#endif
#ifndef NUX_HALT
#define NUX_HALT() while(1) {}
#endif
#ifndef NUX_PIN_MODE
#define NUX_PIN_MODE(pin, mode)
#endif
#ifndef NUX_DIGITAL_WRITE
#define NUX_DIGITAL_WRITE(pin, val)
#endif
#ifndef NUX_DELAY_MS
#define NUX_DELAY_MS(ms)
#endif

uint8_t read_byte() {
    if (pc < 64) {
        return PROGRAM[pc++];
    }
    uint8_t val = PROGRAM[pc] ^ KEY[(pc - 64) % 18];
    pc++;
    return val;
}

void nux_run() {
    if (PROGRAM_LEN >= 64 && PROGRAM[0] == 'A' && PROGRAM[1] == 'N' && PROGRAM[2] == 'U' && PROGRAM[3] == 'X') {
        uint32_t a = 1, b = 0;
        for (int i = 64; i < PROGRAM_LEN; i++) {
            uint8_t byte = PROGRAM[i] ^ KEY[(i - 64) % 18];
            a = (a + byte) % 65521;
            b = (b + a) % 65521;
        }
        uint32_t checksum = (b << 16) | a;
        
        uint32_t expected_checksum = 0;
        expected_checksum |= PROGRAM[4];
        expected_checksum |= PROGRAM[5] << 8;
        expected_checksum |= PROGRAM[6] << 16;
        expected_checksum |= PROGRAM[7] << 24;
        
        if (expected_checksum != 0 && checksum != expected_checksum) {
            /* Security/Integrity Check Failed! */
            NUX_HALT();
        }
        pc = 64; /* Start execution after header */
    } else {
        /* Not a valid executable */
        NUX_HALT();
    }

    while (pc < PROGRAM_LEN) {
        uint8_t opcode = read_byte();
        int32_t a, b;
        
        if (opcode >= 0xA0 && opcode <= 0xAF) {
            PUSH(opcode - 0xA0);
            continue;
        }

        switch (opcode) {
            case 0xB0: /* 2-byte PUSH */
                PUSH((int8_t)read_byte());
                break;
            case 0xB1: /* 3-byte PUSH */
                {
                    int32_t val = 0;
                    val |= (int32_t)read_byte();
                    val |= ((int32_t)read_byte()) << 8;
                    if (val & 0x8000) val |= 0xFFFF0000; /* Sign extend */
                    PUSH(val);
                }
                break;
            case 0xB2: /* 5-byte PUSH */
                {
                    int32_t val = 0;
                    val |= (int32_t)read_byte();
                    val |= ((int32_t)read_byte()) << 8;
                    val |= ((int32_t)read_byte()) << 16;
                    val |= ((int32_t)read_byte()) << 24;
                    PUSH(val);
                }
                break;
            case 0x01: /* OP_PUSH */
                {
                    int32_t val = 0;
                    val |= ((int32_t)read_byte());
                    val |= ((int32_t)read_byte()) << 8;
                    val |= ((int32_t)read_byte()) << 16;
                    val |= ((int32_t)read_byte()) << 24;
                    for(int i = 0; i < 4; i++) read_byte(); /* Skip top 4 bytes of 64-bit int */
                    PUSH(val);
                }
                break;
            case 0x02: /* OP_POP */
                if (sp >= 0) sp--;
                break;
            case 0x10: /* OP_ADD */
                b = POP(); a = POP(); PUSH(a + b);
                break;
            case 0x11: /* OP_SUB */
                b = POP(); a = POP(); PUSH(a - b);
                break;
            case 0x12: /* OP_MUL */
                b = POP(); a = POP(); PUSH(a * b);
                break;
            case 0x13: /* OP_DIV */
                b = POP(); a = POP(); PUSH(b == 0 ? 0 : a / b);
                break;
            case 0x14: /* OP_MOD */
                b = POP(); a = POP(); PUSH(b == 0 ? 0 : a % b);
                break;
            case 0x90: /* OP_EQ */
                b = POP(); a = POP(); PUSH(a == b ? 1 : 0);
                break;
            case 0x91: /* OP_NEQ */
                b = POP(); a = POP(); PUSH(a != b ? 1 : 0);
                break;
            case 0x92: /* OP_LT */
                b = POP(); a = POP(); PUSH(a < b ? 1 : 0);
                break;
            case 0x93: /* OP_GT */
                b = POP(); a = POP(); PUSH(a > b ? 1 : 0);
                break;
            case 0x94: /* OP_LTE */
                b = POP(); a = POP(); PUSH(a <= b ? 1 : 0);
                break;
            case 0x95: /* OP_GTE */
                b = POP(); a = POP(); PUSH(a >= b ? 1 : 0);
                break;
            case 0x18: /* OP_AND */
                b = POP(); a = POP(); PUSH((a && b) ? 1 : 0);
                break;
            case 0x19: /* OP_OR */
                b = POP(); a = POP(); PUSH((a || b) ? 1 : 0);
                break;
            case 0x60: /* OP_JMP */
                {
                    int32_t addr = 0;
                    addr |= ((int32_t)read_byte());
                    addr |= ((int32_t)read_byte()) << 8;
                    addr |= ((int32_t)read_byte()) << 16;
                    addr |= ((int32_t)read_byte()) << 24;
                    for(int i = 0; i < 4; i++) read_byte();
                    pc = addr;
                }
                break;
            case 0x61: /* OP_JE */
                {
                    int32_t addr = 0;
                    addr |= ((int32_t)read_byte());
                    addr |= ((int32_t)read_byte()) << 8;
                    addr |= ((int32_t)read_byte()) << 16;
                    addr |= ((int32_t)read_byte()) << 24;
                    for(int i = 0; i < 4; i++) read_byte();
                    b = POP(); a = POP();
                    if (a == b) pc = addr;
                }
                break;
            case 0x51: /* OP_PRINT_CHAR */
                NUX_PUTC((uint8_t)POP());
                break;
            case 0x40: /* OP_PEEK */
                a = POP(); PUSH(vars[a / 8]);
                break;
            case 0x41: /* OP_POKE */
                a = POP(); b = POP(); vars[a / 8] = b;
                break;
            case 0xE0: /* OP_GPIO_WRITE */
                a = POP(); b = POP(); NUX_DIGITAL_WRITE(a, b);
                break;
            case 0xE2: /* OP_GPIO_MODE */
                a = POP(); b = POP(); NUX_PIN_MODE(a, b);
                break;
            case 0xEA: /* OP_DELAY_MS */
                a = POP(); NUX_DELAY_MS(a);
                break;
            default:
                /* Unknown Opcode - Skip or Halt */
                break;
        }
    }
}

/* For testing on PC: */
#ifdef NUX_MICRO_PC_TEST
#include <stdio.h>
#include <stdlib.h>
#undef NUX_PUTC
#define NUX_PUTC(x) putchar(x)
#undef NUX_HALT
#define NUX_HALT() do { printf("HALTED (Integrity error)\\n"); exit(1); } while(0)
int main() {
    nux_run();
    return 0;
}
#endif
