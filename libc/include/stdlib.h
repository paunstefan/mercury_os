#ifndef _STDLIB_H
#define _STDLIB_H
#if 0
void *malloc(long n);
void free(void *m);
void *calloc(long n, long sz);
void *realloc(void *v, long sz);
#endif
int atoi(const char *s);
long atol(const char *s);

long strtol(const char *s, char **endptr, int base);
unsigned long strtoul(const char *s, char **endptr, int base);

int abs(int n);
long labs(long n);

#if 0
char *getenv(char *name);
void qsort(void *a, int n, int sz, int (*cmp)(void *, void *));
int mkstemp(char *t);
int system(char *cmd);

void srand(unsigned int seed);
int rand(void);
#endif

#endif