# Nux GUI Library

A high-performance, external GUI library for the Nux programming language, built with C++ core (similar to Unreal Engine architecture).

## Features

- **C++ Core**: High-performance rendering with OpenGL
- **Cross-platform**: Works on Windows, Linux, and macOS
- **Easy Integration**: Simple FFI bindings to Nux
- **Modern Widgets**: Button, Label, Panel, and more
- **Event System**: Mouse, keyboard, and window events

## Building

### Prerequisites

- CMake 3.15+
- C++17 compiler
- GLFW 3.3+
- FreeType
- OpenGL

### Build Instructions

```bash
mkdir build && cd build
cmake ..
make
sudo make install
```

## Usage in Nux

```nux
import "gui";

func main() {
    gui.initialize();
    
    var window = new gui.Window(800, 600, "My App");
    var button = new gui.Button("Click Me!");
    button.setPosition(100.0, 100.0);
    window.addWidget(button);
    
    while (!window.shouldClose()) {
        window.pollEvents();
        window.clear(0.2, 0.3, 0.3, 1.0);
        window.render();
        window.swapBuffers();
    }
    
    gui.shutdown();
}
```

## Examples

See the `examples/` directory for more examples:
- `hello_window.nux` - Basic window with button and label
- `button_demo.nux` - Interactive button demonstrations
- `layout_demo.nux` - Layout and positioning examples

## Architecture

```
nux-gui/
├── include/nux_gui/    # C++ headers
├── src/                # C++ implementation
│   ├── core/          # Window, Renderer, Events
│   ├── widgets/       # Button, Label, Panel
│   └── bindings/      # FFI exports
├── bindings/nux/      # Nux wrapper library
└── examples/          # Example applications
```

## License

MIT License - See LICENSE file for details

## Contributing

Contributions are welcome! Please submit pull requests or open issues on GitHub.
