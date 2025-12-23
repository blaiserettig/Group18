bits 64
default rel

segment .text
global mainCRTStartup
extern ExitProcess
extern puts
extern printf

mainCRTStartup:
    push rbp
    mov rbp, rsp
    
    ; Allocate 1MB on stack for array_heap
    ; Touch pages to ensure stack is committed (Stack Probe)
    mov rcx, 1048576
    mov rax, 4096
.probe_loop:
    sub rsp, rax
    test [rsp], rsp ; Touch the page
    sub rcx, rax
    cmp rcx, 0
    jg .probe_loop
    
    ; Save the start of our stack heap to array_ptr
    mov [array_ptr], rsp
    jmp main_entry
func_matrix_multiply:
    push rbp
    mov rbp, rsp
    sub rsp, 192
    mov qword [rbp-8], rcx
    mov qword [rbp-16], rdx
    mov qword [rbp-24], r8
    mov rax, [array_ptr]
    push rax
    sub rsp, 8
    add rax, 24
    mov [array_ptr], rax
    add rsp, 8
    pop rbx
    push rbx
    sub rsp, 8
    mov rax, [array_ptr]
    push rax
    sub rsp, 8
    add rax, 24
    mov [array_ptr], rax
    add rsp, 8
    pop rbx
    push rbx
    sub rsp, 8
    mov eax, 0
    movd xmm0, eax
    movd eax, xmm0
    add rsp, 8
    pop rbx
    mov qword [rbx + 0], rax
    push rbx
    sub rsp, 8
    mov eax, 0
    movd xmm0, eax
    movd eax, xmm0
    add rsp, 8
    pop rbx
    mov qword [rbx + 8], rax
    push rbx
    sub rsp, 8
    mov eax, 0
    movd xmm0, eax
    movd eax, xmm0
    add rsp, 8
    pop rbx
    mov qword [rbx + 16], rax
    mov rax, rbx
    add rsp, 8
    pop rbx
    mov qword [rbx + 0], rax
    push rbx
    sub rsp, 8
    mov rax, [array_ptr]
    push rax
    sub rsp, 8
    add rax, 24
    mov [array_ptr], rax
    add rsp, 8
    pop rbx
    push rbx
    sub rsp, 8
    mov eax, 0
    movd xmm0, eax
    movd eax, xmm0
    add rsp, 8
    pop rbx
    mov qword [rbx + 0], rax
    push rbx
    sub rsp, 8
    mov eax, 0
    movd xmm0, eax
    movd eax, xmm0
    add rsp, 8
    pop rbx
    mov qword [rbx + 8], rax
    push rbx
    sub rsp, 8
    mov eax, 0
    movd xmm0, eax
    movd eax, xmm0
    add rsp, 8
    pop rbx
    mov qword [rbx + 16], rax
    mov rax, rbx
    add rsp, 8
    pop rbx
    mov qword [rbx + 8], rax
    push rbx
    sub rsp, 8
    mov rax, [array_ptr]
    push rax
    sub rsp, 8
    add rax, 24
    mov [array_ptr], rax
    add rsp, 8
    pop rbx
    push rbx
    sub rsp, 8
    mov eax, 0
    movd xmm0, eax
    movd eax, xmm0
    add rsp, 8
    pop rbx
    mov qword [rbx + 0], rax
    push rbx
    sub rsp, 8
    mov eax, 0
    movd xmm0, eax
    movd eax, xmm0
    add rsp, 8
    pop rbx
    mov qword [rbx + 8], rax
    push rbx
    sub rsp, 8
    mov eax, 0
    movd xmm0, eax
    movd eax, xmm0
    add rsp, 8
    pop rbx
    mov qword [rbx + 16], rax
    mov rax, rbx
    add rsp, 8
    pop rbx
    mov qword [rbx + 16], rax
    mov rax, rbx
    mov qword [rbp-32], rax
    mov eax, 0
    mov dword [rbp-40], eax
