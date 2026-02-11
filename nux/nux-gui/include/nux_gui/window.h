#ifndef NUX_GUI_WINDOW_H
#define NUX_GUI_WINDOW_H

#include <string>
#include <memory>

struct GLFWwindow;

namespace NuxGUI {
    
class Renderer;
class Widget;

class Window {
public:
    Window(int width, int height, const std::string& title);
    ~Window();
    
    // Window management
    bool ShouldClose() const;
    void PollEvents();
    void SwapBuffers();
    void Close();
    
    // Window properties
    void SetTitle(const std::string& title);
    void SetSize(int width, int height);
    void GetSize(int& width, int& height) const;
    void SetPosition(int x, int y);
    void GetPosition(int& x, int& y) const;
    
    // Rendering
    void Clear(float r, float g, float b, float a);
    void Render();
    
    // Widget management
    void AddWidget(Widget* widget);
    void RemoveWidget(Widget* widget);
    
    // Internal
    GLFWwindow* GetNativeHandle() const { return m_Window; }
    Renderer* GetRenderer() const { return m_Renderer.get(); }
    
private:
    GLFWwindow* m_Window;
    std::unique_ptr<Renderer> m_Renderer;
    std::vector<Widget*> m_Widgets;
    int m_Width, m_Height;
    std::string m_Title;
};

} // namespace NuxGUI

#endif // NUX_GUI_WINDOW_H
