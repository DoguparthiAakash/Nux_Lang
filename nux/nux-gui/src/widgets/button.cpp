#include "nux_gui/button.h"
#include "nux_gui/renderer.h"

namespace NuxGUI {

Button::Button(const std::string& text)
    : Widget()
    , m_Text(text)
    , m_BackgroundColor(0x4A90E2FF)  // Blue
    , m_TextColor(0xFFFFFFFF)        // White
    , m_HoverColor(0x5BA3F5FF)       // Lighter blue
    , m_PressedColor(0x3A7BC8FF)     // Darker blue
    , m_IsHovered(false)
    , m_IsPressed(false)
{
    SetSize(120.0f, 40.0f);
}

Button::~Button() {
}

void Button::SetText(const std::string& text) {
    m_Text = text;
}

void Button::SetBackgroundColor(uint32_t color) {
    m_BackgroundColor = color;
}

void Button::SetTextColor(uint32_t color) {
    m_TextColor = color;
}

void Button::SetHoverColor(uint32_t color) {
    m_HoverColor = color;
}

void Button::SetPressedColor(uint32_t color) {
    m_PressedColor = color;
}

void Button::SetOnClick(EventCallback callback) {
    SetEventCallback(EventType::MouseButtonUp, callback);
}

void Button::Render(Renderer* renderer) {
    uint32_t bgColor = m_BackgroundColor;
    if (m_IsPressed) {
        bgColor = m_PressedColor;
    } else if (m_IsHovered) {
        bgColor = m_HoverColor;
    }
    
    // Draw background
    renderer->DrawRect(m_X, m_Y, m_Width, m_Height, bgColor);
    
    // Draw border
    renderer->DrawRectOutline(m_X, m_Y, m_Width, m_Height, 0x000000FF, 2.0f);
    
    // Draw text (centered)
    float textX = m_X + m_Width / 2.0f;
    float textY = m_Y + m_Height / 2.0f;
    renderer->DrawText(m_Text.c_str(), textX, textY, 16.0f, m_TextColor);
    
    // Render children
    Widget::Render(renderer);
}

bool Button::HandleEvent(const Event& event) {
    if (!m_Enabled) return false;
    
    switch (event.Type) {
        case EventType::MouseMove:
            m_IsHovered = ContainsPoint(event.MouseX, event.MouseY);
            break;
            
        case EventType::MouseButtonDown:
            if (m_IsHovered && event.Button == MouseButton::Left) {
                m_IsPressed = true;
                return true;
            }
            break;
            
        case EventType::MouseButtonUp:
            if (m_IsPressed && event.Button == MouseButton::Left) {
                m_IsPressed = false;
                if (m_IsHovered) {
                    // Trigger click callback
                    Widget::HandleEvent(event);
                }
                return true;
            }
            break;
            
        default:
            break;
    }
    
    return Widget::HandleEvent(event);
}

} // namespace NuxGUI
