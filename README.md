# MercuryOS

MercuryOS is an experimental operating system written in Rust. The initial implementation is for
the x86-64 architecture(AMD64).

## How to build

### Dependencies

* Rust (nightly)
* GNU Binutils (included in the x86_64_binutils directory)
* Make
* QEMU (or other VM for running)

To simplify and make the building and running of the OS more portable, a Dockerfile is inlcuded that contains all the dependencies.

```bash
docker build -t mercuryos/dev .
docker run --rm -it --entrypoint tmux --name mercury_dev -v  "$(pwd)":/usr/src/mercury_os/ mercuryos/dev
```

The Makefile also contains a `docker` command to start the container. (It uses Podman, which is a drop-in replacement for Docker)

### Build process

After installing the dependencies or starting the Docker container, you can run `make` to build the OS binary.
The OS contains the Multiboot header, so it can be booted using any Multiboot compatible bootloader.

3 files will be created in the root of the project: 

* `kernel.amd64.bin.elf64` - The initial 64bit binary, with debugging symbols (can be used with GDB)
* `kernel.amd64.bin` - Bootable OS binary, without debugging symbols.
* `kernel.amd64.bin.dsm` - Dissassembly of the OS binary.

## Running

To run the created binary you can use any Multiboot compatible bootloader. QEMU can directly boot it, too.
The Makefile contains a `run` command that starts QEMU with the following arguments:

```bash
qemu-system-x86_64 -kernel kernel.amd64.bin -serial stdio -display none
```

## Organization

- Kernel code is placed in the `kernel` Rust crate, which contains the following:
  - `src/` - all the kernel code
  - `src/drivers/` - platform agnostic drivers code
  - `src/arch/` - platform specific internals (including target description, linker script and startup code)
  - `rust-toolchain.toml` - Rust toolchain specifier (nightly)
- `x86_64_binutils` - needed GNU binutils for building the kernel
- `Dockerfile` - Docker image configuration for the build environment
- `iso/` - GRUB binary and config to build a bootable ISO
- `DOCUMENTATION.md` - more detailed documentation of the OS internals