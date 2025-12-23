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
func_fibonacci:
    push rbp
    mov rbp, rsp
    sub rsp, 176
    mov rax, [array_ptr]
    push rax
    sub rsp, 8
    add rax, 160
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
    push rbx
    sub rsp, 8
    mov rax, 0
    add rsp, 8
    pop rbx
    mov qword [rbx + 24], rax
    push rbx
    sub rsp, 8
    mov rax, 0
    add rsp, 8
    pop rbx
    mov qword [rbx + 32], rax
    push rbx
    sub rsp, 8
    mov rax, 0
    add rsp, 8
    pop rbx
    mov qword [rbx + 40], rax
    push rbx
    sub rsp, 8
    mov rax, 0
    add rsp, 8
    pop rbx
    mov qword [rbx + 48], rax
    push rbx
    sub rsp, 8
    mov rax, 0
    add rsp, 8
    pop rbx
    mov qword [rbx + 56], rax
    push rbx
    sub rsp, 8
    mov rax, 0
    add rsp, 8
    pop rbx
    mov qword [rbx + 64], rax
    push rbx
    sub rsp, 8
    mov rax, 0
    add rsp, 8
    pop rbx
    mov qword [rbx + 72], rax
    push rbx
    sub rsp, 8
    mov rax, 0
    add rsp, 8
    pop rbx
    mov qword [rbx + 80], rax
    push rbx
    sub rsp, 8
    mov rax, 0
    add rsp, 8
    pop rbx
    mov qword [rbx + 88], rax
    push rbx
    sub rsp, 8
    mov rax, 0
    add rsp, 8
    pop rbx
    mov qword [rbx + 96], rax
    push rbx
    sub rsp, 8
    mov rax, 0
    add rsp, 8
    pop rbx
    mov qword [rbx + 104], rax
    push rbx
    sub rsp, 8
    mov rax, 0
    add rsp, 8
    pop rbx
    mov qword [rbx + 112], rax
    push rbx
    sub rsp, 8
    mov rax, 0
    add rsp, 8
    pop rbx
    mov qword [rbx + 120], rax
    push rbx
    sub rsp, 8
    mov rax, 0
    add rsp, 8
    pop rbx
    mov qword [rbx + 128], rax
    push rbx
    sub rsp, 8
    mov rax, 0
    add rsp, 8
    pop rbx
    mov qword [rbx + 136], rax
    push rbx
    sub rsp, 8
    mov rax, 0
    add rsp, 8
    pop rbx
    mov qword [rbx + 144], rax
    push rbx
    sub rsp, 8
    mov rax, 0
    add rsp, 8
    pop rbx
    mov qword [rbx + 152], rax
    mov rax, rbx
    mov qword [rbp-8], rax
    mov eax, 2
    mov dword [rbp-16], eax
loop_begin_i_0:
    mov eax, dword [rbp-16]
    mov ebx, 20
    cmp eax, ebx
    jge loop_end_i_0
    mov rax, qword [rbp-16]
    push rax
    mov rbx, 1
    pop rax
    sub eax, ebx
    mov ebx, rax
    movsxd rbx, ebx
    push rbx
    mov rax, qword [rbp-8]
    pop rbx
    mov rax, qword [rax + rbx * 8]
    push rax
    mov rax, qword [rbp-16]
    push rax
    mov rbx, 2
    pop rax
    sub eax, ebx
    mov ebx, rax
    movsxd rbx, ebx
    push rbx
    mov rax, qword [rbp-8]
    pop rbx
    mov rbx, qword [rax + rbx * 8]
    pop rax
    add eax, ebx
    mov eax, eax
    push rax
    sub rsp, 8
    mov ebx, dword [rbp-16]
    movsxd rbx, ebx
    push rbx
    sub rsp, 8
    mov rax, qword [rbp-8]
    add rsp, 8
    pop rbx
    add rsp, 8
    pop rcx
    mov qword [rax + rbx * 8], rcx
    mov eax, dword [rbp-16]
    inc eax
    mov dword [rbp-16], eax
    jmp loop_begin_i_0
loop_end_i_0:
    mov rax, qword [rbp-8]
    leave
    ret
    leave
    ret
func_is_fibonacci:
    push rbp
    mov rbp, rsp
    sub rsp, 32
    mov qword [rbp-8], rcx
    mov qword [rbp-16], rdx
    mov eax, 0
    mov dword [rbp-24], eax
loop_begin_i_1:
    mov eax, dword [rbp-24]
    mov ebx, 20
    cmp eax, ebx
    jge loop_end_i_1
    mov ebx, dword [rbp-24]
    movsxd rbx, ebx
    push rbx
    mov rax, qword [rbp-16]
    pop rbx
    mov rax, qword [rax + rbx * 8]
    push rax
    mov rbx, qword [rbp-8]
    pop rax
    cmp eax, ebx
    sete al
    movzx eax, al
    cmp eax, 0
    je endif_2
    mov rax, 1
    leave
    ret
