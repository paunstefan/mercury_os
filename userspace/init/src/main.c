#include <stdio.h>
#include <unistd.h>
#include <syscall.h>
void main()
{

    printf("C works!\n");
    int *x = (int *)malloc(sizeof(int));
    *x = 42;
    printf("%d\n", *x);

    printf("Executing \"program.bin\"\n");
    syscall_exec("/program.bin");
    printf("Back in parent\n");
    printf("Executing \"program.bin\" again\n");
    syscall_exec("/program.bin");
    printf("Exiting\n");

    while (1)
    {
    }
}