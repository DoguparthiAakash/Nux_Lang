// FFI bindings for Nux language
// This file exports C functions that can be called from Nux

#include "nux_gui/nux_gui.h"
#include <GLFW/glfw3.h>

extern "C" {

// Library initialization
bool nux_gui_initialize() {
    return glfwInit() == GLFW_TRUE;
}

void nux_gui_shutdown() {
    glfwTerminate();
}

void nux_gui_poll_events() {
    glfwPollEvents();
}

// Window functions
void* nux_gui_window_create(int width, int height, const char* title) {
    try {
        return new NuxGUI::Window(width, height, title);
    } catch (...) {
        return nullptr;
    }
}

void nux_gui_window_destroy(void* window) {
    if (window) {
        delete static_cast<NuxGUI::Window*>(window);
    }
}

bool nux_gui_window_should_close(void* window) {
    if (!window) return true;
    return static_cast<NuxGUI::Window*>(window)->ShouldClose();
}

void nux_gui_window_swap_buffers(void* window) {
    if (window) {
        static_cast<NuxGUI::Window*>(window)->SwapBuffers();
    }
}

void nux_gui_window_clear(void* window, float r, float g, float b, float a) {
    if (window) {
        static_cast<NuxGUI::Window*>(window)->Clear(r, g, b, a);
    }
}

void nux_gui_window_render(void* window) {
    if (window) {
        static_cast<NuxGUI::Window*>(window)->Render();
    }
}

void nux_gui_window_set_title(void* window, const char* title) {
    if (window && title) {
        static_cast<NuxGUI::Window*>(window)->SetTitle(title);
    }
}

// Button functions
void* nux_gui_button_create(const char* text) {
    return new NuxGUI::Button(text ? text : "");
}

void nux_gui_button_destroy(void* button) {
    if (button) {
        delete static_cast<NuxGUI::Button*>(button);
    }
}

void nux_gui_button_set_text(void* button, const char* text) {
    if (button && text) {
        static_cast<NuxGUI::Button*>(button)->SetText(text);
    }
}

void nux_gui_button_set_position(void* button, float x, float y) {
    if (button) {
        static_cast<NuxGUI::Button*>(button)->SetPosition(x, y);
    }
}

void nux_gui_button_set_size(void* button, float width, float height) {
    if (button) {
        static_cast<NuxGUI::Button*>(button)->SetSize(width, height);
    }
}

// Label functions
void* nux_gui_label_create(const char* text) {
    return new NuxGUI::Label(text ? text : "");
}

void nux_gui_label_destroy(void* label) {
    if (label) {
        delete static_cast<NuxGUI::Label*>(label);
    }
}

void nux_gui_label_set_text(void* label, const char* text) {
    if (label && text) {
        static_cast<NuxGUI::Label*>(label)->SetText(text);
    }
}

void nux_gui_label_set_position(void* label, float x, float y) {
    if (label) {
        static_cast<NuxGUI::Label*>(label)->SetPosition(x, y);
    }
}

void nux_gui_label_set_font_size(void* label, float size) {
    if (label) {
        static_cast<NuxGUI::Label*>(label)->SetFontSize(size);
    }
}

// Widget management
void nux_gui_window_add_widget(void* window, void* widget) {
    if (window && widget) {
        static_cast<NuxGUI::Window*>(window)->AddWidget(
            static_cast<NuxGUI::Widget*>(widget)
        );
    }
}

} // extern "C"
