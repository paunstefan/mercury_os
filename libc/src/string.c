#include <string.h>

long strlen(char *s)
{
    long len = 0;

    while (0 != s[len])
    {
        len++;
    }
    return len;
}

char *strchr(char *s, int c)
{
    while (*s)
    {
        if (*s == (char)c)
            return (char *)s;
        s++;
    }
    return 0;
}
