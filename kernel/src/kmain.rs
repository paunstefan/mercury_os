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

use crate::task::{Multiprocessing, MULTIPROCESSING};

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
    log!("Hello world! 1={}", 1);
    // Needed stuff
    let mb_info = unsafe { multiboot::MultibootInfo::read(multiboot_info) };
    log!("{:?}", mb_info);

    init_kernel(mb_info);
    {
        let allocator =
            arch::paging::PageAllocator::new_kernel(511, 510, arch::addressing::KERNEL_BASE);
        mm::ALLOCATOR.lock().init(allocator, 1);
    }
    filesystem::initialize_fs(mb_info);

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
    //let mb_info = unsafe { multiboot::MultibootInfo::read(multiboot_info) };
    // unsafe {
    //     // testing frame allocator

    //     let f1 = arch::paging::GLOBAL_FRAME_ALLOCATOR.alloc_next();
    //     log!("{:?}", f1);
    //     let f1 = arch::paging::GLOBAL_FRAME_ALLOCATOR.alloc_next();
    //     log!("{:?}", f1);
    //     arch::paging::GLOBAL_FRAME_ALLOCATOR.free(f1.unwrap());
    //     let f1 = arch::paging::GLOBAL_FRAME_ALLOCATOR.alloc_next();
    //     log!("{:?}", f1);
    // };

    // {
    //     //test page allocator

    //     log!("{:?}", allocator);
    //     let page1 = allocator.alloc_next_page();
    //     log!(
    //         "{:?} {:?}",
    //         page1,
    //         page1
    //             .unwrap()
    //             .start_address
    //             .translate_address(arch::addressing::KERNEL_BASE)
    //     );
    //     let page2 = allocator.alloc_next_page();
    //     log!(
    //         "{:?} {:?}",
    //         page2,
    //         page2
    //             .unwrap()
    //             .start_address
    //             .translate_address(arch::addressing::KERNEL_BASE)
    //     );
    //     allocator.free_vaddr(page1.unwrap().start_address);
    //     let page = allocator.alloc_next_page();
    //     log!(
    //         "{:?} {:?}",
    //         page,
    //         page.unwrap()
    //             .start_address
    //             .translate_address(arch::addressing::KERNEL_BASE)
    //     );
    // }
    let x = alloc::boxed::Box::new(41);
    log!("{:?}", x);
    let mut v = alloc::vec![0, 1, 2];
    log!("{:?}", v);
    v.push(42);
    log!("{:?}", v);

    // {
    //     // testing address translation
    //     let x = 42;
    //     let x_ptr = &x as *const i32;

    //     let vaddr = VirtAddr::from_ptr(x_ptr);
    //     log!("{:?}", vaddr);
    //     log!("{:?}", translate_virtual_address(vaddr));

    //     let (a, b, c) = (vaddr.p4_index(), vaddr.p3_index(), vaddr.p2_index());
    //     log!("{:x} {:x} {:x}", a, b, c);
    //     let remade = VirtAddr::from_table_indexes(a, b, c);
    //     log!("{:?}", remade);
    // }

    // trigger a page fault
    // unsafe {
    //     *(0xdeadbeef as *mut u64) = 42;
    // };
    log!("Sleeping for 100 mseconds...");
    arch::pic::Timer::sleep(100);
    log!("Good sleep");

    let fs_root = unsafe { &**filesystem::FS_ROOT.as_mut().unwrap() };
    for f in fs_root.readdir().unwrap() {
        log!("{:?}", f);
        if let Some(node) = fs_root.finddir(&f.name) {
            let nod = unsafe { &*node };
            if nod.kind == filesystem::Type::File {
                let mut buf = [0u8; 64];
                let size = nod.read(0, nod.size, &mut buf).unwrap();
                log!("Contents: {:?}", &buf[..size]);
            }
        }
    }
    // let nod = filesystem::fopen("/test_program.bin").unwrap();
    // if nod.kind == filesystem::Type::File {
    //     let mut buf = [0u8; 64];
    //     nod.read(0, nod.size, &mut buf);
    //     log!("Contents: {:?}", buf);
    // }

    unsafe {
        MULTIPROCESSING = Some(Multiprocessing::new());
        MULTIPROCESSING.as_mut().unwrap().init("/test_program.bin");
    }

    log!("Did not crash (yet)");
    hlt_loop()
}

fn init_kernel(multiboot: &'static MultibootInfo) {
    arch::gdt::init_tss();
    log!("Initialized TSS");
    arch::interrupts::init_idt();
    log!("Initialized IDT");
    arch::paging::init_pfa(multiboot);
    log!("Initialized PageFrameAllocator");
    unsafe { arch::pic::PICS.lock().initialize() };
    arch::pic::Timer::init_timer(1000); // 1 interrupt per ms
    log!("Initialized PIC and Timer");
    arch::interrupts::enable();
}
