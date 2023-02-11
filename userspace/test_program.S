mov $0x3f8, %dx

mov $'U, %al
outb %al, %dx

mov $'s, %al
outb %al, %dx

mov $'e, %al
outb %al, %dx

mov $'r, %al
outb %al, %dx

mov $22, %rdi
mov $42, %rsi
int $0x80

loop:
    jmp loop
