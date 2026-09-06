#include "nux_gui/window.h"
#include "nux_gui/renderer.h"
#include "nux_gui/widget.h"
#include <GLFW/glfw3.h>
#include <stdexcept>

namespace NuxGUI {

Window::Window(int width, int height, const std::string& title)
    : m_Window(nullptr)
    , m_Width(width)
    , m_Height(height)
    , m_Title(title)
{
    // Create GLFW window
    glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);
    glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);
    
    m_Window = glfwCreateWindow(width, height, title.c_str(), nullptr, nullptr);
    if (!m_Window) {
        throw std::runtime_error("Failed to create GLFW window");
    }
    
    glfwMakeContextCurrent(m_Window);
    glfwSwapInterval(1); // Enable vsync
    
    // Create renderer
    m_Renderer = std::make_unique<Renderer>();
    if (!m_Renderer->Initialize()) {
        throw std::runtime_error("Failed to initialize renderer");
    }
}

Window::~Window() {
    m_Renderer->Shutdown();
    if (m_Window) {
        glfwDestroyWindow(m_Window);
    }
}

bool Window::ShouldClose() const {
    return glfwWindowShouldClose(m_Window);
}

void Window::PollEvents() {
    glfwPollEvents();
}

void Window::SwapBuffers() {
    glfwSwapBuffers(m_Window);
}

void Window::Close() {
    glfwSetWindowShouldClose(m_Window, GLFW_TRUE);
}

void Window::SetTitle(const std::string& title) {
    m_Title = title;
    glfwSetWindowTitle(m_Window, title.c_str());
}

void Window::SetSize(int width, int height) {
    m_Width = width;
    m_Height = height;
    glfwSetWindowSize(m_Window, width, height);
}

void Window::GetSize(int& width, int& height) const {
    glfwGetWindowSize(m_Window, &width, &height);
}

void Window::SetPosition(int x, int y) {
    glfwSetWindowPos(m_Window, x, y);
}

void Window::GetPosition(int& x, int& y) const {
    glfwGetWindowPos(m_Window, &x, &y);
}

void Window::Clear(float r, float g, float b, float a) {
    m_Renderer->Clear(r, g, b, a);
}

void Window::Render() {
    m_Renderer->BeginFrame();
    
    // Render all widgets
    for (Widget* widget : m_Widgets) {
        if (widget && widget->IsVisible()) {
            widget->Render(m_Renderer.get());
        }
    }
    
    m_Renderer->EndFrame();
}

void Window::AddWidget(Widget* widget) {
    if (widget) {
        m_Widgets.push_back(widget);
    }
}

void Window::RemoveWidget(Widget* widget) {
    auto it = std::find(m_Widgets.begin(), m_Widgets.end(), widget);
    if (it != m_Widgets.end()) {
        m_Widgets.erase(it);
    }
}

} // namespace NuxGUI