loop_begin_i_0:
    mov eax, dword [rbp-40]
    mov ebx, dword [rbp-24]
    cmp eax, ebx
    jge loop_end_i_0
    mov rax, [array_ptr]
    push rax
    sub rsp, 8
    add rax, 24
    mov [array_ptr], rax
    add rsp, 8
    pop rbx
    push rbx
    sub rsp, 8
    mov eax, 0
    movd xmm0, eax
    movd eax, xmm0
    add rsp, 8
    pop rbx
    mov qword [rbx + 0], rax
    push rbx
    sub rsp, 8
    mov eax, 0
    movd xmm0, eax
    movd eax, xmm0
    add rsp, 8
    pop rbx
    mov qword [rbx + 8], rax
    push rbx
    sub rsp, 8
    mov eax, 0
    movd xmm0, eax
    movd eax, xmm0
    add rsp, 8
    pop rbx
    mov qword [rbx + 16], rax
    mov rax, rbx
    mov qword [rbp-48], rax
    mov eax, 0
    mov dword [rbp-56], eax
loop_begin_j_1:
    mov eax, dword [rbp-56]
    mov ebx, dword [rbp-24]
    cmp eax, ebx
    jge loop_end_j_1
    mov eax, 0
    movd xmm0, eax
    movss dword [rbp-64], xmm0
    mov eax, 0
    mov dword [rbp-72], eax
loop_begin_k_2:
    mov eax, dword [rbp-72]
    mov ebx, dword [rbp-24]
    cmp eax, ebx
    jge loop_end_k_2
    movss xmm0, dword [rbp-64]
    sub rsp, 16
    movss dword [rsp], xmm0
    mov ebx, dword [rbp-72]
    movsxd rbx, ebx
    push rbx
    mov ebx, dword [rbp-40]
    movsxd rbx, ebx
    push rbx
    mov rax, qword [rbp-8]
    pop rbx
    mov rax, qword [rax + rbx * 8]
    pop rbx
    movss xmm0, dword [rax + rbx * 8]
    sub rsp, 16
    movss dword [rsp], xmm0
    mov ebx, dword [rbp-56]
    movsxd rbx, ebx
    push rbx
    mov ebx, dword [rbp-72]
    movsxd rbx, ebx
    push rbx
    mov rax, qword [rbp-16]
    pop rbx
    mov rax, qword [rax + rbx * 8]
    pop rbx
    movss xmm1, dword [rax + rbx * 8]
    movss xmm0, dword [rsp]
    add rsp, 16
    mulss xmm0, xmm1
    movss xmm1, xmm0
    movss xmm0, dword [rsp]
    add rsp, 16
    addss xmm0, xmm1
    movss dword [rbp-64], xmm0
    mov eax, dword [rbp-72]
    inc eax
    mov dword [rbp-72], eax
    jmp loop_begin_k_2
loop_end_k_2:
    movss xmm0, dword [rbp-64]
    movd eax, xmm0
    push rax
    sub rsp, 8
    mov ebx, dword [rbp-56]
    movsxd rbx, ebx
    push rbx
    sub rsp, 8
    mov rax, qword [rbp-48]
    add rsp, 8
    pop rbx
    add rsp, 8
    pop rcx
    mov qword [rax + rbx * 8], rcx
    mov eax, dword [rbp-56]
    inc eax
    mov dword [rbp-56], eax
    jmp loop_begin_j_1
loop_end_j_1:
    mov rax, qword [rbp-48]
    push rax
    sub rsp, 8
    mov ebx, dword [rbp-40]
    movsxd rbx, ebx
    push rbx
    sub rsp, 8
    mov rax, qword [rbp-32]
    add rsp, 8
    pop rbx
    add rsp, 8
    pop rcx
    mov qword [rax + rbx * 8], rcx
    mov eax, dword [rbp-40]
    inc eax
    mov dword [rbp-40], eax
    jmp loop_begin_i_0
loop_end_i_0:
    mov rax, qword [rbp-32]
    leave
    ret
    leave
    ret
