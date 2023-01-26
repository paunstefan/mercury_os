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

use crate::arch::addressing::{translate_virtual_address, VirtAddr};

mod drivers;

// TODO: make register reading/writing functions
// TODO: use new VirtAddr and PhysAddr abstractions

// Symbol from linker script
// Can't be accessed as variable, but can as function pointer
extern "C" {
    fn kernel_end();
}

#[panic_handler]
pub fn panic_implementation(info: &PanicInfo) -> ! {
    log!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn kmain(multiboot_magic: u64, multiboot_info: u64) {
    log!("Hello world! 1={}", 1);
    init_kernel();
    unsafe {
        asm!("int3", options(nomem, nostack));
    }

    // {
    //     log!("multiboot_magic: 0x{:x}", multiboot_magic);

    //     log!("multiboot_info: 0x{:x}", multiboot_info);
    //     unsafe {
    //         let mb_info = &*((multiboot_info + KERNEL_BASE) as *const multiboot::MultibootInfo);
    //         log!("{:?}", mb_info);

    //         for i in 0..(mb_info.mmap_length / size_of::<multiboot::MmapEntry>() as u32) {
    //             let mmap_entry = &*((mb_info.mmap_addr as u64 + KERNEL_BASE)
    //                 as *const multiboot::MmapEntry)
    //                 .offset(i as isize);

    //             log!("Entry {}: {:?}", i, mmap_entry);
    //         }
    //         log!("kernel_end: 0x{:x}", kernel_end as u64);
    //     }
    // }

    let pml4 = unsafe { arch::paging::active_level_4_table(arch::addressing::KERNEL_BASE) };
    for entry in pml4.iter() {
        if !entry.is_unused() {
            log!("Entry {:?}", entry);
        }
    }

    {
        let x = 42;
        let x_ptr = &x as *const i32;

        let vaddr = VirtAddr::from_ptr(x_ptr);
        log!("{:?}", vaddr);
        log!("{:?}", translate_virtual_address(vaddr));
    }

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
