use crate::arch::paging::PageAllocator;
use alloc::vec::Vec;

pub struct Task {
    pub id: u64,
    pub registers: Registers,
    pub page_allocator: PageAllocator,
}

pub struct Registers {
    pub esp: u64,
    pub ebp: u64,
    pub eip: u64,
}

pub struct Multiprocessing {
    pub tasks: Vec<Task>,
    pub current_id: u64,
}
