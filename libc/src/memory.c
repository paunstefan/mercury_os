#include <stdlib.h>
#include <stdint.h>
#include <stdbool.h>
#include <string.h>
#include <stddef.h>

// Hardcoded heap start from kernel
#define HEAP_START 0x200000
#define PAGE_SIZE 0x200000
#define NO_PAGES 4
typedef struct bump_allocator
{
    uint64_t heap_start;
    uint64_t heap_end;
    uint64_t next;
    uint64_t count;

} bump_allocator;

bump_allocator GLOBAL_ALLOCATOR = (bump_allocator){.heap_start = HEAP_START,
                                                   .heap_end = HEAP_START + PAGE_SIZE * NO_PAGES - 1,
                                                   .next = HEAP_START,
                                                   .count = 0};

void *malloc(long n)
{
    uint64_t alloc_start = GLOBAL_ALLOCATOR.next;
    uint64_t alloc_end = alloc_start + n;

    if (alloc_end > GLOBAL_ALLOCATOR.heap_end)
    {
        return NULL;
    }
    else
    {
        GLOBAL_ALLOCATOR.next = alloc_end;
        GLOBAL_ALLOCATOR.count += 1;
        return (void *)alloc_start;
    }
}
void free(void *m)
{
    GLOBAL_ALLOCATOR.count -= 1;
    if (GLOBAL_ALLOCATOR.count == 0)
    {
        GLOBAL_ALLOCATOR.next = GLOBAL_ALLOCATOR.heap_start;
    }
}
void *calloc(long n, long sz)
{
    void *addr = malloc(n * sz);
    memset(addr, 0, n * sz);
    return addr;
}

// This is not right
void *realloc(void *v, long sz)
{
    void *addr = malloc(sz);
    memcpy(addr, v, sz);
    return addr;
}

static uint64_t align_down(uint64_t address, uint64_t alignment)
{
    return address & ~(alignment - 1);
}

static bool is_aligned(uint64_t address, uint64_t alignment)
{
    return address == align_down(address, alignment);
}

static uint64_t align_up(uint64_t address, uint64_t alignment)
{

    if (is_aligned(address, alignment))
    {
        return address;
    }
    return (address & ~(alignment - 1)) + alignment;
}