endif_2:
    mov ebx, dword [rbp-24]
    movsxd rbx, ebx
    push rbx
    mov rax, qword [rbp-16]
    pop rbx
    mov rax, qword [rax + rbx * 8]
    push rax
    mov rbx, qword [rbp-8]
    pop rax
    cmp eax, ebx
    setg al
    movzx eax, al
    cmp eax, 0
    je endif_3
    mov rax, 0
    leave
    ret
endif_3:
    mov eax, dword [rbp-24]
    inc eax
    mov dword [rbp-24], eax
    jmp loop_begin_i_1
loop_end_i_1:
    mov rax, 0
    leave
    ret
    leave
    ret
func_sum_of_divisors:
    push rbp
    mov rbp, rsp
    sub rsp, 48
    mov qword [rbp-8], rcx
    mov rax, 0
    mov qword [rbp-16], rax
    mov eax, 1
    mov dword [rbp-24], eax
loop_begin_i_4:
    mov eax, dword [rbp-24]
    mov ebx, dword [rbp-8]
    cmp eax, ebx
    jge loop_end_i_4
    mov rax, qword [rbp-8]
    push rax
    mov rbx, qword [rbp-24]
    pop rax
    cdq
    idiv ebx
    mov eax, eax
    mov qword [rbp-32], rax
    mov rax, qword [rbp-32]
    push rax
    mov rbx, qword [rbp-24]
    pop rax
    imul eax, ebx
    mov eax, eax
    mov qword [rbp-40], rax
    mov rax, qword [rbp-40]
    push rax
    mov rbx, qword [rbp-8]
    pop rax
    cmp eax, ebx
    sete al
    movzx eax, al
    cmp eax, 0
    je endif_5
    mov rax, qword [rbp-16]
    push rax
    mov rbx, qword [rbp-24]
    pop rax
    add eax, ebx
    mov eax, eax
    mov qword [rbp-16], rax
endif_5:
    mov eax, dword [rbp-24]
    inc eax
    mov dword [rbp-24], eax
    jmp loop_begin_i_4
loop_end_i_4:
    mov rax, qword [rbp-16]
    leave
    ret
    leave
    ret
func_is_perfect_number:
    push rbp
    mov rbp, rsp
    sub rsp, 32
    mov qword [rbp-8], rcx
    sub rsp, 32
    mov rax, qword [rbp-8]
    mov qword [rsp + 0], rax
    mov rcx, qword [rsp]
    call func_sum_of_divisors
    add rsp, 32
    mov qword [rbp-16], rax
    mov rax, qword [rbp-8]
    push rax
    mov rbx, qword [rbp-8]
    pop rax
    add eax, ebx
    mov eax, eax
    mov qword [rbp-24], rax
    mov rax, qword [rbp-16]
    push rax
    mov rbx, qword [rbp-24]
    pop rax
    cmp eax, ebx
    sete al
    movzx eax, al
    cmp eax, 0
    je endif_6
    mov rax, 1
    leave
    ret
endif_6:
    mov rax, 0
    leave
    ret
    leave
    ret
func_gcd:
    push rbp
    mov rbp, rsp
    sub rsp, 48
    mov qword [rbp-8], rcx
    mov qword [rbp-16], rdx
    mov eax, 0
    mov dword [rbp-24], eax
loop_begin_i_7:
    mov eax, dword [rbp-24]
    mov ebx, 1000
    cmp eax, ebx
    jge loop_end_i_7
    mov rax, qword [rbp-16]
    push rax
    mov rbx, 0
    pop rax
    cmp eax, ebx
    sete al
    movzx eax, al
    cmp eax, 0
    je endif_8
    mov rax, qword [rbp-8]
    leave
    ret
endif_8:
    mov rax, qword [rbp-16]
    mov qword [rbp-32], rax
    mov rax, qword [rbp-8]
    push rax
    mov rbx, qword [rbp-16]
    pop rax
    cdq
    idiv ebx
    mov eax, eax
    mov qword [rbp-40], rax
    mov rax, qword [rbp-8]
    push rax
    mov rax, qword [rbp-40]
    push rax
    mov rbx, qword [rbp-16]
    pop rax
    imul eax, ebx
    mov rbx, rax
    pop rax
    sub eax, ebx
    mov eax, eax
    mov qword [rbp-16], rax
    mov rax, qword [rbp-32]
    mov qword [rbp-8], rax
    mov eax, dword [rbp-24]
    inc eax
    mov dword [rbp-24], eax
    jmp loop_begin_i_7
loop_end_i_7:
    mov rax, qword [rbp-8]
    leave
    ret
    leave
    ret
func_main:
    push rbp
    mov rbp, rsp
    sub rsp, 32
    lea rdx, [str_0]
    lea rcx, [fmt_str_raw]
    sub rsp, 32
    call printf
    add rsp, 32
    sub rsp, 32
    call func_fibonacci
    add rsp, 32
    mov qword [rbp-8], rax
    mov eax, 0
    mov dword [rbp-16], eax
