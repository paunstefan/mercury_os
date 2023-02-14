#ifndef _SYSCALL_H
#define _SYSCALL_H

#include "stdint.h"

#define DECL_SYSCALL0(fn) int64_t syscall_##fn();
#define DECL_SYSCALL1(fn, p1) int64_t syscall_##fn(p1);
#define DECL_SYSCALL2(fn, p1, p2) int64_t syscall_##fn(p1, p2);
#define DECL_SYSCALL3(fn, p1, p2, p3) int64_t syscall_##fn(p1, p2, p3);
#define DECL_SYSCALL4(fn, p1, p2, p3, p4) int64_t syscall_##fn(p1, p2, p3, p4);

DECL_SYSCALL3(read, uint64_t, uint64_t, const uint8_t *)
DECL_SYSCALL3(write, uint64_t, uint64_t, const uint8_t *)
DECL_SYSCALL1(open, const char *)
DECL_SYSCALL1(close, uint64_t)
DECL_SYSCALL1(sleep, uint64_t)
DECL_SYSCALL0(exit)
DECL_SYSCALL0(getpid)
DECL_SYSCALL0(uptime)
DECL_SYSCALL1(exec, const char *)
DECL_SYSCALL1(blit, uint64_t)
DECL_SYSCALL3(fseek, uint64_t, uint64_t, uint64_t)

#define DEFN_SYSCALL0(fn, num)                         \
    int64_t syscall_##fn()                             \
    {                                                  \
        int64_t _ret;                                  \
        register uint64_t _num __asm__("rax") = (num); \
                                                       \
        __asm__ volatile(                              \
            "int $0x80\n"                              \
            : "=a"(_ret)                               \
            : "0"(_num)                                \
            : "r10", "r11", "memory", "cc");           \
        _ret;                                          \
    }

#define DEFN_SYSCALL1(fn, num, P1)                               \
    int64_t syscall_##fn(P1 p1)                                  \
    {                                                            \
        int64_t _ret;                                            \
        register uint64_t _num __asm__("rax") = (num);           \
        register uint64_t _arg1 __asm__("rdi") = (uint64_t)(p1); \
                                                                 \
        __asm__ volatile(                                        \
            "int $0x80\n"                                        \
            : "=a"(_ret)                                         \
            : "r"(_arg1),                                        \
              "0"(_num)                                          \
            : "r10", "r11", "memory", "cc");                     \
        _ret;                                                    \
    }

#define DEFN_SYSCALL2(fn, num, P1, P2)                           \
    int64_t syscall_##fn(P1 p1, P2 p2)                           \
    {                                                            \
        int64_t _ret;                                            \
        register uint64_t _num __asm__("rax") = (num);           \
        register uint64_t _arg1 __asm__("rdi") = (uint64_t)(p1); \
        register uint64_t _arg2 __asm__("rsi") = (uint64_t)(p2); \
                                                                 \
        __asm__ volatile(                                        \
            "int $0x80\n"                                        \
            : "=a"(_ret)                                         \
            : "r"(_arg1), "r"(_arg2),                            \
              "0"(_num)                                          \
            : "r10", "r11", "memory", "cc");                     \
        _ret;                                                    \
    }

#define DEFN_SYSCALL3(fn, num, P1, P2, P3)                       \
    int64_t syscall_##fn(P1 p1, P2 p2, P3 p3)                    \
    {                                                            \
        int64_t _ret;                                            \
        register uint64_t _num __asm__("rax") = (num);           \
        register uint64_t _arg1 __asm__("rdi") = (uint64_t)(p1); \
        register uint64_t _arg2 __asm__("rsi") = (uint64_t)(p2); \
        register uint64_t _arg3 __asm__("rdx") = (uint64_t)(p3); \
                                                                 \
        __asm__ volatile(                                        \
            "int $0x80\n"                                        \
            : "=a"(_ret)                                         \
            : "r"(_arg1), "r"(_arg2), "r"(_arg3),                \
              "0"(_num)                                          \
            : "r10", "r11", "memory", "cc");                     \
        _ret;                                                    \
    }

#define DEFN_SYSCALL4(fn, num, P1, P2, P3, P4)                   \
    int64_t syscall_##fn(P1 p1, P2 p2, P3 p3, P4 p4)             \
    {                                                            \
        int64_t _ret;                                            \
        register uint64_t _num __asm__("rax") = (num);           \
        register uint64_t _arg1 __asm__("rdi") = (uint64_t)(p1); \
        register uint64_t _arg2 __asm__("rsi") = (uint64_t)(p2); \
        register uint64_t _arg3 __asm__("rdx") = (uint64_t)(p3); \
        register uint64_t _arg4 __asm__("rcx") = (uint64_t)(p4); \
                                                                 \
        __asm__ volatile(                                        \
            "int $0x80\n"                                        \
            : "=a"(_ret)                                         \
            : "r"(_arg1), "r"(_arg2), "r"(_arg3), "r"(_arg4),    \
              "0"(_num)                                          \
            : "r10", "r11", "memory", "cc");                     \
        _ret;                                                    \
    }

#endif