#ifndef NUX_GUI_CHECKBOX_H
#define NUX_GUI_CHECKBOX_H

#include "widget.h"
#include <string>

namespace NuxGUI {

class CheckBox : public Widget {
public:
    CheckBox(const std::string& label = "");
    virtual ~CheckBox();
    
    // State
    void SetChecked(bool checked);
    bool IsChecked() const { return m_Checked; }
    void Toggle();
    
    // Label
    void SetLabel(const std::string& label);
    const std::string& GetLabel() const { return m_Label; }
    
    // Styling
    void SetCheckColor(uint32_t color);
    void SetBoxColor(uint32_t color);
    void SetLabelColor(uint32_t color);
    void SetFontSize(float size);
    
    // Callbacks
    void SetOnChanged(EventCallback callback);
    
    // Overrides
    virtual void Render(Renderer* renderer) override;
    virtual bool HandleEvent(const Event& event) override;
    
private:
    bool m_Checked;
    std::string m_Label;
    uint32_t m_CheckColor;
    uint32_t m_BoxColor;
    uint32_t m_LabelColor;
    float m_FontSize;
    bool m_IsHovered;
};

} // namespace NuxGUI

#endif // NUX_GUI_CHECKBOX_H
