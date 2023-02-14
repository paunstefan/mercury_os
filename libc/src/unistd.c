#include <unistd.h>
#include <stdint.h>
#include <syscall.h>

int64_t open(char *path)
{
    return syscall_open(path);
}

int64_t close(int64_t fd)
{
    if (fd < 0)
    {
        return -1;
    }
    return syscall_close(fd);
}

int64_t write(int64_t fd, void *buf, uint64_t n)
{
    if (fd < 0)
    {
        return -1;
    }
    return syscall_write(fd, n, buf);
}

int64_t read(int64_t fd, void *buf, uint64_t n)
{
    if (fd < 0)
    {
        return -1;
    }
    return syscall_read(fd, n, buf);
}

void exit(int status)
{
    syscall_exit();
}

int64_t sleep(uint64_t n)
{
    return syscall_sleep(n);
}

long fseek(int64_t fd, long offset, int whence)
{
    if (fd < 0)
    {
        return -1;
    }
    return syscall_fseek(fd, offset, whence);
}