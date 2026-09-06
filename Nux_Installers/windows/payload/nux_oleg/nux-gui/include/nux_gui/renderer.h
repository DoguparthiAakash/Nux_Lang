#ifndef NUX_GUI_RENDERER_H
#define NUX_GUI_RENDERER_H

#include <cstdint>

namespace NuxGUI {

class Renderer {
public:
    Renderer();
    ~Renderer();
    
    // Initialization
    bool Initialize();
    void Shutdown();
    
    // Frame management
    void BeginFrame();
    void EndFrame();
    void Clear(float r, float g, float b, float a);
    
    // 2D Drawing primitives
    void DrawRect(float x, float y, float width, float height, uint32_t color);
    void DrawRectOutline(float x, float y, float width, float height, uint32_t color, float thickness);
    void DrawCircle(float x, float y, float radius, uint32_t color);
    void DrawLine(float x1, float y1, float x2, float y2, uint32_t color, float thickness);
    
    // Text rendering
    void DrawText(const char* text, float x, float y, float size, uint32_t color);
    
    // Texture rendering
    void DrawTexture(uint32_t textureId, float x, float y, float width, float height);
    
    // State management
    void SetViewport(int x, int y, int width, int height);
    void SetScissor(int x, int y, int width, int height);
    void EnableScissor(bool enable);
    
private:
    uint32_t m_ShaderProgram;
    uint32_t m_VAO, m_VBO, m_EBO;
    bool m_Initialized;
};

} // namespace NuxGUI

#endif // NUX_GUI_RENDERER_H
