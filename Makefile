# CONFIG: Architecture to build for
ARCH ?= amd64

ifeq ($(ARCH),amd64)
    TRIPLE ?= x86_64-elf-
else ifeq ($(ARCH),x86)
    TRIPLE ?= i686-elf-
else
    $(error Unknown architecture $(ARCH))
endif


# Toolchain commands (can be overridden)
CARGO ?= cargo
RUSTC ?= rustc
LD := x86_64_binutils/$(TRIPLE)ld
AS := x86_64_binutils/$(TRIPLE)as
OBJDUMP := x86_64_binutils/$(TRIPLE)objdump
OBJCOPY := x86_64_binutils/$(TRIPLE)objcopy

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

# Objects
OBJS := start.o kernel.a
OBJS := $(OBJS:%=$(OBJDIR)%)
BIN := ./kernel.$(ARCH).bin

# ifdef DEBUG
	RUSTFLAGS += -g
	AS_DEBUG := -g
# endif

.PHONY: all clean PHONY

all: $(BIN)

clean:
	$(RM) -rf $(BIN) $(BIN).dsm $(OBJDIR)

run:
	qemu-system-x86_64 -kernel kernel.amd64.bin -serial stdio -display none

rundebug:
	qemu-system-x86_64 -s -S -kernel kernel.amd64.bin -serial stdio -display none

# Final link command
$(BIN): $(OBJS) kernel/src/arch/$(ARCH)/link.ld
	$(LD) -o $@ $(LINKFLAGS) $(OBJS)
	$(OBJDUMP) -S $@ > $@.dsm
ifeq ($(ARCH),amd64)
	@mv $@ $@.elf64
	@$(OBJCOPY) $@.elf64 -F elf32-i386 $@
	@$(OBJCOPY) --strip-debug $@
endif


# Compile rust kernel object
$(OBJDIR)kernel.a: PHONY Makefile
	@mkdir -p $(dir $@)
	cd kernel; RUSTFLAGS="$(RUSTFLAGS)" $(CARGO) build -Z build-std=core --target=$(TARGETSPEC) --release
	@cp kernel/target/target/release/libkernel.a $@

# Compile architecture's assembly stub
$(OBJDIR)start.o: kernel/src/arch/$(ARCH)/start.S Makefile
	@mkdir -p $(dir $@)
	$(AS) $(ASFLAGS) $(AS_DEBUG) -o $@ $<


# Include dependency files
-include $(OBJDIR)start.d