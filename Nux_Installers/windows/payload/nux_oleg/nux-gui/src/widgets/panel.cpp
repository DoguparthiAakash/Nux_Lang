#include "nux_gui/panel.h"
#include "nux_gui/renderer.h"

namespace NuxGUI {

Panel::Panel()
    : Widget()
    , m_BackgroundColor(0xF0F0F0FF)  // Light gray
    , m_BorderColor(0x000000FF)      // Black
    , m_BorderThickness(1.0f)
{
    SetSize(300.0f, 200.0f);
}

Panel::~Panel() {
}

void Panel::SetBackgroundColor(uint32_t color) {
    m_BackgroundColor = color;
}

void Panel::SetBorderColor(uint32_t color) {
    m_BorderColor = color;
}

void Panel::SetBorderThickness(float thickness) {
    m_BorderThickness = thickness;
}

void Panel::Render(Renderer* renderer) {
    // Draw background
    renderer->DrawRect(m_X, m_Y, m_Width, m_Height, m_BackgroundColor);
    
    // Draw border
    if (m_BorderThickness > 0.0f) {
        renderer->DrawRectOutline(m_X, m_Y, m_Width, m_Height, m_BorderColor, m_BorderThickness);
    }
    
    // Render children
    Widget::Render(renderer);
}

} // namespace NuxGUI
