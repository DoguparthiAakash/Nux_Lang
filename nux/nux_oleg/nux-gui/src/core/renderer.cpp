#include "nux_gui/renderer.h"
#include <glad/glad.h>
#include <cstring>

namespace NuxGUI {

// Simple vertex shader
const char* vertexShaderSource = R"(
#version 330 core
layout (location = 0) in vec2 aPos;
layout (location = 1) in vec4 aColor;

out vec4 vColor;

uniform mat4 uProjection;

void main() {
    gl_Position = uProjection * vec4(aPos, 0.0, 1.0);
    vColor = aColor;
}
)";

// Simple fragment shader
const char* fragmentShaderSource = R"(
#version 330 core
in vec4 vColor;
out vec4 FragColor;

void main() {
    FragColor = vColor;
}
)";

Renderer::Renderer()
    : m_ShaderProgram(0)
    , m_VAO(0)
    , m_VBO(0)
    , m_EBO(0)
    , m_Initialized(false)
{
}

Renderer::~Renderer() {
    Shutdown();
}

bool Renderer::Initialize() {
    if (m_Initialized) return true;
    
    // Load OpenGL functions
    if (!gladLoadGLLoader((GLADloadproc)glfwGetProcAddress)) {
        return false;
    }
    
    // Compile shaders
    GLuint vertexShader = glCreateShader(GL_VERTEX_SHADER);
    glShaderSource(vertexShader, 1, &vertexShaderSource, nullptr);
    glCompileShader(vertexShader);
    
    GLuint fragmentShader = glCreateShader(GL_FRAGMENT_SHADER);
    glShaderSource(fragmentShader, 1, &fragmentShaderSource, nullptr);
    glCompileShader(fragmentShader);
    
    // Link shader program
    m_ShaderProgram = glCreateProgram();
    glAttachShader(m_ShaderProgram, vertexShader);
    glAttachShader(m_ShaderProgram, fragmentShader);
    glLinkProgram(m_ShaderProgram);
    
    glDeleteShader(vertexShader);
    glDeleteShader(fragmentShader);
    
    // Create VAO, VBO, EBO
    glGenVertexArrays(1, &m_VAO);
    glGenBuffers(1, &m_VBO);
    glGenBuffers(1, &m_EBO);
    
    glBindVertexArray(m_VAO);
    glBindBuffer(GL_ARRAY_BUFFER, m_VBO);
    glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, m_EBO);
    
    // Position attribute
    glVertexAttribPointer(0, 2, GL_FLOAT, GL_FALSE, 6 * sizeof(float), (void*)0);
    glEnableVertexAttribArray(0);
    
    // Color attribute
    glVertexAttribPointer(1, 4, GL_FLOAT, GL_FALSE, 6 * sizeof(float), (void*)(2 * sizeof(float)));
    glEnableVertexAttribArray(1);
    
    glBindVertexArray(0);
    
    // Enable blending
    glEnable(GL_BLEND);
    glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
    
    m_Initialized = true;
    return true;
}

void Renderer::Shutdown() {
    if (!m_Initialized) return;
    
    glDeleteVertexArrays(1, &m_VAO);
    glDeleteBuffers(1, &m_VBO);
    glDeleteBuffers(1, &m_EBO);
    glDeleteProgram(m_ShaderProgram);
    
    m_Initialized = false;
}

void Renderer::BeginFrame() {
    glUseProgram(m_ShaderProgram);
}

void Renderer::EndFrame() {
    glUseProgram(0);
}

void Renderer::Clear(float r, float g, float b, float a) {
    glClearColor(r, g, b, a);
    glClear(GL_COLOR_BUFFER_BIT);
}

void Renderer::DrawRect(float x, float y, float width, float height, uint32_t color) {
    float r = ((color >> 24) & 0xFF) / 255.0f;
    float g = ((color >> 16) & 0xFF) / 255.0f;
    float b = ((color >> 8) & 0xFF) / 255.0f;
    float a = (color & 0xFF) / 255.0f;
    
    float vertices[] = {
        x, y, r, g, b, a,
        x + width, y, r, g, b, a,
        x + width, y + height, r, g, b, a,
        x, y + height, r, g, b, a
    };
    
    unsigned int indices[] = {
        0, 1, 2,
        2, 3, 0
    };
    
    glBindVertexArray(m_VAO);
    glBindBuffer(GL_ARRAY_BUFFER, m_VBO);
    glBufferData(GL_ARRAY_BUFFER, sizeof(vertices), vertices, GL_DYNAMIC_DRAW);
    glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, m_EBO);
    glBufferData(GL_ELEMENT_ARRAY_BUFFER, sizeof(indices), indices, GL_DYNAMIC_DRAW);
    
    glDrawElements(GL_TRIANGLES, 6, GL_UNSIGNED_INT, 0);
    glBindVertexArray(0);
}

void Renderer::DrawRectOutline(float x, float y, float width, float height, uint32_t color, float thickness) {
    // Top
    DrawRect(x, y, width, thickness, color);
    // Bottom
    DrawRect(x, y + height - thickness, width, thickness, color);
    // Left
    DrawRect(x, y, thickness, height, color);
    // Right
    DrawRect(x + width - thickness, y, thickness, height, color);
}

void Renderer::DrawCircle(float x, float y, float radius, uint32_t color) {
    // TODO: Implement circle rendering
}

void Renderer::DrawLine(float x1, float y1, float x2, float y2, uint32_t color, float thickness) {
    // TODO: Implement line rendering
}

void Renderer::DrawText(const char* text, float x, float y, float size, uint32_t color) {
    // TODO: Implement text rendering with FreeType
}

void Renderer::DrawTexture(uint32_t textureId, float x, float y, float width, float height) {
    // TODO: Implement texture rendering
}

void Renderer::SetViewport(int x, int y, int width, int height) {
    glViewport(x, y, width, height);
}

void Renderer::SetScissor(int x, int y, int width, int height) {
    glScissor(x, y, width, height);
}

void Renderer::EnableScissor(bool enable) {
    if (enable) {
        glEnable(GL_SCISSOR_TEST);
    } else {
        glDisable(GL_SCISSOR_TEST);
    }
}

} // namespace NuxGUI
