// C++ - Transformer Architecture
// State-of-the-art for NLP and beyond

#ifndef NUX_TRANSFORMER_H
#define NUX_TRANSFORMER_H

#include "layer.h"
#include <vector>

namespace NuxAI {

// Multi-Head Attention Layer
class MultiHeadAttention : public Layer {
private:
    int d_model;        // Model dimension
    int num_heads;      // Number of attention heads
    int head_dim;       // Dimension per head
    
    Linear* query_proj;
    Linear* key_proj;
    Linear* value_proj;
    Linear* output_proj;
    
    Tensor* attention_weights;  // For visualization
    
public:
    MultiHeadAttention(int d_model, int num_heads);
    ~MultiHeadAttention();
    
    Tensor* forward(Tensor* input) override;
    Tensor* backward(Tensor* grad_output) override;
    void update(float learning_rate) override;
    
    // Get attention weights for visualization
    Tensor* get_attention_weights() { return attention_weights; }
};

// Layer Normalization
class LayerNorm : public Layer {
private:
    int normalized_shape;
    Tensor* gamma;      // Scale parameter
    Tensor* beta;       // Shift parameter
    Tensor* mean_cache;
    Tensor* var_cache;
    float epsilon;
    
public:
    LayerNorm(int shape);
    ~LayerNorm();
    
    Tensor* forward(Tensor* input) override;
    Tensor* backward(Tensor* grad_output) override;
    void update(float learning_rate) override;
};

// Feed-Forward Network
class FeedForward : public Layer {
private:
    Linear* fc1;
    Linear* fc2;
    ReLU* activation;
    float dropout_rate;
    
public:
    FeedForward(int d_model, int d_ff, float dropout = 0.1f);
    ~FeedForward();
    
    Tensor* forward(Tensor* input) override;
    Tensor* backward(Tensor* grad_output) override;
    void update(float learning_rate) override;
};

// Transformer Encoder Block
class TransformerBlock : public Layer {
private:
    MultiHeadAttention* attention;
    LayerNorm* norm1;
    FeedForward* ffn;
    LayerNorm* norm2;
    
public:
    TransformerBlock(int d_model, int num_heads, int d_ff);
    ~TransformerBlock();
    
    Tensor* forward(Tensor* input) override;
    Tensor* backward(Tensor* grad_output) override;
    void update(float learning_rate) override;
};

// Complete Transformer Model
class Transformer : public Layer {
private:
    int vocab_size;
    int d_model;
    int num_layers;
    int num_heads;
    int d_ff;
    int max_seq_len;
    
    Tensor* embedding;              // Token embeddings
    Tensor* positional_encoding;    // Positional encodings
    std::vector<TransformerBlock*> blocks;
    LayerNorm* final_norm;
    Linear* output_projection;
    
public:
    Transformer(int vocab_size, int d_model, int num_layers, 
                int num_heads, int d_ff, int max_seq_len);
    ~Transformer();
    
    Tensor* forward(Tensor* input) override;
    Tensor* backward(Tensor* grad_output) override;
    void update(float learning_rate) override;
    
    // Generate text (autoregressive)
    Tensor* generate(Tensor* prompt, int max_length);
};

// Convolutional Layer (for CNNs)
class Conv2D : public Layer {
private:
    int in_channels;
    int out_channels;
    int kernel_size;
    int stride;
    int padding;
    
    Tensor* weights;        // [out_ch, in_ch, k, k]
    Tensor* bias;           // [out_ch]
    Tensor* grad_weights;
    Tensor* grad_bias;
    Tensor* input_cache;
    
public:
    Conv2D(int in_ch, int out_ch, int kernel, int stride = 1, int pad = 0);
    ~Conv2D();
    
    Tensor* forward(Tensor* input) override;
    Tensor* backward(Tensor* grad_output) override;
    void update(float learning_rate) override;
};

// Max Pooling Layer
class MaxPool2D : public Layer {
private:
    int kernel_size;
    int stride;
    Tensor* indices_cache;  // For backward pass
    
public:
    MaxPool2D(int kernel, int stride);
    ~MaxPool2D();
    
    Tensor* forward(Tensor* input) override;
    Tensor* backward(Tensor* grad_output) override;
    void update(float learning_rate) override {}
};

// Batch Normalization
class BatchNorm : public Layer {
private:
    int num_features;
    Tensor* gamma;
    Tensor* beta;
    Tensor* running_mean;
    Tensor* running_var;
    float momentum;
    float epsilon;
    bool training;
    
public:
    BatchNorm(int features);
    ~BatchNorm();
    
    Tensor* forward(Tensor* input) override;
    Tensor* backward(Tensor* grad_output) override;
    void update(float learning_rate) override;
    
    void set_training(bool train) { training = train; }
};

// Dropout Layer
class Dropout : public Layer {
private:
    float dropout_rate;
    Tensor* mask;
    bool training;
    
public:
    Dropout(float rate);
    ~Dropout();
    
    Tensor* forward(Tensor* input) override;
    Tensor* backward(Tensor* grad_output) override;
    void update(float learning_rate) override {}
    
    void set_training(bool train) { training = train; }
};

} // namespace NuxAI

#endif // NUX_TRANSFORMER_H
