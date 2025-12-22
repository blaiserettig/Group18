bits 64
default rel

segment .text
global mainCRTStartup
extern ExitProcess
extern puts
extern printf

mainCRTStartup:
    jmp main_entry
func_is_less_than:
    push rbp
    mov rbp, rsp
    mov rax, qword [rbp+16]
    push rax
    mov rbx, qword [rbp+24]
    pop rax
    cmp eax, ebx
    setl al
    movzx eax, al
    leave
    ret
    leave
    ret
func_is_equal:
    push rbp
    mov rbp, rsp
    mov rax, qword [rbp+16]
    push rax
    mov rbx, qword [rbp+24]
    pop rax
    cmp eax, ebx
    sete al
    movzx eax, al
    leave
    ret
    leave
    ret
func_main:
    push rbp
    mov rbp, rsp
    sub rsp, 112
    mov rax, 0
    mov qword [rbp-8], rax
    mov eax, 0
    mov dword [rbp-16], eax
loop_begin_i:
    mov eax, dword [rbp-16]
    mov ebx, 10
    cmp eax, ebx
    jg loop_end_i
    mov rax, qword [rbp-8]
    push rax
    mov rbx, qword [rbp-16]
    pop rax
    add eax, ebx
    mov eax, eax
    mov qword [rbp-8], rax
    mov eax, dword [rbp-16]
    inc eax
    mov dword [rbp-16], eax
    jmp loop_begin_i
loop_end_i:
    mov rax, 0
    mov qword [rbp-24], rax
    mov rax, 1078530000
    mov qword [rbp-32], rax
    mov rax, qword [rbp-8]
    push rax
    mov rbx, 10
    pop rax
    add eax, ebx
    mov eax, eax
    push rax
    mov rbx, 5
    pop rax
    imul eax, ebx
    mov eax, eax
    push rax
    mov rbx, 2
    pop rax
    cdq
    idiv ebx
    mov eax, eax
    mov qword [rbp-40], rax
    mov rax, 1
    mov qword [rbp-80], rax
    mov rax, 2
    mov qword [rbp-72], rax
    lea rax, [rbp-80]
    mov qword [rbp-64], rax
    mov rax, 3
    mov qword [rbp-96], rax
    mov rax, 4
    mov qword [rbp-88], rax
    lea rax, [rbp-96]
    mov qword [rbp-56], rax
    lea rax, [rbp-64]
    mov qword [rbp-48], rax
    lea rcx, [str_0]
    and rsp, -16
    sub rsp, 32
    call puts
    add rsp, 32
    mov ebx, 1
    movsxd rbx, ebx
    push rbx
    mov ebx, 1
    movsxd rbx, ebx
    push rbx
    mov rax, qword [rbp-48]
    pop rbx
    mov rax, qword [rax + rbx * 8]
    pop rbx
    mov rdx, qword [rax + rbx * 8]
    lea rcx, [fmt_int]
    and rsp, -16
    sub rsp, 32
    call printf
    add rsp, 32
    mov rax, 97
    mov qword [rbp-104], rax
    mov rax, 98
    mov qword [rbp-112], rax
    mov eax, dword [rbp-112]
    push rax
    mov eax, dword [rbp-104]
    push rax
    call func_is_less_than
    add rsp, 16
    mov rdx, rax
    lea rcx, [fmt_int]
    and rsp, -16
    sub rsp, 32
    call printf
    add rsp, 32
    mov rdx, qword [rbp-104]
    lea rcx, [fmt_int]
    and rsp, -16
    sub rsp, 32
    call printf
    add rsp, 32
    mov rdx, 1
    lea rcx, [fmt_int]
    and rsp, -16
    sub rsp, 32
    call printf
    add rsp, 32
    mov eax, dword [rbp-112]
    push rax
    mov eax, dword [rbp-104]
    push rax
    call func_is_less_than
    add rsp, 16
    mov eax, eax
    cmp eax, 0
    je else_0
    mov eax, 2
    leave
    ret
    jmp endif_0
else_0:
    mov eax, dword [rbp-112]
    push rax
    mov eax, dword [rbp-104]
    push rax
    call func_is_equal
    add rsp, 16
    mov eax, eax
    cmp eax, 0
    je else_1
    mov eax, 1
    leave
    ret
    jmp endif_1
else_1:
    mov eax, 0
    leave
    ret
endif_1:
endif_0:
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
str_0 db `Hello, World!`, 0