func_matrix_transpose:
    push rbp
    mov rbp, rsp
    sub rsp, 176
    mov qword [rbp-8], rcx
    mov qword [rbp-16], rdx
    mov rax, [array_ptr]
    push rax
    sub rsp, 8
    add rax, 24
    mov [array_ptr], rax
    add rsp, 8
    pop rbx
    push rbx
    sub rsp, 8
    mov rax, [array_ptr]
    push rax
    sub rsp, 8
    add rax, 24
    mov [array_ptr], rax
    add rsp, 8
    pop rbx
    push rbx
    sub rsp, 8
    mov eax, 0
    movd xmm0, eax
    movd eax, xmm0
    add rsp, 8
    pop rbx
    mov qword [rbx + 0], rax
    push rbx
    sub rsp, 8
    mov eax, 0
    movd xmm0, eax
    movd eax, xmm0
    add rsp, 8
    pop rbx
    mov qword [rbx + 8], rax
    push rbx
    sub rsp, 8
    mov eax, 0
    movd xmm0, eax
    movd eax, xmm0
    add rsp, 8
    pop rbx
    mov qword [rbx + 16], rax
    mov rax, rbx
    add rsp, 8
    pop rbx
    mov qword [rbx + 0], rax
    push rbx
    sub rsp, 8
    mov rax, [array_ptr]
    push rax
    sub rsp, 8
    add rax, 24
    mov [array_ptr], rax
    add rsp, 8
    pop rbx
    push rbx
    sub rsp, 8
    mov eax, 0
    movd xmm0, eax
    movd eax, xmm0
    add rsp, 8
    pop rbx
    mov qword [rbx + 0], rax
    push rbx
    sub rsp, 8
    mov eax, 0
    movd xmm0, eax
    movd eax, xmm0
    add rsp, 8
    pop rbx
    mov qword [rbx + 8], rax
    push rbx
    sub rsp, 8
    mov eax, 0
    movd xmm0, eax
    movd eax, xmm0
    add rsp, 8
    pop rbx
    mov qword [rbx + 16], rax
    mov rax, rbx
    add rsp, 8
    pop rbx
    mov qword [rbx + 8], rax
    push rbx
    sub rsp, 8
    mov rax, [array_ptr]
    push rax
    sub rsp, 8
    add rax, 24
    mov [array_ptr], rax
    add rsp, 8
    pop rbx
    push rbx
    sub rsp, 8
    mov eax, 0
    movd xmm0, eax
    movd eax, xmm0
    add rsp, 8
    pop rbx
    mov qword [rbx + 0], rax
    push rbx
    sub rsp, 8
    mov eax, 0
    movd xmm0, eax
    movd eax, xmm0
    add rsp, 8
    pop rbx
    mov qword [rbx + 8], rax
    push rbx
    sub rsp, 8
    mov eax, 0
    movd xmm0, eax
    movd eax, xmm0
    add rsp, 8
    pop rbx
    mov qword [rbx + 16], rax
    mov rax, rbx
    add rsp, 8
    pop rbx
    mov qword [rbx + 16], rax
    mov rax, rbx
    mov qword [rbp-24], rax
    mov eax, 0
    mov dword [rbp-32], eax
loop_begin_i_3:
    mov eax, dword [rbp-32]
    mov ebx, dword [rbp-16]
    cmp eax, ebx
    jge loop_end_i_3
    mov rax, [array_ptr]
    push rax
    sub rsp, 8
    add rax, 24
    mov [array_ptr], rax
    add rsp, 8
    pop rbx
    push rbx
    sub rsp, 8
    mov eax, 0
    movd xmm0, eax
    movd eax, xmm0
    add rsp, 8
    pop rbx
    mov qword [rbx + 0], rax
    push rbx
    sub rsp, 8
    mov eax, 0
    movd xmm0, eax
    movd eax, xmm0
    add rsp, 8
    pop rbx
    mov qword [rbx + 8], rax
    push rbx
    sub rsp, 8
    mov eax, 0
    movd xmm0, eax
    movd eax, xmm0
    add rsp, 8
    pop rbx
    mov qword [rbx + 16], rax
    mov rax, rbx
    mov qword [rbp-40], rax
    mov eax, 0
    mov dword [rbp-48], eax
loop_begin_j_4:
    mov eax, dword [rbp-48]
    mov ebx, dword [rbp-16]
    cmp eax, ebx
    jge loop_end_j_4
    mov ebx, dword [rbp-32]
    movsxd rbx, ebx
    push rbx
    mov ebx, dword [rbp-48]
    movsxd rbx, ebx
    push rbx
    mov rax, qword [rbp-8]
    pop rbx
    mov rax, qword [rax + rbx * 8]
    pop rbx
    movss xmm0, dword [rax + rbx * 8]
    movd eax, xmm0
    push rax
    sub rsp, 8
    mov ebx, dword [rbp-48]
    movsxd rbx, ebx
    push rbx
    sub rsp, 8
    mov rax, qword [rbp-40]
    add rsp, 8
    pop rbx
    add rsp, 8
    pop rcx
    mov qword [rax + rbx * 8], rcx
    mov eax, dword [rbp-48]
    inc eax
    mov dword [rbp-48], eax
    jmp loop_begin_j_4
