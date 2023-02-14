#ifndef _STRING_H
#define _STRING_H

#if 0
void *memrchr(void *s, int c, long n);

#endif
void *memchr(void *s, int c, long n);

char *strrchr(char *s, int c);
char *strcpy(char *dst, char *src);
void *memmove(void *dst, void *src, long n);
void *memset(void *s, int v, long n);

int memcmp(char *s1, char *s2, long n);
void *memcpy(void *dst, void *src, long n);

char *strchr(char *s, int c);
long strlen(char *s);
int strcmp(char *s1, char *s2);

int strncmp(char *d, char *s, long n);
char *strcat(char *d, char *s);
char *strstr(char *string, char *substring);
char *strncpy(char *d, char *s, long n);

#if 0
char *strdup(const char *s);

#endif

#endif