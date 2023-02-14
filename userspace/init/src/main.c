#include <stdio.h>
#include <unistd.h>
void main()
{

    printf("Hello!\n");

    char str1[20], str2[30];

    printf("Enter name: ");
    scanf("%s", str1);
    putchar(str1[0]);
    printf("\nEnter your website name: ");
    scanf("%s", str2);

    printf("\nEntered Name: %s\n", str1);
    printf("Entered Website:%s", str2);
}