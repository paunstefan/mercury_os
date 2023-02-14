.section .text
.extern main
.globl main
    call main

    mov $5, %rax
    int $0x80

.section .data
.globl errno
errno:	.long 0
