.section .text
.extern main
.globl main
    call main
loop:
    jmp loop
