../x86_64_binutils/x86_64-elf-as -o test_program.elf test_program.S

../x86_64_binutils/x86_64-elf-objcopy -O binary test_program.elf initrd/test_program.bin