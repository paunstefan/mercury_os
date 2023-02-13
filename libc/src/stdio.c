#include <stdio.h>
#include <syscall.h>
#include <stdint.h>
#include <unistd.h>
#include <string.h>
#include <ctype.h>
#include <stdarg.h>

static FILE _stdin = {0, 0};
static FILE _stdout = {0, 0};
static FILE _stderr = {0, 0};
FILE *stdin = &_stdin;
FILE *stdout = &_stdout;
FILE *stderr = &_stderr;

#if 0
FILE *fopen(char *path, char *mode)
{
    FILE *fp;

    fp = malloc(sizeof(*fp));
    memset(fp, 0, sizeof(*fp));
    fp->fd = open(path);
    if (fp->fd < 0)
    {
        free(fp);
        return NULL;
    }

    return fp;
}

int fclose(FILE *fp)
{
    int ret = close(fp->fd);

    return ret;
}
#endif

int fputc(int c, FILE *fp)
{
    uint64_t ret = write(fp->fd, &c, 1);
    if (ret < 0)
    {
        return -1;
    }
    return c;
}

int putchar(int c)
{
    return fputc(c, stdout);
}

static void ostr(FILE *fp, char *s, int wid, int left)
{
    int fill = wid - strlen(s);
    if (!left)
        while (fill-- > 0)
            fputc(' ', fp);
    while (*s)
        fputc((unsigned char)*s++, fp);
    if (left)
        while (fill-- > 0)
            fputc(' ', fp);
}

static int digits(unsigned long n, int base)
{
    int i;
    for (i = 0; n; i++)
        n /= base;
    return i ? i : 1;
}

static char *digs = "0123456789abcdef";
static char *digs_uc = "0123456789ABCDEF";

#define FMT_LEFT 0001   /* flag '-' */
#define FMT_PLUS 0002   /* flag '+' */
#define FMT_BLANK 0004  /* flag ' ' */
#define FMT_ALT 0010    /* flag '#' */
#define FMT_ZERO 0020   /* flag '0' */
#define FMT_SIGNED 0040 /* is the conversion signed? */
#define FMT_UCASE 0100  /* uppercase hex digits? */

static void oint(FILE *fp, unsigned long n, int base,
                 int wid, int bytes, int flags)
{
    char buf[64];
    char *s = buf;
    int neg = 0;
    int sign = '\0';
    char fill;
    int left = flags & FMT_LEFT;
    int alt_form = flags & FMT_ALT;
    int ucase = flags & FMT_UCASE;
    int prefix_len = 0; /* length of sign or base prefix */
    int d;
    int i;

    if (flags & FMT_SIGNED)
    {
        if ((signed long)n < 0)
        {
            neg = 1;
            sign = '-';
            n = -n;
        }
        else
        {
            if (flags & FMT_PLUS)
                sign = '+';
            else if (flags & FMT_BLANK)
                sign = ' ';
        }
        prefix_len = !!sign;
    }
    else if (n > 0 && alt_form)
    {
        prefix_len = base == 16 ? 2 : 1;
    }
    if (bytes == 1)
        n &= 0x000000ff;
    if (bytes == 2)
        n &= 0x0000ffff;
    if (bytes == 4)
        n &= 0xffffffff;
    d = digits(n, base);
    for (i = 0; i < d; i++)
    {
        s[d - i - 1] = ucase ? digs_uc[n % base] : digs[n % base];
        n /= base;
    }
    s[d] = '\0';
    fill = (flags & FMT_ZERO) ? '0' : ' ';
    i = d + prefix_len;
    if (fill == ' ' && !left)
        while (i++ < wid)
            fputc(' ', fp);
    if (sign)
    {
        fputc(sign, fp);
    }
    else if (prefix_len)
    {
        fputc('0', fp);
        if (base == 16)
            fputc(ucase ? 'X' : 'x', fp);
    }
    if (fill == '0' && !left)
        while (i++ < wid)
            fputc('0', fp);
    ostr(fp, buf, 0, 0);
    if (left)
        while (i++ < wid)
            fputc(' ', fp);
}

static char *fmt_flags = "-+ #0";

int vfprintf(FILE *fp, char *fmt, va_list ap)
{
    char *s = fmt;

    while (*s)
    {
        int c = (unsigned char)*s++;
        int wid = 0;
        int bytes = sizeof(int);
        int flags = 0;
        int left;
        char *f;
        if (c != '%')
        {
            fputc(c, fp);
            continue;
        }
        while ((f = strchr(fmt_flags, *s)))
        {
            flags |= 1 << (f - fmt_flags);
            s++;
        }
        left = flags & FMT_LEFT;
        if (*s == '*')
        {
            wid = va_arg(ap, int);
            if (wid < 0)
            {
                flags |= FMT_LEFT;
                wid = -wid;
            }
            s++;
        }
        else
        {
            while (isdigit(*s))
            {
                wid *= 10;
                wid += *s++ - '0';
            }
        }
        while (*s == 'l')
        {
            bytes = sizeof(long);
            s++;
        }
        while (*s == 'h')
        {
            bytes = bytes < sizeof(int) ? sizeof(char) : sizeof(short);
            s++;
        }
        switch ((c = *s++))
        {
        case 'd':
        case 'i':
            flags |= FMT_SIGNED;
            oint(fp, va_arg(ap, long), 10, wid, bytes, flags);
            break;
        case 'u':
            flags &= ~FMT_ALT;
            oint(fp, va_arg(ap, long), 10, wid, bytes, flags);
            break;
        case 'o':
            oint(fp, va_arg(ap, long), 8, wid, bytes, flags);
            break;
        case 'p':
            flags |= FMT_ALT;
        case 'x':
            oint(fp, va_arg(ap, long), 16, wid, bytes, flags);
            break;
        case 'X':
            flags |= FMT_UCASE;
            oint(fp, va_arg(ap, long), 16, wid, bytes, flags);
            break;
        case 'c':
            if (left)
                fputc(va_arg(ap, int), fp);
            while (wid-- > 1)
                fputc(' ', fp);
            if (!left)
                fputc(va_arg(ap, int), fp);
            break;
        case 's':
            ostr(fp, va_arg(ap, char *), wid, left);
            break;
        case 'n':
            *va_arg(ap, int *) = 0;
            break;
        case '\0':
            s--;
            break;
        default:
            fputc(c, fp);
        }
    }
    return 0;
}

int printf(char *fmt, ...)
{
    va_list ap;
    int ret;
    va_start(ap, fmt);
    ret = vfprintf(stdout, fmt, ap);
    va_end(ap);
    return ret;
}