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
func_count_char:
    push rbp
    mov rbp, rsp
    sub rsp, 48
    mov qword [rbp-8], rcx
    mov qword [rbp-16], rdx
    mov qword [rbp-24], r8
    mov rax, 0
    mov qword [rbp-32], rax
    mov eax, 0
    mov dword [rbp-40], eax
loop_begin_i_0:
    mov eax, dword [rbp-40]
    mov ebx, dword [rbp-16]
    cmp eax, ebx
    jge loop_end_i_0
    mov ebx, dword [rbp-40]
    movsxd rbx, ebx
    push rbx
    mov rax, qword [rbp-8]
    pop rbx
    mov rax, qword [rax + rbx * 8]
    push rax
    mov rbx, qword [rbp-24]
    pop rax
    cmp eax, ebx
    sete al
    movzx eax, al
    cmp eax, 0
    je endif_1
    mov rax, qword [rbp-32]
    push rax
    mov rbx, 1
    pop rax
    add eax, ebx
    mov eax, eax
    mov qword [rbp-32], rax
endif_1:
    mov eax, dword [rbp-40]
    inc eax
    mov dword [rbp-40], eax
    jmp loop_begin_i_0
loop_end_i_0:
    mov rax, qword [rbp-32]
    leave
    ret
    leave
    ret
func_find_first:
    push rbp
    mov rbp, rsp
    sub rsp, 32
    mov qword [rbp-8], rcx
    mov qword [rbp-16], rdx
    mov qword [rbp-24], r8
    mov eax, 0
    mov dword [rbp-32], eax
loop_begin_i_2:
    mov eax, dword [rbp-32]
    mov ebx, dword [rbp-16]
    cmp eax, ebx
    jge loop_end_i_2
    mov ebx, dword [rbp-32]
    movsxd rbx, ebx
    push rbx
    mov rax, qword [rbp-8]
    pop rbx
    mov rax, qword [rax + rbx * 8]
    push rax
    mov rbx, qword [rbp-24]
    pop rax
    cmp eax, ebx
    sete al
    movzx eax, al
    cmp eax, 0
    je endif_3
    mov rax, qword [rbp-32]
    leave
    ret
endif_3:
    mov eax, dword [rbp-32]
    inc eax
    mov dword [rbp-32], eax
    jmp loop_begin_i_2
loop_end_i_2:
    mov rax, 1
    neg rax
    leave
    ret
    leave
    ret
func_find_last:
    push rbp
    mov rbp, rsp
    sub rsp, 48
    mov qword [rbp-8], rcx
    mov qword [rbp-16], rdx
    mov qword [rbp-24], r8
    mov rax, 1
    neg rax
    mov qword [rbp-32], rax
    mov eax, 0
    mov dword [rbp-40], eax
loop_begin_i_4:
    mov eax, dword [rbp-40]
    mov ebx, dword [rbp-16]
    cmp eax, ebx
    jge loop_end_i_4
    mov ebx, dword [rbp-40]
    movsxd rbx, ebx
    push rbx
    mov rax, qword [rbp-8]
    pop rbx
    mov rax, qword [rax + rbx * 8]
    push rax
    mov rbx, qword [rbp-24]
    pop rax
    cmp eax, ebx
    sete al
    movzx eax, al
    cmp eax, 0
    je endif_5
    mov rax, qword [rbp-40]
    mov qword [rbp-32], rax
endif_5:
    mov eax, dword [rbp-40]
    inc eax
    mov dword [rbp-40], eax
    jmp loop_begin_i_4
loop_end_i_4:
    mov rax, qword [rbp-32]
    leave
    ret
    leave
    ret
