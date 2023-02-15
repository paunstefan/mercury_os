# CONFIG: Architecture to build for
ARCH ?= amd64

ifeq ($(ARCH),amd64)
    TRIPLE ?= x86_64-elf-
	UTILS_DIR ?= x86_64_binutils/bin
else
    $(error Unknown architecture $(ARCH))
endif

# Dev container runner
RUNNER ?= podman

# Toolchain commands (can be overridden)
CARGO ?= cargo
RUSTC ?= rustc
LD := $(UTILS_DIR)/$(TRIPLE)ld
AS := $(UTILS_DIR)/$(TRIPLE)as
OBJDUMP := $(UTILS_DIR)/$(TRIPLE)objdump
OBJCOPY := $(UTILS_DIR)/$(TRIPLE)objcopy

# Object directory
OBJDIR := .obj/$(ARCH)/

LINKSCRIPT := kernel/src/arch/$(ARCH)/link.ld
TARGETSPEC := src/arch/$(ARCH)/target.json
# Compiler Options
LINKFLAGS := -T $(LINKSCRIPT)
LINKFLAGS += -Map $(OBJDIR)map.txt
LINKFLAGS += --gc-sections
LINKFLAGS += -z max-page-size=0x1000

RUSTFLAGS := --cfg arch__$(ARCH) -C soft-float
RUSTFLAGS += -C panic=abort
CARGOFLAGS := -Z build-std=core,alloc,compiler_builtins -Z build-std-features=compiler-builtins-mem

# Objects
OBJS := start.o kernel.a
OBJS := $(OBJS:%=$(OBJDIR)%)
BIN := ./kernel.$(ARCH).bin

RUSTFLAGS += -g
AS_DEBUG := -g


.PHONY: all clean PHONY

all: $(BIN)

# Final link command
$(BIN): $(OBJS) kernel/src/arch/$(ARCH)/link.ld
	$(LD) -o $@ $(LINKFLAGS) $(OBJS)
	$(OBJDUMP) -S $@ > $@.dsm
ifeq ($(ARCH),amd64)
	@mv $@ $@.elf64
	@$(OBJCOPY) $@.elf64 -F elf32-i386 $@
	@$(OBJCOPY) --strip-debug $@
	@cp $@ iso/boot/$@
endif


# Compile rust kernel object
$(OBJDIR)kernel.a: PHONY Makefile
	@mkdir -p $(dir $@)
	cd kernel; RUSTFLAGS="$(RUSTFLAGS)" $(CARGO) build $(CARGOFLAGS) --target=$(TARGETSPEC) --release
	@cp kernel/target/target/release/libkernel.a $@

# Compile architecture's assembly stub
$(OBJDIR)start.o: kernel/src/arch/$(ARCH)/start.S Makefile
	@mkdir -p $(dir $@)
	$(AS) $(ASFLAGS) $(AS_DEBUG) -o $@ $<


INIT := userspace/initrd/init
LIBC := libc/libc.a

iso: $(BIN) userspace/create_initrd.py
	cd libc && $(MAKE)
	cd userspace/init && $(MAKE)
	python3 userspace/create_initrd.py userspace/initrd
	genisoimage -R -b boot/grub/stage2_eltorito -no-emul-boot -boot-load-size 4 -A os -input-charset utf8 -quiet -boot-info-table -o os.iso iso

clean:
	$(RM) -rf $(BIN) $(BIN).elf64 $(BIN).dsm $(OBJDIR)
	$(RM) -rf iso/boot/$(BIN)
	$(RM) -rf iso/modules/*
	$(RM) -f os.iso

cleanall: clean
	cd userspace/init && $(MAKE) clean
	cd libc && $(MAKE) clean

run:
	qemu-system-x86_64 -kernel kernel.amd64.bin -serial stdio -display none

runiso:
	qemu-system-x86_64  -cdrom os.iso -serial stdio

rundebug:
	qemu-system-x86_64 -s -S -kernel kernel.amd64.bin -serial stdio -display none

docker:
	$(RUNNER) run --rm -it --entrypoint tmux --name mercury_dev -v  "$(shell pwd)":/usr/src/mercury_os/ mercuryos/dev

# Include dependency files
-include $(OBJDIR)start.d