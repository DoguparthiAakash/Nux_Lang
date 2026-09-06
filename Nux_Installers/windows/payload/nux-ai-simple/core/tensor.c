// C - Tensor Implementation
// Simple, clear, easy to understand

#include "tensor.h"
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <math.h>
#include <time.h>

// ============================================
// Helper Functions
// ============================================

// Calculate total size from shape
static int calculate_size(int* shape, int ndim) {
    int size = 1;
    for (int i = 0; i < ndim; i++) {
        size *= shape[i];
    }
    return size;
}

// Calculate strides for indexing
static void calculate_strides(int* strides, int* shape, int ndim) {
    strides[ndim - 1] = 1;
    for (int i = ndim - 2; i >= 0; i--) {
        strides[i] = strides[i + 1] * shape[i + 1];
    }
}

// ============================================
// Creation & Destruction
// ============================================

Tensor* tensor_create(int* shape, int ndim) {
    Tensor* t = (Tensor*)malloc(sizeof(Tensor));
    
    t->ndim = ndim;
    t->size = calculate_size(shape, ndim);
    
    // Allocate and copy shape
    t->shape = (int*)malloc(ndim * sizeof(int));
    memcpy(t->shape, shape, ndim * sizeof(int));
    
    // Calculate strides
    t->strides = (int*)malloc(ndim * sizeof(int));
    calculate_strides(t->strides, shape, ndim);
    
    // Allocate data
    t->data = (float*)calloc(t->size, sizeof(float));
    t->requires_grad = false;
    
    return t;
}

Tensor* tensor_zeros(int* shape, int ndim) {
    return tensor_create(shape, ndim);  // calloc already zeros memory
}

Tensor* tensor_ones(int* shape, int ndim) {
    Tensor* t = tensor_create(shape, ndim);
    for (int i = 0; i < t->size; i++) {
        t->data[i] = 1.0f;
    }
    return t;
}

Tensor* tensor_random(int* shape, int ndim) {
    static bool seeded = false;
    if (!seeded) {
        srand(time(NULL));
        seeded = true;
    }
    
    Tensor* t = tensor_create(shape, ndim);
    for (int i = 0; i < t->size; i++) {
        t->data[i] = ((float)rand() / RAND_MAX) * 2.0f - 1.0f;  // [-1, 1]
    }
    return t;
}

void tensor_free(Tensor* t) {
    if (t) {
        free(t->data);
        free(t->shape);
        free(t->strides);
        free(t);
    }
}

// ============================================
// Basic Operations
// ============================================

Tensor* tensor_add(Tensor* a, Tensor* b) {
    if (!tensor_shapes_equal(a, b)) {
        fprintf(stderr, "Error: Shapes don't match for addition\n");
        return NULL;
    }
    
    Tensor* c = tensor_create(a->shape, a->ndim);
    for (int i = 0; i < a->size; i++) {
        c->data[i] = a->data[i] + b->data[i];
    }
    return c;
}

Tensor* tensor_sub(Tensor* a, Tensor* b) {
    if (!tensor_shapes_equal(a, b)) {
        fprintf(stderr, "Error: Shapes don't match for subtraction\n");
        return NULL;
    }
    
    Tensor* c = tensor_create(a->shape, a->ndim);
    for (int i = 0; i < a->size; i++) {
        c->data[i] = a->data[i] - b->data[i];
    }
    return c;
}

Tensor* tensor_mul(Tensor* a, Tensor* b) {
    if (!tensor_shapes_equal(a, b)) {
        fprintf(stderr, "Error: Shapes don't match for multiplication\n");
        return NULL;
    }
    
    Tensor* c = tensor_create(a->shape, a->ndim);
    for (int i = 0; i < a->size; i++) {
        c->data[i] = a->data[i] * b->data[i];
    }
    return c;
}

Tensor* tensor_div(Tensor* a, Tensor* b) {
    if (!tensor_shapes_equal(a, b)) {
        fprintf(stderr, "Error: Shapes don't match for division\n");
        return NULL;
    }
    
    Tensor* c = tensor_create(a->shape, a->ndim);
    for (int i = 0; i < a->size; i++) {
        c->data[i] = a->data[i] / b->data[i];
    }
    return c;
}

// ============================================
// Scalar Operations
// ============================================

