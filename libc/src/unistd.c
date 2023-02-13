#include <unistd.h>
#include <stdint.h>
#include <syscall.h>

int64_t open(char *path)
{
    return syscall_open(path);
}

int64_t close(uint64_t fd)
{
    return syscall_close(fd);
}
int64_t write(uint64_t fd, void *buf, uint64_t n)
{
    return syscall_write(fd, n, buf);
}
int64_t read(uint64_t fd, void *buf, uint64_t n)
{
    return syscall_read(fd, n, buf);
}

void exit(int status)
{
    syscall_exit();
}

int64_t sleep(uint64_t n)
{
    return syscall_sleep(n * 1000);
}