loop_begin_i_9:
    mov eax, dword [rbp-16]
    mov ebx, 20
    cmp eax, ebx
    jge loop_end_i_9
    mov ebx, dword [rbp-16]
    movsxd rbx, ebx
    push rbx
    mov rax, qword [rbp-8]
    pop rbx
    mov rdx, qword [rax + rbx * 8]
    lea rcx, [fmt_int_raw]
    sub rsp, 32
    call printf
    add rsp, 32
    mov eax, dword [rbp-16]
    inc eax
    mov dword [rbp-16], eax
    jmp loop_begin_i_9
loop_end_i_9:
    lea rdx, [str_1]
    lea rcx, [fmt_str_raw]
    sub rsp, 32
    call printf
    add rsp, 32
    sub rsp, 32
    mov rax, 13
    mov qword [rsp + 0], rax
    mov rax, qword [rbp-8]
    mov qword [rsp + 8], rax
    mov rcx, qword [rsp]
    mov rdx, qword [rsp + 8]
    call func_is_fibonacci
    add rsp, 32
    cmp rax, 0
    jne print_bool_true_10
    lea rcx, [str_false_raw]
    jmp print_bool_end_10
print_bool_true_10:
    lea rcx, [str_true_raw]
print_bool_end_10:
    sub rsp, 32
    mov rdx, rcx
    lea rcx, [fmt_str_raw]
    call printf
    add rsp, 32
    lea rdx, [str_2]
    lea rcx, [fmt_str_raw]
    sub rsp, 32
    call printf
    add rsp, 32
    sub rsp, 32
    mov rax, 14
    mov qword [rsp + 0], rax
    mov rax, qword [rbp-8]
    mov qword [rsp + 8], rax
    mov rcx, qword [rsp]
    mov rdx, qword [rsp + 8]
    call func_is_fibonacci
    add rsp, 32
    cmp rax, 0
    jne print_bool_true_11
    lea rcx, [str_false_raw]
    jmp print_bool_end_11
print_bool_true_11:
    lea rcx, [str_true_raw]
print_bool_end_11:
    sub rsp, 32
    mov rdx, rcx
    lea rcx, [fmt_str_raw]
    call printf
    add rsp, 32
    lea rdx, [str_3]
    lea rcx, [fmt_str_raw]
    sub rsp, 32
    call printf
    add rsp, 32
    mov eax, 1
    mov dword [rbp-16], eax
loop_begin_i_12:
    mov eax, dword [rbp-16]
    mov ebx, 30
    cmp eax, ebx
    jge loop_end_i_12
    sub rsp, 32
    mov rax, qword [rbp-16]
    mov qword [rsp + 0], rax
    mov rcx, qword [rsp]
    call func_is_perfect_number
    add rsp, 32
    mov eax, eax
    cmp eax, 0
    je endif_13
    mov rdx, qword [rbp-16]
    lea rcx, [fmt_int_raw]
    sub rsp, 32
    call printf
    add rsp, 32
endif_13:
    mov eax, dword [rbp-16]
    inc eax
    mov dword [rbp-16], eax
    jmp loop_begin_i_12
loop_end_i_12:
    lea rdx, [str_4]
    lea rcx, [fmt_str_raw]
    sub rsp, 32
    call printf
    add rsp, 32
    sub rsp, 32
    mov rax, 48
    mov qword [rsp + 0], rax
    mov rax, 18
    mov qword [rsp + 8], rax
    mov rcx, qword [rsp]
    mov rdx, qword [rsp + 8]
    call func_gcd
    add rsp, 32
    mov rdx, rax
    lea rcx, [fmt_int_raw]
    sub rsp, 32
    call printf
    add rsp, 32
    lea rdx, [str_5]
    lea rcx, [fmt_str_raw]
    sub rsp, 32
    call printf
    add rsp, 32
    sub rsp, 32
    mov rax, 100
    mov qword [rsp + 0], rax
    mov rax, 35
    mov qword [rsp + 8], rax
    mov rcx, qword [rsp]
    mov rdx, qword [rsp + 8]
    call func_gcd
    add rsp, 32
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
str_1 db `Testing if 13 is a Fibonacci number:`, 0
fmt_int db `%d\n`, 0
str_true_raw db `true`, 0
fmt_float_raw db `%f`, 0
fmt_str db `%s\n`, 0
fmt_float db `%f\n`, 0
str_false db `false\n`, 0
str_3 db `Perfect numbers up to 30:`, 0
str_5 db `GCD of 100 and 35:`, 0
str_2 db `Testing if 14 is a Fibonacci number:`, 0
str_true db `true\n`, 0
fmt_str_raw db `%s`, 0
str_0 db `Fibonacci sequence up to 20:`, 0
str_4 db `GCD of 48 and 18:`, 0
str_false_raw db `false`, 0
fmt_int_raw db `%d`, 0