func_reverse_string:
    push rbp
    mov rbp, rsp
    sub rsp, 192
    mov qword [rbp-8], rcx
    mov qword [rbp-16], rdx
    mov rax, [array_ptr]
    push rax
    sub rsp, 8
    add rax, 160
    mov [array_ptr], rax
    add rsp, 8
    pop rbx
    push rbx
    sub rsp, 8
    mov rax, 120
    add rsp, 8
    pop rbx
    mov qword [rbx + 0], rax
    push rbx
    sub rsp, 8
    mov rax, 120
    add rsp, 8
    pop rbx
    mov qword [rbx + 8], rax
    push rbx
    sub rsp, 8
    mov rax, 120
    add rsp, 8
    pop rbx
    mov qword [rbx + 16], rax
    push rbx
    sub rsp, 8
    mov rax, 120
    add rsp, 8
    pop rbx
    mov qword [rbx + 24], rax
    push rbx
    sub rsp, 8
    mov rax, 120
    add rsp, 8
    pop rbx
    mov qword [rbx + 32], rax
    push rbx
    sub rsp, 8
    mov rax, 120
    add rsp, 8
    pop rbx
    mov qword [rbx + 40], rax
    push rbx
    sub rsp, 8
    mov rax, 120
    add rsp, 8
    pop rbx
    mov qword [rbx + 48], rax
    push rbx
    sub rsp, 8
    mov rax, 120
    add rsp, 8
    pop rbx
    mov qword [rbx + 56], rax
    push rbx
    sub rsp, 8
    mov rax, 120
    add rsp, 8
    pop rbx
    mov qword [rbx + 64], rax
    push rbx
    sub rsp, 8
    mov rax, 120
    add rsp, 8
    pop rbx
    mov qword [rbx + 72], rax
    push rbx
    sub rsp, 8
    mov rax, 120
    add rsp, 8
    pop rbx
    mov qword [rbx + 80], rax
    push rbx
    sub rsp, 8
    mov rax, 120
    add rsp, 8
    pop rbx
    mov qword [rbx + 88], rax
    push rbx
    sub rsp, 8
    mov rax, 120
    add rsp, 8
    pop rbx
    mov qword [rbx + 96], rax
    push rbx
    sub rsp, 8
    mov rax, 120
    add rsp, 8
    pop rbx
    mov qword [rbx + 104], rax
    push rbx
    sub rsp, 8
    mov rax, 120
    add rsp, 8
    pop rbx
    mov qword [rbx + 112], rax
    push rbx
    sub rsp, 8
    mov rax, 120
    add rsp, 8
    pop rbx
    mov qword [rbx + 120], rax
    push rbx
    sub rsp, 8
    mov rax, 120
    add rsp, 8
    pop rbx
    mov qword [rbx + 128], rax
    push rbx
    sub rsp, 8
    mov rax, 120
    add rsp, 8
    pop rbx
    mov qword [rbx + 136], rax
    push rbx
    sub rsp, 8
    mov rax, 120
    add rsp, 8
    pop rbx
    mov qword [rbx + 144], rax
    push rbx
    sub rsp, 8
    mov rax, 120
    add rsp, 8
    pop rbx
    mov qword [rbx + 152], rax
    mov rax, rbx
    mov qword [rbp-24], rax
    mov eax, 0
    mov dword [rbp-32], eax
loop_begin_i_6:
    mov eax, dword [rbp-32]
    mov ebx, dword [rbp-16]
    cmp eax, ebx
    jge loop_end_i_6
    mov rax, qword [rbp-16]
    push rax
    mov rbx, 1
    pop rax
    sub eax, ebx
    mov eax, eax
    push rax
    mov rbx, qword [rbp-32]
    pop rax
    sub eax, ebx
    mov ebx, eax
    movsxd rbx, ebx
    push rbx
    mov rax, qword [rbp-8]
    pop rbx
    mov rax, qword [rax + rbx * 8]
    push rax
    sub rsp, 8
    mov ebx, dword [rbp-32]
    movsxd rbx, ebx
    push rbx
    sub rsp, 8
    mov rax, qword [rbp-24]
    add rsp, 8
    pop rbx
    add rsp, 8
    pop rcx
    mov qword [rax + rbx * 8], rcx
    mov eax, dword [rbp-32]
    inc eax
    mov dword [rbp-32], eax
    jmp loop_begin_i_6
loop_end_i_6:
    mov rax, qword [rbp-24]
    leave
    ret
    leave
    ret
