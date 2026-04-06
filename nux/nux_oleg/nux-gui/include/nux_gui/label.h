#ifndef NUX_GUI_LABEL_H
#define NUX_GUI_LABEL_H

#include "widget.h"
#include <string>

namespace NuxGUI {

enum class TextAlign {
    Left,
    Center,
    Right
};

class Label : public Widget {
public:
    Label(const std::string& text = "");
    virtual ~Label();
    
    // Text
    void SetText(const std::string& text);
    const std::string& GetText() const { return m_Text; }
    
    // Styling
    void SetTextColor(uint32_t color);
    void SetFontSize(float size);
    void SetTextAlign(TextAlign align);
    
    // Overrides
    virtual void Render(Renderer* renderer) override;
    
private:
    std::string m_Text;
    uint32_t m_TextColor;
    float m_FontSize;
    TextAlign m_TextAlign;
};

} // namespace NuxGUI

#endif // NUX_GUI_LABEL_H
