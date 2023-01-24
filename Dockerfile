FROM rust:latest


WORKDIR /usr/src/mercury_os/

RUN rustup target add x86_64-unknown-none

RUN rustup update nightly
RUN rustup component add rust-src --toolchain=nightly

RUN apt-get -qy update
RUN apt install -qy qemu qemu-utils qemu-system-x86 qemu-system-gui genisoimage
RUN apt install -qy llvm
RUN apt install -qy gdb
RUN apt install -qy tmux