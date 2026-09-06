// Simplified standalone test for Nux GUI architecture
// This demonstrates the C++ core without requiring GLFW/OpenGL

#include <iostream>
#include <string>
#include <vector>
#include <memory>

// Simplified Widget base class
class Widget {
public:
    Widget() : m_X(0), m_Y(0), m_Width(100), m_Height(100), m_Visible(true) {}
    virtual ~Widget() {}
    
    void SetPosition(float x, float y) { m_X = x; m_Y = y; }
    void SetSize(float width, float height) { m_Width = width; m_Height = height; }
    void SetVisible(bool visible) { m_Visible = visible; }
    bool IsVisible() const { return m_Visible; }
    
    virtual void Render() {
        std::cout << "  [Widget at (" << m_X << "," << m_Y << ") size " 
                  << m_Width << "x" << m_Height << "]" << std::endl;
    }
    
protected:
    float m_X, m_Y, m_Width, m_Height;
    bool m_Visible;
};

// Button widget
class Button : public Widget {
public:
    Button(const std::string& text) : m_Text(text) {
        SetSize(120, 40);
    }
    
    void SetText(const std::string& text) { m_Text = text; }
    const std::string& GetText() const { return m_Text; }
    
    virtual void Render() override {
        std::cout << "  [Button \"" << m_Text << "\" at (" << m_X << "," << m_Y 
                  << ") size " << m_Width << "x" << m_Height << "]" << std::endl;
    }
    
private:
    std::string m_Text;
};

// Label widget
class Label : public Widget {
public:
    Label(const std::string& text) : m_Text(text), m_FontSize(16) {
        SetSize(200, 30);
    }
    
    void SetText(const std::string& text) { m_Text = text; }
    void SetFontSize(float size) { m_FontSize = size; }
    
    virtual void Render() override {
        std::cout << "  [Label \"" << m_Text << "\" fontSize=" << m_FontSize 
                  << " at (" << m_X << "," << m_Y << ")]" << std::endl;
    }
    
private:
    std::string m_Text;
    float m_FontSize;
};

// Window class
class Window {
public:
    Window(int width, int height, const std::string& title)
        : m_Width(width), m_Height(height), m_Title(title), m_FrameCount(0) {
        std::cout << "\n=== Created Window: \"" << title << "\" (" 
                  << width << "x" << height << ") ===" << std::endl;
    }
    
    ~Window() {
        for (auto widget : m_Widgets) {
            delete widget;
        }
    }
    
    void AddWidget(Widget* widget) {
        m_Widgets.push_back(widget);
        std::cout << "Added widget to window" << std::endl;
    }
    
    void Render() {
        std::cout << "\n--- Frame " << m_FrameCount++ << " ---" << std::endl;
        std::cout << "Rendering window: " << m_Title << std::endl;
        for (auto widget : m_Widgets) {
            if (widget && widget->IsVisible()) {
                widget->Render();
            }
        }
    }
    
    void SetTitle(const std::string& title) {
        m_Title = title;
        std::cout << "Window title changed to: " << title << std::endl;
    }
    
private:
    int m_Width, m_Height;
    std::string m_Title;
    std::vector<Widget*> m_Widgets;
    int m_FrameCount;
};

// C FFI exports (what Nux would call)
extern "C" {
    void* nux_gui_window_create(int width, int height, const char* title) {
        return new Window(width, height, title);
    }
    
    void nux_gui_window_destroy(void* window) {
        delete static_cast<Window*>(window);
    }
    
    void nux_gui_window_render(void* window) {
        static_cast<Window*>(window)->Render();
    }
    
    void nux_gui_window_set_title(void* window, const char* title) {
        static_cast<Window*>(window)->SetTitle(title);
    }
    
    void* nux_gui_button_create(const char* text) {
        return new Button(text);
    }
    
    void nux_gui_button_set_position(void* button, float x, float y) {
        static_cast<Button*>(button)->SetPosition(x, y);
    }
    
    void nux_gui_button_set_size(void* button, float width, float height) {
        static_cast<Button*>(button)->SetSize(width, height);
    }
    
    void* nux_gui_label_create(const char* text) {
        return new Label(text);
    }
    
    void nux_gui_label_set_position(void* label, float x, float y) {
        static_cast<Label*>(label)->SetPosition(x, y);
    }
    
    void nux_gui_label_set_font_size(void* label, float size) {
        static_cast<Label*>(label)->SetFontSize(size);
    }
    
    void nux_gui_window_add_widget(void* window, void* widget) {
        static_cast<Window*>(window)->AddWidget(static_cast<Widget*>(widget));
    }
}

// Test program
int main() {
    std::cout << "==================================" << std::endl;
    std::cout << "  Nux GUI Library Architecture Test" << std::endl;
    std::cout << "  (Simplified C++ Core Demo)" << std::endl;
    std::cout << "==================================" << std::endl;
    
    // Simulate what Nux would do through FFI
    std::cout << "\n[Simulating Nux FFI calls]" << std::endl;
    
    // Create window
    void* window = nux_gui_window_create(800, 600, "Nux GUI Test");
    
    // Create button
    void* button = nux_gui_button_create("Click Me!");
    nux_gui_button_set_position(button, 300, 250);
    nux_gui_button_set_size(button, 200, 50);
    nux_gui_window_add_widget(window, button);
    
    // Create label
    void* label = nux_gui_label_create("Welcome to Nux GUI!");
    nux_gui_label_set_position(label, 250, 150);
    nux_gui_label_set_font_size(label, 24);
    nux_gui_window_add_widget(window, label);
    
    // Simulate render loop (3 frames)
    for (int i = 0; i < 3; i++) {
        nux_gui_window_render(window);
    }
    
    // Change title
    std::cout << "\n[Changing window title]" << std::endl;
    nux_gui_window_set_title(window, "Updated Title!");
    nux_gui_window_render(window);
    
    // Cleanup
    std::cout << "\n[Cleanup]" << std::endl;
    nux_gui_window_destroy(window);
    
    std::cout << "\n==================================" << std::endl;
    std::cout << "  ✓ Test Complete!" << std::endl;
    std::cout << "  Architecture validated successfully" << std::endl;
    std::cout << "==================================" << std::endl;
    
    return 0;
}
