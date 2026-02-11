; x86-64 Assembly - Optimized Matrix Multiplication
; Uses AVX2 SIMD instructions for 8x speedup

section .text
global matmul_avx2_kernel

; Matrix multiplication kernel using AVX2
; Arguments:
;   rdi = float* A (m x k matrix)
;   rsi = float* B (k x n matrix)
;   rdx = float* C (m x n result matrix)
;   rcx = int m (rows of A)
;   r8  = int k (cols of A / rows of B)
;   r9  = int n (cols of B)

matmul_avx2_kernel:
    push rbp
    mov rbp, rsp
    push rbx
    push r12
    push r13
    push r14
    push r15
    
    ; r12 = i (row counter for A)
    ; r13 = j (col counter for B)
    ; r14 = p (inner loop counter)
    
    xor r12, r12                    ; i = 0
    
.outer_loop:
    cmp r12, rcx                    ; if i >= m, done
    jge .done
    
    xor r13, r13                    ; j = 0
    
.middle_loop:
    cmp r13, r9                     ; if j >= n, next row
    jge .outer_next
    
    ; Check if we can process 8 elements at once
    mov rax, r9
    sub rax, r13
    cmp rax, 8
    jl .scalar_loop                 ; Less than 8 elements, use scalar
    
    ; SIMD path - process 8 elements
    vxorps ymm0, ymm0, ymm0         ; sum = 0 (8 floats)
    xor r14, r14                    ; p = 0
    
.inner_simd_loop:
    cmp r14, r8                     ; if p >= k, done with inner loop
    jge .inner_simd_done
    
    ; Load A[i][p] and broadcast to all 8 lanes
    mov rax, r12
    imul rax, r8                    ; i * k
    add rax, r14                    ; i * k + p
    shl rax, 2                      ; * sizeof(float)
    vbroadcastss ymm1, [rdi + rax]  ; ymm1 = [A[i][p], A[i][p], ...]
    
    ; Load B[p][j:j+7]
    mov rax, r14
    imul rax, r9                    ; p * n
    add rax, r13                    ; p * n + j
    shl rax, 2                      ; * sizeof(float)
    vmovups ymm2, [rsi + rax]       ; ymm2 = [B[p][j], ..., B[p][j+7]]
    
    ; Multiply and accumulate
    vfmadd231ps ymm0, ymm1, ymm2    ; sum += A[i][p] * B[p][j:j+7]
    
    inc r14
    jmp .inner_simd_loop
    
.inner_simd_done:
    ; Store result C[i][j:j+7]
    mov rax, r12
    imul rax, r9                    ; i * n
    add rax, r13                    ; i * n + j
    shl rax, 2                      ; * sizeof(float)
    vmovups [rdx + rax], ymm0
    
    add r13, 8                      ; j += 8
    jmp .middle_loop
    
.scalar_loop:
    ; Scalar path for remaining elements
    xorps xmm0, xmm0                ; sum = 0
    xor r14, r14                    ; p = 0
    
.inner_scalar_loop:
    cmp r14, r8
    jge .inner_scalar_done
    
    ; Load A[i][p]
    mov rax, r12
    imul rax, r8
    add rax, r14
    shl rax, 2
    movss xmm1, [rdi + rax]
    
    ; Load B[p][j]
    mov rax, r14
    imul rax, r9
    add rax, r13
    shl rax, 2
    movss xmm2, [rsi + rax]
    
    ; Multiply and accumulate
    mulss xmm1, xmm2
    addss xmm0, xmm1
    
    inc r14
    jmp .inner_scalar_loop
    
.inner_scalar_done:
    ; Store result C[i][j]
    mov rax, r12
    imul rax, r9
    add rax, r13
    shl rax, 2
    movss [rdx + rax], xmm0
    
    inc r13
    jmp .middle_loop
    
.outer_next:
    inc r12
    jmp .outer_loop
    
.done:
    ; Restore registers
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    pop rbp
    ret

; ReLU activation using SIMD
; Arguments:
;   rdi = float* input
;   rsi = float* output
;   rdx = int size
global relu_avx2

relu_avx2:
    push rbp
    mov rbp, rsp
    
    vxorps ymm1, ymm1, ymm1         ; ymm1 = 0 (for comparison)
    xor rax, rax                    ; index = 0
    
.loop:
    cmp rax, rdx
    jge .done
    
    ; Check if we can process 8 elements
    mov rcx, rdx
    sub rcx, rax
    cmp rcx, 8
    jl .scalar
    
    ; SIMD path
    vmovups ymm0, [rdi + rax*4]     ; Load 8 floats
    vmaxps ymm0, ymm0, ymm1         ; max(x, 0)
    vmovups [rsi + rax*4], ymm0     ; Store result
    
    add rax, 8
    jmp .loop
    
.scalar:
    ; Scalar path
    movss xmm0, [rdi + rax*4]
    maxss xmm0, xmm1
    movss [rsi + rax*4], xmm0
    
    inc rax
    jmp .loop
    
.done:
    pop rbp
    ret
