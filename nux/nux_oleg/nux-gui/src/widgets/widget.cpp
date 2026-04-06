#include "nux_gui/widget.h"
#include "nux_gui/renderer.h"

namespace NuxGUI {

Widget::Widget()
    : m_X(0.0f)
    , m_Y(0.0f)
    , m_Width(100.0f)
    , m_Height(100.0f)
    , m_Visible(true)
    , m_Enabled(true)
    , m_Parent(nullptr)
{
}

Widget::~Widget() {
    for (Widget* child : m_Children) {
        delete child;
    }
    m_Children.clear();
}

void Widget::SetPosition(float x, float y) {
    m_X = x;
    m_Y = y;
}

void Widget::GetPosition(float& x, float& y) const {
    x = m_X;
    y = m_Y;
}

void Widget::SetSize(float width, float height) {
    m_Width = width;
    m_Height = height;
}

void Widget::GetSize(float& width, float& height) const {
    width = m_Width;
    height = m_Height;
}

void Widget::SetVisible(bool visible) {
    m_Visible = visible;
}

void Widget::SetEnabled(bool enabled) {
    m_Enabled = enabled;
}

void Widget::AddChild(Widget* child) {
    if (child && child->m_Parent == nullptr) {
        m_Children.push_back(child);
        child->m_Parent = this;
    }
}

void Widget::RemoveChild(Widget* child) {
    auto it = std::find(m_Children.begin(), m_Children.end(), child);
    if (it != m_Children.end()) {
        (*it)->m_Parent = nullptr;
        m_Children.erase(it);
    }
}

void Widget::SetEventCallback(EventType type, EventCallback callback) {
    m_EventCallbacks[type] = callback;
}

bool Widget::HandleEvent(const Event& event) {
    auto it = m_EventCallbacks.find(event.Type);
    if (it != m_EventCallbacks.end()) {
        it->second(event);
        return true;
    }
    return false;
}

void Widget::Render(Renderer* renderer) {
    // Base implementation - render children
    for (Widget* child : m_Children) {
        if (child && child->IsVisible()) {
            child->Render(renderer);
        }
    }
}

void Widget::Update(float deltaTime) {
    // Base implementation - update children
    for (Widget* child : m_Children) {
        if (child) {
            child->Update(deltaTime);
        }
    }
}

bool Widget::ContainsPoint(float x, float y) const {
    return x >= m_X && x <= m_X + m_Width &&
           y >= m_Y && y <= m_Y + m_Height;
}

} // namespace NuxGUI
