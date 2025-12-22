bits 64
default rel

segment .text
global mainCRTStartup
extern ExitProcess
extern puts

mainCRTStartup:
    jmp main_entry
main_entry:
    ret
