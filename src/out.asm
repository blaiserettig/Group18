bits 64
default rel

segment .text
global mainCRTStartup
extern ExitProcess
extern puts
extern printf

mainCRTStartup:
    push rbp
    mov rbp, rsp
    
    ; Allocate 1MB on stack for array_heap
    ; Touch pages to ensure stack is committed (Stack Probe)
    mov rcx, 1048576
    mov rax, 4096
.probe_loop:
    sub rsp, rax
    test [rsp], rsp ; Touch the page
    sub rcx, rax
    cmp rcx, 0
    jg .probe_loop
    
    ; Save the start of our stack heap to array_ptr
    mov [array_ptr], rsp
    jmp main_entry
func_main:
    push rbp
    mov rbp, rsp
    sub rsp, 112
    mov rax, [array_ptr]
    push rax
    sub rsp, 8
    add rax, 24
    mov [array_ptr], rax
    add rsp, 8
    pop rbx
    push rbx
    sub rsp, 8
    mov rax, [array_ptr]
    push rax
    sub rsp, 8
    add rax, 24
    mov [array_ptr], rax
    add rsp, 8
    pop rbx
    push rbx
    sub rsp, 8
    mov rax, 1
    add rsp, 8
    pop rbx
    mov qword [rbx + 0], rax
    push rbx
    sub rsp, 8
    mov rax, 0
    add rsp, 8
    pop rbx
    mov qword [rbx + 8], rax
    push rbx
    sub rsp, 8
    mov rax, 0
    add rsp, 8
    pop rbx
    mov qword [rbx + 16], rax
    mov rax, rbx
    add rsp, 8
    pop rbx
    mov qword [rbx + 0], rax
    push rbx
    sub rsp, 8
    mov rax, [array_ptr]
    push rax
    sub rsp, 8
    add rax, 24
    mov [array_ptr], rax
    add rsp, 8
    pop rbx
    push rbx
    sub rsp, 8
    mov rax, 0
    add rsp, 8
    pop rbx
    mov qword [rbx + 0], rax
    push rbx
    sub rsp, 8
    mov rax, 1
    add rsp, 8
    pop rbx
    mov qword [rbx + 8], rax
    push rbx
    sub rsp, 8
    mov rax, 0
    add rsp, 8
    pop rbx
    mov qword [rbx + 16], rax
    mov rax, rbx
    add rsp, 8
    pop rbx
    mov qword [rbx + 8], rax
    push rbx
    sub rsp, 8
    mov rax, [array_ptr]
    push rax
    sub rsp, 8
    add rax, 24
    mov [array_ptr], rax
    add rsp, 8
    pop rbx
    push rbx
    sub rsp, 8
    mov rax, 0
    add rsp, 8
    pop rbx
    mov qword [rbx + 0], rax
    push rbx
    sub rsp, 8
    mov rax, 0
    add rsp, 8
    pop rbx
    mov qword [rbx + 8], rax
    push rbx
    sub rsp, 8
    mov rax, 1
    add rsp, 8
    pop rbx
    mov qword [rbx + 16], rax
    mov rax, rbx
    add rsp, 8
    pop rbx
    mov qword [rbx + 16], rax
    mov rax, rbx
    mov qword [rbp-8], rax
    mov ebx, 1
    movsxd rbx, ebx
    push rbx
    mov ebx, 1
    movsxd rbx, ebx
    push rbx
    mov rax, qword [rbp-8]
    pop rbx
    mov rax, qword [rax + rbx * 8]
    pop rbx
    mov rax, qword [rax + rbx * 8]
    push rax
    mov rbx, 5
    pop rax
    imul eax, ebx
    mov rdx, rax
    lea rcx, [fmt_int_raw]
    sub rsp, 32
    call printf
    add rsp, 32
    mov rax, 0
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

segment .bss
array_ptr resq 1

segment .data
fmt_float db `%f\n`, 0
str_true db `true\n`, 0
str_false_raw db `false`, 0
fmt_char_raw db `%c`, 0
str_true_raw db `true`, 0
fmt_str_raw db `%s`, 0
fmt_float_raw db `%f`, 0
fmt_char db `%c\n`, 0
fmt_str db `%s\n`, 0
fmt_int db `%d\n`, 0
fmt_int_raw db `%d`, 0
str_false db `false\n`, 0
