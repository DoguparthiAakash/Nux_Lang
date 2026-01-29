#ifndef NUX_COMMON_H
#define NUX_COMMON_H

#include <stdint.h>
#include <stdbool.h>

// Types
typedef int64_t nux_int;
typedef double nux_float;

typedef enum {
    VAL_INT,
    VAL_FLOAT,
    VAL_STRING,
    VAL_PTR // Handle
} ValueType;

typedef struct {
    ValueType type;
    union {
        nux_int i;
        nux_float f;
        char* s;
        void* p;
    } as;
} Value;

// Opcodes (Must match Compiler logic)
typedef enum {
    OP_HALT = 0x00,
    OP_PUSH = 0x01,
    OP_POP = 0x02,
    OP_SWAP = 0x03,
    OP_DUP = 0x04,
    
    // Math
    OP_ADD = 0x10,
    OP_SUB = 0x11,
    OP_MUL = 0x12,
    OP_DIV = 0x13,
    OP_MOD = 0x14,
    OP_POW = 0x15,
    
    // Bitwise
    OP_AND = 0x18,
    OP_OR = 0x19,
    
    // Comparison
    OP_EQ = 0x90,
    OP_NEQ = 0x91,
    OP_LT = 0x92,
    OP_GT = 0x93,
    OP_LTE = 0x94,
    OP_GTE = 0x95,
    
    // Control
    OP_JMP = 0x60,
    OP_JE = 0x61, // Jump if Equal (Zero?) Wait, usually JZ/JNZ
    // Nux VM Logic: JE pops 2 and checks equality? Or checks flag?
    // High level compiler emits: `if x > y`. `x`, `y`, `GT`. Stack: [res].
    // Then `JMP_IF_FALSE`.
    // My previous Rust VM had `OP_JE` but maybe it meant "Jump If False" (0x61)?
    // Let's assume standard stack machine control flow:
    // PUSH condition; JMP_IF_FALSE target.
    // I shall define OP_JMP_FALSE (Jump if top is 0).
    OP_JMP_FALSE = 0x61, 
    
    OP_CALL = 0x70,
    OP_RET = 0x71,
    
    // Memory
    OP_PEEK = 0x40,
    OP_POKE = 0x41,
    OP_GET_LOCAL = 0x44,
    OP_SET_LOCAL = 0x45,
    
    // I/O & Intrinsics
    OP_PRINT = 0x53, // PRINT_VAL
    OP_PRINT_CHAR = 0x51,
    OP_SLEEP = 0x30,
    OP_EXIT = 0xFF,
    
    // Graphics (Intrinsics mapped to Opcodes)
    OP_IMG_ALLOC = 0x31,
    OP_IMG_FREE = 0x32,
    OP_IMG_DRAW = 0x33,
    OP_IMG_GET  = 0x36,
    OP_IMG_SET = 0x3A,
    OP_IMG_FILL = 0x3B,
    
    // Input
    OP_IS_KEY_DOWN = 0x5A,
    
    // Vision
    OP_VISION_DETECT = 0xB0, // Arg: Mode (1=Gray, 2=Edge)

    // Legacy Graphics (Direct)
    OP_GFX_TEXT = 0x3C,
    OP_GFX_RECT = 0x3D
    
} OpCode;

#endif
