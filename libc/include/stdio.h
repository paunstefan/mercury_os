#ifndef _STDIO_H
#define _STDIO_H

#include <stdint.h>
#include <stdarg.h>

#define EOF (-1)
#define putc(c, fp) (fputc(c, fp))
#define getc(fp) (fgetc(fp))

typedef struct
{
    uint64_t fd;
    uint64_t pos;
} FILE;

extern FILE *stdin;
extern FILE *stdout;
extern FILE *stderr;

FILE *fopen(char *path, char *mode);
int fclose(FILE *fp);
int fputc(int c, FILE *fp);
int putchar(int c);

int printf(char *fmt, ...);

int vfprintf(FILE *fp, char *fmt, va_list ap);

#endif