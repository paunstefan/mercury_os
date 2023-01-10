#![feature(panic_handler)]
#![feature(core_intrinsics)]
#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

/// Macros, need to be loaded before everything else due to how rust parses
#[macro_use]
mod macros;

// Achitecture-specific modules
#[cfg(target_arch = "x86_64")]
#[path = "arch/amd64/mod.rs"]
pub mod arch;

mod logging;

use core::arch::asm;
use core::intrinsics;
use core::panic::PanicInfo;

mod drivers;

#[panic_handler]
pub fn panic_implementation(_info: &::core::panic::PanicInfo) -> ! {
    unsafe { intrinsics::abort() }
}

#[no_mangle]
pub fn kmain() {
    log!("Hello world! 1={}", 1);

    arch::interrupts::init_idt();
    unsafe {
        asm!("int3", options(nomem, nostack));
    }

    log!("Did not crash (yet)");
    loop {}
}
