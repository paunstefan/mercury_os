use core::arch::asm;
use core::str::from_utf8_unchecked;

use crate::arch::addressing::VirtAddr;
use crate::arch::paging::PAGE_SIZE;
use crate::arch::registers::Cr3;
use crate::filesystem::VFS_Node;

use crate::{
    arch::{addressing::KERNEL_BASE, paging::PageAllocator},
    filesystem,
};
use alloc::vec::Vec;

pub static mut MULTIPROCESSING: Option<Multiprocessing> = None;

// Hack! Rework this!
static mut PROGRAM_NAME: [u8; 64] = [0; 64];
static mut PROGRAM_NAME_SIZE: usize = 0;
static mut EXITING: bool = false;

extern "C" {
    fn read_rip() -> u64;
}

#[derive(Debug)]
pub struct Task {
    pub id: u64,
    pub registers: Registers,
    pub page_allocator: PageAllocator,
    pub open_fd: Vec<(*mut VFS_Node, u64)>,
}

#[derive(Debug)]
pub struct Registers {
    pub rsp: u64,
    pub rbp: u64,
    pub rip: u64,
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            rsp: 0,
            rbp: 0,
            rip: 0,
        }
    }
}

#[derive(Debug)]
pub struct Multiprocessing {
    pub tasks: Vec<Task>,
    pub current_id: u64,
}

impl Multiprocessing {
    pub fn new() -> Self {
        Multiprocessing {
            tasks: Vec::new(),
            current_id: 0,
        }
    }

    /// Run the initial task
    pub unsafe fn init(&mut self, program_name: &str) {
        // Create a page allocator for the process pages
        let page_allocator = PageAllocator::new_user(KERNEL_BASE);
        let stdin_out = filesystem::fopen("/dev/serial").unwrap() as *mut VFS_Node;
        let mut open_fd = Vec::new();
        open_fd.push((stdin_out, 0));
        let mut task = Task {
            id: self.current_id,
            registers: Registers::new(),
            page_allocator,
            open_fd,
        };

        // Read the executable from the file
        let executable = filesystem::fopen(program_name).unwrap();
        let mut bytes: Vec<u8> = Vec::with_capacity(executable.size);
        bytes.resize(executable.size, 0);
        executable.read(0, executable.size, &mut bytes);

        // Allocate pages for the process
        let _program_mem = task.page_allocator.alloc_next_page(1).unwrap();
        // !! HEAP will start at 0x200000
        let _heap = task.page_allocator.alloc_next_page(4).unwrap();
        let stack = task.page_allocator.alloc_next_page(1).unwrap();
        let stack_end_addr = stack.start_address.0 + PAGE_SIZE - 8;

        // Switch to the process pages
        Cr3::write_raw(task.page_allocator.user_pages_addresses.unwrap()[0].1, 0);
        self.tasks.push(task);

        // Copy program
        core::ptr::copy_nonoverlapping(bytes.as_ptr(), 0 as *mut u8, bytes.len());

        // Set registers and jump
        asm!("cli", "mov rsp, {stack}", "mov rbp, {stack}", "sti", "mov rcx, 0", "jmp rcx", 
                stack = in(reg) stack_end_addr);
    }

    /// Run a new process, pausing the currently running process
    ///
    /// Should only be called from an interrupt context
    /// TODO: this is not working
    pub unsafe fn execute(&mut self, program_name: &str) {
        let rsp: u64;
        let rbp: u64;
        asm!("mov {rsp}, rsp","mov {rbp}, rbp", rsp = out(reg) rsp, rbp = out(reg) rbp, options(nomem, nostack, preserves_flags));

        PROGRAM_NAME[..program_name.len()].copy_from_slice(program_name.as_bytes());
        PROGRAM_NAME_SIZE = program_name.len();

        let rip = read_rip();

        // When the execution returns to the previous process
        if EXITING {
            EXITING = false;
            return;
        }
        EXITING = true;
        self.tasks[self.current_id as usize].registers.rsp = rsp;
        self.tasks[self.current_id as usize].registers.rbp = rbp;
        self.tasks[self.current_id as usize].registers.rip = rip;

        self.current_id += 1;

        // Create a page allocator for the process pages
        let page_allocator = PageAllocator::new_user(KERNEL_BASE);
        let stdin_out = filesystem::fopen("/dev/serial").unwrap() as *mut VFS_Node;
        let mut open_fd = Vec::new();
        open_fd.push((stdin_out, 0));
        let mut task = Task {
            id: self.current_id,
            registers: Registers::new(),
            page_allocator,
            open_fd,
        };

        // Allocate pages for the process
        let _program_mem = task.page_allocator.alloc_next_page(1).unwrap();
        // !! HEAP will start at 0x200000
        let _heap = task.page_allocator.alloc_next_page(4).unwrap();
        let stack = task.page_allocator.alloc_next_page(1).unwrap();
        let stack_end_addr = stack.start_address.0 + PAGE_SIZE - 8;

        self.tasks.push(task);

        // Switch to the process pages
        Cr3::write_raw(
            self.tasks[self.current_id as usize]
                .page_allocator
                .user_pages_addresses
                .unwrap()[0]
                .1,
            0,
        );

        let path = from_utf8_unchecked(&PROGRAM_NAME[..PROGRAM_NAME_SIZE]);
        // Read the executable from the file
        let executable = filesystem::fopen(path).unwrap();
        let mut bytes: Vec<u8> = Vec::with_capacity(executable.size);
        bytes.resize(executable.size, 0);
        executable.read(0, executable.size, &mut bytes);
        // Copy program
        core::ptr::copy_nonoverlapping(bytes.as_ptr(), 0 as *mut u8, bytes.len());

        // Set registers and jump
        asm!("cli", "mov rsp, {stack}", "mov rbp, {stack}", "sti", "mov rcx, 0", "jmp rcx", 
                stack = in(reg) stack_end_addr);
    }

    pub unsafe fn exit(&mut self) {
        let mut task = self.tasks.pop().unwrap();
        self.current_id -= 1;

        for i in 0..6 {
            task.page_allocator.free_vaddr(VirtAddr::new(i * 0x200000));
        }

        // Switch to the process pages
        Cr3::write_raw(
            self.tasks[self.current_id as usize]
                .page_allocator
                .user_pages_addresses
                .unwrap()[0]
                .1,
            0,
        );

        let rsp = self.tasks[self.current_id as usize].registers.rsp;
        let rbp = self.tasks[self.current_id as usize].registers.rbp;
        let rip = self.tasks[self.current_id as usize].registers.rip;

        // Set registers and jump
        asm!("cli", "mov rsp, {rsp}", "mov rbp, {rbp}", "sti", "jmp {rip}", 
                rsp = in(reg) rsp, rbp = in(reg) rbp, rip = in(reg) rip);
    }
}
