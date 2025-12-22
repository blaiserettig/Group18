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
    sub rsp, 120
    mov rax, 10
    mov qword [rbp-32], rax
    mov rax, 20
    mov qword [rbp-24], rax
    mov rax, 30
    mov qword [rbp-16], rax
    lea rax, [rbp-32]
    mov qword [rbp-8], rax
    mov ebx, 0
    movsxd rbx, ebx
    push rbx
    mov rax, qword [rbp-8]
    pop rbx
    mov rax, qword [rax + rbx * 8]
    mov qword [rbp-40], rax
    mov ebx, 1
    movsxd rbx, ebx
    push rbx
    mov rax, qword [rbp-8]
    pop rbx
    mov rax, qword [rax + rbx * 8]
    mov qword [rbp-48], rax
    mov ebx, 2
    movsxd rbx, ebx
    push rbx
    mov rax, qword [rbp-8]
    pop rbx
    mov rax, qword [rax + rbx * 8]
    mov qword [rbp-56], rax
    mov rax, 1
    mov qword [rbp-96], rax
    mov rax, 2
    mov qword [rbp-88], rax
    lea rax, [rbp-96]
    mov qword [rbp-80], rax
    mov rax, 3
    mov qword [rbp-112], rax
    mov rax, 4
    mov qword [rbp-104], rax
    lea rax, [rbp-112]
    mov qword [rbp-72], rax
    lea rax, [rbp-80]
    mov qword [rbp-64], rax
    mov ebx, 1
    movsxd rbx, ebx
    push rbx
    mov ebx, 0
    movsxd rbx, ebx
    push rbx
    mov rax, qword [rbp-64]
    pop rbx
    mov rax, qword [rax + rbx * 8]
    pop rbx
    mov rax, qword [rax + rbx * 8]
    mov qword [rbp-120], rax
    mov rax, qword [rbp-40]
    push rax
    mov rbx, qword [rbp-48]
    pop rax
    add eax, ebx
    mov eax, eax
    push rax
    mov rbx, qword [rbp-56]
    pop rax
    add eax, ebx
    mov eax, eax
    push rax
    mov rbx, qword [rbp-120]
    pop rax
    add eax, ebx
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
fmt_str db `%s\n`, 0
fmt_int db `%d\n`, 0
