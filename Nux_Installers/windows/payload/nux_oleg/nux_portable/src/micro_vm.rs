pub fn generate_micro_vm(bytecode: &[u8]) -> String {
    let mut code = String::new();

    // Convert bytecode to C array literal
    let mut bytecode_str = String::new();
    for (i, byte) in bytecode.iter().enumerate() {
        bytecode_str.push_str(&format!("0x{:02X}", byte));
        if i < bytecode.len() - 1 {
            bytecode_str.push_str(", ");
        }
    }

    code.push_str(&format!(r#"
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
const uint8_t PROGRAM[] = {{ {} }};
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
#define NUX_HALT() while(1) {{}}
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

uint8_t read_byte() {{
    if (pc < 64) {{
        return PROGRAM[pc++];
    }}
    uint8_t val = PROGRAM[pc] ^ KEY[(pc - 64) % 18];
    pc++;
    return val;
}}

void nux_run() {{
    if (PROGRAM_LEN >= 64 && PROGRAM[0] == 'A' && PROGRAM[1] == 'N' && PROGRAM[2] == 'U' && PROGRAM[3] == 'X') {{
        uint32_t a = 1, b = 0;
        for (int i = 64; i < PROGRAM_LEN; i++) {{
            uint8_t byte = PROGRAM[i] ^ KEY[(i - 64) % 18];
            a = (a + byte) % 65521;
            b = (b + a) % 65521;
        }}
        uint32_t checksum = (b << 16) | a;
        
        uint32_t expected_checksum = 0;
        expected_checksum |= PROGRAM[4];
        expected_checksum |= PROGRAM[5] << 8;
        expected_checksum |= PROGRAM[6] << 16;
        expected_checksum |= PROGRAM[7] << 24;
        
        if (expected_checksum != 0 && checksum != expected_checksum) {{
            /* Security/Integrity Check Failed! */
            NUX_HALT();
        }}
        pc = 64; /* Start execution after header */
    }} else {{
        /* Not a valid executable */
        NUX_HALT();
    }}

    #if defined(__GNUC__) && !defined(NUX_NO_COMPUTED_GOTO)
    static void* dispatch_table[256] = {{ [0 ... 255] = &&OP_UNKNOWN }};
    dispatch_table[0xB0] = &&OP_0xB0; dispatch_table[0xB1] = &&OP_0xB1; dispatch_table[0xB2] = &&OP_0xB2;
    dispatch_table[0x01] = &&OP_0x01; dispatch_table[0x02] = &&OP_0x02; dispatch_table[0x10] = &&OP_0x10;
    dispatch_table[0x11] = &&OP_0x11; dispatch_table[0x12] = &&OP_0x12; dispatch_table[0x13] = &&OP_0x13;
    dispatch_table[0x14] = &&OP_0x14; dispatch_table[0x90] = &&OP_0x90; dispatch_table[0x91] = &&OP_0x91;
    dispatch_table[0x92] = &&OP_0x92; dispatch_table[0x93] = &&OP_0x93; dispatch_table[0x94] = &&OP_0x94;
    dispatch_table[0x95] = &&OP_0x95; dispatch_table[0x18] = &&OP_0x18; dispatch_table[0x19] = &&OP_0x19;
    dispatch_table[0x60] = &&OP_0x60; dispatch_table[0x61] = &&OP_0x61; dispatch_table[0x51] = &&OP_0x51;
    dispatch_table[0x40] = &&OP_0x40; dispatch_table[0x41] = &&OP_0x41; dispatch_table[0xE0] = &&OP_0xE0;
    dispatch_table[0xE2] = &&OP_0xE2; dispatch_table[0xEA] = &&OP_0xEA;
    
    #define DISPATCH() do {{ \
        if (pc >= PROGRAM_LEN) goto OP_END; \
        uint8_t op = read_byte(); \
        if (op >= 0xA0 && op <= 0xAF) {{ PUSH(op - 0xA0); DISPATCH(); }} \
        goto *dispatch_table[op]; \
    }} while(0)
    
    #define OP_CASE(opcode) OP_##opcode:
    #define OP_BREAK() DISPATCH()
    
    DISPATCH();
    #else
    #define OP_CASE(opcode) case opcode:
    #define OP_BREAK() break;
    
    while (pc < PROGRAM_LEN) {{
        uint8_t opcode = read_byte();
        int32_t a, b;
        
        if (opcode >= 0xA0 && opcode <= 0xAF) {{
            PUSH(opcode - 0xA0);
            continue;
        }}

        switch (opcode) {{
    #endif
            OP_CASE(0xB0) /* 2-byte PUSH */
                PUSH((int8_t)read_byte());
                OP_BREAK();
            OP_CASE(0xB1) /* 3-byte PUSH */
                {{
                    int32_t val = 0;
                    val |= (int32_t)read_byte();
                    val |= ((int32_t)read_byte()) << 8;
                    if (val & 0x8000) val |= 0xFFFF0000; /* Sign extend */
                    PUSH(val);
                }}
                OP_BREAK();
            OP_CASE(0xB2) /* 5-byte PUSH */
                {{
                    int32_t val = 0;
                    val |= (int32_t)read_byte();
                    val |= ((int32_t)read_byte()) << 8;
                    val |= ((int32_t)read_byte()) << 16;
                    val |= ((int32_t)read_byte()) << 24;
                    PUSH(val);
                }}
                OP_BREAK();
            OP_CASE(0x01) /* OP_PUSH */
                {{
                    int32_t val = 0;
                    val |= ((int32_t)read_byte());
                    val |= ((int32_t)read_byte()) << 8;
                    val |= ((int32_t)read_byte()) << 16;
                    val |= ((int32_t)read_byte()) << 24;
                    for(int i = 0; i < 4; i++) read_byte(); /* Skip top 4 bytes of 64-bit int */
                    PUSH(val);
                }}
                OP_BREAK();
            OP_CASE(0x02) /* OP_POP */
                if (sp >= 0) sp--;
                OP_BREAK();
            OP_CASE(0x10) /* OP_ADD */
                b = POP(); a = POP(); PUSH(a + b);
                OP_BREAK();
            OP_CASE(0x11) /* OP_SUB */
                b = POP(); a = POP(); PUSH(a - b);
                OP_BREAK();
            OP_CASE(0x12) /* OP_MUL */
                b = POP(); a = POP(); PUSH(a * b);
                OP_BREAK();
            OP_CASE(0x13) /* OP_DIV */
                b = POP(); a = POP(); PUSH(b == 0 ? 0 : a / b);
                OP_BREAK();
            OP_CASE(0x14) /* OP_MOD */
                b = POP(); a = POP(); PUSH(b == 0 ? 0 : a % b);
                OP_BREAK();
            OP_CASE(0x90) /* OP_EQ */
                b = POP(); a = POP(); PUSH(a == b ? 1 : 0);
                OP_BREAK();
            OP_CASE(0x91) /* OP_NEQ */
                b = POP(); a = POP(); PUSH(a != b ? 1 : 0);
                OP_BREAK();
            OP_CASE(0x92) /* OP_LT */
                b = POP(); a = POP(); PUSH(a < b ? 1 : 0);
                OP_BREAK();
            OP_CASE(0x93) /* OP_GT */
                b = POP(); a = POP(); PUSH(a > b ? 1 : 0);
                OP_BREAK();
            OP_CASE(0x94) /* OP_LTE */
                b = POP(); a = POP(); PUSH(a <= b ? 1 : 0);
                OP_BREAK();
            OP_CASE(0x95) /* OP_GTE */
                b = POP(); a = POP(); PUSH(a >= b ? 1 : 0);
                OP_BREAK();
            OP_CASE(0x18) /* OP_AND */
                b = POP(); a = POP(); PUSH((a && b) ? 1 : 0);
                OP_BREAK();
            OP_CASE(0x19) /* OP_OR */
                b = POP(); a = POP(); PUSH((a || b) ? 1 : 0);
                OP_BREAK();
            OP_CASE(0x60) /* OP_JMP */
                {{
                    int32_t addr = 0;
                    addr |= ((int32_t)read_byte());
                    addr |= ((int32_t)read_byte()) << 8;
                    addr |= ((int32_t)read_byte()) << 16;
                    addr |= ((int32_t)read_byte()) << 24;
                    for(int i = 0; i < 4; i++) read_byte();
                    pc = addr;
                }}
                OP_BREAK();
            OP_CASE(0x61) /* OP_JE */
                {{
                    int32_t addr = 0;
                    addr |= ((int32_t)read_byte());
                    addr |= ((int32_t)read_byte()) << 8;
                    addr |= ((int32_t)read_byte()) << 16;
                    addr |= ((int32_t)read_byte()) << 24;
                    for(int i = 0; i < 4; i++) read_byte();
                    b = POP(); a = POP();
                    if (a == b) pc = addr;
                }}
                OP_BREAK();
            OP_CASE(0x51) /* OP_PRINT_CHAR */
                NUX_PUTC((uint8_t)POP());
                OP_BREAK();
            OP_CASE(0x40) /* OP_PEEK */
                a = POP(); PUSH(vars[a / 8]);
                OP_BREAK();
            OP_CASE(0x41) /* OP_POKE */
                a = POP(); b = POP(); vars[a / 8] = b;
                OP_BREAK();
            OP_CASE(0xE0) /* OP_GPIO_WRITE */
                a = POP(); b = POP(); NUX_DIGITAL_WRITE(a, b);
                OP_BREAK();
            OP_CASE(0xE2) /* OP_GPIO_MODE */
                a = POP(); b = POP(); NUX_PIN_MODE(a, b);
                OP_BREAK();
            OP_CASE(0xEA) /* OP_DELAY_MS */
                a = POP(); NUX_DELAY_MS(a);
                OP_BREAK();
            OP_UNKNOWN:
            default:
                /* Unknown Opcode - Skip or Halt */
                OP_BREAK();
        }}
    #if defined(__GNUC__) && !defined(NUX_NO_COMPUTED_GOTO)
    OP_END: ;
    #else
    }}
    #endif}}

/* For testing on PC: */
#ifdef NUX_MICRO_PC_TEST
#include <stdio.h>
#include <stdlib.h>
#undef NUX_PUTC
#define NUX_PUTC(x) putchar(x)
#undef NUX_HALT
#define NUX_HALT() do {{ printf("HALTED (Integrity error)\\n"); exit(1); }} while(0)
int main() {{
    nux_run();
    return 0;
}}
#endif
"#, bytecode_str));

    code
}

