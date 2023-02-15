#include <stdio.h>
#include <unistd.h>
#include <syscall.h>
void main()
{

    printf("C works!\n");
    int *x = (int *)malloc(sizeof(int));
    *x = 42;
    printf("%d\n", *x);

    syscall_exec("/program.bin");
}