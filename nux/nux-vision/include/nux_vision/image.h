#ifndef NUX_VISION_IMAGE_H
#define NUX_VISION_IMAGE_H

#include <vector>
#include <string>
#include <memory>

namespace NuxVision {

enum class ColorSpace {
    RGB,
    BGR,
    GRAY,
    HSV,
    LAB
};

class Image {
public:
    Image();
    Image(int width, int height, int channels = 3);
    Image(int width, int height, int channels, const std::vector<uint8_t>& data);
    ~Image();
    
    // Dimensions
    int Width() const { return m_Width; }
    int Height() const { return m_Height; }
    int Channels() const { return m_Channels; }
    
    // Data access
    uint8_t* Data() { return m_Data.data(); }
    const uint8_t* Data() const { return m_Data.data(); }
    uint8_t& At(int y, int x, int c = 0);
    uint8_t At(int y, int x, int c = 0) const;
    
    // I/O
    static Image Load(const std::string& filename);
    void Save(const std::string& filename) const;
    
    // Color space conversion
    Image ConvertColor(ColorSpace from, ColorSpace to) const;
    Image ToGray() const;
    
    // Geometric transformations
    Image Resize(int newWidth, int newHeight) const;
    Image Rotate(double angle) const;
    Image Flip(bool horizontal = true) const;
    Image Crop(int x, int y, int width, int height) const;
    
    // Filters
    Image GaussianBlur(int kernelSize = 5, double sigma = 1.0) const;
    Image MedianBlur(int kernelSize = 5) const;
    Image Sharpen() const;
    Image EdgeDetect() const;  // Sobel
    Image Canny(double threshold1 = 100, double threshold2 = 200) const;
    
    // Morphological operations
    Image Dilate(int kernelSize = 3) const;
    Image Erode(int kernelSize = 3) const;
    
    // Histogram
    std::vector<int> Histogram(int channel = 0) const;
    Image EqualizeHistogram() const;
    
    // Drawing
    void DrawRectangle(int x, int y, int width, int height, 
                       uint8_t r, uint8_t g, uint8_t b, int thickness = 1);
    void DrawCircle(int cx, int cy, int radius,
                    uint8_t r, uint8_t g, uint8_t b, int thickness = 1);
    void DrawLine(int x1, int y1, int x2, int y2,
                  uint8_t r, uint8_t g, uint8_t b, int thickness = 1);
    
private:
    int m_Width;
    int m_Height;
    int m_Channels;
    std::vector<uint8_t> m_Data;
    
    void ApplyKernel(const std::vector<std::vector<double>>& kernel);
};

// Feature detection
struct KeyPoint {
    float x, y;
    float size;
    float angle;
    float response;
};

class FeatureDetector {
public:
    virtual ~FeatureDetector() = default;
    virtual std::vector<KeyPoint> Detect(const Image& img) = 0;
};

class HarrisCornerDetector : public FeatureDetector {
public:
    HarrisCornerDetector(double threshold = 0.01);
    std::vector<KeyPoint> Detect(const Image& img) override;
    
private:
    double m_Threshold;
};

// Object detection
struct DetectionBox {
    int x, y, width, height;
    float confidence;
    int classId;
    std::string className;
};

class ObjectDetector {
public:
    virtual ~ObjectDetector() = default;
    virtual std::vector<DetectionBox> Detect(const Image& img) = 0;
};

// Face detection
class FaceDetector : public ObjectDetector {
public:
    FaceDetector();
    std::vector<DetectionBox> Detect(const Image& img) override;
};

} // namespace NuxVision

#endif // NUX_VISION_IMAGE_H
