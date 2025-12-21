bits 64
default rel

segment .text
global mainCRTStartup

mainCRTStartup:
    mov dword [x], 0
    mov eax, 0
    mov dword [i], eax
loop_begin_i:
    mov eax, dword [i]
    mov ebx, 10
    cmp eax, ebx
    jg loop_end_i
    mov eax, dword [x]
    push rax
    mov ebx, dword [i]
    pop rax
    add eax, ebx
    mov dword [x], eax
    mov eax, dword [i]
    inc eax
    mov dword [i], eax
    jmp loop_begin_i
loop_end_i:
    mov dword [y], 0
    mov dword [z], 1078530000
    mov eax, dword [x]
    push rax
    mov ebx, 10
    pop rax
    add eax, ebx
    mov eax, eax
    push rax
    mov ebx, 5
    pop rax
    imul eax, ebx
    mov eax, eax
    push rax
    mov ebx, 2
    pop rax
    cdq
    idiv ebx
    mov dword [y], eax
    mov dword [c], 97
    mov dword [d], 98
    mov eax, dword [c]
    push rax
    mov ebx, dword [d]
    pop rax
    cmp eax, ebx
    setl al
    movzx eax, al
    mov eax, eax
    cmp eax, 0
    je else_0
    mov eax, 1
    jmp endif_0
else_0:
    mov eax, dword [c]
    push rax
    mov ebx, dword [d]
    pop rax
    cmp eax, ebx
    sete al
    movzx eax, al
    mov eax, eax
    cmp eax, 0
    je else_1
    mov eax, 2
    jmp endif_1
else_1:
    mov eax, 0
endif_1:
endif_0:
    ret

segment .bss
c resd 1
i resd 1
y resd 1
x resd 1
z resd 1
d resd 1
