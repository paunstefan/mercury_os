# Notes

## Rust

Target installation:

```bash
rustup target add x86_64-unknown-none
```

Install nightly toolchain

```bash
rustup update nightly

rustup component add rust-src --toolchain=nightly
```

## Binutils

Download binutils either prebuilt or build them yourself for the needed architecture.

<https://wiki.osdev.org/GCC_Cross-Compiler>

## Boot process

1) Enable PAE in CR4
2) Set CR3 so it points to the PML4
3) Enable IA-32e mode
4) Enable paging
5) Load GDT (for long mode, it will just be flat memory)
6) Long jump to 64bit code


## Running

To run:

```bash
qemu-system-x86_64 -kernel kernel.amd64.bin -serial stdio
```

### Debugging

QEMU has an integrated GDB server that can be used to debug the kernel.

```bash
qemu-system-x86_64 -s -S -kernel kernel.amd64.bin -serial stdio

gdb kernel.amd64.bin.elf64
> target remote localhost:1234
> break kmain
> continue
```

To find the line corresponding to an address use:

```bash
llvm-addr2line -e kernel.amd64.bin.elf64 -i -C -f ffffffff80101b0c
# -i is used for inline functions, tis makes addr2line also output the whole call path
```

## Docker

```bash
podman build -t paunstefan/mercuryos .
podman run --rm -it --entrypoint tmux --name mercury_dev -v  "$(pwd)":/usr/src/mercury_os/ paunstefan/mercuryos
```

## Links

* <https://www.pagetable.com/?p=14>
* <https://forum.osdev.org/viewtopic.php?t=11093>