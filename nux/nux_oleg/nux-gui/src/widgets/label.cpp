#include "nux_gui/label.h"
#include "nux_gui/renderer.h"

namespace NuxGUI {

Label::Label(const std::string& text)
    : Widget()
    , m_Text(text)
    , m_TextColor(0x000000FF)  // Black
    , m_FontSize(16.0f)
    , m_TextAlign(TextAlign::Left)
{
    SetSize(200.0f, 30.0f);
}

Label::~Label() {
}

void Label::SetText(const std::string& text) {
    m_Text = text;
}

void Label::SetTextColor(uint32_t color) {
    m_TextColor = color;
}

void Label::SetFontSize(float size) {
    m_FontSize = size;
}

void Label::SetTextAlign(TextAlign align) {
    m_TextAlign = align;
}

void Label::Render(Renderer* renderer) {
    float textX = m_X;
    
    // Adjust X position based on alignment
    switch (m_TextAlign) {
        case TextAlign::Center:
            textX = m_X + m_Width / 2.0f;
            break;
        case TextAlign::Right:
            textX = m_X + m_Width;
            break;
        default:
            break;
    }
    
    float textY = m_Y + m_Height / 2.0f;
    renderer->DrawText(m_Text.c_str(), textX, textY, m_FontSize, m_TextColor);
    
    // Render children
    Widget::Render(renderer);
}

} // namespace NuxGUI
