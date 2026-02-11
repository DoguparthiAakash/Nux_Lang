// GAS Assembly (.S) - Optimized Matrix Multiplication
// Compatible with GCC/Clang, supports both x86-64 and ARM

#if defined(__x86_64__) || defined(__amd64__)
// x86-64 AVX2 implementation

.text
.globl matmul_avx2_kernel
.type matmul_avx2_kernel, @function

// Matrix multiplication kernel using AVX2
// Arguments:
//   rdi = float* A (m x k matrix)
//   rsi = float* B (k x n matrix)
//   rdx = float* C (m x n result matrix)
//   rcx = int m (rows of A)
//   r8  = int k (cols of A / rows of B)
//   r9  = int n (cols of B)

matmul_avx2_kernel:
    pushq   %rbp
    movq    %rsp, %rbp
    pushq   %rbx
    pushq   %r12
    pushq   %r13
    pushq   %r14
    pushq   %r15
    
    xorq    %r12, %r12              // i = 0
    
.L_outer_loop:
    cmpq    %rcx, %r12              // if i >= m, done
    jge     .L_done
    
    xorq    %r13, %r13              // j = 0
    
.L_middle_loop:
    cmpq    %r9, %r13               // if j >= n, next row
    jge     .L_outer_next
    
    // Check if we can process 8 elements at once
    movq    %r9, %rax
    subq    %r13, %rax
    cmpq    $8, %rax
    jl      .L_scalar_loop
    
    // SIMD path - process 8 elements
    vxorps  %ymm0, %ymm0, %ymm0     // sum = 0
    xorq    %r14, %r14              // p = 0
    
.L_inner_simd_loop:
    cmpq    %r8, %r14               // if p >= k, done
    jge     .L_inner_simd_done
    
    // Load A[i][p] and broadcast
    movq    %r12, %rax
    imulq   %r8, %rax               // i * k
    addq    %r14, %rax              // i * k + p
    shlq    $2, %rax                // * 4
    vbroadcastss (%rdi,%rax), %ymm1
    
    // Load B[p][j:j+7]
    movq    %r14, %rax
    imulq   %r9, %rax               // p * n
    addq    %r13, %rax              // p * n + j
    shlq    $2, %rax                // * 4
    vmovups (%rsi,%rax), %ymm2
    
    // Multiply and accumulate
    vfmadd231ps %ymm2, %ymm1, %ymm0
    
    incq    %r14
    jmp     .L_inner_simd_loop
    
.L_inner_simd_done:
    // Store result C[i][j:j+7]
    movq    %r12, %rax
    imulq   %r9, %rax
    addq    %r13, %rax
    shlq    $2, %rax
    vmovups %ymm0, (%rdx,%rax)
    
    addq    $8, %r13
    jmp     .L_middle_loop
    
.L_scalar_loop:
    // Scalar fallback
    xorps   %xmm0, %xmm0
    xorq    %r14, %r14
    
.L_inner_scalar_loop:
    cmpq    %r8, %r14
    jge     .L_inner_scalar_done
    
    movq    %r12, %rax
    imulq   %r8, %rax
    addq    %r14, %rax
    shlq    $2, %rax
    movss   (%rdi,%rax), %xmm1
    
    movq    %r14, %rax
    imulq   %r9, %rax
    addq    %r13, %rax
    shlq    $2, %rax
    movss   (%rsi,%rax), %xmm2
    
    mulss   %xmm2, %xmm1
    addss   %xmm1, %xmm0
    
    incq    %r14
    jmp     .L_inner_scalar_loop
    
.L_inner_scalar_done:
    movq    %r12, %rax
    imulq   %r9, %rax
    addq    %r13, %rax
    shlq    $2, %rax
    movss   %xmm0, (%rdx,%rax)
    
    incq    %r13
    jmp     .L_middle_loop
    
.L_outer_next:
    incq    %r12
    jmp     .L_outer_loop
    
.L_done:
    popq    %r15
    popq    %r14
    popq    %r13
    popq    %r12
    popq    %rbx
    popq    %rbp
    ret

.size matmul_avx2_kernel, .-matmul_avx2_kernel

// ReLU activation using SIMD
.globl relu_avx2
.type relu_avx2, @function

relu_avx2:
    pushq   %rbp
    movq    %rsp, %rbp
    
    vxorps  %ymm1, %ymm1, %ymm1     // ymm1 = 0
    xorq    %rax, %rax              // index = 0
    
.L_relu_loop:
    cmpq    %rdx, %rax
    jge     .L_relu_done
    
    movq    %rdx, %rcx
    subq    %rax, %rcx
    cmpq    $8, %rcx
    jl      .L_relu_scalar
    
    // SIMD path
    vmovups (%rdi,%rax,4), %ymm0
    vmaxps  %ymm1, %ymm0, %ymm0
    vmovups %ymm0, (%rsi,%rax,4)
    
    addq    $8, %rax
    jmp     .L_relu_loop
    
.L_relu_scalar:
    movss   (%rdi,%rax,4), %xmm0
    maxss   %xmm1, %xmm0
    movss   %xmm0, (%rsi,%rax,4)
    
    incq    %rax
    jmp     .L_relu_loop
    
.L_relu_done:
    popq    %rbp
    ret

.size relu_avx2, .-relu_avx2

#elif defined(__aarch64__) || defined(__arm64__)
// ARM64 NEON implementation

.text
.globl matmul_neon_kernel
.type matmul_neon_kernel, %function

// Matrix multiplication kernel using NEON
// Arguments:
//   x0 = float* A
//   x1 = float* B
//   x2 = float* C
//   x3 = int m
//   x4 = int k
//   x5 = int n

matmul_neon_kernel:
    stp     x29, x30, [sp, #-16]!
    mov     x29, sp
    
    mov     x6, #0                  // i = 0
    
.L_arm_outer:
    cmp     x6, x3
    bge     .L_arm_done
    
    mov     x7, #0                  // j = 0
    
.L_arm_middle:
    cmp     x7, x5
    bge     .L_arm_outer_next
    
    // Process 4 elements with NEON
    eor     v0.16b, v0.16b, v0.16b  // sum = 0
    mov     x8, #0                  // p = 0
    
.L_arm_inner:
    cmp     x8, x4
    bge     .L_arm_inner_done
    
    // Load A[i][p]
    mul     x9, x6, x4
    add     x9, x9, x8
    lsl     x9, x9, #2
    ldr     s1, [x0, x9]
    dup     v1.4s, v1.s[0]
    
    // Load B[p][j:j+3]
    mul     x9, x8, x5
    add     x9, x9, x7
    lsl     x9, x9, #2
    ld1     {v2.4s}, [x1, x9]
    
    // Multiply and accumulate
    fmla    v0.4s, v1.4s, v2.4s
    
    add     x8, x8, #1
    b       .L_arm_inner
    
.L_arm_inner_done:
    // Store result
    mul     x9, x6, x5
    add     x9, x9, x7
    lsl     x9, x9, #2
    st1     {v0.4s}, [x2, x9]
    
    add     x7, x7, #4
    b       .L_arm_middle
    
.L_arm_outer_next:
    add     x6, x6, #1
    b       .L_arm_outer
    
.L_arm_done:
    ldp     x29, x30, [sp], #16
    ret

.size matmul_neon_kernel, .-matmul_neon_kernel

#else
#error "Unsupported architecture"
#endif