func_is_palindrome:
    push rbp
    mov rbp, rsp
    sub rsp, 32
    mov qword [rbp-8], rcx
    mov qword [rbp-16], rdx
    mov rax, qword [rbp-16]
    push rax
    mov rbx, 2
    pop rax
    cdq
    idiv ebx
    mov eax, eax
    mov qword [rbp-24], rax
    mov eax, 0
    mov dword [rbp-32], eax
loop_begin_i_7:
    mov eax, dword [rbp-32]
    mov ebx, dword [rbp-24]
    cmp eax, ebx
    jge loop_end_i_7
    mov ebx, dword [rbp-32]
    movsxd rbx, ebx
    push rbx
    mov rax, qword [rbp-8]
    pop rbx
    mov rax, qword [rax + rbx * 8]
    push rax
    mov rax, qword [rbp-16]
    push rax
    mov rbx, 1
    pop rax
    sub eax, ebx
    mov eax, eax
    push rax
    mov rbx, qword [rbp-32]
    pop rax
    sub eax, ebx
    mov ebx, eax
    movsxd rbx, ebx
    push rbx
    mov rax, qword [rbp-8]
    pop rbx
    mov rbx, qword [rax + rbx * 8]
    pop rax
    cmp eax, ebx
    setne al
    movzx eax, al
    cmp eax, 0
    je endif_8
    mov rax, 0
    leave
    ret
endif_8:
    mov eax, dword [rbp-32]
    inc eax
    mov dword [rbp-32], eax
    jmp loop_begin_i_7
loop_end_i_7:
    mov rax, 1
    leave
    ret
    leave
    ret
func_strings_equal:
    push rbp
    mov rbp, rsp
    sub rsp, 32
    mov qword [rbp-8], rcx
    mov qword [rbp-16], rdx
    mov qword [rbp-24], r8
    mov eax, 0
    mov dword [rbp-32], eax
loop_begin_i_9:
    mov eax, dword [rbp-32]
    mov ebx, dword [rbp-24]
    cmp eax, ebx
    jge loop_end_i_9
    mov ebx, dword [rbp-32]
    movsxd rbx, ebx
    push rbx
    mov rax, qword [rbp-8]
    pop rbx
    mov rax, qword [rax + rbx * 8]
    push rax
    mov ebx, dword [rbp-32]
    movsxd rbx, ebx
    push rbx
    mov rax, qword [rbp-16]
    pop rbx
    mov rbx, qword [rax + rbx * 8]
    pop rax
    cmp eax, ebx
    setne al
    movzx eax, al
    cmp eax, 0
    je endif_10
    mov rax, 0
    leave
    ret
endif_10:
    mov eax, dword [rbp-32]
    inc eax
    mov dword [rbp-32], eax
    jmp loop_begin_i_9
loop_end_i_9:
    mov rax, 1
    leave
    ret
    leave
    ret
func_copy_substring:
    push rbp
    mov rbp, rsp
    sub rsp, 128
    mov qword [rbp-8], rcx
    mov qword [rbp-16], rdx
    mov qword [rbp-24], r8
    mov rax, [array_ptr]
    push rax
    sub rsp, 8
    add rax, 80
    mov [array_ptr], rax
    add rsp, 8
    pop rbx
    push rbx
    sub rsp, 8
    mov rax, 120
    add rsp, 8
    pop rbx
    mov qword [rbx + 0], rax
    push rbx
    sub rsp, 8
    mov rax, 120
    add rsp, 8
    pop rbx
    mov qword [rbx + 8], rax
    push rbx
    sub rsp, 8
    mov rax, 120
    add rsp, 8
    pop rbx
    mov qword [rbx + 16], rax
    push rbx
    sub rsp, 8
    mov rax, 120
    add rsp, 8
    pop rbx
    mov qword [rbx + 24], rax
    push rbx
    sub rsp, 8
    mov rax, 120
    add rsp, 8
    pop rbx
    mov qword [rbx + 32], rax
    push rbx
    sub rsp, 8
    mov rax, 120
    add rsp, 8
    pop rbx
    mov qword [rbx + 40], rax
    push rbx
    sub rsp, 8
    mov rax, 120
    add rsp, 8
    pop rbx
    mov qword [rbx + 48], rax
    push rbx
    sub rsp, 8
    mov rax, 120
    add rsp, 8
    pop rbx
    mov qword [rbx + 56], rax
    push rbx
    sub rsp, 8
    mov rax, 120
    add rsp, 8
    pop rbx
    mov qword [rbx + 64], rax
    push rbx
    sub rsp, 8
    mov rax, 120
    add rsp, 8
    pop rbx
    mov qword [rbx + 72], rax
    mov rax, rbx
    mov qword [rbp-32], rax
    mov rax, 0
    mov qword [rbp-40], rax
    mov eax, dword [rbp-16]
    mov dword [rbp-48], eax
