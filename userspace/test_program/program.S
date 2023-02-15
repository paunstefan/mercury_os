mov $0x3f8, %dx

mov $'H, %al
outb %al, %dx

mov $'E, %al
outb %al, %dx

mov $'L, %al
outb %al, %dx

mov $'L, %al
outb %al, %dx

mov $'O, %al
outb %al, %dx

mov $'\n, %al
outb %al, %dx

mov $5, %rax
int $0x80

mov $'X, %al
outb %al, %dx

loop:
    jmp loop
