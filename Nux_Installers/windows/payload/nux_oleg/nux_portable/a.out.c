
#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>

#define NUX_INT int64_t

size_t nux_total_alloc = 0;
size_t nux_mem_limit = (size_t)-1;

void* nux_alloc(size_t size) {
    if (nux_mem_limit != (size_t)-1 && nux_total_alloc + size > nux_mem_limit) {
        fprintf(stderr, "Runtime Error: Out of Memory (Hit defined limit in C backend)\\n");
        exit(1);
    }
    void* p = malloc(size);
    if (p) nux_total_alloc += size;
    return p;
}

void nux_free(void* ptr) {
    free(ptr);
}

#define NUX_STACK_SIZE 1024
NUX_INT stack[NUX_STACK_SIZE];
int sp = -1;
#endif

// Nux UI FFI Declarations
extern NUX_INT ffi_window_create(NUX_INT title_ptr, NUX_INT width, NUX_INT height);
extern NUX_INT ffi_window_update(void);
extern NUX_INT ffi_draw_rect(NUX_INT x, NUX_INT y, NUX_INT w, NUX_INT h, NUX_INT color);
extern NUX_INT ffi_window_close(void);

NUX_INT vars[1024];

#define PUSH(x) stack[++sp] = (x)
#define POP() stack[sp--]

// Standard IO Mappings
#define NUX_PRINT_VAL(x) printf("%ld", (x))
#define NUX_PRINT_CHAR(x) printf("%c", (char)(x))
#define NUX_INPUT() getchar()
#define NUX_EXIT() return 0

