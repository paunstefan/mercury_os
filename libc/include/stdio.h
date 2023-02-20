#ifndef _STDIO_H
#define _STDIO_H

#include <stdint.h>
#include <stdarg.h>

#define EOF (-1)
#define putc(c, fp) (fputc(c, fp))
#define getc(fp) (fgetc(fp))

typedef struct
{
    int64_t fd;
    int back;          /* pushback buffer */
    char *ibuf, *obuf; /* input/output buffer */
    int isize, osize;  /* ibuf size */
    int ilen, olen;    /* length of data in buf */
    int icur;          /* position in ibuf */
} FILE;

extern FILE *stdin;
extern FILE *stdout;
extern FILE *stderr;

FILE *fopen(char *path, char *mode);
int fclose(FILE *fp);
int fputc(int c, FILE *fp);
int putchar(int c);

int printf(char *fmt, ...);
int fprintf(FILE *fp, char *fmt, ...);
int vfprintf(FILE *fp, char *fmt, va_list ap);
int snprintf(char *dst, int sz, char *fmt, ...);
int fputs(char *s, FILE *fp);
int puts(char *s);
void perror(char *s);

long fwrite(void *s, long sz, long n, FILE *fp);

int fgetc(FILE *fp);
int getchar(void);
int scanf(char *fmt, ...);
int vsscanf(char *s, char *fmt, va_list ap);
int sscanf(char *s, char *fmt, ...);
int fscanf(FILE *fp, char *fmt, ...);
int vfscanf(FILE *fp, char *fmt, va_list ap);

long fread(void *s, long sz, long n, FILE *fp);

long ftell(FILE *stream);
#endif