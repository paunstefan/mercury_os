# MercuryOS documentation

MercuryOS is an experimental Rust operating system. The initial implementation is for
the x86-64 architecture (AMD64 will be used interchangebly as the name).

## Development environment

The OS is written in Rust, with some isolated assembly parts (including the initialization).

Dependencies:
* Rust (nightly)
* gnu-as
* gnu-objdump
* gnu-objcopy
* gnu-ld
* qemu

### Rust config

The Rust nightly channel is needed for some experimental features (such as building the std-core crate),
also the raw x86-64 target.

```bash
rustup target add x86_64-unknown-none
rustup update nightly

cargo install cargo-xbuild
rustup component add rust-src --toolchain=nightly
```

A new target needs to be defined for the OS, and can be found in `kernel/src/arch/*/target.json`.

### Assembly and linking

The OS start file is written in assembly and assembled with `GNU as` and the final image will be linked using 
`GNU ld`.

They will need to be downloaded or built by yourself for the specific target. More information here: <https://wiki.osdev.org/GCC_Cross-Compiler>.

### Running

The Makefile in the root of the project is used to assemble the `start.S` file, the Rust kernel library as a static library and 
finally link everything together.

To run the image using QEMU just run `make run` or `make rundebug` if you want to start with GDB.

## Boot process

This is where the fun starts. The `kernel/src/arch/amd64/start.S` file contains the Multiboot header and initialization code.
Multiboot is a standard used by GRUB and other bootloaders to recognize and start an OS, QEMU also can natively execute Multiboot kernels.

Thanks to the Multiboot header, the kernel will be directly started in 32bit protected mode so it saves some work. What we still have to do is
crate the inital page tables, load the GDT and switch to 64bit long mode.

Switching to Long mode involves the following things:

1) Enable PAE in CR4
Physical Address Extension is a flag that allows the CPU to use addresses longer than 32bits. It is the 5th bit in CR4.
2) Set CR3 so it points to the PML4
PML4 is the 4th page table (Long mode 4 levels of page tables). So it is needed to create some initial page tables and set CR3 to
their address. (More on paging later)
3) Enable Long mode in the MSR register (bit 8)
4) Enable paging
To enable paging you need to set bit 31 in the CR0 register.
1) Load GDT (More on GDT later)
2) Long jump to 64bit code
Using the 64bit kernel code segment in the GDT, jump to some 64bit code
1) Set the segment registers
The segment registers need to be set to the kernel data segment (index 0x10 in this case)
1) Initialize the stack
Leave some space somewhere and point the RSP register to the end of it.
1) Finally, call the start function in the high level code.

* <https://wiki.osdev.org/X86-64>

## Global Descriptor Table

The GDT is a data structure used by x86 processors to define memory segments. They were used for memory management and protection before paging.
Segments were used since the 8086 processor to allow addressing using more than 16 bits, by using SEGMENT:OFFSET as the physical address.

The GDT was used for segmentation on 32bit processors, but on 64bit ones, because the address space is big enough, to simplify things, 
a flat memory model is used. That means all segments encompass the whole address space, they are only used to separate kernel space from
user space.

As the name implies, the GDT is a table of 64bit entries, each entry contains the base address, limit address (both are ignored on 64bit CPUs), and
some flags.

After is is built, the GDT is loaded using the `lgdt` instruction, which expects a pointer structure made out of the size
of the table and the starting address.

In addition to code and data segments, the GDT also contains system segments, more precisely the Task State Segment.
On AMD64 CPUs, this structure holds stack pointers for additional stacks that will be used when changing privilege levels and
when executing an interrupt handler.

* <https://wiki.osdev.org/Global_Descriptor_Table>

## Interrupts

Interrupts are a way to tell the CPU to stop executing the current code and run an Interrupt Service Routine.
They can be caused by an exception (an error that the CPU detected when executing an instruction), 
an external device or by the `INT` instruction.

Interrupts are organized in the Interrupt Descriptor Table, which is similar to the GDT, but holds the addresses
of the interrupt handlers (ISR). The first 32 entries in the IDT are reserved for exceptions and the rest until 256
can be configured by the OS. The IDT is loaded by the CPU using the `lidt` instruction, which expects a pointer
structure like the one for the GDT.

Interrupt handlers differ from simple functions because they use a different calling convention

* <https://wiki.osdev.org/Interrupt>

## Memory management

MercuryOS uses paging for memory management. It uses 2MB pages, that means there is a 3 level page table.
Each page table contains 512 64bit entries, each one pointing to the physical address of the next table,
or, in case of the last table, the start address of the physical memory frame.

Physical addresses on x86_64 are 52bit, while virtual addresses are 48bit.

Page entries are structured as following:

```
64       52 51                12 11            0
+----------+--------------------+--------------+
| Reserved |  Physical address  |    Flags     |
+----------+--------------------+--------------+
```

Because the first 12 bits are flags, each page table must be aligned to 4096 bytes.

To signal that the pages are 2MB in size and not the standard 4KB, the entries in the Page Directory (P2)
have the 7th bit(PS) set.

When the CPU gets to an instruction that involves memory access, it uses the virtual address to index
into the page tables, after the last page table it uses the last 21 bits to offset into the frame.

A virtual address is split as following:

```
47       39 38      30 29      21 20            0
+----------+----------+----------+--------------+
| P4 index | P3 index | P2 index | Frame offset |
+----------+----------+----------+--------------+
```

The memory management system has 3 parts:

- Frame allocator
  
It reserves physical memory in RAM using a bitmap allocator. Each page is represented as a bit that is 0 if unused and 1 if used.
This bitmap is placed 8 bytes after KERNEL_END. When a request for memory is made, it searches for the first free page and returns
its address to the caller. On free, the bit is just set to 0.

The frame allocator is a global structure shared by everyone.

- Page allocator

It allocates virtual memory that is mapped to a physical page previously allocated by the frame allocator. When a process
requests memory, the page allocator finds space in a page table and adds a mapping to a physical address.

One page allocator should exist per process, as each process has its own page tables.

- Heap allocator

The heap allocator is the one that does what `malloc` and `free` usually do. It receives requests for a certain amount of memory
and returns an address where that memory is reserved and mapped.