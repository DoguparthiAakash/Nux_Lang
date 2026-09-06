#ifndef NUX_GUI_PANEL_H
#define NUX_GUI_PANEL_H

#include "widget.h"

namespace NuxGUI {

class Panel : public Widget {
public:
    Panel();
    virtual ~Panel();
    
    // Background
    void SetBackgroundColor(uint32_t color);
    void SetBorderColor(uint32_t color);
    void SetBorderThickness(float thickness);
    
    // Overrides
    virtual void Render(Renderer* renderer) override;
    
private:
    uint32_t m_BackgroundColor;
    uint32_t m_BorderColor;
    float m_BorderThickness;
};

} // namespace NuxGUI

#endif // NUX_GUI_PANEL_H
