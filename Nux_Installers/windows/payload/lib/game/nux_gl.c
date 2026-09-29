#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <stdint.h>

/*
 * nux_gl.c
 * 
 * Mock/Stub implementation of a Graphics backend (like SDL2 + OpenGL/Vulkan)
 * for the Nux game library. Nux compiles and links against these bindings.
 */

typedef struct {
    uint32_t width;
    uint32_t height;
    char* title;
    bool is_open;
} NuxWindow;

NuxWindow* nux_gl_create_window(const char* title, uint32_t width, uint32_t height) {
    NuxWindow* win = (NuxWindow*)malloc(sizeof(NuxWindow));
    win->width = width;
    win->height = height;
    win->title = (char*)title;
    win->is_open = true;
    // printf("[NuxGL] Window Created: %s (%ux%u)\n", title, width, height);
    return win;
}

void nux_gl_destroy_window(NuxWindow* win) {
    if (win) {
        // printf("[NuxGL] Window Destroyed: %s\n", win->title);
        free(win);
    }
}

bool nux_gl_window_is_open(NuxWindow* win) {
    return win != NULL && win->is_open;
}

void nux_gl_poll_events(NuxWindow* win) {
    // Mock event polling
    // Normally this would be SDL_PollEvent or similar.
}

void nux_gl_clear_screen(uint8_t r, uint8_t g, uint8_t b, uint8_t a) {
    // printf("[NuxGL] Screen cleared to color (%u, %u, %u, %u)\n", r, g, b, a);
}

void nux_gl_swap_buffers(NuxWindow* win) {
    // printf("[NuxGL] Buffers swapped.\n");
}

/* Sprite & Rendering */
typedef struct {
    uint32_t texture_id;
    uint32_t width;
    uint32_t height;
} NuxTexture;

NuxTexture* nux_gl_load_texture(const char* filepath) {
    NuxTexture* tex = (NuxTexture*)malloc(sizeof(NuxTexture));
    tex->texture_id = rand() % 1000;
    tex->width = 128; // mock sizes
    tex->height = 128;
    // printf("[NuxGL] Texture loaded from %s (ID: %u)\n", filepath, tex->texture_id);
    return tex;
}

void nux_gl_draw_texture(NuxTexture* tex, float x, float y, float rotation, float scale) {
    // printf("[NuxGL] Drawing texture ID %u at (%.2f, %.2f)\n", tex->texture_id, x, y);
}

/* Input */
bool nux_gl_is_key_down(int keycode) {
    // Mock key check
    return false;
}
