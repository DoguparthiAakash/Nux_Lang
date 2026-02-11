; x86-64 Assembly - SIMD Matrix Multiplication Kernel
; Uses AVX2 for 8-wide float operations

section .text
global matmul_kernel_avx2
global vector_add_avx2
global vector_mul_avx2

; Matrix multiplication kernel
; Arguments: rdi = A, rsi = B, rdx = C, rcx = n
matmul_kernel_avx2:
    push rbp
    mov rbp, rsp
    
    ; Save registers
    push rbx
    push r12
    push r13
    push r14
    push r15
    
    xor r12, r12              ; i = 0
    
.outer_loop:
    cmp r12, rcx
    jge .done
    
    xor r13, r13              ; j = 0
    
.middle_loop:
    cmp r13, rcx
    jge .outer_next
    
    vxorps ymm0, ymm0, ymm0   ; sum = 0
    xor r14, r14              ; k = 0
    
.inner_loop:
    cmp r14, rcx
    jge .inner_done
    
    ; Load A[i][k]
    mov rax, r12
    imul rax, rcx
    add rax, r14
    shl rax, 2                ; * sizeof(float)
    add rax, rdi
    vbroadcastss ymm1, [rax]
    
    ; Load B[k][j:j+7]
    mov rax, r14
    imul rax, rcx
    add rax, r13
    shl rax, 2
    add rax, rsi
    vmovups ymm2, [rax]
    
    ; Multiply and accumulate
    vfmadd231ps ymm0, ymm1, ymm2
    
    add r14, 8
    jmp .inner_loop
    
.inner_done:
    ; Store result
    mov rax, r12
    imul rax, rcx
    add rax, r13
    shl rax, 2
    add rax, rdx
    vmovups [rax], ymm0
    
    add r13, 8
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
    
    mov rsp, rbp
    pop rbp
    ret

; Vector addition (SIMD)
; Arguments: rdi = a, rsi = b, rdx = result, rcx = length
vector_add_avx2:
    push rbp
    mov rbp, rsp
    
    xor rax, rax
    
.loop:
    cmp rax, rcx
    jge .done
    
    vmovups ymm0, [rdi + rax*4]
    vmovups ymm1, [rsi + rax*4]
    vaddps ymm0, ymm0, ymm1
    vmovups [rdx + rax*4], ymm0
    
    add rax, 8
    jmp .loop
    
.done:
    mov rsp, rbp
    pop rbp
    ret

; Vector multiplication (SIMD)
vector_mul_avx2:
    push rbp
    mov rbp, rsp
    
    xor rax, rax
    
.loop:
    cmp rax, rcx
    jge .done
    
    vmovups ymm0, [rdi + rax*4]
    vmovups ymm1, [rsi + rax*4]
    vmulps ymm0, ymm0, ymm1
    vmovups [rdx + rax*4], ymm0
    
    add rax, 8
    jmp .loop
    
.done:
    mov rsp, rbp
    pop rbp
    ret
