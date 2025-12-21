bits 64
default rel

segment .text
global mainCRTStartup
extern ExitProcess

mainCRTStartup:
    jmp main_entry
func_update_result:
    push rbp
    mov rbp, rsp
    mov eax, dword [rbp+16]
    push rax
    mov ebx, dword [rbp+24]
    pop rax
    add eax, ebx
    mov dword [result], eax
    leave
    ret
func_add_more:
    push rbp
    mov rbp, rsp
    mov eax, dword [result]
    push rax
    mov ebx, dword [rbp+16]
    pop rax
    add eax, ebx
    mov eax, eax
    leave
    ret
    leave
    ret
main_entry:
    mov dword [result], 0
    mov eax, 20
    push rax
    mov eax, 10
    push rax
    call func_update_result
    add rsp, 16
    mov eax, 5
    push rax
    call func_add_more
    add rsp, 8
    mov dword [result], eax
    mov eax, dword [result]
    mov rcx, rax
    and rsp, -16
    sub rsp, 32
    call ExitProcess
    ret

segment .bss
result resd 1
