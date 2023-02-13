#include <syscall.h>

DEFN_SYSCALL3(read, 0, uint64_t, uint64_t, const uint8_t *);
DEFN_SYSCALL3(write, 1, uint64_t, uint64_t, const uint8_t *);
DEFN_SYSCALL1(open, 2, uint64_t);
DEFN_SYSCALL1(close, 3, uint64_t);
DEFN_SYSCALL1(sleep, 4, uint64_t);
DEFN_SYSCALL0(exit, 5);
DEFN_SYSCALL0(getpid, 6);
DEFN_SYSCALL0(uptime, 7);
DEFN_SYSCALL1(exec, 8, const char *);
DEFN_SYSCALL1(blit, 9, uint64_t);