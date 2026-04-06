#ifndef NUX_GUI_SLIDER_H
#define NUX_GUI_SLIDER_H

#include "widget.h"

namespace NuxGUI {

class Slider : public Widget {
public:
    Slider(float minValue = 0.0f, float maxValue = 100.0f);
    virtual ~Slider();
    
    // Value
    void SetValue(float value);
    float GetValue() const { return m_Value; }
    void SetMinValue(float minValue);
    float GetMinValue() const { return m_MinValue; }
    void SetMaxValue(float maxValue);
    float GetMaxValue() const { return m_MaxValue; }
    void SetStep(float step);
    
    // Styling
    void SetTrackColor(uint32_t color);
    void SetFillColor(uint32_t color);
    void SetHandleColor(uint32_t color);
    void SetHandleSize(float size);
    
    // Callbacks
    void SetOnValueChanged(EventCallback callback);
    
    // Overrides
    virtual void Render(Renderer* renderer) override;
    virtual bool HandleEvent(const Event& event) override;
    
private:
    float GetNormalizedValue() const;
    void UpdateValueFromMouse(float mouseX);
    
    float m_Value;
    float m_MinValue;
    float m_MaxValue;
    float m_Step;
    uint32_t m_TrackColor;
    uint32_t m_FillColor;
    uint32_t m_HandleColor;
    float m_HandleSize;
    bool m_IsDragging;
};

} // namespace NuxGUI

#endif // NUX_GUI_SLIDER_H
