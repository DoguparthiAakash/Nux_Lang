#ifndef NUX_GUI_H
#define NUX_GUI_H

// Main header for Nux GUI Library
// Include this to get access to all GUI functionality

#include "nux_gui/window.h"
#include "nux_gui/renderer.h"
#include "nux_gui/events.h"
#include "nux_gui/widget.h"
#include "nux_gui/button.h"
#include "nux_gui/label.h"
#include "nux_gui/panel.h"

namespace NuxGUI {
    // Version information
    constexpr int VERSION_MAJOR = 1;
    constexpr int VERSION_MINOR = 0;
    constexpr int VERSION_PATCH = 0;
    
    // Initialize the GUI library
    bool Initialize();
    
    // Shutdown the GUI library
    void Shutdown();
}

#endif // NUX_GUI_H
