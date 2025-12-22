bits 64
default rel

segment .text
global mainCRTStartup
extern ExitProcess
extern puts

mainCRTStartup:
    jmp main_entry
func_main:
    push rbp
    mov rbp, rsp
    lea rax, [str_0]
    mov qword [rbp-8], rax
    mov rcx, dword [rbp-8]
    and rsp, -16
    sub rsp, 32
    call puts
    add rsp, 32
    lea rcx, [str_1]
    and rsp, -16
    sub rsp, 32
    call puts
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
str_1 db `Direct literal print`, 0
str_0 db `Hello, Noble!`, 0