loop_begin_i_11:
    mov eax, dword [rbp-48]
    mov ebx, dword [rbp-24]
    cmp eax, ebx
    jge loop_end_i_11
    mov ebx, dword [rbp-48]
    movsxd rbx, ebx
    push rbx
    mov rax, qword [rbp-8]
    pop rbx
    mov rax, qword [rax + rbx * 8]
    push rax
    sub rsp, 8
    mov ebx, dword [rbp-40]
    movsxd rbx, ebx
    push rbx
    sub rsp, 8
    mov rax, qword [rbp-32]
    add rsp, 8
    pop rbx
    add rsp, 8
    pop rcx
    mov qword [rax + rbx * 8], rcx
    mov rax, qword [rbp-40]
    push rax
    mov rbx, 1
    pop rax
    add eax, ebx
    mov eax, eax
    mov qword [rbp-40], rax
    mov eax, dword [rbp-48]
    inc eax
    mov dword [rbp-48], eax
    jmp loop_begin_i_11
loop_end_i_11:
    mov rax, qword [rbp-32]
    leave
    ret
    leave
    ret
func_replace_char:
    push rbp
    mov rbp, rsp
    sub rsp, 208
    mov qword [rbp-8], rcx
    mov qword [rbp-16], rdx
    mov qword [rbp-24], r8
    mov qword [rbp-32], r9
    mov rax, [array_ptr]
    push rax
    sub rsp, 8
    add rax, 160
    mov [array_ptr], rax
    add rsp, 8
    pop rbx
    push rbx
    sub rsp, 8
    mov rax, 120
    add rsp, 8
    pop rbx
    mov qword [rbx + 0], rax
    push rbx
    sub rsp, 8
    mov rax, 120
    add rsp, 8
    pop rbx
    mov qword [rbx + 8], rax
    push rbx
    sub rsp, 8
    mov rax, 120
    add rsp, 8
    pop rbx
    mov qword [rbx + 16], rax
    push rbx
    sub rsp, 8
    mov rax, 120
    add rsp, 8
    pop rbx
    mov qword [rbx + 24], rax
    push rbx
    sub rsp, 8
    mov rax, 120
    add rsp, 8
    pop rbx
    mov qword [rbx + 32], rax
    push rbx
    sub rsp, 8
    mov rax, 120
    add rsp, 8
    pop rbx
    mov qword [rbx + 40], rax
    push rbx
    sub rsp, 8
    mov rax, 120
    add rsp, 8
    pop rbx
    mov qword [rbx + 48], rax
    push rbx
    sub rsp, 8
    mov rax, 120
    add rsp, 8
    pop rbx
    mov qword [rbx + 56], rax
    push rbx
    sub rsp, 8
    mov rax, 120
    add rsp, 8
    pop rbx
    mov qword [rbx + 64], rax
    push rbx
    sub rsp, 8
    mov rax, 120
    add rsp, 8
    pop rbx
    mov qword [rbx + 72], rax
    push rbx
    sub rsp, 8
    mov rax, 120
    add rsp, 8
    pop rbx
    mov qword [rbx + 80], rax
    push rbx
    sub rsp, 8
    mov rax, 120
    add rsp, 8
    pop rbx
    mov qword [rbx + 88], rax
    push rbx
    sub rsp, 8
    mov rax, 120
    add rsp, 8
    pop rbx
    mov qword [rbx + 96], rax
    push rbx
    sub rsp, 8
    mov rax, 120
    add rsp, 8
    pop rbx
    mov qword [rbx + 104], rax
    push rbx
    sub rsp, 8
    mov rax, 120
    add rsp, 8
    pop rbx
    mov qword [rbx + 112], rax
    push rbx
    sub rsp, 8
    mov rax, 120
    add rsp, 8
    pop rbx
    mov qword [rbx + 120], rax
    push rbx
    sub rsp, 8
    mov rax, 120
    add rsp, 8
    pop rbx
    mov qword [rbx + 128], rax
    push rbx
    sub rsp, 8
    mov rax, 120
    add rsp, 8
    pop rbx
    mov qword [rbx + 136], rax
    push rbx
    sub rsp, 8
    mov rax, 120
    add rsp, 8
    pop rbx
    mov qword [rbx + 144], rax
    push rbx
    sub rsp, 8
    mov rax, 120
    add rsp, 8
    pop rbx
    mov qword [rbx + 152], rax
    mov rax, rbx
    mov qword [rbp-40], rax
    mov eax, 0
    mov dword [rbp-48], eax
