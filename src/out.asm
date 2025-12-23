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
    sub rsp, 136
    mov rax, 1
    mov qword [rbp-56], rax
    mov rax, 2
    mov qword [rbp-48], rax
    mov rax, 3
    mov qword [rbp-40], rax
    lea rax, [rbp-56]
    mov qword [rbp-32], rax
    mov rax, 4
    mov qword [rbp-80], rax
    mov rax, 5
    mov qword [rbp-72], rax
    mov rax, 6
    mov qword [rbp-64], rax
    lea rax, [rbp-80]
    mov qword [rbp-24], rax
    mov rax, 7
    mov qword [rbp-104], rax
    mov rax, 8
    mov qword [rbp-96], rax
    mov rax, 9
    mov qword [rbp-88], rax
    lea rax, [rbp-104]
    mov qword [rbp-16], rax
    lea rax, [rbp-32]
    mov qword [rbp-8], rax
    mov rax, 0
    mov qword [rbp-112], rax
    mov rax, 0
    mov qword [rbp-120], rax
    mov eax, 0
    mov dword [rbp-128], eax
loop_begin_i_0:
    mov eax, dword [rbp-128]
    mov ebx, 3
    cmp eax, ebx
    jge loop_end_i_0
    mov eax, 0
    mov dword [rbp-136], eax
loop_begin_j_1:
    mov eax, dword [rbp-136]
    mov ebx, 2
    cmp eax, ebx
    jge loop_end_j_1
    mov rax, qword [rbp-112]
    push rax
    mov ebx, dword [rbp-136]
    movsxd rbx, ebx
    push rbx
    mov ebx, dword [rbp-128]
    movsxd rbx, ebx
    push rbx
    mov rax, qword [rbp-8]
    pop rbx
    mov rax, qword [rax + rbx * 8]
    pop rbx
    mov rbx, qword [rax + rbx * 8]
    pop rax
    add eax, ebx
    mov eax, eax
    mov qword [rbp-112], rax
    mov rax, qword [rbp-120]
    push rax
    mov rbx, 1
    pop rax
    add eax, ebx
    mov eax, eax
    mov qword [rbp-120], rax
    mov ebx, dword [rbp-136]
    movsxd rbx, ebx
    push rbx
    mov ebx, dword [rbp-128]
    movsxd rbx, ebx
    push rbx
    mov rax, qword [rbp-8]
    pop rbx
    mov rax, qword [rax + rbx * 8]
    pop rbx
    mov rdx, qword [rax + rbx * 8]
    lea rcx, [fmt_int]
    and rsp, -16
    sub rsp, 32
    call printf
    add rsp, 32
    mov eax, dword [rbp-136]
    inc eax
    mov dword [rbp-136], eax
    jmp loop_begin_j_1
loop_end_j_1:
    mov rax, qword [rbp-112]
    push rax
    mov ebx, dword [rbp-136]
    movsxd rbx, ebx
    push rbx
    mov ebx, dword [rbp-128]
    movsxd rbx, ebx
    push rbx
    mov rax, qword [rbp-8]
    pop rbx
    mov rax, qword [rax + rbx * 8]
    pop rbx
    mov rbx, qword [rax + rbx * 8]
    pop rax
    add eax, ebx
    mov eax, eax
    mov qword [rbp-112], rax
    mov rax, qword [rbp-120]
    push rax
    mov rbx, 1
    pop rax
    add eax, ebx
    mov eax, eax
    mov qword [rbp-120], rax
    mov ebx, dword [rbp-136]
    movsxd rbx, ebx
    push rbx
    mov ebx, dword [rbp-128]
    movsxd rbx, ebx
    push rbx
    mov rax, qword [rbp-8]
    pop rbx
    mov rax, qword [rax + rbx * 8]
    pop rbx
    mov rdx, qword [rax + rbx * 8]
    lea rcx, [fmt_int]
    and rsp, -16
    sub rsp, 32
    call printf
    add rsp, 32
    lea rcx, [str_0]
    and rsp, -16
    sub rsp, 32
    call puts
    add rsp, 32
    mov eax, dword [rbp-128]
    inc eax
    mov dword [rbp-128], eax
    jmp loop_begin_i_0
loop_end_i_0:
    lea rcx, [str_1]
    and rsp, -16
    sub rsp, 32
    call puts
    add rsp, 32
    mov rdx, qword [rbp-112]
    lea rcx, [fmt_int]
    and rsp, -16
    sub rsp, 32
    call printf
    add rsp, 32
    lea rcx, [str_2]
    and rsp, -16
    sub rsp, 32
    call puts
    add rsp, 32
    mov rdx, qword [rbp-120]
    lea rcx, [fmt_int]
    and rsp, -16
    sub rsp, 32
    call printf
    add rsp, 32
    mov eax, dword [rbp-112]
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
str_1 db `Sum of 3x3 grid:`, 0
str_2 db `Count of 3x3 grid:`, 0
fmt_str db `%s\n`, 0
str_0 db `\n`, 0
