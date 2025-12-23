bits 64
default rel

segment .text
global mainCRTStartup
extern ExitProcess
extern puts
extern printf

mainCRTStartup:
    jmp main_entry
func_main:
    push rbp
    mov rbp, rsp
    sub rsp, 72
    mov rax, 42
    mov qword [rbp-56], rax
    mov rax, 17
    mov qword [rbp-48], rax
    mov rax, 23
    mov qword [rbp-40], rax
    mov rax, 8
    mov qword [rbp-32], rax
    mov rax, 99
    mov qword [rbp-24], rax
    mov rax, 5
    mov qword [rbp-16], rax
    lea rax, [rbp-56]
    mov qword [rbp-8], rax
    mov ebx, 0
    movsxd rbx, ebx
    push rbx
    mov rax, qword [rbp-8]
    pop rbx
    mov rax, qword [rax + rbx * 8]
    mov qword [rbp-64], rax
    mov eax, 1
    mov dword [rbp-72], eax
loop_begin_i_0:
    mov eax, dword [rbp-72]
    mov ebx, 6
    cmp eax, ebx
    jge loop_end_i_0
    mov ebx, dword [rbp-72]
    movsxd rbx, ebx
    push rbx
    mov rax, qword [rbp-8]
    pop rbx
    mov rax, qword [rax + rbx * 8]
    push rax
    mov rbx, qword [rbp-64]
    pop rax
    cmp eax, ebx
    setl al
    movzx eax, al
    cmp eax, 0
    je endif_1
    mov ebx, dword [rbp-72]
    movsxd rbx, ebx
    push rbx
    mov rax, qword [rbp-8]
    pop rbx
    mov rax, qword [rax + rbx * 8]
    mov qword [rbp-64], rax
endif_1:
    mov eax, dword [rbp-72]
    inc eax
    mov dword [rbp-72], eax
    jmp loop_begin_i_0
loop_end_i_0:
    lea rcx, [str_0]
    and rsp, -16
    sub rsp, 32
    call puts
    add rsp, 32
    mov rdx, qword [rbp-64]
    lea rcx, [fmt_int]
    and rsp, -16
    sub rsp, 32
    call printf
    add rsp, 32
    mov eax, dword [rbp-64]
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
str_0 db `Minimum value:`, 0
fmt_str db `%s\n`, 0
fmt_int db `%d\n`, 0
