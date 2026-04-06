#include "vm.h"
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <termios.h>
#include <fcntl.h>
#include <time.h>
#include <string.h>

// --- Terminal Runtime (Ported) ---
struct termios orig_termios;
int input_key = -1;

void disableRawMode() {
    tcsetattr(STDIN_FILENO, TCSAFLUSH, &orig_termios);
    printf("\033[?25h");
}
void enableRawMode() {
    tcgetattr(STDIN_FILENO, &orig_termios);
    atexit(disableRawMode);
    struct termios raw = orig_termios;
    raw.c_lflag &= ~(ECHO | ICANON);
    raw.c_cc[VMIN] = 0; raw.c_cc[VTIME] = 0;
    tcsetattr(STDIN_FILENO, TCSAFLUSH, &raw);
    printf("\033[?25l"); 
    printf("\033[2J"); // Clear
}

void process_input() {
    char c;
    if (read(STDIN_FILENO, &c, 1) == 1) {
        if (c == 27) {
            char seq[2];
            if (read(STDIN_FILENO, &seq[0], 1) == 1 && read(STDIN_FILENO, &seq[1], 1) == 1) {
                if (seq[0] == '[') {
                    switch (seq[1]) {
                        case 'A': input_key = 0; break; 
                        case 'B': input_key = 1; break; 
                        case 'C': input_key = 3; break; 
                        case 'D': input_key = 2; break; 
                    }
                }
            } else { input_key = 5; } // Esc
        } else {
            if (c == 'w') input_key = 0;
            if (c == 's') input_key = 1;
            if (c == 'a') input_key = 2;
            if (c == 'd') input_key = 3;
            if (c == 'q') input_key = 5;
        }
    }
}

// --- Ext Impl ---
void ext_print(nux_int val) { printf("%ld\n", val); fflush(stdout); }
void ext_print_char(char c) { printf("%c", c); fflush(stdout); }
void ext_sleep(int ms) {
    usleep(ms * 1000);
    input_key = -1; 
}
int ext_is_key_down(int key) {
    process_input();
    return (input_key == key);
}

// Image struct for Runtime
typedef struct {
    int w, h;
    uint32_t* pixels;
} Img;

void ext_img_alloc(VM* vm) {
    Value h = pop(vm); Value w = pop(vm);
    
    Img* img = malloc(sizeof(Img));
    img->w = w.as.i;
    img->h = h.as.i;
    img->pixels = malloc(sizeof(uint32_t) * img->w * img->h);
    
    Value res = { .type = VAL_PTR, .as.p = img };
    push(vm, res);
}

void ext_img_set(VM* vm) {
    Value col = pop(vm);
    Value y = pop(vm);
    Value x = pop(vm);
    Value handle = pop(vm);
    
    Img* img = (Img*)handle.as.p;
    if (img && x.as.i >= 0 && x.as.i < img->w && y.as.i >= 0 && y.as.i < img->h) {
        img->pixels[y.as.i * img->w + x.as.i] = (uint32_t)col.as.i;
    }
}

void ext_img_fill(VM* vm) {
    Value col = pop(vm);
    Value handle = pop(vm);
    Img* img = (Img*)handle.as.p;
    if (img) {
        for(int i=0; i<img->w*img->h; i++) img->pixels[i] = col.as.i;
    }
}

uint32_t ext_img_get_pixel(void* handle, int x, int y) {
    Img* img = (Img*)handle;
    if (img && x >= 0 && x < img->w && y >= 0 && y < img->h) {
        return img->pixels[y * img->w + x];
    }
    return 0;
}

uint32_t* ext_img_get_buffer(void* handle, int* w, int* h) {
    Img* img = (Img*)handle;
    if (img) {
        *w = img->w;
        *h = img->h;
        return img->pixels;
    }
    return NULL;
}


void ext_img_draw(VM* vm) {
    Value y = pop(vm); Value x = pop(vm); Value handle = pop(vm);
    (void)x; (void)y;
    Img* img = (Img*)handle.as.p;
    if (!img) return;
    
    // ANSI Render
    printf("\033[H");
    for(int gy = 0; gy < 24; gy++) {
        for(int gx = 0; gx < 32; gx++) {
            int px = gx * 20 + 10;
            int py = gy * 20 + 10;
            if (px < img->w && py < img->h) {
                uint32_t c = img->pixels[py * img->w + px];
                if ((c & 0xFFFFFF) == 0) printf("\033[40m  ");
                else if ((c & 0xFF0000) > 0x800000) printf("\033[41m  ");
                else if ((c & 0x00FF00) > 0x8000) printf("\033[42m  ");
                else printf("\033[47m  ");
            }
        }
        printf("\033[0m\n");
    }
    fflush(stdout);
}

// --- Legacy GFX Port (ANSI) ---

