bits 64
default rel

segment .text
global mainCRTStartup
extern ExitProcess
extern puts
extern printf

mainCRTStartup:
    jmp main_entry
func_is_identity:
    push rbp
    mov rbp, rsp
    sub rsp, 16
    mov eax, 0
    mov dword [rbp-8], eax
loop_begin_i_0:
    mov eax, dword [rbp-8]
    mov ebx, 3
    cmp eax, ebx
    jge loop_end_i_0
    mov eax, 0
    mov dword [rbp-16], eax
loop_begin_j_1:
    mov eax, dword [rbp-16]
    mov ebx, 3
    cmp eax, ebx
    jge loop_end_j_1
    mov rax, qword [rbp-8]
    push rax
    mov rbx, qword [rbp-16]
    pop rax
    cmp eax, ebx
    sete al
    movzx eax, al
    cmp eax, 0
    je else_2
    mov ebx, dword [rbp-16]
    movsxd rbx, ebx
    push rbx
    mov ebx, dword [rbp-8]
    movsxd rbx, ebx
    push rbx
    mov rax, qword [rbp+16]
    pop rbx
    mov rax, qword [rax + rbx * 8]
    pop rbx
    mov rax, qword [rax + rbx * 8]
    push rax
    mov rbx, 1
    pop rax
    cmp eax, ebx
    setne al
    movzx eax, al
    cmp eax, 0
    je endif_3
    mov eax, 0
    leave
    ret
endif_3:
    jmp endif_2
else_2:
    mov ebx, dword [rbp-16]
    movsxd rbx, ebx
    push rbx
    mov ebx, dword [rbp-8]
    movsxd rbx, ebx
    push rbx
    mov rax, qword [rbp+16]
    pop rbx
    mov rax, qword [rax + rbx * 8]
    pop rbx
    mov rax, qword [rax + rbx * 8]
    push rax
    mov rbx, 0
    pop rax
    cmp eax, ebx
    setne al
    movzx eax, al
    cmp eax, 0
    je endif_4
    mov eax, 0
    leave
    ret
endif_4:
endif_2:
    mov eax, dword [rbp-16]
    inc eax
    mov dword [rbp-16], eax
    jmp loop_begin_j_1
loop_end_j_1:
    mov eax, dword [rbp-8]
    inc eax
    mov dword [rbp-8], eax
    jmp loop_begin_i_0
loop_end_i_0:
    mov eax, 1
    leave
    ret
    leave
    ret
func_main:
    push rbp
    mov rbp, rsp
    sub rsp, 104
    mov rax, 1
    mov qword [rbp-56], rax
    mov rax, 0
    mov qword [rbp-48], rax
    mov rax, 0
    mov qword [rbp-40], rax
    lea rax, [rbp-56]
    mov qword [rbp-32], rax
    mov rax, 0
    mov qword [rbp-80], rax
    mov rax, 1
    mov qword [rbp-72], rax
    mov rax, 0
    mov qword [rbp-64], rax
    lea rax, [rbp-80]
    mov qword [rbp-24], rax
    mov rax, 0
    mov qword [rbp-104], rax
    mov rax, 0
    mov qword [rbp-96], rax
    mov rax, 1
    mov qword [rbp-88], rax
    lea rax, [rbp-104]
    mov qword [rbp-16], rax
    lea rax, [rbp-32]
    mov qword [rbp-8], rax
    lea rcx, [str_0]
    and rsp, -16
    sub rsp, 32
    call puts
    add rsp, 32
    mov eax, dword [rbp-8]
    push rax
    call func_is_identity
    add rsp, 8
    mov rdx, rax
    lea rcx, [fmt_int]
    and rsp, -16
    sub rsp, 32
    call printf
    add rsp, 32
    mov eax, 0
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

segment .data
fmt_int db `%d\n`, 0
fmt_str db `%s\n`, 0
str_0 db `Matrix is identity:`, 0
