#include <stddef.h>
#include <string.h>
#include <ctype.h>
#include <stdlib.h>

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

int memcmp(char *s1, char *s2, long n)
{
    size_t ofs = 0;
    int c1 = 0;

    while (ofs < n && !(c1 = ((unsigned char *)s1)[ofs] - ((unsigned char *)s2)[ofs]))
    {
        ofs++;
    }
    return c1;
}

void *memcpy(void *dst, void *src, long len)
{
    size_t pos = 0;

    while (pos < len)
    {
        ((char *)dst)[pos] = ((char *)src)[pos];
        pos++;
    }
    return dst;
}

void *memmove(void *dst, void *src, long len)
{
    size_t dir, pos;

    pos = len;
    dir = -1;

    if (dst < src)
    {
        pos = -1;
        dir = 1;
    }

    while (len)
    {
        pos += dir;
        ((char *)dst)[pos] = ((char *)src)[pos];
        len--;
    }
    return dst;
}

void *memset(void *dst, int b, long len)
{
    char *p = dst;

    while (len--)
    {
        *(p++) = b;
    }
    return dst;
}

int strcmp(char *a, char *b)
{
    unsigned int c;
    int diff;

    while (!(diff = (unsigned char)*a++ - (c = (unsigned char)*b++)) && c)
        ;
    return diff;
}

int strcasecmp(const char *s1, const char *s2)
{
    const unsigned char *p1 = (const unsigned char *)s1;
    const unsigned char *p2 = (const unsigned char *)s2;
    int result;
    if (p1 == p2)
        return 0;
    while ((result = tolower(*p1) - tolower(*p2++)) == 0)
        if (*p1++ == '\0')
            break;
    return result;
}
int strncasecmp(const char *s1, const char *s2, long n)
{
    const unsigned char *p1 = (const unsigned char *)s1;
    const unsigned char *p2 = (const unsigned char *)s2;
    int result;
    if (p1 == p2)
        return 0;
    while (n-- && (result = tolower(*p1) - tolower(*p2++)) == 0)
        if (*p1++ == '\0')
            break;
    return result;
}

char *strcpy(char *dst, char *src)
{
    char *ret = dst;

    while ((*dst++ = *src++))
        ;
    return ret;
}
#if 0
char *strdup(char *str)
{
    size_t len;
    char *ret;

    len = strlen(str);
    ret = malloc(len + 1);
    if (__builtin_expect(ret != NULL, 1))
        memcpy(ret, str, len + 1);

    return ret;
}
#endif
int strncmp(char *a, char *b, long size)
{
    unsigned int c;
    int diff = 0;

    while (size-- &&
           !(diff = (unsigned char)*a++ - (c = (unsigned char)*b++)) && c)
        ;

    return diff;
}

char *strrchr(char *s, int c)
{
    char *ret = NULL;

    while (*s)
    {
        if (*s == (char)c)
            ret = s;
        s++;
    }
    return (char *)ret;
}

char *strcat(char *d, char *s)
{
    strcpy(d + strlen(d), s);
    return d;
}

char *strstr(char *s, char *r)
{
    int len = strlen(r);
    if (!len)
        return s;
    while (s)
    {
        if (!memcmp(s, r, len))
            return s;
        s = strchr(s + 1, *r);
    }
    return NULL;
}

void *memchr(void *s, int c, long n)
{
    char *big = (char *)s;
    size_t i;
    for (i = 0; i < n; i++)
        if (big[i] == c)
            return (void *)&big[i];
    return NULL;
}

char *strncpy(char *d, char *s, long n)
{
    int len = strlen(s);
    if (len > n)
        len = n;
    memcpy(d, s, len);
    memset(d + len, 0, n - len);
    return d;
}

char *strdup(char *s)
{
    char *dup = (char *)malloc(strlen(s) + 1);
    strcpy(dup, s);
    return dup;
}