loop_begin_i_12:
    mov eax, dword [rbp-48]
    mov ebx, dword [rbp-16]
    cmp eax, ebx
    jge loop_end_i_12
    mov ebx, dword [rbp-48]
    movsxd rbx, ebx
    push rbx
    mov rax, qword [rbp-8]
    pop rbx
    mov rax, qword [rax + rbx * 8]
    push rax
    mov rbx, qword [rbp-24]
    pop rax
    cmp eax, ebx
    sete al
    movzx eax, al
    cmp eax, 0
    je else_13
    mov rax, qword [rbp-32]
    push rax
    sub rsp, 8
    mov ebx, dword [rbp-48]
    movsxd rbx, ebx
    push rbx
    sub rsp, 8
    mov rax, qword [rbp-40]
    add rsp, 8
    pop rbx
    add rsp, 8
    pop rcx
    mov qword [rax + rbx * 8], rcx
    jmp endif_13
else_13:
    mov ebx, dword [rbp-48]
    movsxd rbx, ebx
    push rbx
    mov rax, qword [rbp-8]
    pop rbx
    mov rax, qword [rax + rbx * 8]
    push rax
    sub rsp, 8
    mov ebx, dword [rbp-48]
    movsxd rbx, ebx
    push rbx
    sub rsp, 8
    mov rax, qword [rbp-40]
    add rsp, 8
    pop rbx
    add rsp, 8
    pop rcx
    mov qword [rax + rbx * 8], rcx
endif_13:
    mov eax, dword [rbp-48]
    inc eax
    mov dword [rbp-48], eax
    jmp loop_begin_i_12
loop_end_i_12:
    mov rax, qword [rbp-40]
    leave
    ret
    leave
    ret
func_main:
    push rbp
    mov rbp, rsp
    sub rsp, 176
    mov rax, [array_ptr]
    push rax
    sub rsp, 8
    add rax, 56
    mov [array_ptr], rax
    add rsp, 8
    pop rbx
    push rbx
    sub rsp, 8
    mov rax, 114
    add rsp, 8
    pop rbx
    mov qword [rbx + 0], rax
    push rbx
    sub rsp, 8
    mov rax, 97
    add rsp, 8
    pop rbx
    mov qword [rbx + 8], rax
    push rbx
    sub rsp, 8
    mov rax, 99
    add rsp, 8
    pop rbx
    mov qword [rbx + 16], rax
    push rbx
    sub rsp, 8
    mov rax, 101
    add rsp, 8
    pop rbx
    mov qword [rbx + 24], rax
    push rbx
    sub rsp, 8
    mov rax, 99
    add rsp, 8
    pop rbx
    mov qword [rbx + 32], rax
    push rbx
    sub rsp, 8
    mov rax, 97
    add rsp, 8
    pop rbx
    mov qword [rbx + 40], rax
    push rbx
    sub rsp, 8
    mov rax, 114
    add rsp, 8
    pop rbx
    mov qword [rbx + 48], rax
    mov rax, rbx
    mov qword [rbp-8], rax
    mov rax, 7
    mov qword [rbp-16], rax
    lea rdx, [str_0]
    mov rcx, rdx
    sub rsp, 32
    call puts
    add rsp, 32
    lea rdx, [str_1]
    mov rcx, rdx
    sub rsp, 32
    call puts
    add rsp, 32
    lea rdx, [str_2]
    lea rcx, [fmt_str_raw]
    sub rsp, 32
    call printf
    add rsp, 32
    sub rsp, 32
    mov rax, qword [rbp-8]
    mov qword [rsp + 0], rax
    mov rax, qword [rbp-16]
    mov qword [rsp + 8], rax
    mov rcx, qword [rsp]
    mov rdx, qword [rsp + 8]
    call func_is_palindrome
    add rsp, 32
    cmp rax, 0
    jne print_bool_true_14
    lea rcx, [str_false_raw]
    jmp print_bool_end_14
