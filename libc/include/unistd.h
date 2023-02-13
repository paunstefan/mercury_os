#ifndef _UNISTD_H
#define _UNISTD_H

#include <stdint.h>

int64_t open(char *path);
int64_t close(uint64_t fd);
int64_t write(uint64_t fd, void *buf, uint64_t n);
int64_t read(uint64_t fd, void *buf, uint64_t n);

/* lseek() whence */
#define SEEK_SET 0
#define SEEK_CUR 1
#define SEEK_END 2

long lseek(uint64_t fd, long offset, int whence);

void _exit(int status);

int64_t sleep(uint64_t n);

/* standard file descriptors */
#define STDIN_FILENO 0
#define STDOUT_FILENO 1
#define STDERR_FILENO 2

#endif