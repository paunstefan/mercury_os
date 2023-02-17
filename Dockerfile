FROM rust:latest

WORKDIR /usr/src/mercury_os/

RUN rustup target add x86_64-unknown-none

RUN rustup update nightly
RUN rustup component add rust-src --toolchain=nightly

RUN apt-get -qy update
RUN apt install -qy qemu qemu-utils qemu-system-x86 qemu-system-gui
RUN apt install -qy llvm
RUN apt install -qy gdb
RUN apt install -qy tmux
RUN apt install -qy grub2 xorriso
RUN ln -s /usr/bin/grub-mkrescue /usr/bin/grub2-mkrescue