loop_end_j_4:
    mov rax, qword [rbp-40]
    push rax
    sub rsp, 8
    mov ebx, dword [rbp-32]
    movsxd rbx, ebx
    push rbx
    sub rsp, 8
    mov rax, qword [rbp-24]
    add rsp, 8
    pop rbx
    add rsp, 8
    pop rcx
    mov qword [rax + rbx * 8], rcx
    mov eax, dword [rbp-32]
    inc eax
    mov dword [rbp-32], eax
    jmp loop_begin_i_3
loop_end_i_3:
    mov rax, qword [rbp-24]
    leave
    ret
    leave
    ret
func_is_symmetric:
    push rbp
    mov rbp, rsp
    sub rsp, 48
    mov qword [rbp-8], rcx
    mov qword [rbp-16], rdx
    mov eax, 0
    mov dword [rbp-24], eax
loop_begin_i_5:
    mov eax, dword [rbp-24]
    mov ebx, dword [rbp-16]
    cmp eax, ebx
    jge loop_end_i_5
    mov eax, 0
    mov dword [rbp-32], eax
loop_begin_j_6:
    mov eax, dword [rbp-32]
    mov ebx, dword [rbp-16]
    cmp eax, ebx
    jge loop_end_j_6
    mov ebx, dword [rbp-32]
    movsxd rbx, ebx
    push rbx
    mov ebx, dword [rbp-24]
    movsxd rbx, ebx
    push rbx
    mov rax, qword [rbp-8]
    pop rbx
    mov rax, qword [rax + rbx * 8]
    pop rbx
    movss xmm0, dword [rax + rbx * 8]
    sub rsp, 16
    movss dword [rsp], xmm0
    mov ebx, dword [rbp-24]
    movsxd rbx, ebx
    push rbx
    mov ebx, dword [rbp-32]
    movsxd rbx, ebx
    push rbx
    mov rax, qword [rbp-8]
    pop rbx
    mov rax, qword [rax + rbx * 8]
    pop rbx
    movss xmm1, dword [rax + rbx * 8]
    movss xmm0, dword [rsp]
    add rsp, 16
    subss xmm0, xmm1
    movss dword [rbp-40], xmm0
    movss xmm0, dword [rbp-40]
    sub rsp, 16
    movss dword [rsp], xmm0
    mov eax, 981668463
    movd xmm1, eax
    movss xmm0, dword [rsp]
    add rsp, 16
    ucomiss xmm0, xmm1
    seta al
    movzx eax, al
    cmp eax, 0
    je endif_7
    mov rax, 0
    leave
    ret
endif_7:
    movss xmm0, dword [rbp-40]
    sub rsp, 16
    movss dword [rsp], xmm0
    mov eax, 981668463
    movd xmm0, eax
    mov eax, 0x80000000
    movd xmm1, eax
    xorps xmm0, xmm1
    movss xmm1, xmm0
    movss xmm0, dword [rsp]
    add rsp, 16
    ucomiss xmm0, xmm1
    setb al
    movzx eax, al
    cmp eax, 0
    je endif_8
    mov rax, 0
    leave
    ret
endif_8:
    mov eax, dword [rbp-32]
    inc eax
    mov dword [rbp-32], eax
    jmp loop_begin_j_6
loop_end_j_6:
    mov eax, dword [rbp-24]
    inc eax
    mov dword [rbp-24], eax
    jmp loop_begin_i_5
loop_end_i_5:
    mov rax, 1
    leave
    ret
    leave
    ret
func_matrix_trace:
    push rbp
    mov rbp, rsp
    sub rsp, 32
    mov qword [rbp-8], rcx
    mov qword [rbp-16], rdx
    mov eax, 0
    movd xmm0, eax
    movss dword [rbp-24], xmm0
    mov eax, 0
    mov dword [rbp-32], eax
