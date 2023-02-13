ARCH ?= amd64

ifeq ($(ARCH),amd64)
	TRIPLE ?= x86_64-elf-
	UTILS_DIR ?= x86_64_binutils/bin
else
	$(error Unknown architecture $(ARCH))
endif

CC := ../$(UTILS_DIR)/$(TRIPLE)gcc
LD := ../$(UTILS_DIR)/$(TRIPLE)ld
AS := ../$(UTILS_DIR)/$(TRIPLE)as

OBJDIR := init/obj
SRCDIR := init/src

SOURCES := $(wildcard $(SRCDIR)/*.c)
OBJECTS := $(patsubst $(SRCDIR)/%.c, $(OBJDIR)/%.o, $(SOURCES))

LIBC := ../libc
INCLUDEDIR := ../libc/include

CFLAGS := -I$(INCLUDEDIR) -nostdlib -nostdinc -fno-builtin -fno-stack-protector -nostartfiles -nodefaultlibs
LDFLAGS :=  -T link.ld -L$(LIBC) -l:libc.a

all: initrd/init

initrd/init: $(OBJECTS) start.o
	$(LD) -o initrd/init $(OBJECTS) start.o $(LDFLAGS)

$(OBJDIR)/%.o : $(SRCDIR)/%.c
	$(CC) -c $(CFLAGS) $< -o $@


start.o: start.S
	$(AS) -o $@ $<

clean:
	rm -f initrd/init $(OBJECTS) start.o