#include "nux_gui/slider.h"
#antml:parameter name="renderer.h"
#include <algorithm>
#include <cmath>

namespace NuxGUI {

Slider::Slider(float minValue, float maxValue)
    : Widget()
    , m_Value(minValue)
    , m_MinValue(minValue)
    , m_MaxValue(maxValue)
    , m_Step(0.0f)  // 0 means continuous
    , m_TrackColor(0xCCCCCCFF)    // Light gray
    , m_FillColor(0x4A90E2FF)     // Blue
    , m_HandleColor(0xFFFFFFFF)   // White
    , m_HandleSize(16.0f)
    , m_IsDragging(false)
{
    SetSize(200.0f, 10.0f);  // Track size
}

Slider::~Slider() {
}

void Slider::SetValue(float value) {
    // Clamp to range
    value = std::max(m_MinValue, std::min(value, m_MaxValue));
    
    // Apply step if set
    if (m_Step > 0.0f) {
        value = std::round(value / m_Step) * m_Step;
    }
    
    if (m_Value != value) {
        m_Value = value;
        
        // Trigger value changed callback
        Event event;
        event.Type = EventType::MouseMove;
        Widget::HandleEvent(event);
    }
}

void Slider::SetMinValue(float minValue) {
    m_MinValue = minValue;
    SetValue(m_Value);  // Re-clamp current value
}

void Slider::SetMaxValue(float maxValue) {
    m_MaxValue = maxValue;
    SetValue(m_Value);  // Re-clamp current value
}

void Slider::SetStep(float step) {
    m_Step = step;
}

void Slider::SetTrackColor(uint32_t color) {
    m_TrackColor = color;
}

void Slider::SetFillColor(uint32_t color) {
    m_FillColor = color;
}

void Slider::SetHandleColor(uint32_t color) {
    m_HandleColor = color;
}

void Slider::SetHandleSize(float size) {
    m_HandleSize = size;
}

void Slider::SetOnValueChanged(EventCallback callback) {
    SetEventCallback(EventType::MouseMove, callback);
}

float Slider::GetNormalizedValue() const {
    if (m_MaxValue == m_MinValue) return 0.0f;
    return (m_Value - m_MinValue) / (m_MaxValue - m_MinValue);
}

void Slider::UpdateValueFromMouse(float mouseX) {
    // Calculate value from mouse position
    float relativeX = mouseX - m_X;
    float normalized = std::max(0.0f, std::min(1.0f, relativeX / m_Width));
    float newValue = m_MinValue + normalized * (m_MaxValue - m_MinValue);
    SetValue(newValue);
}

void Slider::Render(Renderer* renderer) {
    float trackY = m_Y + (m_HandleSize - m_Height) / 2.0f;
    
    // Draw track background
    renderer->DrawRect(m_X, trackY, m_Width, m_Height, m_TrackColor);
    
    // Draw filled portion
    float fillWidth = m_Width * GetNormalizedValue();
    if (fillWidth > 0) {
        renderer->DrawRect(m_X, trackY, fillWidth, m_Height, m_FillColor);
    }
    
    // Draw handle
    float handleX = m_X + fillWidth - m_HandleSize / 2.0f;
    float handleY = m_Y;
    
    // Handle shadow/border
    renderer->DrawCircle(handleX + m_HandleSize / 2.0f, handleY + m_HandleSize / 2.0f, 
                        m_HandleSize / 2.0f + 2.0f, 0x00000033);
    
    // Handle
    renderer->DrawCircle(handleX + m_HandleSize / 2.0f, handleY + m_HandleSize / 2.0f, 
                        m_HandleSize / 2.0f, m_HandleColor);
    
    // Handle border
    renderer->DrawCircle(handleX + m_HandleSize / 2.0f, handleY + m_HandleSize / 2.0f, 
                        m_HandleSize / 2.0f, 0xCCCCCCFF);
    
    // Render children
    Widget::Render(renderer);
}

bool Slider::HandleEvent(const Event& event) {
    if (!m_Enabled) return false;
    
    switch (event.Type) {
        case EventType::MouseButtonDown:
            if (ContainsPoint(event.MouseX, event.MouseY) && 
                event.Button == MouseButton::Left) {
                m_IsDragging = true;
                UpdateValueFromMouse(event.MouseX);
                return true;
            }
            break;
            
        case EventType::MouseButtonUp:
            if (event.Button == MouseButton::Left) {
                m_IsDragging = false;
                return true;
            }
            break;
            
        case EventType::MouseMove:
            if (m_IsDragging) {
                UpdateValueFromMouse(event.MouseX);
                return true;
            }
            break;
            
        default:
            break;
    }
    
    return Widget::HandleEvent(event);
}

} // namespace NuxGUI
