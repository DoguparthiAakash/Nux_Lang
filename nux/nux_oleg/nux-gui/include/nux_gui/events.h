#ifndef NUX_GUI_EVENTS_H
#define NUX_GUI_EVENTS_H

#include <functional>

namespace NuxGUI {

enum class EventType {
    None = 0,
    MouseMove,
    MouseButtonDown,
    MouseButtonUp,
    MouseScroll,
    KeyDown,
    KeyUp,
    KeyChar,
    WindowResize,
    WindowClose
};

enum class MouseButton {
    Left = 0,
    Right = 1,
    Middle = 2
};

enum class KeyCode {
    Unknown = -1,
    Space = 32,
    A = 65, B, C, D, E, F, G, H, I, J, K, L, M,
    N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    Escape = 256,
    Enter, Tab, Backspace, Insert, Delete,
    Right, Left, Down, Up,
    // Add more as needed
};

struct Event {
    EventType Type;
    
    // Mouse event data
    float MouseX, MouseY;
    float MouseDeltaX, MouseDeltaY;
    MouseButton Button;
    float ScrollX, ScrollY;
    
    // Keyboard event data
    KeyCode Key;
    char Character;
    bool Shift, Ctrl, Alt;
    
    // Window event data
    int WindowWidth, WindowHeight;
};

using EventCallback = std::function<void(const Event&)>;

class EventDispatcher {
public:
    void AddListener(EventType type, EventCallback callback);
    void RemoveListener(EventType type);
    void Dispatch(const Event& event);
    
private:
    std::unordered_map<EventType, std::vector<EventCallback>> m_Listeners;
};

} // namespace NuxGUI

#endif // NUX_GUI_EVENTS_H