int main() {
    goto __start_execution;
    goto skip__native_window_create;
_native_window_create:
    /* Unknown: GET_LOCAL 0 */
    /* Unknown: GET_LOCAL 1 */
    /* Unknown: OP_IMG_ALLOC */
    PUSH(0);
    return 0;
skip__native_window_create:
    goto skip__native_window_update;
_native_window_update:
    /* Unknown: GET_LOCAL 0 */
    /* Unknown: 0 */
    /* Unknown: 0 */
    ffi_window_update();
    PUSH(0);
    return 0;
skip__native_window_update:
    goto skip__native_gfx_clear;
_native_gfx_clear:
    /* Unknown: GET_LOCAL 0 */
    /* Unknown: GET_LOCAL 1 */
    /* Unknown: OP_IMG_FILL */
    PUSH(0);
    return 0;
skip__native_gfx_clear:
    goto skip__native_draw_pixel;
_native_draw_pixel:
    /* Unknown: GET_LOCAL 0 */
    /* Unknown: GET_LOCAL 1 */
    /* Unknown: GET_LOCAL 2 */
    /* Unknown: GET_LOCAL 3 */
    { NUX_INT c = POP(); NUX_INT y = POP(); NUX_INT x = POP(); ffi_draw_rect(x, y, 1, 1, c); }
    PUSH(0);
    return 0;
skip__native_draw_pixel:
    goto skip_Window_init;
Window_init:
    /* Unknown: OP_GET_LOCAL 0 */
    PUSH(1);
    /* Unknown: OP_ADD */
    /* Unknown: GET_LOCAL 1 */
    { NUX_INT addr = POP(); NUX_INT val = POP(); vars[addr / 8] = val; }
    /* Unknown: OP_GET_LOCAL 0 */
    PUSH(2);
    /* Unknown: OP_ADD */
    /* Unknown: GET_LOCAL 2 */
    { NUX_INT addr = POP(); NUX_INT val = POP(); vars[addr / 8] = val; }
    /* Unknown: OP_GET_LOCAL 0 */
    PUSH(0);
    /* Unknown: OP_ADD */
    /* Unknown: GET_LOCAL 1 */
    /* Unknown: GET_LOCAL 2 */
    /* Call _native_window_create not fully implemented in C transpiler yet */
    { NUX_INT addr = POP(); NUX_INT val = POP(); vars[addr / 8] = val; }
    /* Unknown: GET_LOCAL 0 */
    return 0;
    PUSH(0);
    return 0;
skip_Window_init:
    goto skip_Window_update;
Window_update:
    /* Unknown: GET_LOCAL 0 */
    PUSH(0);
    /* Unknown: OP_ADD */
    { NUX_INT addr = POP(); PUSH(vars[addr / 8]); }
    /* Call _native_window_update not fully implemented in C transpiler yet */
    /* Unknown: POP */
    PUSH(0);
    return 0;
skip_Window_update:
    goto skip_Window_clear;
Window_clear:
    /* Unknown: GET_LOCAL 0 */
    PUSH(0);
    /* Unknown: OP_ADD */
    { NUX_INT addr = POP(); PUSH(vars[addr / 8]); }
    /* Unknown: GET_LOCAL 1 */
    /* Call _native_gfx_clear not fully implemented in C transpiler yet */
    /* Unknown: POP */
    PUSH(0);
    return 0;
skip_Window_clear:
    goto skip_Window_draw_pixel;
Window_draw_pixel:
    /* Unknown: GET_LOCAL 1 */
    PUSH(0);
    { NUX_INT b = POP(); NUX_INT a = POP(); PUSH(a >= b ? 1 : 0); }
    PUSH(0);
    { NUX_INT b = POP(); NUX_INT a = POP(); if (a == b) goto __if_else_0; }
    /* Unknown: GET_LOCAL 1 */
    /* Unknown: GET_LOCAL 0 */
    PUSH(1);
    /* Unknown: OP_ADD */
    { NUX_INT addr = POP(); PUSH(vars[addr / 8]); }
    { NUX_INT b = POP(); NUX_INT a = POP(); PUSH(a < b ? 1 : 0); }
    PUSH(0);
    { NUX_INT b = POP(); NUX_INT a = POP(); if (a == b) goto __if_else_1; }
    /* Unknown: GET_LOCAL 2 */
    PUSH(0);
    { NUX_INT b = POP(); NUX_INT a = POP(); PUSH(a >= b ? 1 : 0); }
    PUSH(0);
    { NUX_INT b = POP(); NUX_INT a = POP(); if (a == b) goto __if_else_2; }
    /* Unknown: GET_LOCAL 2 */
    /* Unknown: GET_LOCAL 0 */
    PUSH(2);
    /* Unknown: OP_ADD */
    { NUX_INT addr = POP(); PUSH(vars[addr / 8]); }
    { NUX_INT b = POP(); NUX_INT a = POP(); PUSH(a < b ? 1 : 0); }
    PUSH(0);
    { NUX_INT b = POP(); NUX_INT a = POP(); if (a == b) goto __if_else_3; }
    /* Unknown: GET_LOCAL 0 */
    PUSH(0);
    /* Unknown: OP_ADD */
    { NUX_INT addr = POP(); PUSH(vars[addr / 8]); }
    /* Unknown: GET_LOCAL 1 */
    /* Unknown: GET_LOCAL 2 */
    /* Unknown: GET_LOCAL 3 */
    /* Call _native_draw_pixel not fully implemented in C transpiler yet */
    /* Unknown: POP */
    goto __if_end_3;
__if_else_3:
__if_end_3:
    goto __if_end_2;
__if_else_2:
__if_end_2:
    goto __if_end_1;
__if_else_1:
__if_end_1:
    goto __if_end_0;
__if_else_0:
__if_end_0:
    PUSH(0);
    return 0;
skip_Window_draw_pixel:
    goto skip_Vector2_init;
Vector2_init:
    /* Unknown: OP_GET_LOCAL 0 */
    PUSH(0);
    /* Unknown: OP_ADD */
    /* Unknown: GET_LOCAL 1 */
    { NUX_INT addr = POP(); NUX_INT val = POP(); vars[addr / 8] = val; }
    /* Unknown: OP_GET_LOCAL 0 */
    PUSH(1);
    /* Unknown: OP_ADD */
    /* Unknown: GET_LOCAL 2 */
    { NUX_INT addr = POP(); NUX_INT val = POP(); vars[addr / 8] = val; }
    /* Unknown: GET_LOCAL 0 */
    return 0;
    PUSH(0);
    return 0;
skip_Vector2_init:
    goto skip_memset;
memset:
    /* Unknown: GET_LOCAL 0 */
    /* Unknown: GET_LOCAL 0 */
    /* Unknown: GET_LOCAL 2 */
    { NUX_INT b = POP(); NUX_INT a = POP(); PUSH(a + b); }
__while_start_4:
    /* Unknown: GET_LOCAL 3 */
    /* Unknown: GET_LOCAL 4 */
    { NUX_INT b = POP(); NUX_INT a = POP(); PUSH(a < b ? 1 : 0); }
    PUSH(0);
    { NUX_INT b = POP(); NUX_INT a = POP(); if (a == b) goto __while_end_4; }
    /* Unknown: GET_LOCAL 3 */
    /* Unknown: GET_LOCAL 1 */
    { NUX_INT addr = POP(); NUX_INT val = POP(); vars[addr / 8] = val; }
    /* Unknown: GET_LOCAL 3 */
    PUSH(1);
    { NUX_INT b = POP(); NUX_INT a = POP(); PUSH(a + b); }
    /* Unknown: SET_LOCAL 3 */
    goto __while_start_4;
__while_end_4:
    PUSH(0);
    return 0;
skip_memset:
    goto skip_memcpy;
memcpy:
    /* Unknown: GET_LOCAL 0 */
    /* Unknown: GET_LOCAL 1 */
    /* Unknown: GET_LOCAL 0 */
    /* Unknown: GET_LOCAL 2 */
    { NUX_INT b = POP(); NUX_INT a = POP(); PUSH(a + b); }
    /* Unknown: GET_LOCAL 0 */
    /* Unknown: GET_LOCAL 1 */
    { NUX_INT b = POP(); NUX_INT a = POP(); PUSH(a > b ? 1 : 0); }
    PUSH(0);
    { NUX_INT b = POP(); NUX_INT a = POP(); if (a == b) goto __if_else_5; }
    /* Unknown: GET_LOCAL 0 */
    /* Unknown: GET_LOCAL 1 */
    /* Unknown: GET_LOCAL 2 */
    { NUX_INT b = POP(); NUX_INT a = POP(); PUSH(a + b); }
    { NUX_INT b = POP(); NUX_INT a = POP(); PUSH(a < b ? 1 : 0); }
    PUSH(0);
    { NUX_INT b = POP(); NUX_INT a = POP(); if (a == b) goto __if_else_6; }
    /* Unknown: GET_LOCAL 0 */
    /* Unknown: GET_LOCAL 2 */
    { NUX_INT b = POP(); NUX_INT a = POP(); PUSH(a + b); }
    PUSH(1);
    { NUX_INT b = POP(); NUX_INT a = POP(); PUSH(a - b); }
    /* Unknown: SET_LOCAL 3 */
    /* Unknown: GET_LOCAL 1 */
    /* Unknown: GET_LOCAL 2 */
    { NUX_INT b = POP(); NUX_INT a = POP(); PUSH(a + b); }
    PUSH(1);
    { NUX_INT b = POP(); NUX_INT a = POP(); PUSH(a - b); }
    /* Unknown: SET_LOCAL 4 */
    /* Unknown: GET_LOCAL 0 */
    PUSH(1);
    { NUX_INT b = POP(); NUX_INT a = POP(); PUSH(a - b); }
__while_start_7:
    /* Unknown: GET_LOCAL 3 */
    /* Unknown: GET_LOCAL 6 */
    { NUX_INT b = POP(); NUX_INT a = POP(); PUSH(a > b ? 1 : 0); }
    PUSH(0);
    { NUX_INT b = POP(); NUX_INT a = POP(); if (a == b) goto __while_end_7; }
    /* Unknown: GET_LOCAL 3 */
    /* Unknown: GET_LOCAL 4 */
    { NUX_INT addr = POP(); PUSH(vars[addr / 8]); }
    { NUX_INT addr = POP(); NUX_INT val = POP(); vars[addr / 8] = val; }
    /* Unknown: GET_LOCAL 3 */
    PUSH(1);
    { NUX_INT b = POP(); NUX_INT a = POP(); PUSH(a - b); }
    /* Unknown: SET_LOCAL 3 */
    /* Unknown: GET_LOCAL 4 */
    PUSH(1);
    { NUX_INT b = POP(); NUX_INT a = POP(); PUSH(a - b); }
    /* Unknown: SET_LOCAL 4 */
    goto __while_start_7;
__while_end_7:
    PUSH(0);
    return 0;
    goto __if_end_6;
__if_else_6:
__if_end_6:
    goto __if_end_5;
__if_else_5:
__if_end_5:
__while_start_8:
    /* Unknown: GET_LOCAL 3 */
    /* Unknown: GET_LOCAL 5 */
    { NUX_INT b = POP(); NUX_INT a = POP(); PUSH(a < b ? 1 : 0); }
    PUSH(0);
    { NUX_INT b = POP(); NUX_INT a = POP(); if (a == b) goto __while_end_8; }
    /* Unknown: GET_LOCAL 3 */
    /* Unknown: GET_LOCAL 4 */
    { NUX_INT addr = POP(); PUSH(vars[addr / 8]); }
    { NUX_INT addr = POP(); NUX_INT val = POP(); vars[addr / 8] = val; }
    /* Unknown: GET_LOCAL 3 */
    PUSH(1);
    { NUX_INT b = POP(); NUX_INT a = POP(); PUSH(a + b); }
    /* Unknown: SET_LOCAL 3 */
    /* Unknown: GET_LOCAL 4 */
    PUSH(1);
    { NUX_INT b = POP(); NUX_INT a = POP(); PUSH(a + b); }
    /* Unknown: SET_LOCAL 4 */
    goto __while_start_8;
__while_end_8:
    PUSH(0);
    return 0;
    PUSH(0);
    return 0;
skip_memcpy:
    goto skip_Tensor_init;
Tensor_init:
    /* Unknown: OP_GET_LOCAL 0 */
    PUSH(1);
    /* Unknown: OP_ADD */
    /* Unknown: GET_LOCAL 1 */
    { NUX_INT addr = POP(); NUX_INT val = POP(); vars[addr / 8] = val; }
    /* Unknown: OP_GET_LOCAL 0 */
    PUSH(2);
    /* Unknown: OP_ADD */
    /* Unknown: GET_LOCAL 2 */
    { NUX_INT addr = POP(); NUX_INT val = POP(); vars[addr / 8] = val; }
    /* Unknown: OP_GET_LOCAL 0 */
    PUSH(3);
    /* Unknown: OP_ADD */
    /* Unknown: GET_LOCAL 1 */
    /* Unknown: GET_LOCAL 2 */
    { NUX_INT b = POP(); NUX_INT a = POP(); PUSH(a * b); }
    PUSH(8);
    { NUX_INT b = POP(); NUX_INT a = POP(); PUSH(a * b); }
    { NUX_INT addr = POP(); NUX_INT val = POP(); vars[addr / 8] = val; }
    /* Unknown: OP_GET_LOCAL 0 */
    PUSH(0);
    /* Unknown: OP_ADD */
    /* Unknown: GET_LOCAL 0 */
    PUSH(3);
    /* Unknown: OP_ADD */
    { NUX_INT addr = POP(); PUSH(vars[addr / 8]); }
    { NUX_INT size = POP(); PUSH((NUX_INT)nux_alloc(size)); }
    { NUX_INT addr = POP(); NUX_INT val = POP(); vars[addr / 8] = val; }
    /* Unknown: GET_LOCAL 0 */
    return 0;
    PUSH(0);
    return 0;
skip_Tensor_init:
    goto skip_Tensor_matmul;
Tensor_matmul:
    /* Unknown: GET_LOCAL 0 */
    PUSH(2);
    /* Unknown: OP_ADD */
    { NUX_INT addr = POP(); PUSH(vars[addr / 8]); }
    /* Unknown: GET_LOCAL 1 */
    PUSH(1);
    /* Unknown: OP_ADD */
    { NUX_INT addr = POP(); PUSH(vars[addr / 8]); }
    { NUX_INT b = POP(); NUX_INT a = POP(); PUSH(a == b ? 1 : 0); }
    { NUX_INT val = POP(); if (val == 0) { fprintf(stderr, "Verification Failed!\n"); exit(1); } }
    PUSH(4);
    PUSH(1);
    /* Unknown: OP_IMG_ALLOC */
    /* Unknown: OP_GET_LOCAL 2 */
    /* Unknown: GET_LOCAL 0 */
    PUSH(1);
    /* Unknown: OP_ADD */
    { NUX_INT addr = POP(); PUSH(vars[addr / 8]); }
    /* Unknown: GET_LOCAL 1 */
    PUSH(2);
    /* Unknown: OP_ADD */
    { NUX_INT addr = POP(); PUSH(vars[addr / 8]); }
    /* Call result_init not fully implemented in C transpiler yet */
    /* Unknown: POP */
    /* Unknown: GET_LOCAL 0 */
    PUSH(0);
    /* Unknown: OP_ADD */
    { NUX_INT addr = POP(); PUSH(vars[addr / 8]); }
    /* Unknown: GET_LOCAL 1 */
    PUSH(0);
    /* Unknown: OP_ADD */
    { NUX_INT addr = POP(); PUSH(vars[addr / 8]); }
    /* Unknown: GET_LOCAL 2 */
    PUSH(0);
    /* Unknown: OP_ADD */
    { NUX_INT addr = POP(); PUSH(vars[addr / 8]); }
    /* Unknown: GET_LOCAL 0 */
    PUSH(1);
    /* Unknown: OP_ADD */
    { NUX_INT addr = POP(); PUSH(vars[addr / 8]); }
    /* Unknown: GET_LOCAL 0 */
    PUSH(2);
    /* Unknown: OP_ADD */
    { NUX_INT addr = POP(); PUSH(vars[addr / 8]); }
    /* Unknown: GET_LOCAL 1 */
    PUSH(2);
    /* Unknown: OP_ADD */
    { NUX_INT addr = POP(); PUSH(vars[addr / 8]); }
    /* Unknown: GET_LOCAL 3 */
    /* Unknown: GET_LOCAL 4 */
    /* Unknown: GET_LOCAL 5 */
    /* Unknown: GET_LOCAL 6 */
    /* Unknown: GET_LOCAL 7 */
    /* Unknown: GET_LOCAL 8 */
    /* Unknown: OP_MATMUL_SIMD */
    /* Unknown: GET_LOCAL 2 */
    return 0;
    PUSH(0);
    return 0;
skip_Tensor_matmul:
    goto skip_Tensor_dispose;
Tensor_dispose:
    /* Unknown: GET_LOCAL 0 */
    PUSH(0);
    /* Unknown: OP_ADD */
    { NUX_INT addr = POP(); PUSH(vars[addr / 8]); }
    PUSH(0);
    { NUX_INT b = POP(); NUX_INT a = POP(); PUSH(a > b ? 1 : 0); }
    PUSH(0);
    { NUX_INT b = POP(); NUX_INT a = POP(); if (a == b) goto __if_else_9; }
    /* Unknown: GET_LOCAL 0 */
    PUSH(0);
    /* Unknown: OP_ADD */
    { NUX_INT addr = POP(); PUSH(vars[addr / 8]); }
    { NUX_INT addr = POP(); nux_free((void*)addr); }
    /* Unknown: OP_GET_LOCAL 0 */
    PUSH(0);
    /* Unknown: OP_ADD */
    PUSH(0);
    { NUX_INT addr = POP(); NUX_INT val = POP(); vars[addr / 8] = val; }
    goto __if_end_9;
__if_else_9:
__if_end_9:
    PUSH(0);
    return 0;
skip_Tensor_dispose:
    goto skip_GPUCompute_dispatch;
GPUCompute_dispatch:
    /* Unknown: GET_LOCAL 2 */
    /* Unknown: GET_LOCAL 1 */
    /* Unknown: OP_GPU_DISPATCH */
    PUSH(0);
    return 0;
skip_GPUCompute_dispatch:
    goto skip_Vector3_init;
Vector3_init:
    /* Unknown: OP_GET_LOCAL 0 */
    PUSH(0);
    /* Unknown: OP_ADD */
    /* Unknown: GET_LOCAL 1 */
    { NUX_INT addr = POP(); NUX_INT val = POP(); vars[addr / 8] = val; }
    /* Unknown: OP_GET_LOCAL 0 */
    PUSH(1);
    /* Unknown: OP_ADD */
    /* Unknown: GET_LOCAL 2 */
    { NUX_INT addr = POP(); NUX_INT val = POP(); vars[addr / 8] = val; }
    /* Unknown: OP_GET_LOCAL 0 */
    PUSH(2);
    /* Unknown: OP_ADD */
    /* Unknown: GET_LOCAL 3 */
    { NUX_INT addr = POP(); NUX_INT val = POP(); vars[addr / 8] = val; }
    /* Unknown: GET_LOCAL 0 */
    return 0;
    PUSH(0);
    return 0;
skip_Vector3_init:
    goto skip_Camera3D_init;
Camera3D_init:
    /* Unknown: OP_GET_LOCAL 0 */
    PUSH(0);
    /* Unknown: OP_ADD */
    /* Unknown: GET_LOCAL 1 */
    { NUX_INT addr = POP(); NUX_INT val = POP(); vars[addr / 8] = val; }
    /* Unknown: OP_GET_LOCAL 0 */
    PUSH(1);
    /* Unknown: OP_ADD */
    /* Unknown: GET_LOCAL 2 */
    { NUX_INT addr = POP(); NUX_INT val = POP(); vars[addr / 8] = val; }
    /* Unknown: OP_GET_LOCAL 0 */
    PUSH(2);
    /* Unknown: OP_ADD */
    PUSH(4591870180066957722);
    { NUX_INT addr = POP(); NUX_INT val = POP(); vars[addr / 8] = val; }
    /* Unknown: OP_GET_LOCAL 0 */
    PUSH(3);
    /* Unknown: OP_ADD */
    PUSH(4652007308841189376);
    { NUX_INT addr = POP(); NUX_INT val = POP(); vars[addr / 8] = val; }
    /* Unknown: GET_LOCAL 0 */
    return 0;
    PUSH(0);
    return 0;
skip_Camera3D_init:
    goto skip_Camera3D_project;
Camera3D_project:
    PUSH(2);
    PUSH(1);
    /* Unknown: OP_IMG_ALLOC */
    /* Unknown: OP_GET_LOCAL 2 */
    PUSH(0);
    PUSH(0);
    /* Call p_init not fully implemented in C transpiler yet */
    /* Unknown: POP */
    /* Unknown: GET_LOCAL 1 */
    PUSH(2);
    /* Unknown: OP_ADD */
    { NUX_INT addr = POP(); PUSH(vars[addr / 8]); }
    /* Unknown: GET_LOCAL 3 */
    PUSH(1);
    { NUX_INT b = POP(); NUX_INT a = POP(); PUSH(a < b ? 1 : 0); }
    PUSH(0);
    { NUX_INT b = POP(); NUX_INT a = POP(); if (a == b) goto __if_else_10; }
    PUSH(1);
    /* Unknown: SET_LOCAL 3 */
    goto __if_end_10;
__if_else_10:
__if_end_10:
    /* Unknown: GET_LOCAL 0 */
    PUSH(0);
    /* Unknown: OP_ADD */
    { NUX_INT addr = POP(); PUSH(vars[addr / 8]); }
    /* Unknown: GET_LOCAL 3 */
    { NUX_INT b = POP(); NUX_INT a = POP(); PUSH(a / b); }
    /* Unknown: OP_GET_LOCAL 2 */
    PUSH(0);
    /* Unknown: OP_ADD */
    /* Unknown: GET_LOCAL 1 */
    PUSH(0);
    /* Unknown: OP_ADD */
    { NUX_INT addr = POP(); PUSH(vars[addr / 8]); }
    /* Unknown: GET_LOCAL 4 */
    { NUX_INT b = POP(); NUX_INT a = POP(); PUSH(a * b); }
    /* Unknown: GET_LOCAL 0 */
    PUSH(1);
    /* Unknown: OP_ADD */
    { NUX_INT addr = POP(); PUSH(vars[addr / 8]); }
    { NUX_INT b = POP(); NUX_INT a = POP(); PUSH(a * b); }
    { NUX_INT addr = POP(); NUX_INT val = POP(); vars[addr / 8] = val; }
    /* Unknown: OP_GET_LOCAL 2 */
    PUSH(1);
    /* Unknown: OP_ADD */
    /* Unknown: GET_LOCAL 1 */
    PUSH(1);
    /* Unknown: OP_ADD */
    { NUX_INT addr = POP(); PUSH(vars[addr / 8]); }
    /* Unknown: GET_LOCAL 4 */
    { NUX_INT b = POP(); NUX_INT a = POP(); PUSH(a * b); }
    { NUX_INT addr = POP(); NUX_INT val = POP(); vars[addr / 8] = val; }
    /* Unknown: GET_LOCAL 2 */
    return 0;
    PUSH(0);
    return 0;
skip_Camera3D_project:
    goto skip_arr_new;
arr_new:
    /* Unknown: GET_LOCAL 0 */
    { NUX_INT size = POP(); PUSH((NUX_INT)nux_alloc(size)); }
    PUSH(0);
    return 0;
skip_arr_new:
    goto skip_arr_set;
arr_set:
    /* Unknown: GET_LOCAL 2 */
    /* Unknown: GET_LOCAL 0 */
    /* Unknown: GET_LOCAL 1 */
    /* Unknown: OP_ADD */
    { NUX_INT addr = POP(); NUX_INT val = POP(); vars[addr / 8] = val; }
    PUSH(0);
    return 0;
skip_arr_set:
    goto skip_arr_get;
arr_get:
    /* Unknown: GET_LOCAL 0 */
    /* Unknown: GET_LOCAL 1 */
    /* Unknown: OP_ADD */
    { NUX_INT addr = POP(); PUSH(vars[addr / 8]); }
    PUSH(0);
    return 0;
skip_arr_get:
    goto skip_main;
main:
    /* Call Window not fully implemented in C transpiler yet */
    /* Unknown: OP_GET_LOCAL 0 */
    PUSH(800);
    PUSH(600);
    /* Call win_init not fully implemented in C transpiler yet */
    /* Unknown: POP */
    /* Call Camera3D not fully implemented in C transpiler yet */
    /* Unknown: OP_GET_LOCAL 1 */
    PUSH(600);
    PUSH(1);
    /* Call cam_init not fully implemented in C transpiler yet */
    /* Unknown: POP */
    PUSH(8);
    /* Call arr_new not fully implemented in C transpiler yet */
    PUSH(8);
    /* Call arr_new not fully implemented in C transpiler yet */
    PUSH(8);
    /* Call arr_new not fully implemented in C transpiler yet */
    /* Unknown: GET_LOCAL 2 */
    PUSH(0);
    PUSH(100);
    /* Call arr_set not fully implemented in C transpiler yet */
    /* Unknown: POP */
    /* Unknown: GET_LOCAL 3 */
    PUSH(0);
    PUSH(0);
    /* Call arr_set not fully implemented in C transpiler yet */
    /* Unknown: POP */
    /* Unknown: GET_LOCAL 4 */
    PUSH(0);
    PUSH(500);
    /* Call arr_set not fully implemented in C transpiler yet */
    /* Unknown: POP */
    /* Unknown: GET_LOCAL 2 */
    PUSH(1);
    PUSH(70);
    /* Call arr_set not fully implemented in C transpiler yet */
    /* Unknown: POP */
    /* Unknown: GET_LOCAL 3 */
    PUSH(1);
    PUSH(70);
    /* Call arr_set not fully implemented in C transpiler yet */
    /* Unknown: POP */
    /* Unknown: GET_LOCAL 4 */
    PUSH(1);
    PUSH(500);
    /* Call arr_set not fully implemented in C transpiler yet */
    /* Unknown: POP */
    /* Unknown: GET_LOCAL 2 */
    PUSH(2);
    PUSH(0);
    /* Call arr_set not fully implemented in C transpiler yet */
    /* Unknown: POP */
    /* Unknown: GET_LOCAL 3 */
    PUSH(2);
    PUSH(100);
    /* Call arr_set not fully implemented in C transpiler yet */
    /* Unknown: POP */
    /* Unknown: GET_LOCAL 4 */
    PUSH(2);
    PUSH(500);
    /* Call arr_set not fully implemented in C transpiler yet */
    /* Unknown: POP */
    /* Unknown: GET_LOCAL 2 */
    PUSH(3);
    PUSH(-70);
    /* Call arr_set not fully implemented in C transpiler yet */
    /* Unknown: POP */
    /* Unknown: GET_LOCAL 3 */
    PUSH(3);
    PUSH(70);
    /* Call arr_set not fully implemented in C transpiler yet */
    /* Unknown: POP */
    /* Unknown: GET_LOCAL 4 */
    PUSH(3);
    PUSH(500);
    /* Call arr_set not fully implemented in C transpiler yet */
    /* Unknown: POP */
    /* Unknown: GET_LOCAL 2 */
    PUSH(4);
    PUSH(-100);
    /* Call arr_set not fully implemented in C transpiler yet */
    /* Unknown: POP */
    /* Unknown: GET_LOCAL 3 */
    PUSH(4);
    PUSH(0);
    /* Call arr_set not fully implemented in C transpiler yet */
    /* Unknown: POP */
    /* Unknown: GET_LOCAL 4 */
    PUSH(4);
    PUSH(500);
    /* Call arr_set not fully implemented in C transpiler yet */
    /* Unknown: POP */
    /* Unknown: GET_LOCAL 2 */
    PUSH(5);
    PUSH(-70);
    /* Call arr_set not fully implemented in C transpiler yet */
    /* Unknown: POP */
    /* Unknown: GET_LOCAL 3 */
    PUSH(5);
    PUSH(-70);
    /* Call arr_set not fully implemented in C transpiler yet */
    /* Unknown: POP */
    /* Unknown: GET_LOCAL 4 */
    PUSH(5);
    PUSH(500);
    /* Call arr_set not fully implemented in C transpiler yet */
    /* Unknown: POP */
    /* Unknown: GET_LOCAL 2 */
    PUSH(6);
    PUSH(0);
    /* Call arr_set not fully implemented in C transpiler yet */
    /* Unknown: POP */
    /* Unknown: GET_LOCAL 3 */
    PUSH(6);
    PUSH(-100);
    /* Call arr_set not fully implemented in C transpiler yet */
    /* Unknown: POP */
    /* Unknown: GET_LOCAL 4 */
    PUSH(6);
    PUSH(500);
    /* Call arr_set not fully implemented in C transpiler yet */
    /* Unknown: POP */
    /* Unknown: GET_LOCAL 2 */
    PUSH(7);
    PUSH(70);
    /* Call arr_set not fully implemented in C transpiler yet */
    /* Unknown: POP */
    /* Unknown: GET_LOCAL 3 */
    PUSH(7);
    PUSH(-70);
    /* Call arr_set not fully implemented in C transpiler yet */
    /* Unknown: POP */
    /* Unknown: GET_LOCAL 4 */
    PUSH(7);
    PUSH(500);
    /* Call arr_set not fully implemented in C transpiler yet */
    /* Unknown: POP */
    PUSH(0);
    PUSH(8);
    /* Call Vector3 not fully implemented in C transpiler yet */
__while_start_11:
    /* Unknown: GET_LOCAL 5 */
    PUSH(500);
    { NUX_INT b = POP(); NUX_INT a = POP(); PUSH(a < b ? 1 : 0); }
    PUSH(0);
    { NUX_INT b = POP(); NUX_INT a = POP(); if (a == b) goto __while_end_11; }
    /* Unknown: OP_GET_LOCAL 0 */
    PUSH(0);
    /* Call win_clear not fully implemented in C transpiler yet */
    /* Unknown: POP */
    PUSH(0);
__while_start_12:
    /* Unknown: GET_LOCAL 8 */
    /* Unknown: GET_LOCAL 6 */
    { NUX_INT b = POP(); NUX_INT a = POP(); PUSH(a < b ? 1 : 0); }
    PUSH(0);
    { NUX_INT b = POP(); NUX_INT a = POP(); if (a == b) goto __while_end_12; }
    PUSH(500);
    /* Unknown: GET_LOCAL 5 */
    PUSH(300);
    /* Unknown: MOD */
    { NUX_INT b = POP(); NUX_INT a = POP(); PUSH(a - b); }
    /* Unknown: GET_LOCAL 2 */
    /* Unknown: GET_LOCAL 8 */
    /* Call arr_get not fully implemented in C transpiler yet */
    /* Unknown: GET_LOCAL 3 */
    /* Unknown: GET_LOCAL 8 */
    /* Call arr_get not fully implemented in C transpiler yet */
    /* Unknown: OP_GET_LOCAL 7 */
    /* Unknown: GET_LOCAL 10 */
    /* Unknown: GET_LOCAL 11 */
    /* Unknown: GET_LOCAL 9 */
    /* Call pt_init not fully implemented in C transpiler yet */
    /* Unknown: POP */
    /* Unknown: GET_LOCAL 1 */
    /* Unknown: GET_LOCAL 7 */
    /* Call Camera3D_project not fully implemented in C transpiler yet */
    /* Unknown: GET_LOCAL 12 */
    PUSH(0);
    /* Unknown: OP_ADD */
    { NUX_INT addr = POP(); PUSH(vars[addr / 8]); }
    PUSH(400);
    { NUX_INT b = POP(); NUX_INT a = POP(); PUSH(a + b); }
    /* Unknown: GET_LOCAL 12 */
    PUSH(1);
    /* Unknown: OP_ADD */
    { NUX_INT addr = POP(); PUSH(vars[addr / 8]); }
    PUSH(300);
    { NUX_INT b = POP(); NUX_INT a = POP(); PUSH(a + b); }
    /* Unknown: OP_GET_LOCAL 0 */
    /* Unknown: GET_LOCAL 13 */
    /* Unknown: GET_LOCAL 14 */
    PUSH(16711935);
    /* Call win_draw_pixel not fully implemented in C transpiler yet */
    /* Unknown: POP */
    /* Unknown: OP_GET_LOCAL 0 */
    /* Unknown: GET_LOCAL 13 */
    PUSH(1);
    { NUX_INT b = POP(); NUX_INT a = POP(); PUSH(a + b); }
    /* Unknown: GET_LOCAL 14 */
    PUSH(16711935);
    /* Call win_draw_pixel not fully implemented in C transpiler yet */
    /* Unknown: POP */
    /* Unknown: OP_GET_LOCAL 0 */
    /* Unknown: GET_LOCAL 13 */
    PUSH(1);
    { NUX_INT b = POP(); NUX_INT a = POP(); PUSH(a - b); }
    /* Unknown: GET_LOCAL 14 */
    PUSH(16711935);
    /* Call win_draw_pixel not fully implemented in C transpiler yet */
    /* Unknown: POP */
    /* Unknown: OP_GET_LOCAL 0 */
    /* Unknown: GET_LOCAL 13 */
    /* Unknown: GET_LOCAL 14 */
    PUSH(1);
    { NUX_INT b = POP(); NUX_INT a = POP(); PUSH(a + b); }
    PUSH(16711935);
    /* Call win_draw_pixel not fully implemented in C transpiler yet */
    /* Unknown: POP */
    /* Unknown: OP_GET_LOCAL 0 */
    /* Unknown: GET_LOCAL 13 */
    /* Unknown: GET_LOCAL 14 */
    PUSH(1);
    { NUX_INT b = POP(); NUX_INT a = POP(); PUSH(a - b); }
    PUSH(16711935);
    /* Call win_draw_pixel not fully implemented in C transpiler yet */
    /* Unknown: POP */
    /* Unknown: GET_LOCAL 8 */
    PUSH(1);
    { NUX_INT b = POP(); NUX_INT a = POP(); PUSH(a + b); }
    /* Unknown: SET_LOCAL 8 */
    goto __while_start_12;
__while_end_12:
    /* Unknown: OP_GET_LOCAL 0 */
    /* Call win_update not fully implemented in C transpiler yet */
    /* Unknown: POP */
    /* Unknown: GET_LOCAL 5 */
    PUSH(1);
    { NUX_INT b = POP(); NUX_INT a = POP(); PUSH(a + b); }
    /* Unknown: SET_LOCAL 5 */
    goto __while_start_11;
__while_end_11:
    PUSH(0);
    return 0;
skip_main:
__start_execution:
    NUX_EXIT();
    return 0;
}