print_bool_true_14:
    lea rcx, [str_true_raw]
print_bool_end_14:
    sub rsp, 32
    call puts
    add rsp, 32
    lea rdx, [str_1]
    mov rcx, rdx
    sub rsp, 32
    call puts
    add rsp, 32
    lea rdx, [str_3]
    lea rcx, [fmt_str_raw]
    sub rsp, 32
    call printf
    add rsp, 32
    sub rsp, 32
    mov rax, qword [rbp-8]
    mov qword [rsp + 0], rax
    mov rax, qword [rbp-16]
    mov qword [rsp + 8], rax
    mov rax, 97
    mov qword [rsp + 16], rax
    mov rcx, qword [rsp]
    mov rdx, qword [rsp + 8]
    mov r8, qword [rsp + 16]
    call func_count_char
    add rsp, 32
    mov rdx, rax
    lea rcx, [fmt_int]
    sub rsp, 32
    call printf
    add rsp, 32
    lea rdx, [str_1]
    mov rcx, rdx
    sub rsp, 32
    call puts
    add rsp, 32
    lea rdx, [str_4]
    lea rcx, [fmt_str_raw]
    sub rsp, 32
    call printf
    add rsp, 32
    sub rsp, 32
    mov rax, qword [rbp-8]
    mov qword [rsp + 0], rax
    mov rax, qword [rbp-16]
    mov qword [rsp + 8], rax
    mov rax, 99
    mov qword [rsp + 16], rax
    mov rcx, qword [rsp]
    mov rdx, qword [rsp + 8]
    mov r8, qword [rsp + 16]
    call func_count_char
    add rsp, 32
    mov rdx, rax
    lea rcx, [fmt_int]
    sub rsp, 32
    call printf
    add rsp, 32
    lea rdx, [str_1]
    mov rcx, rdx
    sub rsp, 32
    call puts
    add rsp, 32
    lea rdx, [str_5]
    lea rcx, [fmt_str_raw]
    sub rsp, 32
    call printf
    add rsp, 32
    sub rsp, 32
    mov rax, qword [rbp-8]
    mov qword [rsp + 0], rax
    mov rax, qword [rbp-16]
    mov qword [rsp + 8], rax
    mov rax, 99
    mov qword [rsp + 16], rax
    mov rcx, qword [rsp]
    mov rdx, qword [rsp + 8]
    mov r8, qword [rsp + 16]
    call func_find_first
    add rsp, 32
    mov rdx, rax
    lea rcx, [fmt_int]
    sub rsp, 32
    call printf
    add rsp, 32
    lea rdx, [str_1]
    mov rcx, rdx
    sub rsp, 32
    call puts
    add rsp, 32
    lea rdx, [str_6]
    lea rcx, [fmt_str_raw]
    sub rsp, 32
    call printf
    add rsp, 32
    sub rsp, 32
    mov rax, qword [rbp-8]
    mov qword [rsp + 0], rax
    mov rax, qword [rbp-16]
    mov qword [rsp + 8], rax
    mov rax, 99
    mov qword [rsp + 16], rax
    mov rcx, qword [rsp]
    mov rdx, qword [rsp + 8]
    mov r8, qword [rsp + 16]
    call func_find_last
    add rsp, 32
    mov rdx, rax
    lea rcx, [fmt_int]
    sub rsp, 32
    call printf
    add rsp, 32
    lea rdx, [str_1]
    mov rcx, rdx
    sub rsp, 32
    call puts
    add rsp, 32
    sub rsp, 32
    mov rax, qword [rbp-8]
    mov qword [rsp + 0], rax
    mov rax, qword [rbp-16]
    mov qword [rsp + 8], rax
    mov rcx, qword [rsp]
    mov rdx, qword [rsp + 8]
    call func_reverse_string
    add rsp, 32
    mov qword [rbp-24], rax
    lea rdx, [str_7]
    mov rcx, rdx
    sub rsp, 32
    call puts
    add rsp, 32
    mov eax, 0
    mov dword [rbp-32], eax
