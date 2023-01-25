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
mod multiboot;

use core::arch::asm;
use core::intrinsics;
use core::mem::size_of;
use core::panic::PanicInfo;

mod drivers;

// TODO: make register reading/writing functions

#[cfg(target_arch = "x86_64")]
pub const KERNEL_BASE: u64 = 0xFFFFFFFF80000000;

// Symbol from linker script
// Can't be accessed as variable, but can as function pointer
extern "C" {
    fn kernel_end();
}

#[panic_handler]
pub fn panic_implementation(_info: &PanicInfo) -> ! {
    unsafe { intrinsics::abort() }
}

#[no_mangle]
pub extern "C" fn kmain(multiboot_magic: u64, multiboot_info: u64) {
    log!("Hello world! 1={}", 1);

    init_kernel();
    unsafe {
        asm!("int3", options(nomem, nostack));
    }

    {
        log!("multiboot_magic: 0x{:x}", multiboot_magic);

        log!("multiboot_info: 0x{:x}", multiboot_info);
        unsafe {
            let mb_info = &*((multiboot_info + KERNEL_BASE) as *const multiboot::MultibootInfo);
            log!("{:?}", mb_info);

            for i in 0..(mb_info.mmap_length / size_of::<multiboot::MmapEntry>() as u32) {
                let mmap_entry = &*((mb_info.mmap_addr as u64 + KERNEL_BASE)
                    as *const multiboot::MmapEntry)
                    .offset(i as isize);

                log!("Entry {}: {:?}", i, mmap_entry);
            }
            log!("kernel_end: 0x{:x}", kernel_end as u64);
        }
    }

    log!("PML4: {:?}", arch::registers::Cr3::read());

    // trigger a page fault
    // unsafe {
    //     *(0xdeadbeef as *mut u64) = 42;
    // };

    log!("Did not crash (yet)");
    loop {}
}

fn init_kernel() {
    arch::gdt::init_tss();
    log!("Initialized TSS");
    arch::interrupts::init_idt();
    log!("Initialized IDT");
}
