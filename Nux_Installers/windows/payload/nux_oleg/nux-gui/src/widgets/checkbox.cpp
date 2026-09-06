#include "nux_gui/checkbox.h"
#include "nux_gui/renderer.h"

namespace NuxGUI {

CheckBox::CheckBox(const std::string& label)
    : Widget()
    , m_Checked(false)
    , m_Label(label)
    , m_CheckColor(0x4A90E2FF)    // Blue
    , m_BoxColor(0xFFFFFFFF)      // White
    , m_LabelColor(0x000000FF)    // Black
    , m_FontSize(16.0f)
    , m_IsHovered(false)
{
    SetSize(20.0f, 20.0f);  // Box size
}

CheckBox::~CheckBox() {
}

void CheckBox::SetChecked(bool checked) {
    if (m_Checked != checked) {
        m_Checked = checked;
        
        // Trigger changed callback
        Event event;
        event.Type = EventType::MouseButtonUp;
        Widget::HandleEvent(event);
    }
}

bool CheckBox::Toggle() {
    SetChecked(!m_Checked);
    return m_Checked;
}

void CheckBox::SetLabel(const std::string& label) {
    m_Label = label;
}

void CheckBox::SetCheckColor(uint32_t color) {
    m_CheckColor = color;
}

void CheckBox::SetBoxColor(uint32_t color) {
    m_BoxColor = color;
}

void CheckBox::SetLabelColor(uint32_t color) {
    m_LabelColor = color;
}

void CheckBox::SetFontSize(float size) {
    m_FontSize = size;
}

void CheckBox::SetOnChanged(EventCallback callback) {
    SetEventCallback(EventType::MouseButtonUp, callback);
}

void CheckBox::Render(Renderer* renderer) {
    // Draw box background
    uint32_t boxColor = m_IsHovered ? 0xF0F0F0FF : m_BoxColor;
    renderer->DrawRect(m_X, m_Y, m_Width, m_Height, boxColor);
    
    // Draw box border
    renderer->DrawRectOutline(m_X, m_Y, m_Width, m_Height, 0xCCCCCCFF, 2.0f);
    
    // Draw checkmark if checked
    if (m_Checked) {
        // Simple checkmark (filled rect for now, could be improved with lines)
        float padding = 4.0f;
        renderer->DrawRect(
            m_X + padding, 
            m_Y + padding, 
            m_Width - padding * 2, 
            m_Height - padding * 2, 
            m_CheckColor
        );
    }
    
    // Draw label
    if (!m_Label.empty()) {
        float labelX = m_X + m_Width + 8.0f;  // Spacing after box
        float labelY = m_Y + m_Height / 2.0f;
        renderer->DrawText(m_Label.c_str(), labelX, labelY, m_FontSize, m_LabelColor);
    }
    
    // Render children
    Widget::Render(renderer);
}

bool CheckBox::HandleEvent(const Event& event) {
    if (!m_Enabled) return false;
    
    // Expand hit area to include label
    float hitWidth = m_Width;
    if (!m_Label.empty()) {
        hitWidth += 8.0f + (m_Label.length() * 8.0f);  // Approximate
    }
    
    switch (event.Type) {
        case EventType::MouseMove:
            m_IsHovered = (event.MouseX >= m_X && event.MouseX <= m_X + hitWidth &&
                          event.MouseY >= m_Y && event.MouseY <= m_Y + m_Height);
            break;
            
        case EventType::MouseButtonUp:
            if (m_IsHovered && event.Button == MouseButton::Left) {
                Toggle();
                return true;
            }
            break;
            
        default:
            break;
    }
    
    return Widget::HandleEvent(event);
}

} // namespace NuxGUI
