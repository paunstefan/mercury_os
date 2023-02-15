#!/bin/bash
BINUTILS_DIR="../../x86_64_binutils/bin"

$BINUTILS_DIR/x86_64-elf-as -o program.elf program.S

$BINUTILS_DIR/x86_64-elf-objcopy -O binary program.elf ../initrd/program.bin