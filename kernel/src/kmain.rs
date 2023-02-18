#![feature(core_intrinsics)]
#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

/// Macros, need to be loaded before everything else due to how rust parses
#[macro_use]
mod macros;

extern crate alloc;

// Achitecture-specific modules
#[cfg(target_arch = "x86_64")]
#[path = "arch/amd64/mod.rs"]
pub mod arch;

mod drivers;
mod filesystem;
mod logging;
mod mm;
mod multiboot;
mod sync;
mod syscall;
mod task;
mod utils;

use core::mem::size_of;
use core::panic::PanicInfo;

use multiboot::MultibootInfo;

use crate::{
    drivers::framebuffer::{Framebuffer, FRAMEBUFFER},
    task::{Multiprocessing, MULTIPROCESSING},
};

// TODO IMPORTANT: check unsafe usage

#[panic_handler]
pub fn panic_implementation(info: &PanicInfo) -> ! {
    log!("{}", info);
    loop {}
}

pub fn hlt_loop() -> ! {
    loop {
        arch::interrupts::hlt();
    }
}

#[no_mangle]
pub extern "C" fn kmain(multiboot_magic: u64, multiboot_info: u64) {
    // Needed stuff
    let mb_info = unsafe { multiboot::MultibootInfo::read(multiboot_info) };
    log!("{:#?}", mb_info);

    unsafe {
        init_kernel(mb_info);
    }

    let fb_addr = mm::ALLOCATOR
        .lock()
        .page_allocator
        .as_mut()
        .unwrap()
        .alloc_framebuffer(arch::addressing::PhysAddr::new(mb_info.framebuffer.addr));

    Framebuffer::init(
        fb_addr.start_address,
        mb_info.framebuffer.width as usize,
        mb_info.framebuffer.height as usize,
        mb_info.framebuffer.bpp,
    );

    // End needed stuff

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

    let fs_root = unsafe { &**filesystem::FS_ROOT.as_mut().unwrap() };
    for f in fs_root.readdir().unwrap() {
        log!("{:?}", f);
    }

    unsafe {
        FRAMEBUFFER
            .as_mut()
            .unwrap()
            .fill(drivers::framebuffer::Rgb::new(0, 0, 255));
    }

    log!(":)\n\n");

    unsafe {
        MULTIPROCESSING = Some(Multiprocessing::new());
        MULTIPROCESSING.as_mut().unwrap().init("/init");
    }

    hlt_loop()
}

unsafe fn init_kernel(multiboot: &'static MultibootInfo) {
    arch::gdt::init_tss();
    log!("Initialized TSS");
    arch::interrupts::init_idt();
    log!("Initialized IDT");
    arch::paging::init_pfa(multiboot);
    log!("Initialized PageFrameAllocator");
    arch::pic::PICS.lock().initialize();
    arch::pic::Timer::init_timer(1000); // 1 interrupt per ms
    log!("Initialized PIC and Timer");
    arch::interrupts::enable();
    let allocator =
        arch::paging::PageAllocator::new_kernel(511, 510, arch::addressing::KERNEL_BASE);
    mm::ALLOCATOR.lock().init(allocator, 6);
    log!("Initialized heap allocator");
    filesystem::initialize_fs(multiboot);
    log!("Initialized filesystem");
}