loop_begin_i_9:
    mov eax, dword [rbp-32]
    mov ebx, dword [rbp-16]
    cmp eax, ebx
    jge loop_end_i_9
    movss xmm0, dword [rbp-24]
    sub rsp, 16
    movss dword [rsp], xmm0
    mov ebx, dword [rbp-32]
    movsxd rbx, ebx
    push rbx
    mov ebx, dword [rbp-32]
    movsxd rbx, ebx
    push rbx
    mov rax, qword [rbp-8]
    pop rbx
    mov rax, qword [rax + rbx * 8]
    pop rbx
    movss xmm1, dword [rax + rbx * 8]
    movss xmm0, dword [rsp]
    add rsp, 16
    addss xmm0, xmm1
    movss dword [rbp-24], xmm0
    mov eax, dword [rbp-32]
    inc eax
    mov dword [rbp-32], eax
    jmp loop_begin_i_9
loop_end_i_9:
    movss xmm0, dword [rbp-24]
    leave
    ret
    leave
    ret
func_matrix_max:
    push rbp
    mov rbp, rsp
    sub rsp, 48
    mov qword [rbp-8], rcx
    mov qword [rbp-16], rdx
    mov ebx, 0
    movsxd rbx, ebx
    push rbx
    mov ebx, 0
    movsxd rbx, ebx
    push rbx
    mov rax, qword [rbp-8]
    pop rbx
    mov rax, qword [rax + rbx * 8]
    pop rbx
    movss xmm0, dword [rax + rbx * 8]
    movss dword [rbp-24], xmm0
    mov eax, 0
    mov dword [rbp-32], eax
loop_begin_i_10:
    mov eax, dword [rbp-32]
    mov ebx, dword [rbp-16]
    cmp eax, ebx
    jge loop_end_i_10
    mov eax, 0
    mov dword [rbp-40], eax
loop_begin_j_11:
    mov eax, dword [rbp-40]
    mov ebx, dword [rbp-16]
    cmp eax, ebx
    jge loop_end_j_11
    mov ebx, dword [rbp-40]
    movsxd rbx, ebx
    push rbx
    mov ebx, dword [rbp-32]
    movsxd rbx, ebx
    push rbx
    mov rax, qword [rbp-8]
    pop rbx
    mov rax, qword [rax + rbx * 8]
    pop rbx
    movss xmm0, dword [rax + rbx * 8]
    sub rsp, 16
    movss dword [rsp], xmm0
    movss xmm1, dword [rbp-24]
    movss xmm0, dword [rsp]
    add rsp, 16
    ucomiss xmm0, xmm1
    seta al
    movzx eax, al
    cmp eax, 0
    je endif_12
    mov ebx, dword [rbp-40]
    movsxd rbx, ebx
    push rbx
    mov ebx, dword [rbp-32]
    movsxd rbx, ebx
    push rbx
    mov rax, qword [rbp-8]
    pop rbx
    mov rax, qword [rax + rbx * 8]
    pop rbx
    movss xmm0, dword [rax + rbx * 8]
    movss dword [rbp-24], xmm0
endif_12:
    mov eax, dword [rbp-40]
    inc eax
    mov dword [rbp-40], eax
    jmp loop_begin_j_11
loop_end_j_11:
    mov eax, dword [rbp-32]
    inc eax
    mov dword [rbp-32], eax
    jmp loop_begin_i_10
loop_end_i_10:
    movss xmm0, dword [rbp-24]
    leave
    ret
    leave
    ret
func_compare_matrices:
    push rbp
    mov rbp, rsp
    sub rsp, 48
    mov qword [rbp-8], rcx
    mov qword [rbp-16], rdx
    mov qword [rbp-24], r8
    mov eax, 0
    mov dword [rbp-32], eax
loop_begin_i_13:
    mov eax, dword [rbp-32]
    mov ebx, dword [rbp-24]
    cmp eax, ebx
    jge loop_end_i_13
    mov eax, 0
    mov dword [rbp-40], eax
