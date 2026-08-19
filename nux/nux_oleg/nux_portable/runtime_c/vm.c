#include "vm.h"
#include "vision/vision.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

void vm_init(VM* vm) {
    vm->sp = 0;
    vm->csp = 0;
    vm->fp = 0;
    memset(vm->heap, 0, HEAP_SIZE);
    vm->heap_strings_count = 0;
    vm->heap_arrays_count = 0;
}

void vm_load(VM* vm, uint8_t* code, int size, int entry_offset) {
    vm->code = code;
    vm->code_size = size;
    vm->ip = code + entry_offset;
}

// Stack Helpers
void push(VM* vm, Value v) { vm->stack[vm->sp++] = v; }
Value pop(VM* vm) { return vm->stack[--vm->sp]; }
Value peek(VM* vm) { return vm->stack[vm->sp - 1]; }

// Read Helpers
uint8_t read_byte(VM* vm) { return *vm->ip++; }
int64_t read_long(VM* vm) {
    int64_t v = *(int64_t*)vm->ip;
    vm->ip += 8;
    return v;
}

void vm_run(VM* vm) {
    printf("VM: Running...\n");
    while (vm->ip < vm->code + vm->code_size) {
        uint8_t op = read_byte(vm);
        switch (op) {
            case OP_HALT: return;
            case OP_EXIT: return;
            
            case OP_PUSH: {
                int64_t val = read_long(vm);
                Value v = { .type = VAL_INT, .as.i = val };
                push(vm, v);
                break;
            }
            case OP_POP: pop(vm); break;
            case OP_SWAP: {
                Value a = pop(vm);
                Value b = pop(vm);
                push(vm, a);
                push(vm, b);
                break;
            }
            case OP_DUP: push(vm, peek(vm)); break;
            
            // Math
            // Math
            case OP_ADD: {
                Value b = pop(vm); Value a = pop(vm);
                if (a.type == VAL_FLOAT || b.type == VAL_FLOAT) {
                    double fa = (a.type == VAL_FLOAT) ? a.as.f : (double)a.as.i;
                    double fb = (b.type == VAL_FLOAT) ? b.as.f : (double)b.as.i;
                    push(vm, (Value){.type=VAL_FLOAT, .as.f=fa+fb});
                } else {
                    push(vm, (Value){.type=VAL_INT, .as.i=a.as.i + b.as.i});
                }
                break;
            }
            case OP_SUB: {
                Value b = pop(vm); Value a = pop(vm);
                if (a.type == VAL_FLOAT || b.type == VAL_FLOAT) {
                    double fa = (a.type == VAL_FLOAT) ? a.as.f : (double)a.as.i;
                    double fb = (b.type == VAL_FLOAT) ? b.as.f : (double)b.as.i;
                    push(vm, (Value){.type=VAL_FLOAT, .as.f=fa-fb});
                } else {
                    push(vm, (Value){.type=VAL_INT, .as.i=a.as.i - b.as.i});
                }
                break;
            }
            case OP_MUL: {
                Value b = pop(vm); Value a = pop(vm);
                if (a.type == VAL_FLOAT || b.type == VAL_FLOAT) {
                    double fa = (a.type == VAL_FLOAT) ? a.as.f : (double)a.as.i;
                    double fb = (b.type == VAL_FLOAT) ? b.as.f : (double)b.as.i;
                    push(vm, (Value){.type=VAL_FLOAT, .as.f=fa*fb});
                } else {
                    push(vm, (Value){.type=VAL_INT, .as.i=a.as.i * b.as.i});
                }
                break;
            }
            case OP_DIV: {
                Value b = pop(vm); Value a = pop(vm);
                if (a.type == VAL_FLOAT || b.type == VAL_FLOAT) {
                    double fa = (a.type == VAL_FLOAT) ? a.as.f : (double)a.as.i;
                    double fb = (b.type == VAL_FLOAT) ? b.as.f : (double)b.as.i;
                    if (fb == 0.0) { printf("Div by 0.0\n"); return; }
                    push(vm, (Value){.type=VAL_FLOAT, .as.f=fa/fb});
                } else {
                    if (b.as.i == 0) { printf("Div by 0\n"); return; }
                    push(vm, (Value){.type=VAL_INT, .as.i=a.as.i / b.as.i});
                }
                break;
            }
            
            // Comparison
            // Comparison
            case OP_GT: {
                Value b = pop(vm); Value a = pop(vm);
                int res = 0;
                if (a.type == VAL_FLOAT || b.type == VAL_FLOAT) {
                    double fa = (a.type == VAL_FLOAT) ? a.as.f : (double)a.as.i;
                    double fb = (b.type == VAL_FLOAT) ? b.as.f : (double)b.as.i;
                    res = (fa > fb);
                } else { res = (a.as.i > b.as.i); }
                push(vm, (Value){.type = VAL_INT, .as.i = res});
                break;
            }
            case OP_LT: {
                Value b = pop(vm); Value a = pop(vm);
                 int res = 0;
                if (a.type == VAL_FLOAT || b.type == VAL_FLOAT) {
                    double fa = (a.type == VAL_FLOAT) ? a.as.f : (double)a.as.i;
                    double fb = (b.type == VAL_FLOAT) ? b.as.f : (double)b.as.i;
                    res = (fa < fb);
                } else { res = (a.as.i < b.as.i); }
                push(vm, (Value){.type = VAL_INT, .as.i = res});
                break;
            }
            case OP_EQ: {
                Value b = pop(vm); Value a = pop(vm);
                int res = 0;
                if (a.type == VAL_FLOAT || b.type == VAL_FLOAT) {
                    double fa = (a.type == VAL_FLOAT) ? a.as.f : (double)a.as.i;
                    double fb = (b.type == VAL_FLOAT) ? b.as.f : (double)b.as.i;
                    res = (fa == fb);
                } else { res = (a.as.i == b.as.i); }
                push(vm, (Value){.type = VAL_INT, .as.i = res});
                break;
            }
            case OP_NEQ: {
                Value b = pop(vm); Value a = pop(vm);
                int res = 0;
                if (a.type == VAL_FLOAT || b.type == VAL_FLOAT) {
                    double fa = (a.type == VAL_FLOAT) ? a.as.f : (double)a.as.i;
                    double fb = (b.type == VAL_FLOAT) ? b.as.f : (double)b.as.i;
                    res = (fa != fb);
                } else { res = (a.as.i != b.as.i); }
                push(vm, (Value){.type = VAL_INT, .as.i = res});
                break;
            }
            
            // Control
            case OP_JMP: {
                int64_t offset = read_long(vm); // Relative or Absolute? Rust VM seems absolute?
                // Rust VM: `self.ip = target`.
                // Compiler emits 8 bytes absolute offset.
                // But compiler code: `(target_addr as i64)`. Yes absolute offset from start.
                vm->ip = vm->code + offset;
                break;
            }
            case OP_JMP_FALSE: {
                int64_t offset = read_long(vm);
                Value cond = pop(vm);
                if (cond.as.i == 0) {
                    vm->ip = vm->code + offset;
                }
                break;
            }
            case OP_CALL: { // CALL target, num_args
                int64_t target = read_long(vm);
                int64_t num_args = read_long(vm); // consume arg count
                vm->call_stack[vm->csp] = vm->ip;
                vm->fp_stack[vm->csp] = vm->fp;
                vm->csp++;
                vm->fp = vm->sp - num_args; // FP points to first arg
                vm->ip = vm->code + target;
                break;
            }
            case OP_RET: {
                if (vm->csp == 0) return;
                Value ret_val = pop(vm); // Return value
                vm->csp--;
                vm->sp = vm->fp; // Restore SP (pop args/locals)
                vm->fp = vm->fp_stack[vm->csp]; // Restore FP
                vm->ip = vm->call_stack[vm->csp]; // Restore IP
                push(vm, ret_val); // Push return value
                break;
            }
            
            // Locals
            case OP_GET_LOCAL: {
                int64_t offset = read_long(vm);
                push(vm, vm->stack[vm->fp + offset]);
                break;
            }
            case OP_SET_LOCAL: {
                int64_t offset = read_long(vm);
                Value v = pop(vm); // Read value
                vm->stack[vm->fp + offset] = v;
                // Don't push? `x = 5` is statement. But `x = 5` as expr?
                // Compiler: `x = expr`. `expr` is on stack. `SET_LOCAL` consumes it?
                // In Rust VM: `let v = self.pop(); self.stack[bp + idx] = v;`. Yes consumes.
                break;
            }
            
            case OP_PRINT: {
                Value v = pop(vm);
                ext_print(v.as.i);
                break;
            }
            case OP_PRINT_CHAR: {
                Value v = pop(vm);
                ext_print_char((char)v.as.i);
                break;
            }
            case OP_SLEEP: {
                Value v = pop(vm);
                ext_sleep((int)v.as.i);
                break;
            }
            
            // Memory (Heap)
            case OP_PEEK: { // PEEK(ptr) -> val
                Value ptr = pop(vm);
                if (ptr.as.i >= 0 && ptr.as.i < HEAP_SIZE - 8) {
                    int64_t val = *(int64_t*)(&vm->heap[ptr.as.i]);
                    push(vm, (Value){.type=VAL_INT, .as.i=val});
                } else { push(vm, (Value){.type=VAL_INT, .as.i=0}); }
                break;
            }
            case OP_POKE: { // POKE(ptr, val)
                Value val = pop(vm);
                Value ptr = pop(vm);
                if (ptr.as.i >= 0 && ptr.as.i < HEAP_SIZE - 8) {
                    *(int64_t*)(&vm->heap[ptr.as.i]) = val.as.i;
                }
                break;
            }
            
            // Graphics Hooks
            case OP_IMG_ALLOC: { // w, h -> handle
                Value h = pop(vm); Value w = pop(vm);
                // We return handle (pointer or index).
                // ext_img_alloc should handle logic and return handle.
                // But external hooks usually don't modify stack directly in this C design?
                // I'll update ext_img_alloc to return handle.
                // Wait, C wrapper: 
                // Push args before calling?
                push(vm, w); push(vm, h);
                ext_img_alloc(vm); // Pass VM to allow it to pop/push
                break;
            }
            case OP_IMG_SET: { // handle, x, y, col
                // Args are on stack. Logic:
                // pop color, pop y, pop x, pop handle.
                // Call ext.
                ext_img_set(vm);
                break;
            }
            case OP_IMG_GET: {
                Value y = pop(vm);
                Value x = pop(vm);
                Value handle = pop(vm);
                uint32_t col = ext_img_get_pixel(handle.as.p, (int)x.as.i, (int)y.as.i);
                push(vm, (Value){.type=VAL_INT, .as.i = col});
                break;
            }
            case OP_IMG_FILL: { // handle, col
                ext_img_fill(vm);
                break;
            }
            case OP_IMG_DRAW: { // handle, x, y
                ext_img_draw(vm);
                break;
            }
            case OP_IS_KEY_DOWN: { // key -> bool (0/1)
                Value key = pop(vm);
                int res = ext_is_key_down((int)key.as.i);
                push(vm, (Value){.type=VAL_INT, .as.i=res});
                break;
            }

            case OP_VISION_DETECT: {
               Value mode = pop(vm);
               Value handle = pop(vm);
               int w, h;
               uint32_t* buf = ext_img_get_buffer(handle.as.p, &w, &h);
               if (buf) {
                   vision_process((int)mode.as.i, buf, w, h);
               }
               push(vm, (Value){.type=VAL_INT, .as.i=1});
               break;
            }

            // Legacy GFX (ANSI Fallback)
            case OP_GFX_TEXT: {
                ext_gfx_text(vm);
                break;
            }
            case OP_GFX_RECT: {
                ext_gfx_rect(vm);
                break;
            }
            
            case OP_ARRAY_ALLOC: {
                Value count_val = pop(vm);
                int count = (int)count_val.as.i;
                if (vm->heap_arrays_count < 10000) {
                    nux_int* data = (nux_int*)malloc(sizeof(nux_int) * count);
                    for (int i = 0; i < count; i++) {
                        data[count - 1 - i] = pop(vm).as.i;
                    }
                    vm->heap_arrays[vm->heap_arrays_count].data = data;
                    vm->heap_arrays[vm->heap_arrays_count].size = count;
                    push(vm, (Value){.type = VAL_INT, .as.i = vm->heap_arrays_count});
                    vm->heap_arrays_count++;
                } else {
                    push(vm, (Value){.type = VAL_INT, .as.i = -1});
                }
                break;
            }
            case OP_ARRAY_GET: {
                Value idx_val = pop(vm);
                Value arr_id_val = pop(vm);
                int idx = (int)idx_val.as.i;
                int arr_id = (int)arr_id_val.as.i;
                if (arr_id >= 0 && arr_id < vm->heap_arrays_count) {
                    if (idx >= 0 && idx < vm->heap_arrays[arr_id].size) {
                        push(vm, (Value){.type = VAL_INT, .as.i = vm->heap_arrays[arr_id].data[idx]});
                    } else {
                        printf("Array index out of bounds: %d\n", idx);
                        exit(1);
                    }
                } else {
                    printf("Invalid array ID: %d\n", arr_id);
                    exit(1);
                }
                break;
            }
            case OP_ARRAY_SET: {
                Value val = pop(vm);
                Value idx_val = pop(vm);
                Value arr_id_val = pop(vm);
                int idx = (int)idx_val.as.i;
                int arr_id = (int)arr_id_val.as.i;
                if (arr_id >= 0 && arr_id < vm->heap_arrays_count) {
                    if (idx >= 0 && idx < vm->heap_arrays[arr_id].size) {
                        vm->heap_arrays[arr_id].data[idx] = val.as.i;
                    } else {
                        printf("Array index out of bounds: %d\n", idx);
                        exit(1);
                    }
                } else {
                    printf("Invalid array ID: %d\n", arr_id);
                    exit(1);
                }
                break;
            }
            case OP_FS_READ: {
                Value path_id_val = pop(vm);
                int path_id = (int)path_id_val.as.i;
                char* path = "";
                if (path_id >= 0 && path_id < vm->heap_strings_count) {
                    path = vm->heap_strings[path_id];
                }
                FILE* f = fopen(path, "rb");
                if (f) {
                    fseek(f, 0, SEEK_END);
                    long fsize = ftell(f);
                    fseek(f, 0, SEEK_SET);
                    char* content = (char*)malloc(fsize + 1);
                    fread(content, 1, fsize, f);
                    content[fsize] = '\0';
                    fclose(f);
                    
                    if (vm->heap_strings_count < 10000) {
                        vm->heap_strings[vm->heap_strings_count] = content;
                        push(vm, (Value){.type = VAL_INT, .as.i = vm->heap_strings_count});
                        vm->heap_strings_count++;
                    } else {
                        free(content);
                        push(vm, (Value){.type = VAL_INT, .as.i = -1});
                    }
                } else {
                    if (vm->heap_strings_count < 10000) {
                        vm->heap_strings[vm->heap_strings_count] = strdup("");
                        push(vm, (Value){.type = VAL_INT, .as.i = vm->heap_strings_count});
                        vm->heap_strings_count++;
                    } else {
                        push(vm, (Value){.type = VAL_INT, .as.i = -1});
                    }
                }
                break;
            }
            case OP_FS_WRITE: {
                Value data_id_val = pop(vm);
                Value path_id_val = pop(vm);
                int data_id = (int)data_id_val.as.i;
                int path_id = (int)path_id_val.as.i;
                char* path = "";
                char* data = "";
                if (path_id >= 0 && path_id < vm->heap_strings_count) path = vm->heap_strings[path_id];
                if (data_id >= 0 && data_id < vm->heap_strings_count) data = vm->heap_strings[data_id];
                FILE* f = fopen(path, "wb");
                int success = 0;
                if (f) {
                    fwrite(data, 1, strlen(data), f);
                    fclose(f);
                    success = 1;
                }
                push(vm, (Value){.type = VAL_INT, .as.i = success});
                break;
            }
            case OP_FS_EXISTS: {
                Value path_id_val = pop(vm);
                int path_id = (int)path_id_val.as.i;
                char* path = "";
                if (path_id >= 0 && path_id < vm->heap_strings_count) path = vm->heap_strings[path_id];
                FILE* f = fopen(path, "rb");
                int success = 0;
                if (f) {
                    fclose(f);
                    success = 1;
                }
                push(vm, (Value){.type = VAL_INT, .as.i = success});
                break;
            }
            case OP_OS_ENV: {
                Value key_id_val = pop(vm);
                int key_id = (int)key_id_val.as.i;
                char* key = "";
                if (key_id >= 0 && key_id < vm->heap_strings_count) key = vm->heap_strings[key_id];
                char* val = getenv(key);
                char* res = val ? strdup(val) : strdup("");
                if (vm->heap_strings_count < 10000) {
                    vm->heap_strings[vm->heap_strings_count] = res;
                    push(vm, (Value){.type = VAL_INT, .as.i = vm->heap_strings_count});
                    vm->heap_strings_count++;
                } else {
                    free(res);
                    push(vm, (Value){.type = VAL_INT, .as.i = -1});
                }
                break;
            }
            case 0x68: { // OP_PUSH_STR
                uint8_t next_op = read_u8(vm);
                if (next_op != 0x01) {
                    printf("Expected PUSH length after OP_PUSH_STR\n");
                    exit(1);
                }
                int len = (int)read_long(vm);
                char* s = (char*)malloc(len + 1);
                for (int i = 0; i < len; i++) {
                    s[i] = (char)read_u8(vm);
                }
                s[len] = '\0';
                if (vm->heap_strings_count < 10000) {
                    vm->heap_strings[vm->heap_strings_count] = s;
                    push(vm, (Value){.type = VAL_INT, .as.i = vm->heap_strings_count});
                    vm->heap_strings_count++;
                } else {
                    free(s);
                    push(vm, (Value){.type = VAL_INT, .as.i = -1});
                }
                break;
            }
            case 0x6B: { // OP_PRINT_STR
                Value id_val = pop(vm);
                int id = (int)id_val.as.i;
                if (id >= 0 && id < vm->heap_strings_count) {
                    printf("%s", vm->heap_strings[id]);
                }
                break;
            }
            case 0x6C: { // OP_STR_LEN
                Value id_val = pop(vm);
                int id = (int)id_val.as.i;
                if (id >= 0 && id < vm->heap_strings_count) {
                    push(vm, (Value){.type = VAL_INT, .as.i = strlen(vm->heap_strings[id])});
                } else {
                    push(vm, (Value){.type = VAL_INT, .as.i = 0});
                }
                break;
            }
            case 0x6D: { // OP_STR_CHAR
                Value idx_val = pop(vm);
                Value id_val = pop(vm);
                int idx = (int)idx_val.as.i;
                int id = (int)id_val.as.i;
                if (id >= 0 && id < vm->heap_strings_count) {
                    char* s = vm->heap_strings[id];
                    if (idx >= 0 && idx < strlen(s)) {
                        push(vm, (Value){.type = VAL_INT, .as.i = s[idx]});
                    } else {
                        push(vm, (Value){.type = VAL_INT, .as.i = 0});
                    }
                } else {
                    push(vm, (Value){.type = VAL_INT, .as.i = 0});
                }
                break;
            }
            case 0x6E: { // OP_STR_SUB
                Value end_val = pop(vm);
                Value start_val = pop(vm);
                Value id_val = pop(vm);
                int end = (int)end_val.as.i;
                int start = (int)start_val.as.i;
                int id = (int)id_val.as.i;
                
                char* sub = strdup("");
                if (id >= 0 && id < vm->heap_strings_count) {
                    char* s = vm->heap_strings[id];
                    int len = strlen(s);
                    if (start >= 0 && start <= len && end >= 0 && end <= len && start <= end) {
                        free(sub);
                        sub = (char*)malloc(end - start + 1);
                        strncpy(sub, s + start, end - start);
                        sub[end - start] = '\0';
                    }
                }
                
                if (vm->heap_strings_count < 10000) {
                    vm->heap_strings[vm->heap_strings_count] = sub;
                    push(vm, (Value){.type = VAL_INT, .as.i = vm->heap_strings_count});
                    vm->heap_strings_count++;
                } else {
                    free(sub);
                    push(vm, (Value){.type = VAL_INT, .as.i = -1});
                }
                break;
            }
            
            default:
                printf("Unknown Op: %02X\n", op);
                break;
        }
    }
}
