#ifndef NUX_VM_H
#define NUX_VM_H

#include "common.h"

#define STACK_SIZE 1024
#define HEAP_SIZE 65536
#define MAX_CALL_STACK 256

typedef struct {
    uint8_t* ip;
    uint8_t* code;
    int code_size;
    
    Value stack[STACK_SIZE];
    int sp; // Stack Pointer
    
    nux_int fp; // Frame Pointer (for locals)
    
    // Call Stack
    uint8_t* call_stack[MAX_CALL_STACK];
    nux_int fp_stack[MAX_CALL_STACK];
    int csp;
    
    // Heap (Simple byte array)
    uint8_t heap[HEAP_SIZE];
} VM;

void vm_init(VM* vm);
void vm_load(VM* vm, uint8_t* code, int size, int entry_offset);
void vm_run(VM* vm);

// Stack access for Extensions
void push(VM* vm, Value v);
Value pop(VM* vm);
Value peek(VM* vm);

// External Hooks (Implemented in main/runtime)
void ext_print(nux_int val);
void ext_print_char(char c);
void ext_sleep(int ms);
void ext_img_alloc(VM* vm); // Stack args
void ext_img_draw(VM* vm);
void ext_img_set(VM* vm);
void ext_img_fill(VM* vm);
int ext_is_key_down(int key);
uint32_t ext_img_get_pixel(void* handle, int x, int y);
uint32_t* ext_img_get_buffer(void* handle, int* w, int* h);
void ext_gfx_text(VM* vm);
void ext_gfx_rect(VM* vm);

#endif
