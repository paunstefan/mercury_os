#ifndef _UNISTD_H
#define _UNISTD_H

#include <stdint.h>

int64_t open(char *path);
int64_t close(int64_t fd);
int64_t write(int64_t fd, void *buf, uint64_t n);
int64_t read(int64_t fd, void *buf, uint64_t n);

/* lseek() whence */
#define SEEK_SET 0
#define SEEK_CUR 1
#define SEEK_END 2

long fseek(int64_t fd, long offset, int whence);

void exit(int status);

int64_t sleep(uint64_t n);

/* standard file descriptors */
#define STDIN_FILENO 0
#define STDOUT_FILENO 0
#define STDERR_FILENO 0

#endif