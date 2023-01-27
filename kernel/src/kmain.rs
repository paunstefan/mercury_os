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
use core::mem::size_of;
use core::panic::PanicInfo;

use crate::arch::{
    addressing::{translate_virtual_address, VirtAddr},
    paging::PageFrameAllocator,
};

mod drivers;

// TODO: make register reading/writing functions
// TODO: use new VirtAddr and PhysAddr abstractions

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

    {
        log!("multiboot_magic: 0x{:x}", multiboot_magic);

        log!("multiboot_info: 0x{:x}", multiboot_info);
        unsafe {
            let mb_info = multiboot::MultibootInfo::read(multiboot_info);
            //log!("{:?}", mb_info);

            for i in 0..(mb_info.mmap_length / size_of::<multiboot::MmapEntry>() as u32) {
                let mmap_entry = &*((mb_info.mmap_addr as u64 + arch::addressing::KERNEL_BASE)
                    as *const multiboot::MmapEntry)
                    .add(i as usize);

                log!("Entry {}: {:?}", i, mmap_entry);
            }
        }
    }
    let mb_info = unsafe { multiboot::MultibootInfo::read(multiboot_info) };
    unsafe {
        let pfa = PageFrameAllocator::init(mb_info);
        log!("{:#?}", pfa);
    };

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
