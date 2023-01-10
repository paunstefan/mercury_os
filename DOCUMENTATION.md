# MercuryOS documentation

MercuryOS is an experimental Rust operating system. The initial implementation is for
the x86-64 architecture.

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

To run the image using QEMU just execute the `run.sh` script.

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

After is is built, the GDT is loaded using the `ldgdt` instruction.

* <https://wiki.osdev.org/Global_Descriptor_Table>

## Interrupts

## Memory management