loop_begin_i_15:
    mov eax, dword [rbp-32]
    mov ebx, dword [rbp-16]
    cmp eax, ebx
    jge loop_end_i_15
    mov ebx, dword [rbp-32]
    movsxd rbx, ebx
    push rbx
    mov rax, qword [rbp-24]
    pop rbx
    mov rdx, qword [rax + rbx * 8]
    lea rcx, [fmt_char_raw]
    sub rsp, 32
    call printf
    add rsp, 32
    mov eax, dword [rbp-32]
    inc eax
    mov dword [rbp-32], eax
    jmp loop_begin_i_15
loop_end_i_15:
    lea rdx, [str_8]
    mov rcx, rdx
    sub rsp, 32
    call puts
    add rsp, 32
    lea rdx, [str_9]
    lea rcx, [fmt_str_raw]
    sub rsp, 32
    call printf
    add rsp, 32
    sub rsp, 32
    mov rax, qword [rbp-8]
    mov qword [rsp + 0], rax
    mov rax, qword [rbp-24]
    mov qword [rsp + 8], rax
    mov rax, qword [rbp-16]
    mov qword [rsp + 16], rax
    mov rcx, qword [rsp]
    mov rdx, qword [rsp + 8]
    mov r8, qword [rsp + 16]
    call func_strings_equal
    add rsp, 32
    cmp rax, 0
    jne print_bool_true_16
    lea rcx, [str_false_raw]
    jmp print_bool_end_16
print_bool_true_16:
    lea rcx, [str_true_raw]
print_bool_end_16:
    sub rsp, 32
    call puts
    add rsp, 32
    lea rdx, [str_1]
    mov rcx, rdx
    sub rsp, 32
    call puts
    add rsp, 32
    mov rax, [array_ptr]
    push rax
    sub rsp, 8
    add rax, 40
    mov [array_ptr], rax
    add rsp, 8
    pop rbx
    push rbx
    sub rsp, 8
    mov rax, 104
    add rsp, 8
    pop rbx
    mov qword [rbx + 0], rax
    push rbx
    sub rsp, 8
    mov rax, 101
    add rsp, 8
    pop rbx
    mov qword [rbx + 8], rax
    push rbx
    sub rsp, 8
    mov rax, 108
    add rsp, 8
    pop rbx
    mov qword [rbx + 16], rax
    push rbx
    sub rsp, 8
    mov rax, 108
    add rsp, 8
    pop rbx
    mov qword [rbx + 24], rax
    push rbx
    sub rsp, 8
    mov rax, 111
    add rsp, 8
    pop rbx
    mov qword [rbx + 32], rax
    mov rax, rbx
    mov qword [rbp-32], rax
    mov rax, 5
    mov qword [rbp-40], rax
    lea rdx, [str_10]
    lea rcx, [fmt_str_raw]
    sub rsp, 32
    call printf
    add rsp, 32
    sub rsp, 32
    mov rax, qword [rbp-32]
    mov qword [rsp + 0], rax
    mov rax, qword [rbp-40]
    mov qword [rsp + 8], rax
    mov rcx, qword [rsp]
    mov rdx, qword [rsp + 8]
    call func_is_palindrome
    add rsp, 32
    cmp rax, 0
    jne print_bool_true_17
    lea rcx, [str_false_raw]
    jmp print_bool_end_17
print_bool_true_17:
    lea rcx, [str_true_raw]
