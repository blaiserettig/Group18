bits 64
default rel

segment .text
global mainCRTStartup
extern ExitProcess
extern puts

mainCRTStartup:
    jmp main_entry
main_entry:
    mov eax, 10
    mov dword [a + 0], eax
    mov eax, 20
    mov dword [a + 8], eax
    mov eax, 30
    mov dword [a + 16], eax
    mov ebx, 1
    movsxd rbx, ebx
    mov eax, dword [a + rbx * 8]
    mov dword [x], eax
    mov eax, dword [x]
    mov rcx, rax
    and rsp, -16
    sub rsp, 32
    call ExitProcess
    ret

segment .bss
x resd 1
a resd 1
