// x86 port IO
#[path = "../x86_generic/io.rs"]
mod x86_io;

#[path = "../x86_generic/serial.rs"]
pub mod x86_serial;

#[path = "../x86_generic/registers.rs"]
pub mod registers;

pub mod interrupts;

pub mod gdt;

pub mod paging;
