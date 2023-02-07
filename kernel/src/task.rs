use crate::arch::paging::PageAllocator;

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