loop_begin_j_14:
    mov eax, dword [rbp-40]
    mov ebx, dword [rbp-24]
    cmp eax, ebx
    jge loop_end_j_14
    mov ebx, dword [rbp-40]
    movsxd rbx, ebx
    push rbx
    mov ebx, dword [rbp-32]
    movsxd rbx, ebx
    push rbx
    mov rax, qword [rbp-8]
    pop rbx
    mov rax, qword [rax + rbx * 8]
    pop rbx
    movss xmm0, dword [rax + rbx * 8]
    sub rsp, 16
    movss dword [rsp], xmm0
    mov ebx, dword [rbp-40]
    movsxd rbx, ebx
    push rbx
    mov ebx, dword [rbp-32]
    movsxd rbx, ebx
    push rbx
    mov rax, qword [rbp-16]
    pop rbx
    mov rax, qword [rax + rbx * 8]
    pop rbx
    movss xmm1, dword [rax + rbx * 8]
    movss xmm0, dword [rsp]
    add rsp, 16
    ucomiss xmm0, xmm1
    setne al
    mov ah, al
    setp al
    or al, ah
    movzx eax, al
    cmp eax, 0
    je endif_15
    mov rax, 0
    leave
    ret
endif_15:
    mov eax, dword [rbp-40]
    inc eax
    mov dword [rbp-40], eax
    jmp loop_begin_j_14
loop_end_j_14:
    mov eax, dword [rbp-32]
    inc eax
    mov dword [rbp-32], eax
    jmp loop_begin_i_13
loop_end_i_13:
    mov rax, 1
    leave
    ret
    leave
    ret