void ext_gfx_rect(VM* vm) {
    Value col = pop(vm);
    Value h = pop(vm);
    Value w = pop(vm);
    Value y = pop(vm);
    Value x = pop(vm);
    
    // Convert color int to nearest ANSI (Simple logic)
    // 0xFFFFFF -> White (47), 0x000000 -> Black (40)
    // 0xFF0000 (Red) -> 41
    // 0x00FF00 (Green) -> 42
    // 0x0000FF (Blue) -> 44
    
    int c = (int)col.as.i;
    int ansi_bg = 40; // Default Black
    if ((c & 0xFF0000) > 0x800000) ansi_bg = 41; // Red
    else if ((c & 0x00FF00) > 0x8000) ansi_bg = 42; // Green
    else if ((c & 0x0000FF) > 0x80) ansi_bg = 44; // Blue
    else if (c > 0x808080) ansi_bg = 47; // White/Light
    
    // Draw using ANSI cursor
    // Term coords: 1,1 based.
    // Scale? Legacy 800x600? Terminal 80x24.
    // Scale down by 10/20?
    // Let's assume script uses virtual pixels and we scale / 10 for X, / 20 for Y.
    
    int tx = (int)x.as.i / 10 + 1;
    int ty = (int)y.as.i / 20 + 1;
    int tw = (int)w.as.i / 10;
    int th = (int)h.as.i / 20;
    
    if (tw < 1) tw = 1;
    if (th < 1) th = 1;
    
    printf("\033[%dm", ansi_bg);
    for (int i = 0; i < th; i++) {
        printf("\033[%d;%dH", ty + i, tx);
        for(int j=0; j<tw; j++) printf(" ");
    }
    printf("\033[0m\n"); // Reset
    fflush(stdout);
}

void ext_gfx_text(VM* vm) {
    Value col = pop(vm); // Color
    Value str = pop(vm); // String ptr
    Value y = pop(vm);
    Value x = pop(vm);
    
    // Extract String
    // Logic: In Nux Portable, strings are usually pointers to data segment.
    // The Value type might be VAL_PTR or VAL_STRING.
    char* s = NULL;
    if (str.type == VAL_STRING) s = str.as.s;
    else if (str.type == VAL_PTR) s = (char*)str.as.p;
    else if (str.type == VAL_INT) s = (char*)(uintptr_t)str.as.i; // Unsafe cast if passed as int addr
    
    if (!s) { printf("Text: <null>\n"); return; }
    
    // Scale coords
    int tx = (int)x.as.i / 10 + 1;
    int ty = (int)y.as.i / 20 + 1;
    
    // Color
    int c = (int)col.as.i;
    int ansi_fg = 37; // White
    if ((c & 0xFF0000) > 0x800000) ansi_fg = 31;
    else if ((c & 0x00FF00) > 0x8000) ansi_fg = 32;
    else if ((c & 0x0000FF) > 0x80) ansi_fg = 34;
    else if (c < 0x404040) ansi_fg = 30; // Black
    
    printf("\033[%d;%dH\033[%dm%s\033[0m", ty, tx, ansi_fg, s);
    fflush(stdout);
}

// --- Main ---
int main(int argc, char** argv) {
    if (argc < 2) {
        printf("Usage: nux run <file.nuxi>\n");
        return 1;
    }
    
    // Resolve Exe Directory
    char self_path[1024];
    ssize_t len = readlink("/proc/self/exe", self_path, sizeof(self_path)-1);
    if (len != -1) {
        self_path[len] = '\0';
        char* last_slash = strrchr(self_path, '/');
        if (last_slash) *last_slash = '\0'; // Truncate to dir
    } else {
        strcpy(self_path, "."); // Fallback
    }

    // CLI Proxy Logic
    char* cmd_arg = argv[1];
    
    // Pass-through commands to nuxc (Rust Compiler)
    if (strcmp(cmd_arg, "build") == 0 || 
        strcmp(cmd_arg, "compile") == 0 || 
        strcmp(cmd_arg, "edit") == 0 || 
        strcmp(cmd_arg, "version") == 0 || 
        strcmp(cmd_arg, "translate") == 0 ||
        strcmp(cmd_arg, "update") == 0) {
        
        char cmd[2048];
        snprintf(cmd, 2048, "%s/nuxc", self_path);
        for(int i=1; i<argc; i++) {
            strncat(cmd, " ", 2048 - strlen(cmd) - 1);
            strncat(cmd, argv[i], 2048 - strlen(cmd) - 1);
        }
        return system(cmd);
    }

    // Handle "run" explicitly or implicit file
    char* filename = cmd_arg;
    if (strcmp(cmd_arg, "run") == 0) {
        if (argc < 3) { printf("Usage: nux run <file>\n"); return 1; }
        filename = argv[2];
    }

    // Check for source file compilation
    char* ext = strrchr(filename, '.');
    char out_path[256];
    
    if (ext && strcmp(ext, ".nux") == 0) {
        // Explicit Output Path
        snprintf(out_path, 256, "%s.nuxi", filename);
        
        char cmd[2048];
        snprintf(cmd, 2048, "%s/nuxc build %s %s", self_path, filename, out_path);
        
        int res = system(cmd);
        if (res != 0) {
            printf("Compilation Failed.\n");
            return 1;
        }
        
        filename = out_path; // Switch target to compiled bytecode
        printf("Nux: Loading Bytecode %s...\n", filename);
    }
    
    FILE* f = fopen(filename, "rb");
    if (!f) { printf("Cannot open %s\n", filename); return 1; }
    
    fseek(f, 0, SEEK_END);
    long size = ftell(f);
    fseek(f, 0, SEEK_SET);
    
    uint8_t* code = malloc(size);
    fread(code, 1, size, f);
    fclose(f);
    
    // Check Header "ANUX"
    if (size < 64 || memcmp(code, "ANUX", 4) != 0) {
        printf("Invalid Nux Binary (Missing ANUX header)\n");
        // return 1; // Strict check disabled for now if rust compiler mismatch
    }
    
    // Offset 64 is code start
    VM vm;
    vm_init(&vm);
    // enableRawMode();
    vm_load(&vm, code, size, 64);
    vm_run(&vm);
    
    return 0;
}
