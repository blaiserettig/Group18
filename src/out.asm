bits 64
default rel

segment .text
global mainCRTStartup
extern ExitProcess
extern puts

mainCRTStartup:
    jmp main_entry
main_entry:
    lea rcx, [str_0]
    and rsp, -16
    sub rsp, 32
    call puts
    add rsp, 32
    ret

segment .data
str_0 db `Hello, World!`, 0