func_main:
    push rbp
    mov rbp, rsp
    sub rsp, 256
    mov rax, 3
    mov qword [rbp-8], rax
    mov rax, [array_ptr]
    push rax
    sub rsp, 8
    add rax, 24
    mov [array_ptr], rax
    add rsp, 8
    pop rbx
    push rbx
    sub rsp, 8
    mov rax, [array_ptr]
    push rax
    sub rsp, 8
    add rax, 24
    mov [array_ptr], rax
    add rsp, 8
    pop rbx
    push rbx
    sub rsp, 8
    mov eax, 1069547520
    movd xmm0, eax
    movd eax, xmm0
    add rsp, 8
    pop rbx
    mov qword [rbx + 0], rax
    push rbx
    sub rsp, 8
    mov eax, 1073741824
    movd xmm0, eax
    movd eax, xmm0
    add rsp, 8
    pop rbx
    mov qword [rbx + 8], rax
    push rbx
    sub rsp, 8
    mov eax, 1077936128
    movd xmm0, eax
    movd eax, xmm0
    add rsp, 8
    pop rbx
    mov qword [rbx + 16], rax
    mov rax, rbx
    add rsp, 8
    pop rbx
    mov qword [rbx + 0], rax
    push rbx
    sub rsp, 8
    mov rax, [array_ptr]
    push rax
    sub rsp, 8
    add rax, 24
    mov [array_ptr], rax
    add rsp, 8
    pop rbx
    push rbx
    sub rsp, 8
    mov eax, 1082130432
    movd xmm0, eax
    movd eax, xmm0
    add rsp, 8
    pop rbx
    mov qword [rbx + 0], rax
    push rbx
    sub rsp, 8
    mov eax, 1085276160
    movd xmm0, eax
    movd eax, xmm0
    add rsp, 8
    pop rbx
    mov qword [rbx + 8], rax
    push rbx
    sub rsp, 8
    mov eax, 1086324736
    movd xmm0, eax
    movd eax, xmm0
    add rsp, 8
    pop rbx
    mov qword [rbx + 16], rax
    mov rax, rbx
    add rsp, 8
    pop rbx
    mov qword [rbx + 8], rax
    push rbx
    sub rsp, 8
    mov rax, [array_ptr]
    push rax
    sub rsp, 8
    add rax, 24
    mov [array_ptr], rax
    add rsp, 8
    pop rbx
    push rbx
    sub rsp, 8
    mov eax, 1088421888
    movd xmm0, eax
    movd eax, xmm0
    add rsp, 8
    pop rbx
    mov qword [rbx + 0], rax
    push rbx
    sub rsp, 8
    mov eax, 1090519040
    movd xmm0, eax
    movd eax, xmm0
    add rsp, 8
    pop rbx
    mov qword [rbx + 8], rax
    push rbx
    sub rsp, 8
    mov eax, 1092091904
    movd xmm0, eax
    movd eax, xmm0
    add rsp, 8
    pop rbx
    mov qword [rbx + 16], rax
    mov rax, rbx
    add rsp, 8
    pop rbx
    mov qword [rbx + 16], rax
    mov rax, rbx
    mov qword [rbp-16], rax
    mov rax, [array_ptr]
    push rax
    sub rsp, 8
    add rax, 24
    mov [array_ptr], rax
    add rsp, 8
    pop rbx
    push rbx
    sub rsp, 8
    mov rax, [array_ptr]
    push rax
    sub rsp, 8
    add rax, 24
    mov [array_ptr], rax
    add rsp, 8
    pop rbx
    push rbx
    sub rsp, 8
    mov eax, 1065353216
    movd xmm0, eax
    movd eax, xmm0
    add rsp, 8
    pop rbx
    mov qword [rbx + 0], rax
    push rbx
    sub rsp, 8
    mov eax, 0
    movd xmm0, eax
    movd eax, xmm0
    add rsp, 8
    pop rbx
    mov qword [rbx + 8], rax
    push rbx
    sub rsp, 8
    mov eax, 0
    movd xmm0, eax
    movd eax, xmm0
    add rsp, 8
    pop rbx
    mov qword [rbx + 16], rax
    mov rax, rbx
    add rsp, 8
    pop rbx
    mov qword [rbx + 0], rax
    push rbx
    sub rsp, 8
    mov rax, [array_ptr]
    push rax
    sub rsp, 8
    add rax, 24
    mov [array_ptr], rax
    add rsp, 8
    pop rbx
    push rbx
    sub rsp, 8
    mov eax, 0
    movd xmm0, eax
    movd eax, xmm0
    add rsp, 8
    pop rbx
    mov qword [rbx + 0], rax
    push rbx
    sub rsp, 8
    mov eax, 1065353216
    movd xmm0, eax
    movd eax, xmm0
    add rsp, 8
    pop rbx
    mov qword [rbx + 8], rax
    push rbx
    sub rsp, 8
    mov eax, 0
    movd xmm0, eax
    movd eax, xmm0
    add rsp, 8
    pop rbx
    mov qword [rbx + 16], rax
    mov rax, rbx
    add rsp, 8
    pop rbx
    mov qword [rbx + 8], rax
    push rbx
    sub rsp, 8
    mov rax, [array_ptr]
    push rax
    sub rsp, 8
    add rax, 24
    mov [array_ptr], rax
    add rsp, 8
    pop rbx
    push rbx
    sub rsp, 8
    mov eax, 0
    movd xmm0, eax
    movd eax, xmm0
    add rsp, 8
    pop rbx
    mov qword [rbx + 0], rax
    push rbx
    sub rsp, 8
    mov eax, 0
    movd xmm0, eax
    movd eax, xmm0
    add rsp, 8
    pop rbx
    mov qword [rbx + 8], rax
    push rbx
    sub rsp, 8
    mov eax, 1065353216
    movd xmm0, eax
    movd eax, xmm0
    add rsp, 8
    pop rbx
    mov qword [rbx + 16], rax
    mov rax, rbx
    add rsp, 8
    pop rbx
    mov qword [rbx + 16], rax
    mov rax, rbx
    mov qword [rbp-24], rax
    lea rcx, [str_0]
    sub rsp, 32
    call puts
    add rsp, 32
    sub rsp, 32
    mov rax, qword [rbp-16]
    mov qword [rsp + 0], rax
    mov rax, qword [rbp-24]
    mov qword [rsp + 8], rax
    mov rax, qword [rbp-8]
    mov qword [rsp + 16], rax
    mov rcx, qword [rsp]
    mov rdx, qword [rsp + 8]
    mov r8, qword [rsp + 16]
    call func_matrix_multiply
    add rsp, 32
    mov qword [rbp-32], rax
    lea rcx, [str_1]
    sub rsp, 32
    call puts
    add rsp, 32
    sub rsp, 32
    mov rax, qword [rbp-32]
    mov qword [rsp + 0], rax
    mov rax, qword [rbp-16]
    mov qword [rsp + 8], rax
    mov rax, qword [rbp-8]
    mov qword [rsp + 16], rax
    mov rcx, qword [rsp]
    mov rdx, qword [rsp + 8]
    mov r8, qword [rsp + 16]
    call func_compare_matrices
    add rsp, 32
    mov rdx, rax
    lea rcx, [fmt_int]
    sub rsp, 32
    call printf
    add rsp, 32
    sub rsp, 32
    mov rax, qword [rbp-24]
    mov qword [rsp + 0], rax
    mov rax, qword [rbp-8]
    mov qword [rsp + 8], rax
    mov rcx, qword [rsp]
    mov rdx, qword [rsp + 8]
    call func_matrix_transpose
    add rsp, 32
    mov qword [rbp-40], rax
    lea rcx, [str_2]
    sub rsp, 32
    call puts
    add rsp, 32
    sub rsp, 32
    mov rax, qword [rbp-40]
    mov qword [rsp + 0], rax
    mov rax, qword [rbp-8]
    mov qword [rsp + 8], rax
    mov rcx, qword [rsp]
    mov rdx, qword [rsp + 8]
    call func_is_symmetric
    add rsp, 32
    mov rdx, rax
    lea rcx, [fmt_int]
    sub rsp, 32
    call printf
    add rsp, 32
    sub rsp, 32
    mov rax, qword [rbp-16]
    mov qword [rsp + 0], rax
    mov rax, qword [rbp-8]
    mov qword [rsp + 8], rax
    mov rcx, qword [rsp]
    mov rdx, qword [rsp + 8]
    call func_matrix_trace
    add rsp, 32
    movss dword [rbp-48], xmm0
    lea rcx, [str_3]
    sub rsp, 32
    call puts
    add rsp, 32
    movss xmm1, dword [rbp-48]
    cvtss2sd xmm1, xmm1
    movq rdx, xmm1
    lea rcx, [fmt_float]
    sub rsp, 32
    mov eax, 1
    call printf
    add rsp, 32
    sub rsp, 32
    mov rax, qword [rbp-16]
    mov qword [rsp + 0], rax
    mov rax, qword [rbp-8]
    mov qword [rsp + 8], rax
    mov rcx, qword [rsp]
    mov rdx, qword [rsp + 8]
    call func_matrix_max
    add rsp, 32
    movss dword [rbp-56], xmm0
    lea rcx, [str_4]
    sub rsp, 32
    call puts
    add rsp, 32
    movss xmm1, dword [rbp-56]
    cvtss2sd xmm1, xmm1
    movq rdx, xmm1
    lea rcx, [fmt_float]
    sub rsp, 32
    mov eax, 1
    call printf
    add rsp, 32
    movss xmm0, dword [rbp-56]
    sub rsp, 16
    movss dword [rsp], xmm0
    mov eax, 1091567616
    movd xmm1, eax
    movss xmm0, dword [rsp]
    add rsp, 16
    ucomiss xmm0, xmm1
    seta al
    movzx eax, al
    cmp eax, 0
    je else_16
    movss xmm0, dword [rbp-56]
    sub rsp, 16
    movss dword [rsp], xmm0
    mov eax, 1092616192
    movd xmm1, eax
    movss xmm0, dword [rsp]
    add rsp, 16
    ucomiss xmm0, xmm1
    setbe al
    movzx eax, al
    cmp eax, 0
    je endif_17
    lea rcx, [str_5]
    sub rsp, 32
    call puts
    add rsp, 32
endif_17:
    jmp endif_16
else_16:
    lea rcx, [str_6]
    sub rsp, 32
    call puts
    add rsp, 32
endif_16:
    mov rax, 0
    leave
    ret
    leave
    ret
main_entry:
    call func_main
    mov rcx, rax
    and rsp, -16
    sub rsp, 32
    call ExitProcess

segment .bss
array_ptr resq 1

segment .data
fmt_int db `%d\n`, 0
str_5 db `Max is between 9 and 10`, 0
str_1 db `Matrix A * Identity = A:`, 0
str_2 db `Identity is symmetric:`, 0
str_0 db `Testing matrix operations:`, 0
str_4 db `Maximum value in matrix A:`, 0
str_6 db `Max is 9 or less`, 0
fmt_str db `%s\n`, 0
fmt_float db `%f\n`, 0
str_3 db `Trace of matrix A:`, 0
