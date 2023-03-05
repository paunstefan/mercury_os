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
5) Load GDT (More on GDT later)
6) Long jump to 64bit code
Using the 64bit kernel code segment in the GDT, jump to some 64bit code
7) Set the segment registers
The segment registers need to be set to the kernel data segment (index 0x10 in this case)
8) Initialize the stack
Leave some space somewhere and point the RSP register to the end of it.
9) Finally, call the start function in the high level code.

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

Interrupt handlers differ from simple functions because they use a different calling convention, that is 
saving all registers as opposed to just some of them.

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

* Frame allocator
  
It reserves physical memory in RAM using a bitmap allocator. Each page is represented as a bit that is 0 if unused and 1 if used.
This bitmap is placed 8 bytes after KERNEL_END. When a request for memory is made, it searches for the first free page and returns
its address to the caller. On free, the bit is just set to 0.

The frame allocator is a global structure shared by everyone.

* Page allocator

It allocates virtual memory that is mapped to a physical page previously allocated by the frame allocator. When a process
requests memory, the page allocator finds space in a page table and adds a mapping to a physical address.

One page allocator should exist per process, as each process has its own page tables.

* Heap allocator

The heap allocator is the one that does what `malloc` and `free` usually do. It receives requests for a certain amount of memory
and returns an address where that memory is reserved and mapped.

Memory management is at the moment the most duct tape-y part of the OS and is in need of a rework.

* <https://wiki.osdev.org/Memory_management>
## Filesystem

For reading and writing files, at the moment MercuryOS uses a simple RAMDisk filesystem. It is a single file with a header
containing the number of files, each file's name and location, followed by the file's contents. It is loaded into memory
as a GRUB module. 

* <https://wiki.osdev.org/File_Systems>

## Processes

Running different programs is the main reason to use an OS, so processes/tasks are probably the most important part.
MercuryOS for the moment uses a monotasking system, where each program must run to completion before execution
returns to the parent process(similar to old DOS systems).

The OS keeps a stack of tasks and runs the process at the top until completion (or until that process spawns a new process).
Each task holds its own page allocator as pages are created per process and a list of open file descriptors (the serial port
used as stdin and stdout is opened by default by the OS for all processes). 

The kernel has to start the initial process (called `init` in this case). A process is started in the following way
(first 3 steps are skipped for `init`):

* The RSP and RBP registers are saved
* The RIP is saved using a small assembly function
* The 3 registers are put into the current running task's structure
* A new page allocator is created, together with the needed pages
* Memory is allocated for the program text, heap and stack
* The CR3 register it written with the new PML4
* Program text is read from disk and written in its address space (at address 0)
* The RSP, RBP are set to the new task's values
* A `JMP` is executed to this process' address 0

Task exiting (returning to the parent task) is done as following:
* Current task is popped from the tasks stack
* Its allocated memory is freed
* CR3 is switched to the old process' PML4
* RSP, RBP and RIP are set according to the values saved when switched to the child task

Because switching tasks happens during an interrupt (see next section), all othe registers are saved
on the process' stack and restored by the interrupt handler when execution restarts.

The way processes work now in MercuryOS is pretty badly done, a scheduler based preemptive multitasking system
should be implemented in the future.

* <https://wiki.osdev.org/Processes_and_Threads>

## System calls

To be able to do useful stuff (such as interacting with the hardware), user programs need to be able to
use functionality reserved for the kernel. This is done using system calls.

A system call is nothing more than a software interrupt (or a fancy one when using the `syscall` instruction).
A different interrupt handler is used, as more control over the saved registers and the stack was needed so the
`syscall_asm` handler was created, that saves all registers on the stack and preserves the RAX register to return to
the caller.

When a process wants to execute a system call, it interrupts the CPU using the `INT 0x80` instruction, with the 
interrupt number in the RAX register and the rest of the arguments in the following registers(in order): 
RDI, RSI, RDX, RCX, R8, R9.

The syscalls supported as of now by MercuryOS are:

* 0 -> read(fd, length, buffer_addr)
* 1 -> write(fd, length, buffer_addr)
* 2 -> open(path_addr)
* 3 -> close(fd)
* 4 -> sleep(ms)
* 5 -> exit()
* 6 -> getpid()
* 7 -> uptime()
* 8 -> exec(path_addr)
* 9 -> blit(address)
* 10 -> fseek(fd, offset, whence)

* <https://wiki.osdev.org/System_Calls>