#ifndef NUX_GUI_BUTTON_H
#define NUX_GUI_BUTTON_H

#include "widget.h"
#include <string>

namespace NuxGUI {

class Button : public Widget {
public:
    Button(const std::string& text = "");
    virtual ~Button();
    
    // Text
    void SetText(const std::string& text);
    const std::string& GetText() const { return m_Text; }
    
    // Colors
    void SetBackgroundColor(uint32_t color);
    void SetTextColor(uint32_t color);
    void SetHoverColor(uint32_t color);
    void SetPressedColor(uint32_t color);
    
    // Callbacks
    void SetOnClick(EventCallback callback);
    
    // Overrides
    virtual void Render(Renderer* renderer) override;
    virtual bool HandleEvent(const Event& event) override;
    
private:
    std::string m_Text;
    uint32_t m_BackgroundColor;
    uint32_t m_TextColor;
    uint32_t m_HoverColor;
    uint32_t m_PressedColor;
    bool m_IsHovered;
    bool m_IsPressed;
};

} // namespace NuxGUI

#endif // NUX_GUI_BUTTON_H
