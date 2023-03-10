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
int strcasecmp(const char *s1, const char *s2);
int strncasecmp(const char *s1, const char *s2, long n);

int strncmp(char *d, char *s, long n);
char *strcat(char *d, char *s);
char *strstr(char *string, char *substring);
char *strncpy(char *d, char *s, long n);
char *strdup(char *s);

#endif