print_bool_end_17:
    sub rsp, 32
    call puts
    add rsp, 32
    lea rdx, [str_1]
    mov rcx, rdx
    sub rsp, 32
    call puts
    add rsp, 32
    sub rsp, 32
    mov rax, qword [rbp-32]
    mov qword [rsp + 0], rax
    mov rax, qword [rbp-40]
    mov qword [rsp + 8], rax
    mov rax, 108
    mov qword [rsp + 16], rax
    mov rax, 112
    mov qword [rsp + 24], rax
    mov rcx, qword [rsp]
    mov rdx, qword [rsp + 8]
    mov r8, qword [rsp + 16]
    mov r9, qword [rsp + 24]
    call func_replace_char
    add rsp, 32
    mov qword [rbp-48], rax
    lea rdx, [str_11]
    mov rcx, rdx
    sub rsp, 32
    call puts
    add rsp, 32
    mov eax, 0
    mov dword [rbp-56], eax
loop_begin_i_18:
    mov eax, dword [rbp-56]
    mov ebx, dword [rbp-40]
    cmp eax, ebx
    jge loop_end_i_18
    mov ebx, dword [rbp-56]
    movsxd rbx, ebx
    push rbx
    mov rax, qword [rbp-48]
    pop rbx
    mov rdx, qword [rax + rbx * 8]
    lea rcx, [fmt_char_raw]
    sub rsp, 32
    call printf
    add rsp, 32
    mov eax, dword [rbp-56]
    inc eax
    mov dword [rbp-56], eax
    jmp loop_begin_i_18
loop_end_i_18:
    lea rdx, [str_8]
    mov rcx, rdx
    sub rsp, 32
    call puts
    add rsp, 32
    sub rsp, 32
    mov rax, qword [rbp-32]
    mov qword [rsp + 0], rax
    mov rax, 1
    mov qword [rsp + 8], rax
    mov rax, 4
    mov qword [rsp + 16], rax
    mov rcx, qword [rsp]
    mov rdx, qword [rsp + 8]
    mov r8, qword [rsp + 16]
    call func_copy_substring
    add rsp, 32
    mov qword [rbp-56], rax
    lea rdx, [str_12]
    mov rcx, rdx
    sub rsp, 32
    call puts
    add rsp, 32
    mov eax, 0
    mov dword [rbp-64], eax
loop_begin_i_19:
    mov eax, dword [rbp-64]
    mov ebx, 3
    cmp eax, ebx
    jge loop_end_i_19
    mov ebx, dword [rbp-64]
    movsxd rbx, ebx
    push rbx
    mov rax, qword [rbp-56]
    pop rbx
    mov rdx, qword [rax + rbx * 8]
    lea rcx, [fmt_char_raw]
    sub rsp, 32
    call printf
    add rsp, 32
    mov eax, dword [rbp-64]
    inc eax
    mov dword [rbp-64], eax
    jmp loop_begin_i_19
loop_end_i_19:
    lea rdx, [str_8]
    mov rcx, rdx
    sub rsp, 32
    call puts
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
fmt_int db `%d\n`, 0
str_1 db ``, 0
str_false_raw db `false`, 0
fmt_str db `%s\n`, 0
str_5 db `First occurrence of 'c': `, 0
str_true_raw db `true`, 0
str_9 db `Original equals reversed: `, 0
fmt_str_raw db `%s`, 0
str_6 db `Last occurrence of 'c': `, 0
fmt_float_raw db `%f`, 0
str_3 db `Count of 'a': `, 0
str_false db `false\n`, 0
str_2 db `Is palindrome: `, 0
str_11 db `Replace 'l' with 'p' in hello:`, 0
fmt_int_raw db `%d`, 0
str_12 db `Substring from index 1 to 4:`, 0
str_0 db `Testing: racecar`, 0
str_8 db `\n`, 0
fmt_char db `%c\n`, 0
str_true db `true\n`, 0
fmt_char_raw db `%c`, 0
str_4 db `Count of 'c': `, 0
str_7 db `Reversed (should be same):`, 0
str_10 db `Is 'hello' a palindrome: `, 0
fmt_float db `%f\n`, 0
