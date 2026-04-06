#ifndef NUX_GUI_WIDGET_H
#define NUX_GUI_WIDGET_H

#include "events.h"
#include <string>
#include <vector>

namespace NuxGUI {

class Renderer;

class Widget {
public:
    Widget();
    virtual ~Widget();
    
    // Position and size
    void SetPosition(float x, float y);
    void GetPosition(float& x, float& y) const;
    void SetSize(float width, float height);
    void GetSize(float& width, float& height) const;
    
    // Visibility
    void SetVisible(bool visible);
    bool IsVisible() const { return m_Visible; }
    
    // Enabled state
    void SetEnabled(bool enabled);
    bool IsEnabled() const { return m_Enabled; }
    
    // Hierarchy
    void AddChild(Widget* child);
    void RemoveChild(Widget* child);
    Widget* GetParent() const { return m_Parent; }
    
    // Event handling
    void SetEventCallback(EventType type, EventCallback callback);
    virtual bool HandleEvent(const Event& event);
    
    // Rendering
    virtual void Render(Renderer* renderer);
    virtual void Update(float deltaTime);
    
    // Hit testing
    virtual bool ContainsPoint(float x, float y) const;
    
protected:
    float m_X, m_Y;
    float m_Width, m_Height;
    bool m_Visible;
    bool m_Enabled;
    Widget* m_Parent;
    std::vector<Widget*> m_Children;
    std::unordered_map<EventType, EventCallback> m_EventCallbacks;
};

} // namespace NuxGUI

#endif // NUX_GUI_WIDGET_H
