#![feature(panic_handler)]
#![feature(core_intrinsics)]
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
    loop {}
}
