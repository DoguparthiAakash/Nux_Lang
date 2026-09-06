#ifndef NUX_GUI_TEXTBOX_H
#define NUX_GUI_TEXTBOX_H

#include "widget.h"
#include <string>

namespace NuxGUI {

class TextBox : public Widget {
public:
    TextBox(const std::string& placeholder = "");
    virtual ~TextBox();
    
    // Text management
    void SetText(const std::string& text);
    const std::string& GetText() const { return m_Text; }
    void SetPlaceholder(const std::string& placeholder);
    const std::string& GetPlaceholder() const { return m_Placeholder; }
    
    // Styling
    void SetTextColor(uint32_t color);
    void SetBackgroundColor(uint32_t color);
    void SetBorderColor(uint32_t color);
    void SetPlaceholderColor(uint32_t color);
    void SetFontSize(float size);
    
    // State
    void SetReadOnly(bool readOnly);
    bool IsReadOnly() const { return m_ReadOnly; }
    void SetMaxLength(int maxLength);
    void SetFocused(bool focused);
    bool IsFocused() const { return m_Focused; }
    
    // Callbacks
    void SetOnTextChanged(EventCallback callback);
    void SetOnEnter(EventCallback callback);
    
    // Overrides
    virtual void Render(Renderer* renderer) override;
    virtual bool HandleEvent(const Event& event) override;
    
private:
    void InsertChar(char c);
    void DeleteChar();
    void MoveCursor(int delta);
    
    std::string m_Text;
    std::string m_Placeholder;
    uint32_t m_TextColor;
    uint32_t m_BackgroundColor;
    uint32_t m_BorderColor;
    uint32_t m_PlaceholderColor;
    float m_FontSize;
    bool m_ReadOnly;
    bool m_Focused;
    int m_MaxLength;
    int m_CursorPos;
};

} // namespace NuxGUI

#endif // NUX_GUI_TEXTBOX_H
