ARCH ?= amd64

ifeq ($(ARCH),amd64)
	TRIPLE ?= x86_64-elf-
	UTILS_DIR ?= x86_64_binutils/bin
else
	$(error Unknown architecture $(ARCH))
endif

CC := ../$(UTILS_DIR)/$(TRIPLE)gcc
AR := ../$(UTILS_DIR)/$(TRIPLE)ar


OBJDIR := obj
SRCDIR := src
INCLUDEDIR := include

SOURCES := $(wildcard $(SRCDIR)/*.c)
OBJECTS := $(patsubst $(SRCDIR)/%.c, $(OBJDIR)/%.o, $(SOURCES))

CFLAGS := -I$(INCLUDEDIR) -nostdlib  -nostdinc -nodefaultlibs -fno-builtin

all: libc.a

libc.a: $(OBJECTS)
	$(AR) rcs $@ $^

$(OBJDIR)/%.o : $(SRCDIR)/%.c
	$(CC) -c $(CFLAGS) $< -o $@

clean:
	rm -f  $(OBJECTS) libc.a