Tensor* tensor_add_scalar(Tensor* a, float scalar) {
    Tensor* c = tensor_create(a->shape, a->ndim);
    for (int i = 0; i < a->size; i++) {
        c->data[i] = a->data[i] + scalar;
    }
    return c;
}

Tensor* tensor_mul_scalar(Tensor* a, float scalar) {
    Tensor* c = tensor_create(a->shape, a->ndim);
    for (int i = 0; i < a->size; i++) {
        c->data[i] = a->data[i] * scalar;
    }
    return c;
}

// ============================================
// Matrix Operations
// ============================================

Tensor* tensor_matmul(Tensor* a, Tensor* b) {
    if (a->ndim != 2 || b->ndim != 2) {
        fprintf(stderr, "Error: matmul requires 2D tensors\n");
        return NULL;
    }
    
    int m = a->shape[0];
    int k = a->shape[1];
    int n = b->shape[1];
    
    if (k != b->shape[0]) {
        fprintf(stderr, "Error: incompatible shapes for matmul\n");
        return NULL;
    }
    
    int shape[2] = {m, n};
    Tensor* c = tensor_create(shape, 2);
    
    // Simple matrix multiplication (will be replaced by Assembly SIMD)
    for (int i = 0; i < m; i++) {
        for (int j = 0; j < n; j++) {
            float sum = 0.0f;
            for (int p = 0; p < k; p++) {
                sum += a->data[i * k + p] * b->data[p * n + j];
            }
            c->data[i * n + j] = sum;
        }
    }
    
    return c;
}

Tensor* tensor_transpose(Tensor* a) {
    if (a->ndim != 2) {
        fprintf(stderr, "Error: transpose requires 2D tensor\n");
        return NULL;
    }
    
    int shape[2] = {a->shape[1], a->shape[0]};
    Tensor* b = tensor_create(shape, 2);
    
    for (int i = 0; i < a->shape[0]; i++) {
        for (int j = 0; j < a->shape[1]; j++) {
            b->data[j * a->shape[0] + i] = a->data[i * a->shape[1] + j];
        }
    }
    
    return b;
}

// ============================================
// Reduction Operations
// ============================================

float tensor_sum(Tensor* t) {
    float sum = 0.0f;
    for (int i = 0; i < t->size; i++) {
        sum += t->data[i];
    }
    return sum;
}

float tensor_mean(Tensor* t) {
    return tensor_sum(t) / t->size;
}

float tensor_max(Tensor* t) {
    float max_val = t->data[0];
    for (int i = 1; i < t->size; i++) {
        if (t->data[i] > max_val) {
            max_val = t->data[i];
        }
    }
    return max_val;
}

float tensor_min(Tensor* t) {
    float min_val = t->data[0];
    for (int i = 1; i < t->size; i++) {
        if (t->data[i] < min_val) {
            min_val = t->data[i];
        }
    }
    return min_val;
}

// ============================================
// Utility Functions
// ============================================

void tensor_print(Tensor* t) {
    printf("Tensor(shape=[");
    for (int i = 0; i < t->ndim; i++) {
        printf("%d", t->shape[i]);
        if (i < t->ndim - 1) printf(", ");
    }
    printf("], data=[\n");
    
    if (t->ndim == 1) {
        for (int i = 0; i < t->size; i++) {
            printf("  %.4f", t->data[i]);
            if (i < t->size - 1) printf(",");
            printf("\n");
        }
    } else if (t->ndim == 2) {
        for (int i = 0; i < t->shape[0]; i++) {
            printf("  [");
            for (int j = 0; j < t->shape[1]; j++) {
                printf("%.4f", t->data[i * t->shape[1] + j]);
                if (j < t->shape[1] - 1) printf(", ");
            }
            printf("]");
            if (i < t->shape[0] - 1) printf(",");
            printf("\n");
        }
    }
    
    printf("])\n");
}

Tensor* tensor_copy(Tensor* t) {
    Tensor* copy = tensor_create(t->shape, t->ndim);
    memcpy(copy->data, t->data, t->size * sizeof(float));
    copy->requires_grad = t->requires_grad;
    return copy;
}

bool tensor_shapes_equal(Tensor* a, Tensor* b) {
    if (a->ndim != b->ndim) return false;
    for (int i = 0; i < a->ndim; i++) {
        if (a->shape[i] != b->shape[i]) return false;
    }
    return true;
}
