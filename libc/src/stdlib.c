#include <stdlib.h>
#include <limits.h>
#include <errno.h>
#include <ctype.h>

static int digit(char c, int base)
{
    int d;
    if (c <= '9')
    {
        d = c - '0';
    }
    else if (c <= 'Z')
    {
        d = 10 + c - 'A';
    }
    else
    {
        d = 10 + c - 'a';
    }
    return d < base ? d : -1;
}

int abs(int n)
{
    return n >= 0 ? n : -n;
}

long labs(long n)
{
    return n >= 0 ? n : -n;
}

long atol(const char *s)
{
    unsigned long ret = 0;
    unsigned long d;
    int neg = 0;

    if (*s == '-')
    {
        neg = 1;
        s++;
    }

    while (1)
    {
        d = (*s++) - '0';
        if (d > 9)
            break;
        ret *= 10;
        ret += d;
    }

    return neg ? -ret : ret;
}

int atoi(const char *s)
{
    return atol(s);
}

long strtol(const char *s, char **endptr, int base)
{
    int sgn = 1;
    int overflow = 0;
    long num;
    int dig;
    while (isspace(*s))
        s++;
    if (*s == '-' || *s == '+')
        sgn = ',' - *s++;
    if (base == 0)
    {
        if (*s == '0')
        {
            if (s[1] == 'x' || s[1] == 'X')
                base = 16;
            else
                base = 8;
        }
        else
        {
            base = 10;
        }
    }
    if (base == 16 && *s == '0' && (s[1] == 'x' || s[1] == 'X'))
        s += 2;
    for (num = 0; (dig = digit(*s, base)) >= 0; s++)
    {
        if (num > LONG_MAX / base)
            overflow = 1;
        num *= base;
        if (num > LONG_MAX - dig)
            overflow = 1;
        num += dig;
    }
    if (endptr)
        *endptr = s;
    if (overflow)
    {
        num = sgn > 0 ? LONG_MAX : LONG_MIN;
        errno = ERANGE;
    }
    else
    {
        num *= sgn;
    }
    return num;
}

unsigned long strtoul(const char *s, char **endptr, int base)
{
    int sgn = 1;
    int overflow = 0;
    unsigned long num;
    int dig;
    while (isspace(*s))
        s++;
    if (*s == '-' || *s == '+')
        sgn = ',' - *s++;
    if (base == 0)
    {
        if (*s == '0')
        {
            if (s[1] == 'x' || s[1] == 'X')
                base = 16;
            else
                base = 8;
        }
        else
        {
            base = 10;
        }
    }
    if (base == 16 && *s == '0' && (s[1] == 'x' || s[1] == 'X'))
        s += 2;
    for (num = 0; (dig = digit(*s, base)) >= 0; s++)
    {
        if (num > (unsigned long)ULONG_MAX / base)
            overflow = 1;
        num *= base;
        if (num > (unsigned long)ULONG_MAX - dig)
            overflow = 1;
        num += dig;
    }
    if (endptr)
        *endptr = s;
    if (overflow)
    {
        num = ULONG_MAX;
        errno = ERANGE;
    }
    else
    {
        num *= sgn;
    }
    return num;
}