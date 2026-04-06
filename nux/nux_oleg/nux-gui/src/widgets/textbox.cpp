#include "nux_gui/textbox.h"
#include "nux_gui/renderer.h"
#include <algorithm>

namespace NuxGUI {

TextBox::TextBox(const std::string& placeholder)
    : Widget()
    , m_Text("")
    , m_Placeholder(placeholder)
    , m_TextColor(0x000000FF)        // Black
    , m_BackgroundColor(0xFFFFFFFF)  // White
    , m_BorderColor(0xCCCCCCFF)      // Light gray
    , m_PlaceholderColor(0x999999FF) // Gray
    , m_FontSize(16.0f)
    , m_ReadOnly(false)
    , m_Focused(false)
    , m_MaxLength(-1)
    , m_CursorPos(0)
{
    SetSize(200.0f, 35.0f);
}

TextBox::~TextBox() {
}

void TextBox::SetText(const std::string& text) {
    if (m_MaxLength > 0 && text.length() > static_cast<size_t>(m_MaxLength)) {
        m_Text = text.substr(0, m_MaxLength);
    } else {
        m_Text = text;
    }
    m_CursorPos = std::min(m_CursorPos, static_cast<int>(m_Text.length()));
    
    // Trigger text changed callback
    Event event;
    event.Type = EventType::KeyChar;
    HandleEvent(event);
}

void TextBox::SetPlaceholder(const std::string& placeholder) {
    m_Placeholder = placeholder;
}

void TextBox::SetTextColor(uint32_t color) {
    m_TextColor = color;
}

void TextBox::SetBackgroundColor(uint32_t color) {
    m_BackgroundColor = color;
}

void TextBox::SetBorderColor(uint32_t color) {
    m_BorderColor = color;
}

void TextBox::SetPlaceholderColor(uint32_t color) {
    m_PlaceholderColor = color;
}

void TextBox::SetFontSize(float size) {
    m_FontSize = size;
}

void TextBox::SetReadOnly(bool readOnly) {
    m_ReadOnly = readOnly;
}

void TextBox::SetMaxLength(int maxLength) {
    m_MaxLength = maxLength;
}

void TextBox::SetFocused(bool focused) {
    m_Focused = focused;
}

void TextBox::SetOnTextChanged(EventCallback callback) {
    SetEventCallback(EventType::KeyChar, callback);
}

void TextBox::SetOnEnter(EventCallback callback) {
    SetEventCallback(EventType::KeyDown, callback);
}

void TextBox::Render(Renderer* renderer) {
    // Draw background
    renderer->DrawRect(m_X, m_Y, m_Width, m_Height, m_BackgroundColor);
    
    // Draw border (thicker if focused)
    float borderThickness = m_Focused ? 2.0f : 1.0f;
    uint32_t borderColor = m_Focused ? 0x4A90E2FF : m_BorderColor;
    renderer->DrawRectOutline(m_X, m_Y, m_Width, m_Height, borderColor, borderThickness);
    
    // Draw text or placeholder
    float textX = m_X + 8.0f;  // Padding
    float textY = m_Y + m_Height / 2.0f;
    
    if (m_Text.empty() && !m_Placeholder.empty()) {
        // Draw placeholder
        renderer->DrawText(m_Placeholder.c_str(), textX, textY, m_FontSize, m_PlaceholderColor);
    } else {
        // Draw text
        renderer->DrawText(m_Text.c_str(), textX, textY, m_FontSize, m_TextColor);
        
        // Draw cursor if focused
        if (m_Focused) {
            // Simple cursor rendering (vertical line)
            float cursorX = textX + (m_CursorPos * 8.0f); // Approximate char width
            renderer->DrawLine(cursorX, m_Y + 5.0f, cursorX, m_Y + m_Height - 5.0f, 
                             m_TextColor, 2.0f);
        }
    }
    
    // Render children
    Widget::Render(renderer);
}

bool TextBox::HandleEvent(const Event& event) {
    if (!m_Enabled) return false;
    
    switch (event.Type) {
        case EventType::MouseButtonDown:
            if (ContainsPoint(event.MouseX, event.MouseY)) {
                SetFocused(true);
                return true;
            } else {
                SetFocused(false);
            }
            break;
            
        case EventType::KeyChar:
            if (m_Focused && !m_ReadOnly) {
                InsertChar(event.Character);
                // Trigger text changed callback
                Widget::HandleEvent(event);
                return true;
            }
            break;
            
        case EventType::KeyDown:
            if (m_Focused) {
                if (event.Key == KeyCode::Backspace) {
                    DeleteChar();
                    return true;
                } else if (event.Key == KeyCode::Enter) {
                    // Trigger enter callback
                    Widget::HandleEvent(event);
                    return true;
                } else if (event.Key == KeyCode::Left) {
                    MoveCursor(-1);
                    return true;
                } else if (event.Key == KeyCode::Right) {
                    MoveCursor(1);
                    return true;
                }
            }
            break;
            
        default:
            break;
    }
    
    return Widget::HandleEvent(event);
}

void TextBox::InsertChar(char c) {
    if (m_ReadOnly) return;
    if (m_MaxLength > 0 && m_Text.length() >= static_cast<size_t>(m_MaxLength)) return;
    
    // Only allow printable characters
    if (c >= 32 && c <= 126) {
        m_Text.insert(m_CursorPos, 1, c);
        m_CursorPos++;
    }
}

void TextBox::DeleteChar() {
    if (m_ReadOnly) return;
    if (m_CursorPos > 0 && !m_Text.empty()) {
        m_Text.erase(m_CursorPos - 1, 1);
        m_CursorPos--;
    }
}

void TextBox::MoveCursor(int delta) {
    m_CursorPos += delta;
    m_CursorPos = std::max(0, std::min(m_CursorPos, static_cast<int>(m_Text.length())));
}

} // namespace NuxGUI
