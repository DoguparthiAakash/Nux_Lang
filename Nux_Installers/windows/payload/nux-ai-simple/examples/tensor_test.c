// C - Simple Tensor Test

#include "tensor.h"
#include <stdio.h>

int main() {
    printf("=== Nux AI Tensor Test ===\n\n");
    
    // Test 1: Create tensors
    printf("Test 1: Creating tensors\n");
    int shape1[] = {2, 3};
    Tensor* a = tensor_random(shape1, 2);
    printf("Tensor A:\n");
    tensor_print(a);
    
    Tensor* b = tensor_random(shape1, 2);
    printf("\nTensor B:\n");
    tensor_print(b);
    
    // Test 2: Addition
    printf("\nTest 2: Addition (A + B)\n");
    Tensor* c = tensor_add(a, b);
    tensor_print(c);
    
    // Test 3: Matrix multiplication
    printf("\nTest 3: Matrix multiplication\n");
    int shape2[] = {3, 2};
    Tensor* d = tensor_random(shape2, 2);
    printf("Tensor D (3x2):\n");
    tensor_print(d);
    
    Tensor* e = tensor_matmul(a, d);  // 2x3 @ 3x2 = 2x2
    printf("\nA @ D (2x2):\n");
    tensor_print(e);
    
    // Test 4: Transpose
    printf("\nTest 4: Transpose\n");
    Tensor* a_t = tensor_transpose(a);
    printf("A^T (3x2):\n");
    tensor_print(a_t);
    
    // Test 5: Reductions
    printf("\nTest 5: Reductions\n");
    printf("Sum of A: %.4f\n", tensor_sum(a));
    printf("Mean of A: %.4f\n", tensor_mean(a));
    printf("Max of A: %.4f\n", tensor_max(a));
    printf("Min of A: %.4f\n", tensor_min(a));
    
    // Cleanup
    tensor_free(a);
    tensor_free(b);
    tensor_free(c);
    tensor_free(d);
    tensor_free(e);
    tensor_free(a_t);
    
    printf("\n=== All tests passed! ===\n");
    return 0;
}
